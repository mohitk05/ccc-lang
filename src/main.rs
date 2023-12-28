use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

#[derive(Clone)]
#[derive(Debug)]
struct Scope {
    symbol_table: HashMap::<String, i32>,
    functions: HashMap<String, Pair<'static, Rule>>
}

impl Scope {
    fn add_variable(&mut self, name: String, value: i32) {
        self.symbol_table.insert(name, value);
    }

    fn add_function(&mut self, name: String, value: Pair<'static, Rule>) {
        self.functions.insert(name, value);
    }
}

#[derive(Debug)]
struct ScopeStack {
    pub stack: Vec<Scope>
}

impl ScopeStack {
    fn get_identifier_val(&self, name: String) -> Option<i32> {
        for scope in self.stack.iter().rev() {
            let result = scope.symbol_table.get(&name);

            if result.is_some() {
                return result.copied();
            } else {
                continue;
            }
        }

        None
    }

    fn exit_scope(&mut self) {
        self.stack.pop();
    }

    fn last(&self) -> &Scope {
        self.stack.last().unwrap()
    }
}

lazy_static! {
    static ref P_CONTENTS: String = fs::read_to_string("test.lang")
    .expect("Should have been able to read the file");
}

fn main() {
    let successful_parse = LangParser::parse(Rule::program, &P_CONTENTS.as_str());

    let pair = successful_parse.unwrap().next().unwrap();
    let mut scope_stack = ScopeStack {
        stack: Vec::new()
    };
    scope_stack.stack.push(Scope {
        symbol_table: HashMap::new(),
        functions: HashMap::new()
    });
    process(pair.clone(), &mut scope_stack);
}

fn process(pair: Pair<'static, Rule>, scope_stack: &mut ScopeStack) -> i32 {
    match pair.as_rule() {
        Rule::statement |
        Rule::program => {
            let pairs = pair.into_inner();

            pairs.for_each(|p| {
                process(p, scope_stack);
            });

            0
        },
        Rule::function_decl => {
            let mut pairs = pair.clone().into_inner();
            let scope: &mut Scope = scope_stack.stack.last_mut().unwrap();
            scope.add_function(String::from(pairs.next().unwrap().as_str()), pair.clone());
            0
        },
        Rule::expression => {
            let mut pairs = pair.clone().into_inner();
            if pairs.len() > 1 {
                // assignment
                let id = pairs.next().unwrap();
                let right_side = pairs.next().unwrap();
                let ret_val = process_returnable(right_side, scope_stack);
                let scope: &mut Scope = scope_stack.stack.last_mut().unwrap();
                scope.add_variable(String::from(id.as_str().trim()), ret_val);
            } else {
                process_returnable(pairs.next().unwrap(), scope_stack);
            }
            0
        },
        Rule::equation | Rule::call | Rule::term | Rule::identifier => {
            process_returnable(pair, scope_stack)
        },
        _ => 0
    }
}

// equation, call, term, identifier
fn process_returnable(pair: Pair<Rule>, scope_stack: &mut ScopeStack) -> i32 {
    match pair.as_rule() {
        Rule::equation => {
            let mut pairs = pair.into_inner();
            let left = process_returnable(pairs.next().unwrap(), scope_stack);
            let op = pairs.next().unwrap();
            let right = process_returnable(pairs.next().unwrap(), scope_stack);
            if op.as_str() == "+" {
                left + right
            } else {
                left - right
            }
        },
        Rule::term => {
            let mut pairs = pair.clone().into_inner();
            if pairs.len() > 0 {
                process_returnable(pairs.next().unwrap(), scope_stack)
            } else {
                str::parse(pair.as_str()).unwrap()
            }
        },
        Rule::identifier => {
            scope_stack.get_identifier_val(String::from(pair.as_str().trim())).unwrap()
        },
        Rule::call => {
            let mut pairs = pair.into_inner();
            let scope = scope_stack.last();
            let fn_name = pairs.next().unwrap().as_str().trim();
            if fn_name == "print" {
                println!("Log: {}", process_returnable(pairs.next().unwrap().into_inner().next().unwrap(), scope_stack));
                return 0;
            } else {
                let function = scope.functions.get(&String::from(fn_name)).unwrap();
                let mut function_pairs = function.clone().into_inner();
                let _id = function_pairs.next().unwrap();
                let params = function_pairs.next().unwrap().into_inner();
                let body = function_pairs.next().unwrap();
                let mut args = pairs.next().unwrap().into_inner();

                let mut new_st = HashMap::<String, i32>::new();
                params.for_each(|p| {
                    let corresponding_arg = args.next().unwrap();
                    for scope in scope_stack.stack.iter().rev() {
                        let result = scope.symbol_table.get(&String::from(corresponding_arg.as_str().trim()));
                        if result.is_some() {
                            new_st.insert(String::from(p.as_str().trim()), *result.unwrap());
                        } else {
                            continue;
                        }
                    }
                });

                let new_scope = Scope {
                    symbol_table: new_st,
                    functions: HashMap::new()
                };

                scope_stack.stack.push(new_scope);
                let ret_val = process_returnable(body, scope_stack);
                scope_stack.exit_scope();
                return ret_val;
            }
        },
        Rule::args => {
            // let mut pairs = pair.into_inner();
            // pairs.for_each(|p| {

            // })
            // process_returnable(pairs.next().unwrap(), scope_stack)
            0
        }
        _ => 0
    }
}