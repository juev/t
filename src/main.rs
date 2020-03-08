extern crate getopts;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use getopts::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
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

    let donefile = format!(".{}.done", taskfile);
    let mut donepath = PathBuf::from(&taskdir);
    donepath.push(donefile);

    let (tasks, done) = read_files(taskpath, donepath);

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        "Print tasks".to_string()
    };
    println!("{}", input);
}

fn hash(str: String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&str);
    hasher.result_str()
}

fn read_files(
    taskpath: PathBuf,
    donefile: PathBuf,
) -> (HashMap<String, String>, HashMap<String, String>) {
    if !Path::new(&taskpath).exists() {
        println!("File {} does not exist...", taskpath.to_str().unwrap());
        exit(1);
    }
    let contents = fs::read_to_string(taskpath).unwrap_or_else(|_| "".to_string());
    println!("{}", contents);

    let contents_done = fs::read_to_string(donefile).unwrap_or_else(|_| "".to_string());
    println!("{}", contents);

    let mut tasks: HashMap<String, String> = HashMap::new();
    let mut done: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        tasks.insert(hash(line.to_string()), line.to_string());
    }

    for line in contents_done.lines() {
        done.insert(hash(line.to_string()), line.to_string());
    }

    (tasks, done)
}
