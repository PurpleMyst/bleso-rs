use std::mem;
use std::str::Chars;

use values::Value;


fn parse_impl(parts: &mut Vec<Value>, chars: &mut Chars) {
    let mut buf = String::new();

    macro_rules! append_value {
        ($buf: ident) => {
            if !$buf.is_empty() {
                parts.push(Value::from($buf));
            }
        }
    }

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        match c {
            '(' => parse_impl(parts, chars),

            ')' => break,

            ' ' => {
                let old_buf = mem::replace(&mut buf, String::new());

                append_value!(old_buf);
            }

            c => {
                buf.push(c);
            }
        }
    }

    append_value!(buf);
}

pub fn parse(code: &str) -> Value {
    let mut parts = Vec::new();
    parse_impl(&mut parts, &mut code.chars());
    Value::from(parts)
}
