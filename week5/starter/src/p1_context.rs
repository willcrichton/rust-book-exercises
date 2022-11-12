//! P1: Context variables
//!
//! A context is a dynamically scoped variable. It allows a user to define a variable
//! at an outer scope and access it at an inner scope, but without explicitly passing
//! the variable through each scope.
//!
//! Your task is to implement a data structure `Context<T>` that represents a context
//! variable. `Context` has three methods:
//!
//! * `new()`: creates a new context with no value. This should be called at the global scope.
//!
//! * `set(t)`: changes the context to hold the value `t`.
//!   The context should reset to its previous value after the syntactic scope of `set` ends.
//!   This would be a good place to use the `Drop` trait!
//!
//! * `get()`: retrieves the latest value of the context, if it exists.
//!
//! See `context_test` for an example of the expected behavior of each function. You should
//! define the type signature and implementation of each function.
//!
//! To simplify your implementation, you get to assume `T: Copy`. Note that to make using `Context`
//! thread-safe, if you need to use interior mutability, you should use a 
//! [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) instead of a `RefCell`.


pub struct Context<T> {
    values: () // TODO
}

impl<T: Copy> Context<T> {
    pub fn new() {} // TODO

    pub fn set() {} // TODO

    pub fn get() {} // TODO
}


#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CTX: Context<usize> = Context::new();
    }

    #[test]
    fn context_test() {
        assert_eq!(CTX.get(), None);

        let _g = CTX.set(0);
        assert_eq!(CTX.get(), Some(0));

        fn inner() {
            assert_eq!(CTX.get(), Some(0));

            let _g = CTX.set(1);
            assert_eq!(CTX.get(), Some(1));
        }
        inner();

        assert_eq!(CTX.get(), Some(0));
    }
}
