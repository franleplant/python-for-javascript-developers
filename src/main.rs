use regex::Regex;
use std::fs;
use std::fs::File;
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

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("./DOC.md").expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);

    let mut parser = Parser::new();
    let blocks = parser.parse(&contents)?;
    //println!("blocks {:?}", blocks);
    //
    //run_in_memory(blocks)?;

    let prefix = "DOC";
    let tmp_dir = "./tmp";
      fs::create_dir_all(tmp_dir).unwrap_or_else(|why| {
                  println!("! {:?}", why.kind());
                      });
    for block in blocks.into_iter() {
        let ext = block.get_file_ext();
        let line = block.get_start_line();
        let path_raw = format!("{}/{}__{}.{}", tmp_dir, prefix, line, ext);
        let path = Path::new(&path_raw);
        let path_display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path_display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(block.code.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path_display, why),
            Ok(_) => println!("successfully wrote to {}", path_display),
        }
    }

    Ok(())
}
