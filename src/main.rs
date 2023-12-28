use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

fn main() {
    let successful_parse = LangParser::parse(Rule::expression, "1 + 4");

    let pair = successful_parse.unwrap().next().unwrap();

    println!("{:?}", pair);

    println!("Result: {:?}", process(pair));
}

fn process(pair: Pair<Rule>) -> i32 {
    match pair.as_rule() {
        Rule::expression => {
            let mut pairs = pair.into_inner();
            let left = pairs.next().unwrap();
            let op = pairs.next().unwrap();
            let right = pairs.next().unwrap();

            let left_val = process(left);
            let right_val = process(right);
            if op.as_str() == "+" {
                left_val + right_val
            } else {
                left_val - right_val

            }
        },
        Rule::op => {
            0
        },
        Rule::term => {
            str::parse(pair.as_str().trim()).unwrap()
        },
        _ => {
            0
        }
    }
}