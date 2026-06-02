pub mod error;
pub mod expr;
pub mod grammar;
pub mod immap;
pub mod value;

use value::Value;
use clap::Parser;
use error::Result;
use grammar::DnjParser;
use std::{path::PathBuf, process::exit};

use crate::expr::{Expr, ExprSet};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Root description file
    #[arg(short, long)]
    input: PathBuf,
}

fn run(args: Args) -> Result<()> {
    let expr: Expr<Value> = DnjParser::parse_file(args.input)?.bind(ExprSet::new());
    println!("input: {:#}", expr);
    let resolved = expr.eval().unwrap();
    println!("output: {:#}", resolved);
    Ok(())
}

fn main() {
    match run(Args::parse()) {
        Ok(_) => {
            exit(0);
        }
        Err(err) => {
            println!("{}", err);
            println!("{:#?}", err);
            exit(1);
        }
    }
}
