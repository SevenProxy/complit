use std::{
    env,
    collections::HashMap
};

use disturbed::{ 
    Token,
    Stmt, 
    Parse,
    Eval,
    Read,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <namefile.kj>");
        std::process::exit(1);
    }

    let path_file: &String = &args[1];
    let mut read: Read = Read::new((path_file).to_string());
    let lexer: Result<Vec<Token>, ()> = read.file_read();

    let mut parser: Parse = Parse::new(lexer.expect("REASON"));
    let ast: Vec<Stmt> = parser.parse_all();

    let mut vars: HashMap<String, i64> = HashMap::new();

    for stmt in ast {
        match stmt {
            Stmt::Let(name, expr ) => {
                let val = Eval::eval_operation(&Eval, &expr, &vars);
                vars.insert(name, val);
            },
            Stmt::Print(expr) => {
                println!("{:?}", Eval::eval_operation(&Eval, &expr, &vars));
            }
        }
    }
}
