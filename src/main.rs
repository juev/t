extern crate getopts;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use getopts::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help screen");
    opts.optflag("", "done", "list done tasks instead of unfinished ones");
    opts.optopt("e", "edit", "edit TASK to contain TEXT", "TASK");
    opts.optopt("f", "finish", "mark TASK as finished", "TASK");
    opts.optopt("r", "remove", "Remove TASK from list", "TASK");
    opts.optopt("l", "list", "work on LIST", "LIST");
    opts.optopt("t", "taskdir", "work on the lists in DIR", "DIR");
    opts.optflag(
        "d",
        "delete-if-empty",
        "delete the task file if it becomes empty",
    );
    opts.optopt("g", "grep", "print only tasks that contain WORD", "WORD");
    opts.optflag(
        "v",
        "verbose",
        "print more detailed output (full task ids, etc)",
    );
    opts.optflag(
        "q",
        "quiet",
        "print less detailed output (no task ids, etc)",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = "t is for people that want do things, not organize their tasks

Usage: t [-t DIR] [-l LIST] [options] [TEXT]";
        print!("{}", opts.usage(&brief));
        return;
    }

    let path = env::current_dir().unwrap();
    let taskfile = matches.opt_str("l").unwrap_or_else(|| "tasks".to_string());
    let taskdir = matches
        .opt_str("t")
        .unwrap_or_else(|| path.to_str().unwrap().to_string());

    let mut taskpath = PathBuf::from(&taskdir);
    taskpath.push(&taskfile);

    println!("taskdir: {}", taskpath.to_str().unwrap().to_string());

    // read files
    let donefile = format!(
        ".{}.done",
        taskpath
            .as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    );
    let mut donepath = PathBuf::from(taskpath.as_path().parent().unwrap().to_path_buf());
    donepath.push(donefile);

    let contents = fs::read_to_string(&taskpath).unwrap_or_else(|_| "".to_string());
    println!("{}", contents);

    let contents_done = fs::read_to_string(&donepath).unwrap_or_else(|_| "".to_string());
    println!("{}", contents);

    let mut tasks: HashMap<String, String> = HashMap::new();
    let mut done: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        tasks.insert(hash(&line.to_string()), line.to_string());
    }

    for line in contents_done.lines() {
        done.insert(hash(&line.to_string()), line.to_string());
    }

    if matches.opt_present("done") {
        for (hash, task) in done {
            println!("{} - {}", hash, task);
        }
        return;
    }

    if !matches.free.is_empty() {
        let task = matches.free.join(" ");
        tasks.insert(hash(&task), task);
        let delete_empty = matches.opt_present("d");
        println!("{:?}", delete_empty);
        write_files(&tasks, &done, &taskpath, delete_empty);
        return;
    }

    for (hash, task) in tasks {
        println!("{} - {}", hash, task);
    }
}

fn hash(str: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&str);
    hasher.result_str()
}

fn write_files(
    tasks: &HashMap<String, String>,
    done: &HashMap<String, String>,
    taskpath: &PathBuf,
    delete_empty: bool,
) {
    let donefile = format!(
        ".{}.done",
        taskpath
            .as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    );
    let mut donepath = PathBuf::from(taskpath.as_path().parent().unwrap().to_path_buf());
    donepath.push(donefile);

    if delete_empty {
        if Path::new(&taskpath).exists() {
            fs::remove_file(taskpath).unwrap();
            fs::remove_file(donepath).unwrap();
        }
        return;
    }
    let mut data = String::new();
    for (hash, task) in tasks {
        data = format!("{}\n{} - {}\n", data, hash, task);
    }
    println!("{:?}", tasks);
    println!("{:?}", data);
    fs::write(taskpath, data).expect("Unable to write task file");
    let mut data = String::new();
    for (hash, task) in done {
        data = format!("{}\n{} - {}\n", data, hash, task);
    }
    fs::write(donepath, data).expect("Unable to write donefile");
}
