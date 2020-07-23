extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::error::Error;

use pest::Parser;

#[derive(Parser)]
#[grammar = "tilde.pest"]
pub struct TildeParser;

fn build_html_from_expr(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::expr => build_html_from_expr(pair.into_inner().next().unwrap()),
        Rule::heading => {
            // let str = pair.next().unwrap().as_str();

            println!("{:#?}", pair);
            let mut pair = pair.into_inner();
            let header = pair.next().unwrap().as_span();
            let header_level = header.start();
            println!("{:#?}", header);
            let header_str = header.as_str();
            return format!("<h{0}>{1}</h{0}>", header_level - 1, header_str);
        }
        Rule::quote => {
            let str = &pair.as_str();
            let str = &str[1..str.len()];
            return format!("<blockquote>{}</blockquote>", str.replace(">","").to_string());
        }
        Rule::newline => {
            return "<br />".to_string();
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}


fn parse_tilde_file(_filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut html = vec![];

    let unparsed_file = fs::read_to_string(_filename).expect("cannot read file");

    let pairs = TildeParser::parse(Rule::file, &unparsed_file)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                html.push(build_html_from_expr(pair));
            },
            _ => {}
        }
    }

    Ok(html)
}

fn usage() {
    let the_version = env!("CARGO_PKG_VERSION");
    println!("parser for tilde, a language like markdown but supercharged");
    println!("Version: {}", the_version);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let html = parse_tilde_file(&args[1]).expect("unsuccessful parse");
            println!("{:#?}", &html);
        },
        _ => usage()
    }
}
