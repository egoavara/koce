use std::ops::Deref;
use std::fmt::{Display, Formatter, Error};
use koce::{Expression, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Path {
    Root,
    Current,
    Child(Box<Path>, String),
    Temporary(Box<Path>, usize),
}
pub enum PathError{
    PathRuleViolation,
}
impl Path {
    pub fn from_expression(expr: &Expression) -> Result<Self, PathError> {
        pub fn inner(expr: &Expression) -> Result<Path, ParserError> {
            match expr {
                Expression::Argument(Value::Name(name)) => {
                    Ok(Path::Child(Box::new(Path::Current), name.clone()))
                }
                Expression::Member(l, r) => {
                    Ok(inner(l.deref())?.append(inner(r.deref())?))
                }
                Expression::Cast(_, _) => {
                    //TODO
                    unimplemented!()
                }
                _ => Err(ParserError::PathRuleViolation)
            }
        }
        inner(expr)
    }
}
impl Path {
    pub fn is_absolute(&self) -> bool{
        match self {
            Path::Root => true,
            Path::Current => false,
            Path::Child(prev, _) => (*prev).is_absolute(),
            Path::Temporary(prev, _) => (*prev).is_absolute(),
        }
    }
    pub fn append(self, other: Path) -> Self {
        match other {
            Path::Root => Path::Root,
            Path::Current => self,
            Path::Child(prev, value) => Path::Child(Box::new(self.append(*prev)), value),
            Path::Temporary(prev, value) => Path::Temporary(Box::new(self.append(*prev)), value),
        }
    }
    pub fn child(self, n: &str) -> Self {
        let mut v = Vec::new();
        v.push(1);
        Path::Child(Box::new(self), n.to_string())
    }

    fn is_local(&self) -> bool{
        match self {
            Path::Child(parent, _) | Path::Temporary(parent, _) => match AsRef::as_ref(parent) {
                Path::Current => true ,
                _ => false
            },
            _ => false
        }
    }
    pub fn get_local(&self) -> Option<String>{
        match self {
            Path::Child(parent, v)=> match AsRef::as_ref(parent) {
                Path::Current => Some(v.clone()),
                _ => None
            },
            _ => None
        }
    }
    pub fn get_temporary(&self) -> Option<usize>{
        match self {
            Path::Temporary(parent, v)=> match AsRef::as_ref(parent) {
                Path::Current => Some(*v),
                _ => None
            },
            _ => None
        }
    }
}
impl Display for Path{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.is_local(){
            match self {
                Path::Child(_, n) => {
                    return f.write_fmt(format_args!("{}", n))
                }
                Path::Temporary(_, n) => {
                    return f.write_fmt(format_args!("${}", n))
                }
                _ => {}
            }
        }
        match self {
            Path::Root => {
                f.write_str("")
            },
            Path::Current => {
                f.write_str(".")
            },
            Path::Child(prev, value) => {
                f.write_fmt(format_args!("{}/{}", *prev, value))
            },
            Path::Temporary(prev, value) => {
                f.write_fmt(format_args!("{}/${}", *prev, value))
            },
        }
    }
}