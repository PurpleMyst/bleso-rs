use std::rc::Rc;
use std::fmt;
use std::mem;

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

impl From<Vec<Rc<Value>>> for Value {
    fn from(mut vec: Vec<Rc<Value>>) -> Self {
        let mut conductor = Rc::try_unwrap(vec.pop().unwrap()).unwrap();

        vec.reverse();
        for value in vec.into_iter() {
            // XXX: Is there something better we can pass as a second argument? Constructing a new
            // list seems wasteful, honestly.
            let old_conductor = mem::replace(&mut conductor, Value::empty_list());
            let new_conductor = Value::cons(value, Rc::new(old_conductor));
            conductor = new_conductor;
        }

        conductor
    }
}

// This Display trait is mostly for debugging.
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
