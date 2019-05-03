use std::fmt::{Display, Formatter, Error};
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Path {
    Root,
    Current,
    Next(Box<Path>, String),
}
impl Path {
    pub fn local(name: &str) -> Self{
        Path::Root.append(name)
    }
    pub fn append(self, name: &str) -> Self {
        Path::Next(Box::new(self), name.to_string())
    }
    pub fn extends(self, names: Vec<&str>) -> Self {
        names.into_iter().fold(self, |res, x| {
            res.append(x)
        })
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
                f.write_str("/")?;
                f.write_str(path.as_str())
            }
        }
    }
}
impl Hash for Path{
    fn hash<H: Hasher>(&self, state: &mut H) {

    }
}
