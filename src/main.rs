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

            create_file(&path_base,"config","[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n\tlogallrefupdates = true\n\tignorecase = true\n\tprecomposeunicode = true\n");
            create_file(
                &path_base,
                "description",
                "Unnamed repository; edit this file 'description' to name the repository.\n",
            );

            create_file(&path_base,"info/exclude","# git ls-files --others --exclude-from=.git/info/exclude\n# Lines that start with '#' are comments.\n# For a project mostly in C, the following would be a good set of\n# exclude patterns (uncomment them if you want to use them):\n# *.[oa]\n# *~\n");
            create_file(&path_base, "HEAD", "ref: refs/heads/master");
        }
    }
}

fn create_file(path: &PathBuf, name: &str, content: &str) {
    let mut p = PathBuf::from(path);
    p.push(name);

    let mut ofile = File::create(&p).expect("unable to create file");
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
