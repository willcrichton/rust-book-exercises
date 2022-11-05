use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let path = args
        .next()
        .expect("You have to provide a path to an image to process");
    let iters = match args.next() {
        Some(n) => n
            .parse::<usize>()
            .expect("Number of iterations must be a numeral string"),
        None => 50,
    };

    let mut img = week3::p2_img::Image::load(path).unwrap();

    for i in 0..iters {
        println!("Iteration {i}");
        img = img.carve();
    }

    img.save("output.jpg").unwrap();
    println!("Done!");
}
