use std::collections::HashMap;

use logos::Logos;
use disturbed::{ Token, Stmt, Parse, Eval };

fn main() {
    let input = "
        let x = 1 + 2 * 3;
        print x;
    ";

    let lexer = Token::lexer(input)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    
    println!("{:?}", lexer);

    let mut parser = Parse::new(lexer);
    let ast = parser.parse_all();

    let mut vars = HashMap::new();

    for stmt in ast {
        match stmt {
            Stmt::Let(name, expr ) => {
                let val = Eval::eval_operation(&Eval, &expr, &vars);
                vars.insert(name, val);
            },
            Stmt::Print(expr) => {
                println!("{}", Eval::eval_operation(&Eval, &expr, &vars));
            }
        }
    }
}
