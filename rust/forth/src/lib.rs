pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
use core::slice::Iter;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            ..Default::default()
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    fn do_arithmetic(&mut self, operation: &str) -> Result {
        let top = self.pop_stack()?;
        let second_top = self.pop_stack()?;
        let result = match operation {
            "+" => second_top + top,
            "-" => second_top - top,
            "*" => second_top * top,
            _ => {
                if top == 0 {
                    return Err(Error::DivisionByZero);
                }
                second_top / top
            }
        };
        self.stack.push(result);
        Ok(())
    }

    fn pop_stack(&mut self) -> std::result::Result<Value, Error> {
        match self.stack.pop() {
            Some(val) => Ok(val),
            None => Err(Error::StackUnderflow),
        }
    }

    fn peek_stack(&mut self) -> std::result::Result<Value, Error> {
        match self.stack.iter().last() {
            Some(&val) => Ok(val),
            None => Err(Error::StackUnderflow),
        }
    }

    fn create_definition(&mut self, tokens: &mut Iter<String>) -> Result {
        // Get the name
        let name = match tokens.next() {
            Some(n) => {
                if n.chars().all(|x| !x.is_ascii_digit()) {
                    n
                } else {
                    return Err(Error::InvalidWord);
                }
            }
            None => return Err(Error::InvalidWord),
        };

        // If there was already a definition with the same name then replace all the possible
        // name->definitions pairs that used this name as part of the definition body, before
        // updating the defintion body of this name
        if self.definitions.get(name).is_some() {
            let defs = self.definitions.clone();
            for (k, v) in defs {
                // Cannot use replace on v because it might be a substring of something other value e.g:
                // ": foo 10 ;"
                // ": bar foobar ;"
                // ": foo 15 ;"
                // In this case if we used replace we would replace bar definition to:
                // ": bar 15bar ;"
                let new_v: String = v
                    .split_whitespace()
                    .map(|word| {
                        if word == name {
                            self.definitions.get(name).unwrap()
                        } else {
                            word
                        }
                    })
                    .collect();
                self.definitions.entry(k).and_modify(|v| {
                    if *v != new_v {
                        *v = new_v
                    }
                });
            }
        }

        // Get the definition body
        let mut definition = String::new();
        let mut token = tokens.next();
        while token.is_some() && token.unwrap() != ";" {
            // If the definition is using the same name, then there's a previous definition with
            // this name and we should expand it (only this case and not other otherwise we are
            // applying eager expansion which is exploitable see tests/alloc-attack.rs)
            if name == token.unwrap() {
                definition.push_str(self.definitions.get(name).unwrap());
            } else {
                definition.push_str(token.unwrap());
            }
            definition.push(' ');
            token = tokens.next();
        }

        // Trim the extra space at the end of definition just because it looks better when debugging the definitions hashmap
        definition = definition.trim_end().to_string();

        // Check that it ends with a ;, if not then return error as per README
        if token.is_none() {
            return Err(Error::InvalidWord);
        }

        self.definitions.insert(name.to_string(), definition);
        Ok(())
    }

    fn handle_definition(&mut self, token: &str) -> Result {
        // Need to clone since we use inmutable borrow of self (when using the get on definitions)
        // while at the same time having a self mutable borrow of definition for eval
        let v = self.definitions.clone();
        let def = v.get(token).unwrap();
        self.eval(def)
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens: Vec<String> = input.split_whitespace().map(|t| t.to_uppercase()).collect();
        let mut iter = tokens.iter();
        while let Some(token) = iter.next() {
            let token_str = token.as_str();
            match self.definitions.get(token_str) {
                // Try to handle definitions first since they could overload pretty much anything
                Some(_) => self.handle_definition(token_str)?,
                // If there are no definitions with that name then use try to match to the arithmetic operators, functions and numbers
                None => match token.as_str() {
                    "+" | "-" | "*" | "/" => self.do_arithmetic(token)?,
                    // Duplicates the top item
                    "DUP" => {
                        let val = self.peek_stack()?;
                        self.stack.push(val);
                    }
                    // Discards the top stack item
                    "DROP" => {
                        self.pop_stack()?;
                    }
                    // Swaps the two top stack items
                    "SWAP" => {
                        let top = self.pop_stack()?;
                        let second_top = self.pop_stack()?;
                        self.stack.push(top);
                        self.stack.push(second_top);
                    }

                    // Copies the second item to top
                    "OVER" => match self.stack.iter().rev().nth(1) {
                        Some(&second_top) => self.stack.push(second_top),
                        None => return Err(Error::StackUnderflow),
                    },
                    // Define name -> definitions
                    ":" => {
                        self.create_definition(&mut iter)?;
                    }
                    // Push numbers to stack or throw error for UnknownWord since it's none of the above
                    _ => {
                        match token.parse() {
                            Ok(val) => self.stack.push(val),
                            Err(_) => return Err(Error::UnknownWord),
                        };
                    }
                },
            };
        }
        Ok(())
    }
}
