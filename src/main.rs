#[macro_use]
extern crate clap;
mod cli;
mod config;

use chrono::Local;
use config::get_config;
use std::cmp::min;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::time;
use threadpool::ThreadPool;

fn expect_exit1<T>(val: Option<T>, msg: &str) -> T {
    if let Some(val) = val {
        val
    } else {
        eprintln!("{}", msg);
        exit(1);
    }
}

fn main() {
    let conf = get_config();
    let command = expect_exit1(conf.command, "[ERROR] command is not specified");
    let input_dir = expect_exit1(conf.input_dir, "[ERROR] input directory is not specified");
    let output_dir = expect_exit1(conf.output_dir, "[ERROR] output directory is not specified");
    let savefile_name = conf.logfile.unwrap_or(String::from("log.ltsv"));
    let threads = conf.threads.unwrap_or(10);

    let input_dir = input_dir.as_str();
    let output_dir = output_dir.as_str();

    // dbg!(command);
    // dbg!(input_dir);
    // dbg!(output_dir);
    // dbg!(savefile_name);
    // dbg!(threads);

    if !Path::new(output_dir).is_dir() {
        eprintln!("[ERROR] Specified out-dir is not directory: {}", output_dir);
        exit(1);
    }

    let filelist = get_filelist(input_dir).unwrap();
    let threads = min(threads, filelist.len());
    let mut cnt = filelist.len();

    let pool = ThreadPool::new(threads);
    let (sender, reciever) = mpsc::channel();
    for (index, input_path) in filelist.into_iter().enumerate() {
        let sender = sender.clone();
        let output_path = gen_output_path(&input_path, output_dir);
        let command = command.to_owned();
        pool.execute(move || {
            let now = Local::now();
            eprintln!("[INFO] Running: case {}", index + 1);
            let (_status, millis) = exec(index + 1, command, &input_path, &output_path);
            eprintln!("[INFO] Finish: case {}", index + 1);
            sender.send((input_path, output_path, millis, now)).unwrap();
        });
    }

    pool.join();
    let mut logs = vec![];
    for msg in reciever {
        logs.push(msg);
        cnt -= 1;
        if cnt == 0 {
            break;
        }
    }

    let savefile = fs::File::create(savefile_name).unwrap();
    let mut writer = BufWriter::new(savefile);
    for (input_path, output_path, millis, now) in logs {
        let now = now.format("%Y-%m-%d %T").to_string();
        let line = format!(
            "command:{}\tinputfile:{}\toutputfile:{}\truntime:{}ms\ttime:{}",
            command,
            input_path.display(),
            output_path.display(),
            millis,
            now
        );
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all("\n".as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}

fn get_filelist(dirname: &str) -> io::Result<Vec<PathBuf>> {
    let mut filelist = vec![];
    for entry in fs::read_dir(dirname)? {
        let path = entry?.path();
        if !path.is_dir() {
            filelist.push(path);
        }
    }
    Ok(filelist)
}

fn gen_output_path(path: &PathBuf, dir: &str) -> PathBuf {
    let filename = [path.file_name().unwrap().to_str().unwrap(), ".out"].concat();
    Path::new(dir).join(filename)
}

fn exec(
    pid: usize,
    command: String,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> (ExitStatus, u128) {
    let input = fs::File::open(input_path).unwrap();
    let output = fs::File::create(output_path).unwrap();

    let arr: Vec<String> = command.split_whitespace().map(|e| e.to_string()).collect();
    let arr = arr.as_slice();
    let (command, args) = (&arr[0], &arr[1..]);

    let mut com = Command::new(command);
    com.args(args)
        .stdin(input)
        .stdout(output)
        .stderr(Stdio::piped());

    let start = time::Instant::now();
    let mut child = com.spawn().expect("[ERROR] running command is failed");
    let child_stderr = child.stderr.as_mut().unwrap();
    let reader = BufReader::new(child_stderr);
    for line in reader.lines() {
        eprintln!("[DEBUG] stderr(case {}): {}", pid, line.unwrap());
    }
    let status = child.wait().unwrap();
    let elapsed = start.elapsed();

    (status, elapsed.as_millis())
}
