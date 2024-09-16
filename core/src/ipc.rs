use std::io::{self, BufRead};

pub fn register_ipc_command<F>(name: &str, fun: F)
where
    F: Fn(Vec<String>),
{
    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Ok(line) = lines.next().unwrap() {
        process_line(&line, name, &fun);
    }
}

fn process_line<F>(line: &str, name: &str, fun: &F)
where
    F: Fn(Vec<String>),
{
    let args = split_args(line, "::");
    if args[0] == name {
        fun(args);
    }
}

fn split_args(line: &str, separator: &str) -> Vec<String> {
    let splitted = line.split(separator);

    let mut out: Vec<String> = Vec::new();

    splitted.for_each(|item| {
        out.push(item.to_string());
    });

    out
}
