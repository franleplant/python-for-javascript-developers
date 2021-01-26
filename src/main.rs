use regex::Regex;
use std::fs;

#[derive(Debug)]
struct CodeBlock {
    lang: Option<String>,
    code: String,
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

fn main() -> Result<(), String>{
    let contents = fs::read_to_string("./DOC.md").expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);

    let mut parser = Parser::new();
    let blocks = parser.parse(&contents)?;
    //println!("blocks {:?}", blocks);

    for block in blocks.into_iter() {
        let lang = block.lang.clone().unwrap_or("no_lang".to_string());

        match lang.as_str() {
            "python" => println!("found py {:?}", block.code),
            "javascript" => println!("found js {:?}", block.code),

            _ => println!("skipping code block {:?}", block)
        }


    }

    Ok(())
}
