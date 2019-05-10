use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};

use koce::ast::{Expression, Value};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Path {
    Root,
    Current,
    Next(Box<Path>, String),
    Specify(Box<Path>, String),
    Temporary(Box<Path>, usize),
}

impl Path {
    pub fn from_expr(expr: &Expression) -> Option<Self> {
        fn inner(fold : Path, expr: &Expression) -> Option<Path> {
            match expr {
                Expression::Argument(v) => {
                    match v {
                        Value::Name(n) => {
                            Some(Path::Current.next(n.as_str()))
                        }
                        _ => { None }
                    }
                }
                Expression::Member(lvs, rvs) => {
                    match AsRef::as_ref(rvs) {
                        Expression::Argument(v) => {
                            match v {
                                Value::Name(n) => {
                                    Some(inner(fold, AsRef::as_ref(lvs))?.next(n.as_str()))
                                }
                                _ => { None }
                            }
                        },
                        Expression::Member(_, _) | Expression::Cast(_, _)=> {
                            Some(inner(inner(fold, AsRef::as_ref(lvs))?, AsRef::as_ref(rvs))?)
                        },
                        _ => {None}
                    }
                }
                Expression::Cast(lvs, rvs) => {
                    match AsRef::as_ref(rvs) {
                        Expression::Argument(v) => {
                            match v {
                                Value::Name(n) => {
                                    Some(inner(fold, AsRef::as_ref(lvs))?.specify(n.as_str()))
                                }
                                _ => { None }
                            }
                        },
                        Expression::Member(_, _) | Expression::Cast(_, _)=> {
                            Some(inner(inner(fold, AsRef::as_ref(lvs))?, AsRef::as_ref(rvs))?)
                        },
                        _ => {None}
                    }
                }
                _ => None
            }
        };
        inner(Path::Current, expr)
    }
    pub fn append(self, d: Self) -> Self {
        fn inner(dst: Path, src: Path) -> Path {
            match src {
                Path::Root => {
                    Path::Root
                }
                Path::Current => {
                    dst
                }
                Path::Next(p, v) => {
                    Path::Next(Box::new(inner(dst, *p)), v)
                }
                Path::Specify(p, v) => {
                    Path::Specify(Box::new(inner(dst, *p)), v)
                }
                Path::Temporary(p, v) => {
                    Path::Temporary(Box::new(inner(dst, *p)), v)
                }
            }
        }
        inner(self, d)
    }
    pub fn next(self, name: &str) -> Self {
        Path::Next(Box::new(self), name.to_string())
    }
    pub fn specify(self, name: &str) -> Self {
        Path::Specify(Box::new(self), name.to_string())
    }
    pub fn temporary(self, num: usize) -> Self {
        Path::Temporary(Box::new(self), num)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Path::Root => {
                Ok(())
            }
            Path::Current => {
                f.write_str(".")
            }
            Path::Next(prev, path) => {
                prev.fmt(f)?;
                f.write_fmt(format_args!("/{}", path))
            }
            Path::Temporary(prev, temp) => {
                prev.fmt(f)?;
                f.write_fmt(format_args!("/${}", temp))
            }
            Path::Specify(prev, to) => {
                prev.fmt(f)?;
                f.write_fmt(format_args!("@{}", to))
            }
        }
    }
}

impl Hash for Path{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(format!("{}", self).as_bytes());
    }
}