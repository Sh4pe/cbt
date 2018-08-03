extern crate clipboard;

use std::process::{Command, Stdio, exit};
use std::thread;
use std::time;
use std::io::{Write};
use std::env;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

struct ShellTransformation {
    command: String
}

impl ShellTransformation {
    fn new(command: String) -> ShellTransformation {
        ShellTransformation { command }
    }

    fn apply(&self, s: &String) -> String {
        let mut process = Command::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdin = process.stdin.as_mut().expect("unable to get stdin");
            stdin.write(s.as_bytes()).unwrap();
        }
        let output = process.wait_with_output().expect("unable to get output");
        String::from_utf8(output.stdout).unwrap()
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
    if let Err(err) = poll_and_transform(300, transformations) {
        println!("Error: {}", err);
        exit(1);
    }
}
