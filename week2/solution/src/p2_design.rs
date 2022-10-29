//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
  // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
  //     we do not need to change the size or order of the collection, but do need to change the elements.
  // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
  // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
  //     function that works for both types, so we arbitrarily pick f32 for now.
  v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
  for n in v.iter_mut() {
    *n = n.round();
  }
}

#[test]
fn round_all_test() {
  let mut v = vec![0.3, 0.7];
  round_all(&mut v);
  assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains(
  // haystack could be either a &Vec<_>, Vec<_>, or &[_]. A slice is preferred since we do not mutate the inputs.
  // haystack could either be a &[String] or &[&str]. It is probably more common to have collections of owned strings
  //    than string references, so &[String] is preferable.
  haystack: &[String],
  // needle could be either a String, &String, or &str. Again we do not mutate the needle nor need to consume ownership,
  //   so a string slice &str is preferred.
  needle: &str,
) -> Vec<usize> // could be either a Vec<usize>, a Vec<&str>, or a Vec<bool>. usize has the disadvantage that it could become
                // out of sync with the original haystack, and the advantage that it doesn't borrow the haystack. usize is
                // more compact than a bool vector for sparse outputs.
{
  haystack
    .iter()
    .enumerate()
    .filter(|(_, s)| s.contains(needle))
    .map(|(i, _)| i)
    .collect()
}

#[test]
fn find_contains_test() {
  let haystack = vec![String::from("hello"), String::from("world")];
  let needle = "w";
  assert_eq!(find_contains(&haystack, needle), vec![1]);
}

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(
  // buf could only be an &mut String. If buf is &String, this function cannot mutate it.
  // If buf is String, then the caller cannot use the buffer, or we would have to return a new
  // buffer, which isn't idiomatic for buffer APIs.
  buf: &mut String,
  // delims could be a Vec<_> or a tuple (_, _). Because we always have two delimters, a fixed-size
  // tuple is sensible. The components could either be char or &str, we arbitrarily choose char here.
  // Either way, the type will be copyable, so we should take an owned tuple and not a reference.
  delims: (char, char),
  // frac should be a floating point number, we arbitrarily pick f32 here. Floats are also copyable,
  // so we take an owned f32 rather than a reference.
  frac: f32,
)
// No return type needed
{
  let n = (frac * 10.).round().clamp(0., 10.) as usize;
  buf.push(delims.0);
  for _ in 0..n {
    buf.push('=');
  }
  for _ in 0..(10 - n) {
    buf.push(' ');
  }
  buf.push(delims.1);
}

#[test]
fn test_fill_progress_bar() {
  let mut buf = String::new();
  fill_progress_bar(&mut buf, ('[', ']'), 0.2);
  assert_eq!(buf, "[==        ]");
}
