mod values;
mod parser;

fn main() {
    println!("{}", parser::parse("(+ 1 2)"));
}
