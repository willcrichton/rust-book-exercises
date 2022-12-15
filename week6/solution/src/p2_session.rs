//! P2: Session-typed channels
//!
//! Session types are a technique for describing a communication protocol between two parties.
//! For example, this is a session that describes sending a number, receiving a number, and
//! ending the connection:
//!
//!     S = send i32; recv i32; ε
//!
//! The "dual" of a session is the corresponding protocol for the opposite side of the connection:
//!
//!     dual(S) = recv i32; send i32; ε
//!
//! Below is an API that implements session-typed channels. A user can describe a session type
//! using the [`Recv`], [`Send`], and [`Close`] structures, e.g. `Send<i32, Recv<i32, Close>>`. The
//! trait [`HasDual`] can compute the dual of any session type.
//!
//! A session-typed channel [`Chan<S>`] can only send messages allowed by the type of `S`.
//! For example, a channel `c` of type `Chan<Send<usize, Close>>` only allows `c.send(n)` to be
//! called. Moreover, each method consumes ownership of `c`, and returns a new channel with an
//! updated session type. See the function `send_recv_test` for a complete example of how to use
//! the API.
//!
//! Your task is to implement two new constructs: "offer" and "choose". An offer lets the opposing
//! party select one of two branches to follow, and a choose picks a branch. The function
//! `incr_server_test` shows an example of how to use a session offer and choose. You should design
//! and implement the API such that `incr_server_test` can compile and pass.
//!
//! **Note:** to run `incr_server_test`, you have to explicitly enable it by running
//! `cargo test --all-features`.
//!
//! **Note:** in order to send arbitrary data types through a channel, the [`Chan`] internally uses
//! the [`Any`] trait. Take a look at the `Any` docs for information on how to use it:
//! <https://doc.rust-lang.org/std/any/index.html#examples>

use std::any::Any;
use std::marker::{self, PhantomData};
use std::sync::mpsc;

/// Receive a message of type `T`, then change the session to `S`.
// Note: the `PhantomData` type is needed because Rust will complain if a structure has
// unused type parameters.
pub struct Recv<T, S>(PhantomData<(T, S)>);

/// Send a message of type `T`, then change the session to `S`.
pub struct Send<T, S>(PhantomData<(T, S)>);

/// Close the session
pub struct Close;

pub struct Choose<S1, S2>(PhantomData<(S1, S2)>);
pub struct Offer<S1, S2>(PhantomData<(S1, S2)>);

/// Compute the dual of a session type.
pub trait HasDual {
    type Dual;
}

impl HasDual for Close {
    type Dual = Close;
}

impl<T, S: HasDual> HasDual for Recv<T, S> {
    type Dual = Send<T, S::Dual>;
}

impl<T, S: HasDual> HasDual for Send<T, S> {
    type Dual = Recv<T, S::Dual>;
}

impl<S1: HasDual, S2: HasDual> HasDual for Choose<S1, S2> {
    type Dual = Offer<S1::Dual, S2::Dual>;
}

impl<S1: HasDual, S2: HasDual> HasDual for Offer<S1, S2> {
    type Dual = Choose<S1::Dual, S2::Dual>;
}

pub struct Chan<S> {
    sender: mpsc::Sender<Box<dyn Any + marker::Send + 'static>>,
    receiver: mpsc::Receiver<Box<dyn Any + marker::Send + 'static>>,
    _marker: PhantomData<S>,
}

impl<S: HasDual> Chan<S> {
    pub fn both() -> (Chan<S>, Chan<S::Dual>) {
        let (server_sender, client_receiver) = mpsc::channel();
        let (client_sender, server_receiver) = mpsc::channel();
        (
            Chan {
                sender: server_sender,
                receiver: server_receiver,
                _marker: PhantomData,
            },
            Chan {
                sender: client_sender,
                receiver: client_receiver,
                _marker: PhantomData,
            },
        )
    }
}

impl Chan<Close> {
    pub fn close(self) {}
}

macro_rules! cast_channel {
    ($self:expr) => {
        Chan {
            sender: $self.sender,
            receiver: $self.receiver,
            _marker: PhantomData,
        }
    };
}

impl<T: marker::Send + 'static, S> Chan<Send<T, S>> {
    pub fn send(self, t: T) -> Chan<S> {
        self.sender.send(Box::new(t)).unwrap();
        cast_channel!(self)
    }
}

impl<T: 'static, S> Chan<Recv<T, S>> {
    pub fn recv(self) -> (Chan<S>, T) {
        let t = *self.receiver.recv().unwrap().downcast::<T>().unwrap();
        let c = cast_channel!(self);
        (c, t)
    }
}

impl<S1, S2> Chan<Choose<S1, S2>> {
    pub fn choose_left(self) -> Chan<S1> {
        self.sender.send(Box::new(false)).unwrap();
        cast_channel!(self)
    }

    pub fn choose_right(self) -> Chan<S1> {
        self.sender.send(Box::new(true)).unwrap();
        cast_channel!(self)
    }
}

pub enum Branch<S1, S2> {
    Left(Chan<S1>),
    Right(Chan<S2>),
}

impl<S1, S2> Chan<Offer<S1, S2>> {
    pub fn offer(self) -> Branch<S1, S2> {
        let right = self.receiver.recv().unwrap().downcast::<bool>().unwrap();
        if *right {
            Branch::Right(cast_channel!(self))
        } else {
            Branch::Left(cast_channel!(self))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn send_recv_test() {
        type Session = Send<i32, Recv<i32, Close>>;
        let (server, client) = Chan::<Session>::both();

        let server = server.send(42);
        let (client, n) = client.recv();
        assert_eq!(n, 42);

        let client = client.send(-42);
        let (server, n2) = server.recv();
        assert_eq!(n2, -42);

        server.close();
        client.close();
    }

    #[test]
    #[cfg(feature = "incr-server-test")]
    fn incr_server_test() {
        use std::thread;

        type Session = Offer<Recv<usize, Send<usize, Close>>, Close>;
        let (server, client) = Chan::<Session>::both();

        let server = thread::spawn(move || match server.offer() {
            Branch::Left(c) => {
                let (c, n) = c.recv();
                let c = c.send(n + 1);
                c.close()
            }
            Branch::Right(c) => c.close(),
        });

        let client = thread::spawn(move || {
            let c = client.choose_left();
            let c = c.send(1);
            let (c, n) = c.recv();
            assert_eq!(n, 2);
            c.close();
        });

        server.join().unwrap();
        client.join().unwrap();
    }
}
