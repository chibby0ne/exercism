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
            // If the definition is using the same name, then theres a previous definition with
            // this name and we should expand it
            if name == token.unwrap() {
                definition.push_str(self.definitions.get(name).unwrap());
            } else {
                definition.push_str(token.unwrap());
            }
            definition.push(' ');
            token = tokens.next();
        }
        definition = definition.trim_end().to_string();

        // Check that it ends with a ;
        if token.is_none() {
            return Err(Error::InvalidWord);
        }

        self.definitions.insert(name.to_string(), definition);
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens: Vec<String> = input.split_whitespace().map(|t| t.to_uppercase()).collect();
        let mut iter = tokens.iter();
        while let Some(token) = iter.next() {
            match token.as_str() {
                // Handle arithmetic
                "+" | "-" | "*" | "/" => {
                    if self.definitions.get(token).is_some() {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    } else {
                        self.do_arithmetic(token)?
                    }
                }
                // Duplicates the top item
                "DUP" => {
                    if self.definitions.get(token).is_some() {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    } else {
                        let val = self.peek_stack()?;
                        self.stack.push(val);
                    }
                }
                // Discards the top stack item
                "DROP" => {
                    if self.definitions.get(token).is_some() {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    } else {
                        self.pop_stack()?;
                    }
                }
                // Swaps the two top stack items
                "SWAP" => {
                    if self.definitions.get(token).is_some() {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    } else {
                        let top = self.pop_stack()?;
                        let second_top = self.pop_stack()?;
                        self.stack.push(top);
                        self.stack.push(second_top);
                    }
                }

                // Copies the second item to top
                "OVER" => {
                    if self.definitions.get(token).is_some() {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    } else {
                        match self.stack.iter().rev().nth(1) {
                            Some(&second_top) => self.stack.push(second_top),
                            None => return Err(Error::StackUnderflow),
                        }
                    }
                }
                // Define definitions
                ":" => {
                    self.create_definition(&mut iter)?;
                }
                _ => {
                    if token.chars().all(|c| c.is_ascii_digit()) {
                        self.stack.push(token.parse().unwrap());
                    } else if self.definitions.get(token).is_none() {
                        return Err(Error::UnknownWord);
                    } else {
                        let v = self.definitions.clone();
                        let def = v.get(token).unwrap();
                        self.eval(def)?;
                    }

                    // Do the definition
                    // self.eval(

                    // match token.parse() {
                    //     Ok(val) => {
                    //         self.stack.push(val);
                    //     }
                    //     Err(_) => {
                    //         return Err(Error::UnknownWord);
                    //     }
                    // };
                }
            };
        }
        Ok(())
    }
}
