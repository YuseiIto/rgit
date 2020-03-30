use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let output = "Hello, world!";
        let mut ofile = File::create("hello_world.txt").expect("unable to create file");
        ofile.write_all(output.as_bytes()).expect("unable to write");
        ofile.flush().expect("Flush failed.")
    }
}
