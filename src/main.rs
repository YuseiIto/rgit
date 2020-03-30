use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
extern crate dirs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd = match args.get(1) {
            Some(v) => v,
            None => panic!("No command provided"),
        };

        let home_path = match dirs::home_dir() {
            Some(v) => v,
            None => panic!("Failed to get home path"),
        };

        let mut path_base = PathBuf::from(&home_path);

        //NOTE: will be command-line argument
        path_base.push("desktop");
        path_base.push("tmp");
        path_base.push("rgit");
        //---

        fs::create_dir(&path_base).expect("Failed to create directory");

        let mut hw_path = PathBuf::from(&path_base);
        hw_path.push("HelloWorld.txt");

        if cmd == &String::from("init") {
            //Init command runned
            let output = "Hello, world!";
            let mut ofile = File::create(&hw_path).expect("unable to create file");
            ofile.write_all(output.as_bytes()).expect("unable to write");
            ofile.flush().expect("Flush failed.")
        }
    }
}
