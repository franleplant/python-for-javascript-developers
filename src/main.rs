use regex::Regex;
use std::fs;
use std::process::Command;
use std::str;

#[derive(Debug)]
struct CodeBlock {
    lang: Option<String>,
    code: String,
    start: usize,
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

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("./DOC.md").expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);

    let mut parser = Parser::new();
    let blocks = parser.parse(&contents)?;
    //println!("blocks {:?}", blocks);

    for block in blocks.into_iter() {
        let lang = block.lang.clone().unwrap_or("no_lang".to_string());

        match lang.as_str() {
            "javascript" => {
                println!("evaluating {} block at line {}", lang, block.start + 1);
                let output = Command::new("node")
                    .arg("-e")
                    .arg(&block.code)
                    .output()
                    .expect(&format!("failed to execute, is node installed and available?"));

                if !output.status.success() {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            }

            "python" => {
                println!("evaluating {} block at line {}", lang, block.start + 1);
                let output = Command::new("python3")
                    .arg("-c")
                    .arg(&block.code)
                    .output()
                    .expect(&format!("failed to execute, is python3 installed and available?"));

                if !output.status.success() {
                    println!("ERROR");
                    println!("{}", str::from_utf8(&output.stderr).unwrap())
                }
            },

            _ => println!("skipping code block {:?}", block),
        }
    }

    Ok(())
}
