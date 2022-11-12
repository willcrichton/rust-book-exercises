//! P2: Memoizing functions
//!
//! Memoization is the process of caching the outputs of a function so they can be
//! retrieved later without re-computation. In this problem, you will design an
//! ergonomic and memory-safe memoization data structure.
//!
//! We have provided you a basic skeleton for the data structure, and a set of unit tests
//! that define the specification. `memo = Memo::new(f)` should produce a memoized version of
//! a function `f` such that if `f(input)` is `output`, then `memo.call(input)` is `&output`.
//! That is, the Memo structure returns immutable references to cached outputs.
//!
//! Your task is to design the cache and implement the `new` and `call` functions to pass
//! the unit tests. Some considerations:
//!
//! * While `Memo::call` may mutate its interior state, a user of `Memo` should not have to
//!   care about that mutability -- `Memo::call` should work even if `Memo` is outwardly immutable.
//!   Consider a `RefCell`!
//! 
//! * If you store a collection of outputs in the cache, remember that mutations to collections
//!   can cause them to reallocate (e.g. `Vec::push`, `HashMap::insert`, etc.). So any *direct*
//!   references to collection items will be invalidated. Take a look at the Pin data structure:
//!   <https://doc.rust-lang.org/std/pin/index.html>
//! 
//! * You may need to convince Rust that a variable lives longer than its given lifetime.
//!   You can use the unsafe operation [`mem::transmute`](https://doc.rust-lang.org/std/mem/fn.transmute.html)
//!   for this. But beware! Only use `transmute` with the utmost caution!
//! 
//! Note that there is a unit test `memo_scope_test` that is commented out. It *should not compile*.
//! So you can try commenting it in, and verifying that you get a compiler error.


pub struct Memo<Func, Input, Output> {
    func: Func,
    cache: () // TODO
}

impl<Func, Input, Output> Memo<Func, Input, Output>
where
    Func: Fn(Input) -> Output,
{
    pub fn new(func: Func) -> Self {
        Memo {
            func,
            cache: () // TODO
        }
    }


    pub fn call() {} // TODO
}
/* END SOLUTION */

#[cfg(test)]
mod test {
    use super::*;

    fn make_bar(n: usize) -> String {
        "-".repeat(n)
    }

    #[test]
    fn memo_basic_test() {
        let make_bar = Memo::new(make_bar);
        let b1 = make_bar.call(2);
        assert_eq!(b1, "--");

        let b2 = make_bar.call(5);
        assert_eq!(b2, "-----");

        let b3 = make_bar.call(5);
        assert!(!std::ptr::eq(b1, b2));
        assert!(std::ptr::eq(b2, b3));
    }

    #[test]
    fn memo_ref_stability_test() {
        let make_bar = Memo::new(make_bar);
        let b1 = make_bar.call(1);
        assert_eq!(b1, "-");

        // If you get a segfault, that means b1 is being invalidated
        // after memoizing a bunch more data!
        for i in 2..1000 {
            make_bar.call(i);
        }

        assert_eq!(b1, "-");
    }

    #[test]
    fn memo_copyable_test() {
        let make_bar = Memo::new(make_bar);
        let make_bar2 = &make_bar;
        let b1 = make_bar.call(1);
        let b2 = make_bar2.call(1);
        assert!(std::ptr::eq(b1, b2))
    }

    // #[test]
    // fn memo_scope_test() {
    //     let b = {
    //         let make_bar = Memo::new(make_bar);
    //         make_bar.call(3)
    //     };
    //     println!("{b}");
    // }
}
