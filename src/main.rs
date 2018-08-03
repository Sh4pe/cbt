extern crate clipboard;

use std::process::{Command, Stdio, Output, exit};
use std::thread;
use std::time;
use std::io::{Write};
use std::env;
use std::fmt;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

struct ShellTransformation {
    command: String
}

impl ShellTransformation {
    fn new(command: String) -> ShellTransformation {
        ShellTransformation { command }
    }

    fn get_filter_output(&self, s: &String) -> std::io::Result<Output> {
        let mut process = Command::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdin = process.stdin.as_mut().expect("unable to get stdin");
            stdin.write(s.as_bytes()).unwrap();
        }
        process.wait_with_output()
    }

    fn is_valid_filter(&self) -> bool {
        let output = self.get_filter_output(&String::new()).unwrap();
        output.status.success()
    }

    fn apply(&self, s: &String) -> String {
        let output = self.get_filter_output(s).expect("unable to get output");
        String::from_utf8(output.stdout).unwrap()
    }
}

impl fmt::Display for ShellTransformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

fn poll_and_transform(poll_wait: u64, transformations: Vec<ShellTransformation>) -> Result<(), String> {
    assert!(poll_wait > 0, "can't wait for 0ms!");
    let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
    match context.get_contents() {
        Ok(old_content) => {
            let mut old_content = old_content;
            loop {
                thread::sleep(time::Duration::from_millis(poll_wait));
                let mut new_content = context.get_contents();
                match new_content {
                    Err(e) => return Err(format!("{}", e)),
                    Ok(new_content) => {
                        if new_content != old_content {
                            if transformations.len() == 0 {
                                println!("{}", new_content);
                            } else {
                                let transformed_content = transformations.iter()
                                    .fold(new_content.clone(), |acc, x| x.apply(&acc) );
                                if transformed_content.len() > 0 {
                                    print!("{}", transformed_content);
                                }
                            };
                            old_content = new_content;
                        }
                    }
                }
            }
        },
        Err(e) => return Err(format!("{}", e))
    }
}

fn main() {
    let transformations : Vec<_> = env::args()
        .skip(1)
        .map(ShellTransformation::new)
        .collect();
    if let Some(transformation) = transformations.iter().find(|t| !t.is_valid_filter()) {
        println!("Error: '{}' does not appear to be a valid filter", transformation);
        exit(1);
    }
    if let Err(err) = poll_and_transform(300, transformations) {
        println!("Error: {}", err);
        exit(1);
    }
}
