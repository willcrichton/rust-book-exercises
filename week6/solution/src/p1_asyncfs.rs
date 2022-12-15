// As before, you may find these instructions easier to read in Rustdoc. You can generate
// the docs by running `cargo doc --no-deps --open` and navigating to this page.

//! P1: Async file read
//!
//! Rust supports async/await-style programming through the [`Future`] trait. In this problem, you
//! will implement a future for asynchronously reading a file. See the `read_test` function below
//! for an example of how such a usage of the future would look.
//!
//! The basic strategy is like this: given a `file` of type [`File`], then `file.read_async()` will return
//! a data structure [`ReadFile`] that represents a future, i.e. at some point it will return the bytes
//! that are read. The [`ReadFile`] future should launch a system thread which reads the file into a buffer.
//! When the thread is done, then [`Future::poll`] should return [`Poll::Ready`].
//!
//! Note that the future is responsible for "waking" itself once the thread has completed. For an example of
//! how to do this, see the Rust Async Book: <https://rust-lang.github.io/async-book/02_execution/03_wakeups.html>
//!
//! Your task is to implement the [`ReadFile`] data type and methods, specifically [`AsyncFile::read_async`] and
//! [`Future::poll`]. You can run `cargo test read` to check your solution.
//!
//! Beware: your design MUST not allow the promise to live longer than the `File` that it holds! You can double
//! check this is true by uncommenting `read_bad_scope_test` below, and ensuring it does not compile.

use std::{
    fs::File,
    future::Future,
    io,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use std::{
    io::Read,
    mem,
    sync::{Arc, Mutex},
    task::Waker,
    thread::{self, JoinHandle},
};

/// Extension trait for asynchronous methods on [`File`].
pub trait AsyncFile {
    /// The type of the future returned by `read_async`.
    type ReadFuture<'a>: Future<Output = io::Result<Vec<u8>>>
    where
        Self: 'a;

    /// Asynchronously reads all of a file's contents into a buffer.
    fn read_async<'a>(&'a mut self) -> Self::ReadFuture<'a>;
}

/// The file reading future.
pub struct ReadFile<'a> {
    waker: Arc<Mutex<Option<Waker>>>,
    handle: Option<JoinHandle<io::Result<Vec<u8>>>>,
    _marker: PhantomData<&'a ()>,
}

// This impl constructs the future when the user calls `file.read_async()`.
impl AsyncFile for File {
    type ReadFuture<'a> = ReadFile<'a>;

    fn read_async<'a>(&'a mut self) -> ReadFile<'a> {
        let file = unsafe { mem::transmute::<&'a mut File, &'static mut File>(self) };
        let waker: Arc<Mutex<Option<Waker>>> = Arc::new(Mutex::new(None));
        let waker_ref = Arc::clone(&waker);
        let handle = thread::spawn(move || {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            if let Some(waker) = waker_ref.lock().unwrap().take() {
                waker.wake();
            }
            Ok(buf)
        });
        ReadFile {
            waker,
            handle: Some(handle),
            _marker: PhantomData,
        }
    }
}

// This impl polls the future for completion, returning the value inside if it's ready.
impl<'a> Future for ReadFile<'a> {
    type Output = io::Result<Vec<u8>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.handle.as_ref().unwrap().is_finished() {
            Poll::Ready(self.handle.take().unwrap().join().unwrap())
        } else {
            *self.waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn read_test() {
        let path = std::env::temp_dir().join("foo.txt");
        let contents = "hello world";
        fs::write(&path, contents).unwrap();
        let mut file = File::open(&path).unwrap();
        let buf = file.read_async().await.unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), contents);
    }

    // #[tokio::test]
    // async fn read_bad_scope_test() {
    //   fs::write("foo.txt", "hello world").unwrap();
    //   let future = {
    //     let mut file = File::open("foo.txt").unwrap();
    //     file.read_async()
    //   };
    //   let buf = future.await.unwrap();
    //   assert_eq!(String::from_utf8(buf).unwrap(), "hello world");
    // }
}
