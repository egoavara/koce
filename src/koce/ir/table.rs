use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error, Write};

use super::{Path, Symbol};

use std::cell::{RefCell, Ref, RefMut};
use std::rc::{Rc, Weak};
use core::borrow::BorrowMut;

pub enum Permission {
    Open,
    Exclusive,
    Internal,
    Private,
}


#[derive(Debug)]
pub struct Table {
    pub data: Symbol,
    pub parent: Weak<RefCell<Table>>,
    pub children: Vec<Rc<RefCell<Table>>>,
}

impl Table {
    pub fn root(s: Symbol) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Table {
            data: s,
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }
    pub fn append(to: &Rc<RefCell<Self>>, s: Symbol) -> Rc<RefCell<Self>> {
        let temp = Rc::new(RefCell::new(Table {
            data: s,
            parent: Rc::downgrade(to),
            children: Vec::new(),
        }));
        RefCell::borrow_mut(to).children.push(Rc::clone(&temp));
        temp
    }
    pub fn remove(e: &Rc<RefCell<Self>>){
        if let Some(data) = RefCell::borrow(e).parent.upgrade(){
            RefCell::borrow_mut(&*data).children.retain(|x|{
                !Rc::ptr_eq(x, e)
            })
        }
    }
    pub fn depth(&self) -> usize {
        match self.parent.upgrade() {
            Some(e) => {
                e.borrow().depth() + 1
            }
            None => {
                0
            }
        }
    }
}
impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.fmt_n(f, 0)
    }
}
impl Table {
    fn fmt_n(&self, f: &mut Formatter, n: usize) -> Result<(), Error> {
        for i in 0..n {
            f.write_str("    ")?;
        }
        f.write_fmt(format_args!("{:?}", self.data))?;
        for x in &self.children {
            f.write_char('\n')?;
            x.borrow().fmt_n(f, n + 1)?;
        }
        Ok(())
    }
}