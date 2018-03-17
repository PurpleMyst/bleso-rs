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

    fn parts_to_sexpr(mut self) -> Value {
        let mut conductor = Rc::try_unwrap(self.parts.pop().unwrap()).unwrap();

        self.parts.reverse();
        for value in self.parts.into_iter() {
            // XXX: Is there something better we can pass as a second argument? Constructing a new
            // list seems wasteful, honestly.
            let old_conductor = mem::replace(&mut conductor, Value::empty_list());
            let new_conductor = Value::cons(value, Rc::new(old_conductor));
            conductor = new_conductor;
        }

        conductor
    }
}

pub fn parse(code: &str) -> Value {
    let mut parser = Parser::new(code);
    parser.parse();
    parser.parts_to_sexpr()
}
