use std::rc::Rc;
use std::mem;
use std::str::Chars;

use values::Value;

#[derive(Debug)]
struct Parser<'a> {
    code: Chars<'a>,

    parts: Vec<Rc<Value>>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code: code.chars(),
            parts: Vec::new(),
        }
    }

    fn append_value(&mut self, buf: String) {
        let value = if let Ok(num) = buf.parse::<isize>() {
            Value::Integer(num)
        } else {
            Value::Symbol(buf)
        };

        self.parts.push(Rc::new(value));
    }

    fn parse(&mut self) {
        let mut buf = String::new();

        loop {
            let c = match self.code.next() {
                Some(c) => c,
                None => break,
            };

            // TODO: Add strings.
            match c {
                '(' => self.parse(),

                ')' => break,

                ' ' => {
                    let old_buf = mem::replace(&mut buf, String::new());

                    if !old_buf.is_empty() {
                        self.append_value(old_buf);
                    }
                }

                c => {
                    buf.push(c);
                }
            }
        }

        if !buf.is_empty() {
            self.append_value(buf);
        }
    }

    fn root(self) -> Value {
        Value::from(self.parts)
    }
}

pub fn parse(code: &str) -> Value {
    let mut parser = Parser::new(code);
    parser.parse();
    parser.root()
}
