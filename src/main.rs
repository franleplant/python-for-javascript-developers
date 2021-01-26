use regex::Regex;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::str;

#[derive(Debug)]
struct CodeBlock {
    lang: Option<String>,
    code: String,
    start: usize,
}

impl CodeBlock {
    fn get_lang(&self) -> String {
        let mut lang = self.lang.clone().unwrap_or("no_lang".to_string());
        if lang.len() == 0 {
            lang = "no_lang".to_string();
        }

        return lang;
    }

    fn get_file_ext(&self) -> &str {
        match self.get_lang().as_str() {
            "javascript" => "js",
            "python" => "py",
            _ => "no_ext",
        }
    }

    fn get_start_line(&self) -> usize {
        self.start + 1
    }
}

struct Parser<'a> {
    block_start: Regex,
    block_end: Regex,
    index: usize,
    lines: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    fn new() -> Parser<'a> {
        Parser {
            block_start: Regex::new(r"^```(?P<language>.*)").unwrap(),
            block_end: Regex::new(r"```").unwrap(),
            index: 0,
            lines: vec![],
        }
    }

    fn get_line(&self) -> Option<&'a str> {
        return self.lines.get(self.index).map(|string| *string);
    }

    fn parse(&mut self, input: &'a String) -> Result<Vec<CodeBlock>, String> {
        self.lines = input.lines().collect();
        self.index = 0;

        let mut blocks: Vec<CodeBlock> = vec![];

        while let Some(line) = self.get_line() {
            //println!("line {}", line);

            if let Some(cap) = self.block_start.captures(line) {
                let start = self.index;
                let lang = cap.name("language").map(|m| m.as_str().to_string());
                let mut code: Vec<&str> = vec![];

                self.index += 1;
                loop {
                    if let Some(line) = self.get_line() {
                        if self.block_end.is_match(line) {
                            break;
                        }

                        code.push(line);
                        self.index += 1;
                    } else {
                        return Err("Unterminated code block".to_string());
                    }
                }

                let block = CodeBlock {
                    start,
                    lang,
                    code: code.join("\n").to_string(),
                };

                //println!("found block {:?}", block);
                blocks.push(block)
            }

            self.index += 1;
        }

        return Ok(blocks);
    }
}

fn run_in_memory(blocks: Vec<CodeBlock>) -> Result<(), String> {
    for block in blocks.into_iter() {
        let lang = block.get_lang();

        match lang.as_str() {
            "javascript" => {
                println!(">>> evaluating {} block at line {}", lang, block.start + 1);
                let output = Command::new("node")
                    .arg("--experimental-vm-modules")
                    .arg("--experimental-modules")
                    .arg("--input-type=module")
                    .arg("-e")
                    .arg(&block.code)
                    .output()
                    .expect(&format!(
                        "failed to execute, is node installed and available?"
                    ));

                if output.status.success() {
                    println!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap())
                } else {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            }

            "python" => {
                println!(">>> evaluating {} block at line {}", lang, block.start + 1);
                let output = Command::new("python3")
                    .arg("-c")
                    .arg(&block.code)
                    .output()
                    .expect(&format!(
                        "failed to execute, is python3 installed and available?"
                    ));

                if output.status.success() {
                    println!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap())
                } else {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            }

            _ => println!("xxx skipping {} block at line {}", lang, block.start + 1),
        }
    }

    Ok(())
}

fn fs_extract(prefix: &str, tmp_dir: &str, blocks: Vec<CodeBlock>) -> io::Result<()> {
    fs::remove_dir_all(tmp_dir).unwrap_or_else(|why| {
        println!("error cleaning tmp dir {:?}", why);
    });
    fs::create_dir_all(tmp_dir).unwrap_or_else(|why| {
        println!("error creating tmp dir {:?}", why.kind());
    });

    for block in blocks.into_iter() {
        let ext = block.get_file_ext();
        let line = block.get_start_line();
        let path_raw = format!("{}/{}__{}.{}", tmp_dir, prefix, line, ext);
        let path = Path::new(&path_raw);
        let path_display = path.display();

        println!("creating file {}", path_display);
        let mut file = File::create(&path)?;
        println!("writting file {}", path_display);
        file.write_all(block.code.as_bytes())?;
    }

    Ok(())
}

fn check_node() -> Result<String, String> {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect(&format!(
            "failed to execute, is node installed and available?"
        ));

    if output.status.success() {
        let version = str::from_utf8(&output.stdout).unwrap();
        println!("node --version {}", version);
        return Ok(version.to_string());
    } else {
        println!("{}", str::from_utf8(&output.stderr).unwrap());
        return Err(("node not found".to_string()));
    }
}

fn check_python() -> Result<String, String> {
    let output = Command::new("python3")
        .arg("--version")
        .output()
        .expect(&format!(
            "failed to execute, is python3 installed and available?"
        ));

    if output.status.success() {
        let version = str::from_utf8(&output.stdout).unwrap();
        println!("python3 --version {}", version);
        return Ok(version.to_string());
    } else {
        println!("{}", str::from_utf8(&output.stderr).unwrap());
        return Err(("python3 not found".to_string()));
    }
}

fn fs_run(tmp_dir: &str) -> io::Result<()> {
    println!("run files in {}", tmp_dir);
    check_node().unwrap();
    check_python().unwrap();

    for entry in fs::read_dir(tmp_dir)? {
        let entry = entry?;
        let path_raw = entry.path();
        let path = Path::new(&path_raw);
        match path
            .extension()
            .map(|ext| ext.to_str().unwrap_or("no_ext"))
            .unwrap_or("no_ext")
        {
            "js" => {
                println!(">>> evaluating {}", path.display());
                let output = Command::new("node")
                    .arg(path.to_str().unwrap())
                    .output()
                    .expect(&format!(
                        "failed to execute, is node installed and available?"
                    ));

                if output.status.success() {
                    println!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap())
                } else {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            }

            "py" => {
                println!(">>> evaluating {}", path.display());
                let output = Command::new("python3")
                    .arg(path.to_str().unwrap())
                    .output()
                    .expect(&format!(
                        "failed to execute, is python3 installed and available?"
                    ));

                if output.status.success() {
                    println!("stdout:\n{}", str::from_utf8(&output.stdout).unwrap())
                } else {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            }

            _ => println!("fs_run skipping {}", path.display()),
        }
        //println!("found {:?} {:?}", path, path.extension());
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("./DOC.md").expect("Something went wrong reading the file");

    let mut parser = Parser::new();
    let blocks = parser.parse(&contents)?;
    //run_in_memory(blocks)?;

    let prefix = "DOC";
    let tmp_dir = "./tmp";

    fs_extract(prefix, tmp_dir, blocks).unwrap();
    fs_run(tmp_dir).unwrap();

    Ok(())
}
