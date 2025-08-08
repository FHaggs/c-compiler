use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
mod asm;
mod ast;
mod lexer;
mod token;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Bad usage");

    if let Some(file_path) = args.get(2) {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let c = buffer.chars();
        if let Some(opt) = args.get(1) {
            match opt.as_str() {
                "--lex" => {
                    let res = lexer::tokenize(c);

                    for token in res {
                        dbg!(token);
                    }
                }
                "--parse" => todo!(),
                "--codegen" => {
                    let mut parser = ast::Parser::new(lexer::tokenize(c));
                    let program = parser.parse_program();
                    let asm = asm::codegen(program);
                    println!("{:#?}", asm);
                }
                "--print" => {
                    let mut parser = ast::Parser::new(lexer::tokenize(c));
                    let program = parser.parse_program();
                    println!("{:#?}", program);
                }
                _ => panic!("Unkown option {}", opt),
            };
        }
    }
    Ok(())
}
