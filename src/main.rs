use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
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

        path_base.push(".git");
        fs::create_dir_all(&path_base).expect("Failed to create directory");

        if cmd == &String::from("init") {
            //Init command runned
            let dirs = vec![
                "hooks",
                "info",
                "objects",
                "objects/info",
                "objects/pack",
                "refs",
                "refs/heads",
                "refs/tags",
            ];
            dig_dirs(&path_base, &dirs);

            let mut hw_path = PathBuf::from(&path_base);
            hw_path.push("HelloWorld.txt");
            let output = "Hello, world!";
            create_file(&hw_path, &output);
        }
    }
}

fn create_file(path: &PathBuf, content: &str) {
    let mut ofile = File::create(path).expect("unable to create file");
    ofile
        .write_all(content.as_bytes())
        .expect("unable to write");
    ofile.flush().expect("Flush failed.");
}

fn dig_dirs(path_base: &PathBuf, dirs: &Vec<&str>) {
    for elm in dirs {
        let mut p = PathBuf::from(path_base);
        p.push(elm);
        fs::create_dir_all(&p).expect("Dig direcroty failed")
    }
}
