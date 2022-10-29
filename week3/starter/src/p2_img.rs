//! NOTE: these instructions use rich text, so you should read them as a webpage.
//! Run `cargo doc --no-deps --open` and then navigate to the `p2_img` page.
//!
//! # P2: Optimizing image algorithms
//!
//! This file provides an `Image` data structure for representing grayscale 2D images.
//! This file also provides an implementation of a seam carving algorithm. Seam carving
//! is a form of content-aware resizing, i.e. it lets you make an image smaller without
//! distorting or cropping important content. I would recommend reading the "Process" section
//! of the Wikipedia article to understand the basics of the algorithm:
//!
//! <https://en.wikipedia.org/wiki/Seam_carving>
//!
//! You can see the algorithm in action by running it on one of the provided images:
//!
//! ```bash
//! cargo run --bin p2_img_bin --release -- castle.jpg 50
//! open output.jpg
//! ```
//!
//! However, the implementation provided is very slow for large images. You can see this
//! by running it on a 4k image:
//!
//! ```bash
//! cargo run --bin p2_img_bin --release -- landscape.jpg 5
//! ```
//!
//! In this problem, you will optimize the performance of the seam carving algorithm,
//! and thereby learn about the performance characteristics of Rust's data structures.
//!
//!
//! ## P2a: Read a profile
//!
//! Performance optimization always starts with profiling! So your first task is to generate a
//! profile of this algorithm, and then figure out which functions are taking the longest.
//!
//! Profiling tools are different for each operating system. I'll explain how to profile on Mac
//! and Linux. If you're on Windows, I would use the Linux instructions via WSL.
//!
//! ### Mac
//! You're going to use the Instruments application. First, download Xcode if you haven't already.
//! Then run:
//!
//! ```bash
//! cargo install cargo-instruments
//! ```
//!
//! Then generate a profile by running:
//!
//! ```bash
//! cargo instruments --release --bin p2_img_bin -t time -- vaporwave.jpeg 50
//! ```
//!
//! This will open an Instruments window. On the bottom right it will give a view on the "Heaviest Stack Trace".
//! Click the entry that says `week3::p2_img::Image::carve`, and start exploring!
//!
//! ### Linux
//! You're going to use a Flamegraph. First, follow the installation instructions on the `flamegraph` crate:
//! <https://github.com/flamegraph-rs/flamegraph#installation>
//!
//! Then generate a flamegraph by running:
//!
//! ```bash
//! cargo flamegraph --bin p2_img_bin -- vaporwave.jpeg 50
//! ```
//!
//! This will generate a file `flamegraph.svg`. Open that file in your browser, and start exploring!
//!
//! ## P2b: Optimize the code
//!
//! Next, your goal is to use your insights from the profile to optimize the algorithm's implementation.
//! The reference solution is between 30-60x faster than the starter code, tested on an M1 Macbook Pro and
//! a Google Cloud x86 VM.
//!
//! You can optimize the code however you want. I recommend starting with the most expensive functions identified in the profile. Some considerations:
//! * **Redundant computation:** are any values being computed more often than they need to be? Can you just compute them once?
//! * **Unnecessary allocation:** is memory being allocated repeatedly? Could you allocate that memory up-front, or not at all?
//!   Note that many Rust data structures have constructor functions that specify an initial capacity.
//! * **Abstraction tax:** is the compiler missing optimization opportunities by the use of abstractions?
//!   Can you write more specialized code to reveal those opportunities?
//!
//! To statistically test whether your optimizations actually impact performance, you can use the provided benchmark suite by running:
//!
//! ```bash
//! cargo bench
//! ```
//!
//! The three provided images (`castle`, `vaporwave`, `landscape`) are used to benchmark seam carving on images of increasing resolution.
//! These benchmarks use the [Criterion](https://bheisler.github.io/criterion.rs/book/index.html) library to repeatedly sample
//! a function's performance, and then compute statistics on whether its performance changes.
//!
//! Note that some benchmarks will take a long time to run initially --- you can run a specificy benchmark by passing the `BENCH`
//! environment variable, e.g.
//!
//! ```bash
//! BENCH=castle cargo bench
//! ```

