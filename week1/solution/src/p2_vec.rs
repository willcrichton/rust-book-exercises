//! An important performance optimization for array calculations is the use of vector instructions,
//! i.e. SIMD (https://en.wikipedia.org/wiki/Single_instruction,_multiple_data). Vector instructions
//! process entire chunks of data, rather than a single piece of data.
//!
//! The two key concepts are vectors (here, arrays of four f64s) and masks (arrays of four booleans).
//! A vector contains the data, and the masks are used to implement control-flow without branching.

use criterion::black_box;

pub type Vec4 = [f64; 4];
pub type Mask4 = [bool; 4];

// Problem 2a: implement the following Vec4 library functions.

/// Adds two vectors together point-wise, i.e. a[i] + b[i]
///
/// Run `cargo test vec4_add` to check your answer.
pub fn vec4_add(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// Multiplies two vectors together point-wise
///
/// Run `cargo test vec4_mul` to check your answer.
pub fn vec4_mul(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// Returns a vector v where v[i] = vtrue[i] if mask[i] is true, else v[i] = vfalse[i]
///
/// Run `cargo test vec4_select` to check your answer.
pub fn vec4_select(mask: Mask4, vtrue: Vec4, vfalse: Vec4) -> Vec4 {
    [
        if mask[0] { vtrue[0] } else { vfalse[0] },
        if mask[1] { vtrue[1] } else { vfalse[1] },
        if mask[2] { vtrue[2] } else { vfalse[2] },
        if mask[3] { vtrue[3] } else { vfalse[3] },
    ]
}

/// Returns a mask of whether a[i] > b[i]
///
/// Run `cargo test vec4_gt` to check your answer.
pub fn vec4_gt(a: Vec4, b: Vec4) -> Mask4 {
    [a[0] > b[0], a[1] > b[1], a[2] > b[2], a[3] > b[3]]
}

/// Baseline computation written in traditional iterative style.
pub fn baseline(a: Vec4, b: Vec4) -> Vec4 {
    let mut c = [0.; 4];
    for i in 0..4 {
        // black_box so the compiler doesn't auto-vectorize this loop :-)
        if black_box(a[i] > b[i]) {
            c[i] = a[i] * b[i]
        } else {
            c[i] = a[i] + b[i];
        }
    }
    return c;
}

/// Problem 2b: write a vectorized version of the baseline computation.
///
/// It should only use the Vec4 functions you implemented before.
///
/// Run `cargo bench` and you should see a speedup versus the baseline!
/// I get a ~2x speedup on my 2021 Macbook Pro.
///
/// Run `cargo test vectorized` to check your answer.
pub fn vectorized(a: Vec4, b: Vec4) -> Vec4 {
    vec4_select(vec4_gt(a, b), vec4_mul(a, b), vec4_add(a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    const A: Vec4 = [0., 1., 2., 3.];
    const B: Vec4 = [4., 3., 2., 1.];
    const M: Mask4 = [false, false, true, true];

    #[test]
    fn test_vec4_add() {
        assert_eq!(vec4_add(A, B), [4., 4., 4., 4.])
    }

    #[test]
    fn test_vec4_mul() {
        assert_eq!(vec4_mul(A, B), [0., 3., 4., 3.]);
    }

    #[test]
    fn test_vec4_select() {
        assert_eq!(vec4_select(M, A, B), [4., 3., 2., 3.]);
    }

    #[test]
    fn test_vec4_gt() {
        assert_eq!(vec4_gt(A, B), [false, false, false, true]);
    }

    #[test]
    fn test_vectorized() {
        assert_eq!(vectorized(A, B), baseline(A, B));
    }
}
