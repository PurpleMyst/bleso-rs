use std::rc::Rc;
use std::fmt;
use std::mem;

#[derive(Debug)]
#[allow(dead_code)]
/// A Bleso value. The Cons enum variant is meant to be a black box, that everything except Value
/// itself should interact with only via the public methods provided.
pub enum Value {
    Integer(isize),
    Symbol(String),

    Cons(Option<(Rc<Value>, Option<Rc<Value>>)>),
}

#[allow(dead_code)]
impl Value {
    // cons handling {{{1
    pub fn empty_cons() -> Self {
        Value::Cons(None)
    }

    pub fn cons(car: Rc<Value>, cdr: Rc<Value>) -> Value {
        Value::Cons(Some((car, Some(cdr))))
    }

    pub fn car(&self) -> Rc<Value> {
        match self {
            &Value::Cons(None) => panic!("Can't get the car of an empty cons"),
            &Value::Cons(Some((ref car, _))) => Rc::clone(car),
            _ => panic!("Can't get the car of a non-cons value"),
        }
    }

    pub fn cdr(&self) -> Rc<Value> {
        match self {
            &Value::Cons(Some((_, Some(ref cdr)))) => Rc::clone(cdr),
            &Value::Cons(_) => panic!("Can't get the cdr of a cons that doesn't have one!"),
            _ => panic!("Can't get the car of a non-cons value"),
        }
    }
    // 1}}}
}

impl From<Vec<Value>> for Value {
    fn from(mut vec: Vec<Value>) -> Self {
        let mut conductor = vec.pop().unwrap();

        vec.reverse();
        for value in vec.into_iter() {
            let old_conductor = mem::replace(&mut conductor, Value::empty_cons());
            let new_conductor = Value::cons(Rc::new(value), Rc::new(old_conductor));
            conductor = new_conductor;
        }

        conductor
    }
}

impl From<String> for Value {
    fn from(buf: String) -> Self {
        // TODO: Add strings.
        if let Ok(num) = buf.parse::<isize>() {
            Value::Integer(num)
        } else {
            Value::Symbol(buf)
        }
    }
}

// This Display trait is mostly for debugging.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Integer(ref n) => write!(f, "{}", n),

            &Value::Symbol(ref s) => write!(f, "{}", s),

            &Value::Cons(Some((ref car, ref cdr))) => {
                write!(f, "({}", car)?;

                if let &Some(ref cdr) = cdr {
                    write!(f, " {}", cdr)?;
                }

                write!(f, ")")
            }

            &Value::Cons(None) => write!(f, "()"),
        }
    }
}

// vim: foldmethod=marker