use image::EncodableLayout;
use std::{collections::HashMap, path::Path};

#[derive(Clone)]
pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

/// Data structure to hold energies.
type Energies = HashMap<(usize, usize), usize>;

impl Image {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: u8) {
        self.pixels[x + y * self.width] = pixel;
    }

    pub fn offset(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let x2 = (x as isize) + dx;
        let y2 = (y as isize) + dy;
        (x2 >= 0 && (x2 as usize) < self.width && y2 >= 0 && (y2 as usize) < self.height)
            .then_some((x2 as usize, y2 as usize))
    }

    /// Generates the initial mapping from pixels to energy. The initial energy of a pixel
    /// is the average difference of the pixel versus its neighbors.
    pub fn compute_initial_energy(&self) -> Energies {
        let mut energies = HashMap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut diffs = Vec::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if let Some((x2, y2)) = self.offset(x, y, dx, dy) {
                            diffs.push(self.get(x, y).abs_diff(self.get(x2, y2)) as usize);
                        }
                    }
                }
                energies.insert((x, y), diffs.iter().sum::<usize>() / diffs.len());
            }
        }

        energies
    }

    pub fn propagate_energy(&self, energies: &mut Energies) {
        for y in 0..self.height {
            for x in 0..self.width {
                let emin = (-1..=1)
                    .filter_map(|dx| self.offset(x, y, dx, -1))
                    .map(|(x, y)| energies[&(x, y)])
                    .min()
                    .unwrap_or(0);
                *energies.get_mut(&(x, y)).unwrap() += emin;
            }
        }
    }

    pub fn find_seam(&self, energies: &Energies) -> Vec<usize> {
        let (y_seed, _) = (0..self.width)
            .map(|x| (x, energies[&(x, self.height - 1)]))
            .min_by_key(|(_, e)| *e)
            .unwrap();
        let mut min_seam = vec![y_seed];
        for y in 0..(self.height - 1) {
            let (x, _) = (-1..=1)
                .filter_map(|dx| self.offset(min_seam[y], self.height - y - 1, dx, -1))
                .map(|(x2, y2)| (x2, energies[&(x2, y2)]))
                .min_by_key(|(_, e)| *e)
                .unwrap();
            min_seam.push(x);
        }
        min_seam
    }


    /// Takes a vertical seam as a vector [x_1, ... x_n] of x-values,
    /// and removes it from the image.
    pub fn remove_seam(&self, seam: &[usize]) -> Image {
        let mut pixels = vec![0; (self.width - 1) * self.height];
        for y in 0..self.height {
            let x_r = seam[y];
            let src_row = &self.pixels[y * self.width..(y + 1) * self.width];
            let dst_row = &mut pixels[y * (self.width - 1)..(y + 1) * (self.width - 1)];
            dst_row[0..x_r].copy_from_slice(&src_row[0..x_r]);
            dst_row[x_r..].copy_from_slice(&src_row[x_r + 1..]);
        }

        Image {
            width: self.width - 1,
            height: self.height,
            pixels,
        }
    }

    /// Carves out the single lowest-energy seam from an image.
    ///
    /// Returns a new image with the seam removed.
    pub fn carve(&self) -> Self {
        let mut energies = self.compute_initial_energy();
        self.propagate_energy(&mut energies);
        let min_seam = self.find_seam(&energies);
        self.remove_seam(&min_seam)
    }

    pub fn load(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let path = path.as_ref();
        let img = image::io::Reader::open(path)?.decode()?;
        let img = image::imageops::colorops::grayscale(&img);
        Ok(Image {
            width: img.width() as usize,
            height: img.height() as usize,
            pixels: img.as_bytes().to_vec(),
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let path = path.as_ref();
        let mut img = image::GrayImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                img.put_pixel(x as u32, y as u32, image::Luma([self.get(x, y)]))
            }
        }
        img.save(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn img_test() {
        let mut img = Image::load("input.jpg").unwrap();

        for _ in 0..50 {
            img = img.carve();
        }

        img.save("output.jpg").unwrap();
    }
}
