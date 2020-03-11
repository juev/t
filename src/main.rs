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

    if !Path::new(&taskdir).exists() {
        eprintln!("Directory does not exist: {}", taskdir);
        return;
    }
    // read files
    let mut taskpath = PathBuf::from(&taskdir);
    taskpath.push(&taskfile);

    let mut donepath = taskpath.as_path().parent().unwrap().to_path_buf();
    donepath.push(donefile);

    let contents = fs::read_to_string(&taskpath).unwrap_or_else(|_| "".to_string());
    let contents_done = fs::read_to_string(&donepath).unwrap_or_else(|_| "".to_string());

    let mut tasks: HashMap<String, String> = HashMap::new();
    let mut done: HashMap<String, String> = HashMap::new();
    let mut prefixes: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        tasks.insert(get_sha256(&line.to_string()), line.to_string());
    }

    for line in contents_done.lines() {
        done.insert(get_sha256(&line.to_string()), line.to_string());
    }

    // fill prefixes
    for (hash, _) in tasks.iter() {
        prefixes.insert(get_prefix(&prefixes, hash.as_str()), String::from(hash));
    }

    // commands
    if matches.opt_present("done") {
        for (_, task) in done {
            println!("{}", task);
        }
        return;
    }
    let delete_empty = matches.opt_present("d");
    // finish task
    if matches.opt_present("f") {
        let task = matches.opt_str("f").unwrap();
        let task = get_hash(task.as_str(), &tasks, &prefixes).unwrap();
        for t in task.split(',') {
            let t = String::from(t);
            let key = String::from(&t);
            if tasks.contains_key(&t) {
                done.insert(t, tasks.get(&key).unwrap().to_string());
                tasks.remove(&key);
            } else {
                println!("Task does not exist: {}", &t);
            }
        }
        write_files(tasks, done, taskpath, donepath, delete_empty);
        return;
    }
    // remove task
    if matches.opt_present("r") {
        let task = matches.opt_str("r").unwrap();
        let task = get_hash(task.as_str(), &tasks, &prefixes).unwrap();
        for t in task.split(',') {
            let t = String::from(t);
            if tasks.contains_key(&t) {
                tasks.remove(&t);
            } else {
                println!("Task does not exist: {}", &task);
            }
        }
        write_files(tasks, done, taskpath, donepath, delete_empty);
        return;
    }

    // edit task
    if matches.opt_present("e") {
        let task = matches.opt_str("e").unwrap();
        let task = get_hash(task.as_str(), &tasks, &prefixes).unwrap();
        if tasks.contains_key(&task) {
            if !matches.free.is_empty() {
                tasks.remove(&task);
                let task = matches.free.join(" ");
                tasks.insert(get_sha256(&task), task);
                write_files(tasks, done, taskpath, donepath, delete_empty);
            } else {
                println!("Please provide text for new task.");
                return;
            }
        } else {
            println!("Task does not exist: {}", &task);
        }
        return;
    }

    // add new task
    if !matches.free.is_empty() {
        let task = matches.free.join(" ");
        tasks.insert(get_sha256(&task), task);
        write_files(tasks, done, taskpath, donepath, delete_empty);
        return;
    }

    // print tasks
    let word = matches.opt_str("g").unwrap_or_else(|| "".to_string());
    for (hash, task) in tasks.iter() {
        if task.contains(&word) {
            if matches.opt_present("q") {
                println!("{}", task);
            } else if matches.opt_present("v") {
                println!("{} - {}", hash, task);
            } else if let Some(prefix) = get_prefix_by_hash(&prefixes, hash) {
                println!("{} - {}", prefix, task);
            }
        }
    }
}

fn get_sha256(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&str);
    hasher.result_str()
}

fn get_prefix(prefixes: &HashMap<String, String>, hash: &str) -> String {
    for i in 1..hash.len() {
        let prefix = &hash[..i];
        if !prefixes.contains_key(prefix) {
            return String::from(prefix);
        }
    }
    String::from(hash)
}

fn get_prefix_by_hash(prefixes: &HashMap<String, String>, hash: &str) -> Option<String> {
    for (id, prefix) in prefixes.iter() {
        if hash == prefix {
            return Some(String::from(id));
        }
    }
    None
}

fn get_hash(
    id: &str,
    tasks: &HashMap<String, String>,
    prefixes: &HashMap<String, String>,
) -> Option<String> {
    if prefixes.contains_key(id) {
        return Some(prefixes.get(id).unwrap().to_string());
    } else if tasks.contains_key(id) {
        return Some(String::from(id));
    }
    None
}

fn write_files(
    tasks: HashMap<String, String>,
    done: HashMap<String, String>,
    taskpath: PathBuf,
    donepath: PathBuf,
    delete_empty: bool,
) {
    if delete_empty && tasks.is_empty() {
        if Path::new(&taskpath).exists() {
            fs::remove_file(taskpath).unwrap();
            fs::remove_file(donepath).unwrap();
        }
        return;
    }
    //tasks
    let mut taskfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(taskpath.to_str().unwrap())
        .unwrap();

    for (_, task) in tasks {
        writeln!(taskfile, "{}", task).unwrap();
    }
    taskfile.sync_all().unwrap();
    //done
    let mut donefile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(donepath.to_str().unwrap())
        .unwrap();

    for (_, task) in done {
        writeln!(donefile, "{}", task).unwrap();
    }
    donefile.sync_all().unwrap();
}
