extern crate getopts;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use getopts::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
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
    let donefile = format!(".{}.done", taskfile);
    let taskdir = matches
        .opt_str("t")
        .unwrap_or_else(|| path.to_str().unwrap().to_string());

    let mut taskpath = PathBuf::from(&taskdir);
    taskpath.push(&taskfile);

    // read files
    let mut donepath = PathBuf::from(taskpath.as_path().parent().unwrap().to_path_buf());
    donepath.push(donefile);

    let contents = fs::read_to_string(&taskpath).unwrap_or_else(|_| "".to_string());
    let contents_done = fs::read_to_string(&donepath).unwrap_or_else(|_| "".to_string());

    let mut tasks: HashMap<String, String> = HashMap::new();
    let mut done: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        tasks.insert(hash(&line.to_string()), line.to_string());
    }

    for line in contents_done.lines() {
        done.insert(hash(&line.to_string()), line.to_string());
    }

    if matches.opt_present("done") {
        for (_, task) in done {
            println!("{}", task);
        }
        return;
    }

    let mut write = false;

    // finish task
    if matches.opt_present("f") {
        let task = matches.opt_str("f").unwrap();
        let key = matches.opt_str("f").unwrap();
        done.insert(task, tasks.get(&key).unwrap().to_string());
        tasks.remove(&key);
        write = true;
    }

    // add new task
    if !matches.free.is_empty() {
        let task = matches.free.join(" ");
        tasks.insert(hash(&task), task);
        write = true;
    }

    // write files
    if write {
        if matches.opt_present("d") && tasks.is_empty() {
            if Path::new(&taskpath).exists() {
                fs::remove_file(taskpath).unwrap();
                fs::remove_file(donepath).unwrap();
            }
            return;
        }
        // TODO fix write to file
        //tasks
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(taskpath.to_str().unwrap())
            .unwrap();

        for (_, task) in &tasks {
            writeln!(file, "{}", task).unwrap();
        }

        //done
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(donepath.to_str().unwrap())
            .unwrap();

        for (_, task) in &done {
            writeln!(file, "{}", task).unwrap();
        }
        return;
    }

    // print tasks
    for (hash, task) in tasks {
        println!("{} - {}", hash, task);
    }
}

fn hash(str: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&str);
    hasher.result_str()
}
