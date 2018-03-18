use std::mem;
use std::str::Chars;

use values::Value;


fn parse_impl(chars: &mut Chars) -> Value {
    let mut parts = Vec::new();
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
            '(' => parts.push(parse_impl(chars)),

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
    Value::from(parts)
}

pub fn parse(code: &str) -> Value {
    parse_impl(&mut code.chars())
}
