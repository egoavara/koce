use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

use nom::types::CompleteStr;

use koce::{Expression, Value};
use std::slice::Iter;

#[derive(Debug, Clone)]
pub enum PathError {
    PathRuleViolation,
    PathExpressionFail,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathNode {
    Current,
    Parent,
    Node(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    raw: Vec<PathNode>
}

impl Path {
    pub fn new(raw: Vec<PathNode>) -> Self {
        Path { raw }
    }
    pub fn is_relative(&self) -> bool {
        match self.raw.get(0) {
            None => false,
            Some(some) => {
                match some {
                    PathNode::Node(_) => false,
                    _ => true
                }
            }
        }
    }
    pub fn root_local(&self) -> Option<String> {
        match self.raw.get(0) {
            None => None,
            Some(some) => {
                match some {
                    PathNode::Node(name) => Some(name.clone()),
                    _ => None
                }
            }
        }
    }
    pub fn from_expression(expr: &Expression) -> Result<Self, PathError> {
        match expr {
            Expression::Argument(Value::Name(name)) => {
                Ok(Path::new(vec![PathNode::Node(name.clone())]))
            }
            Expression::Member(l, r) => {
                Ok(Self::from_expression(l.deref())?.append(Self::from_expression(l.deref())?))
            }
            Expression::Cast(_, _) => {
                //TODO
                unimplemented!()
            }
            _ => Err(PathError::PathRuleViolation)
        }
    }
    pub fn from_expression_str(expr: &str) -> Result<Self, PathError> {
        let (left, expr) = super::parse_expr(CompleteStr(expr)).map_err(|_| PathError::PathExpressionFail)?;
        if left.0.len() > 0 {
            Err(PathError::PathExpressionFail)
        } else {
            Self::from_expression(&expr)
        }
    }
    pub fn add(mut self, node: PathNode) -> Self {
        self.raw.push(node);
        self
    }

    pub fn add_all(mut self, nodes: Vec<PathNode>) -> Self {
        self.raw.extend(nodes);
        self
    }

    pub fn append(mut self, p: Path) -> Self {
        self.raw.extend(p.raw);
        self
    }
    pub fn trim(self) -> Self {
        if self.is_relative() {
            Self::new(self.raw.into_iter().fold(vec![PathNode::Current], |mut res, x| {
                match x {
                    PathNode::Current => {}
                    PathNode::Parent => {
                        match res.len() {
                            1 => {
                                match res.get(0).unwrap() {
                                    PathNode::Current => { res = vec![PathNode::Parent] }
                                    PathNode::Parent => { res.push(PathNode::Parent) }
                                    PathNode::Node(_) => {
                                        res.remove(0);
                                        res.push(PathNode::Current)
                                    }
                                }
                            }
                            _ => {
                                res.remove(res.len() - 1);
                            }
                        }
                    }
                    PathNode::Node(_) => {res.push(x)}
                }
                res
            }))
        } else {
            Self::new(self.raw.into_iter().fold(Vec::new(), |mut res, x| {
                match x {
                    PathNode::Current => {}
                    PathNode::Parent => {
                        if res.len() > 0{ res.remove(res.len() - 1); }
                    }
                    PathNode::Node(_) => { res.push(x); }
                }
                res
            }))
        }
    }
    pub fn iter(&self) -> Iter<PathNode>{
        self.raw.iter()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(
            self.raw.iter().map(|x| {
                match x {
                    PathNode::Current => { "." }
                    PathNode::Parent => { ".." }
                    PathNode::Node(t) => { t.as_str() }
                }
            }).fold(String::new(), |res, x| {
                res + "/" + x
            }).as_str().chars().skip(if self.is_relative() {
                1
            } else {
                0
            }).collect::<String>().as_str()
        )
    }
}