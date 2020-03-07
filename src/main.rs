extern crate getopts;

use getopts::*;
use std::env;
use std::fs;
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

    let donefile = format!(".{}.done", taskfile);
    let mut donepath = PathBuf::from(&taskdir);
    donepath.push(donefile);

    let contents = fs::read_to_string(taskpath).expect("Something went wrong reading the file");
    println!("{}", contents);

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        "Print tasks".to_string()
    };
    println!("{}", input);
}
