mod values;
mod parser;

use std::rc::Rc;
use values::Value;

fn main() {
    let root = parser::parse("(1 (2 (3 (4)))) (1 (2 (3 4)))");
    println!("{}", root);
}
