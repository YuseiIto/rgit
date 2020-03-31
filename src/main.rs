use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
extern crate chrono;
extern crate dirs;
extern crate sha1;
use deflate::{deflate_bytes_zlib_conf, Compression};
use std::os::macos::fs::MetadataExt;

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
        } else if cmd == &String::from("add") {
            let mut p = PathBuf::from(&path_base);
            p.pop();

            let name = match args.get(2) {
                Some(v) => v,
                None => panic!("No file specified"),
            };

            p.push(name);
            let meta = fs::metadata(&p).expect("Failed to get metadata");
            let size = meta.len();

            let mut head = String::from("blob ");
            head.push_str(&size.to_string());
            head.push('\0');

            let head = head.into_bytes();

            let mut buf = Vec::new();
            buf.extend(head.iter().cloned());

            let data = read_bytes(&p);
            buf.extend(data.iter().cloned());

            let mut hasher = sha1::Sha1::new();
            hasher.update(&buf);
            let hash = hasher.digest().to_string();

            let dir_name = match hash.get(0..2) {
                Some(v) => v,
                None => panic!("Cannot get dir_name"),
            };

            let file_name = match hash.get(2..(hash.len())) {
                Some(v) => v,
                None => panic!("Cannot get file_name"),
            };

            let mut obj_path = PathBuf::from(&path_base);
            obj_path.push("objects");

            dig_dirs(&obj_path, &vec![dir_name]);

            obj_path.push(&dir_name);
            obj_path.push(file_name);

            let compressed = deflate_bytes_zlib_conf(&buf, Compression::Fast);
            fs::write(&obj_path, &compressed).unwrap();

            //Create index

            let mut buf = Vec::new();
            let mut signature = String::from("DIRC").into_bytes();
            buf.append(&mut signature);
            buf.append(&mut vec![0, 0, 0, 2]);
            buf.append(&mut vec![0, 0, 0, 1]); //TODO: Make this dynamic(Entry count)

            //Create entry
            let dt: chrono::DateTime<chrono::Local> = chrono::Local::now();
            let timestamp: u64 = dt.timestamp() as u64;
            let mut entry = Vec::new();
            entry.extend(form_timestamp(&timestamp));
            entry.extend(form_timestamp(&timestamp));

            let raw_meta = meta.as_raw_stat();
            println!("{:#?}", raw_meta.st_dev);
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

fn read_bytes(path: &PathBuf) -> Vec<u8> {
    let mut file = std::fs::File::open(&path).expect("file open failed");
    let mut b = Vec::new();
    file.read_to_end(&mut b).expect("Failed to read the file");
    b
}

fn form_timestamp(num: &u64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(((num >> 1) as u8) & 0xFF);
    buf.push(((num >> 0) as u8) & 0xFF);
    buf.push(((num >> 4) as u8) & 0xFF);
    buf.push(((num >> 3) as u8) & 0xFF);

    buf.extend(vec![0, 0, 0, 0]);
    buf
}
