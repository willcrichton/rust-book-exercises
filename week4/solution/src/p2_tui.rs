//! P2: Terminal user interface
//!
//! This problem explores the differences between designing systems with
//! classes and traits. The adjacent file `tui.cpp` provides a C++ implementation
//! of a terminal user inteface (TUI), i.e. a simple set of graphical elements that
//! can be drawn into the terminal. The C++ code uses classes, inheritance, and
//! virtual methods.
//!
//! To see the C++ code in action, you can build and execute the code by running:
//!
//! ```bash
//! ./run.cpp-sh
//! ```
//!
//! Your task is to redesign the C++ TUI API into Rust. Your API should similarly
//! contain data structures that represent Text, Heading, and Container. You should
//! replicate the behavior of `main` in `tui.cpp` into the `container_test` function.
//!
//! Note: Cargo's test harness silences printing by default. You can prevent that
//! behavior by running:
//!
//! ```bash
//! cargo test container -- --nocapture
//! ```

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

pub trait Element {
    fn dimensions(&self) -> Dimensions;
    fn render(&self);
}

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text { text }
    }
}

impl Element for Text {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.text.len(),
            height: 1,
        }
    }

    fn render(&self) {
        print!("{}", self.text);
    }
}

pub struct Heading {
    text: Text,
}

impl Heading {
    pub fn new(text: String) -> Self {
        Heading {
            text: Text::new(text),
        }
    }
}

impl Element for Heading {
    fn dimensions(&self) -> Dimensions {
        self.text.dimensions()
    }

    fn render(&self) {
        print!("\u{001b}[1m");
        self.text.render();
        print!("\u{001b}[0m")
    }
}

pub struct Container {
    children: Vec<Box<dyn Element>>,
}

impl Container {
    pub fn new(children: Vec<Box<dyn Element>>) -> Self {
        Container { children }
    }
}

impl Element for Container {
    fn dimensions(&self) -> Dimensions {
        let child_dims = self
            .children
            .iter()
            .map(|c| c.dimensions())
            .collect::<Vec<_>>();
        let width = child_dims.iter().map(|dims| dims.width).max().unwrap_or(0) + 2;
        let height = child_dims.iter().map(|dims| dims.height).sum::<usize>();
        Dimensions { width, height }
    }

    fn render(&self) {
        let dims = self.dimensions();
        let render_line = || {
            println!("+{}+", "-".repeat(dims.width - 2));
        };
        render_line();

        for child in &self.children {
            let child_dims = child.dimensions();
            print!("|");
            child.render();
            println!("{}|", " ".repeat(dims.width - 2 - child_dims.width))
        }

        render_line();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn container_test() {
        let text = Heading::new("Hello world".into());
        let text2 = Text::new("This is a long string of text".into());
        let container = Container::new(vec![Box::new(text), Box::new(text2)]);
        container.render();
    }
}
