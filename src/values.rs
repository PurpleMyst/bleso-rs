use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
    Integer(isize),
    Symbol(String),
    String(String),
    Cons(Cons),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Cons {
    Empty,
    Cons(Rc<Value>, Option<Rc<Value>>),
}

#[allow(dead_code)]
impl Value {
    pub fn empty_list() -> Self {
        Value::Cons(Cons::Empty)
    }

    pub fn singleton(car: Rc<Value>) -> Value {
        Value::Cons(Cons::Cons(car, None))
    }

    pub fn cons(car: Rc<Value>, cdr: Rc<Value>) -> Value {
        Value::Cons(Cons::Cons(car, Some(cdr)))
    }

    pub fn car(&self) -> Rc<Value> {
        match self {
            &Value::Cons(Cons::Empty) => panic!("Can't get the car of an empty cons"),
            &Value::Cons(Cons::Cons(ref car, _)) => Rc::clone(car),
            _ => panic!("Can't get the car of a non-cons value"),
        }
    }
}

// This Display insitrait is mostly for debugging.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Integer(ref n) => write!(f, "{}", n),

            &Value::Symbol(ref s) => write!(f, "{}", s),

            &Value::String(ref s) => write!(f, "{:?}", s),

            &Value::Cons(Cons::Cons(ref car, ref cdr)) => {
                write!(f, "({}", car)?;

                if let &Some(ref cdr) = cdr {
                    write!(f, " {}", cdr)?;
                }

                write!(f, ")")
            }

            &Value::Cons(Cons::Empty) => write!(f, "()"),
        }
    }
}
