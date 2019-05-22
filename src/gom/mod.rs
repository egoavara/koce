use std::cell::Cell;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use std::cell::Ref;
use std::cell::RefMut;

#[derive(Debug)]
pub struct GOM<T> {
    root: Option<Handle<T>>
}

impl<T> GOM<T> {
    pub fn new(root: Option<Handle<T>>) -> Self {
        GOM { root }
    }
    pub fn setup(t: T) -> Self {
        GOM {
            root: Some(Rc::new(RefCell::new(Node {
                parent: Weak::new(),
                children: Vec::new(),
                data: t,
            })))
        }
    }

    pub fn explore(&self) -> Option<Explorer<T>> {
        match self.root {
            None => None,
            Some(ref x) => Some(Explorer {
                curr: Rc::clone(x)
            }),
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    parent: WeakHandle<T>,
    children: Vec<Handle<T>>,
    pub data: T,
}

pub type Handle<T> = Rc<RefCell<Node<T>>>;
pub type WeakHandle<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Explorer<T> {
    curr: Handle<T>,
}

impl<T> Explorer<T> {
    pub fn new(curr: Handle<T>) -> Self {
        Explorer { curr }
    }
    pub fn parent(self) -> Result<Self, Self> {

        let a = self.curr.borrow().parent.upgrade();
        match a {
            None => Err(self),
            Some(some) => Ok(Explorer::new(some)),
        }
    }
    pub fn parent_or_else(self) -> Self {
        self.parent().unwrap_or_else(|x| x)
    }
    pub fn child(self, i: usize) -> Result<Self, Self> {
        let temp = self.curr.borrow().children.get(i).map(|x| Rc::clone(x));
        match temp {
            None => Err(self),
            Some(some) => Ok(Explorer::new(some)),
        }
    }
    pub fn child_or_else(self, i: usize) -> Self {
        self.child(i).unwrap_or_else(|x| x)
    }
    pub fn find_child<F>(self, f: F) -> Result<Self, Self> where F: Fn(&T) -> bool {
        let temp = self.curr.borrow().children.iter().find(|x| {
            f(&x.borrow().data)
        }).map(|x| Rc::clone(x));
        match temp {
            None => Err(self),
            Some(some) => Ok(Explorer::new(some)),
        }
    }
    pub fn find_child_or_else<F>(self, f: F) -> Result<Self, Self> where F: Fn(&T) -> bool {
        self.find_child(f).map(|x| x)
    }
    pub fn next_sibling(self) -> Result<Self, Self> {
        let a = self.curr.borrow().parent.upgrade();
        match a {
            None => Err(self),
            Some(par) => {
                match par.borrow().children.iter().enumerate().find_map(|(idx, elem)| {
                    if Rc::ptr_eq(&self.curr, elem) {
                        Some(idx)
                    } else {
                        None
                    }
                }) {
                    None => { unreachable!() }
                    Some(some) => {
                        match par.borrow().children.get(some + 1) {
                            None => Err(self),
                            Some(some) => Ok(Explorer::new(Rc::clone(some))),
                        }
                    }
                }
            }
        }
    }
    pub fn is_root(&self) -> bool {
        self.curr.borrow().parent.upgrade().is_none()
    }

    pub fn add_child(&self, t: T) -> Self {
        let res = Rc::new(RefCell::new(Node {
            parent: Rc::downgrade(&self.curr),
            children: Vec::new(),
            data: t,
        }));
        self.curr.borrow_mut().children.push(Rc::clone(&res));
        Explorer::new(res)
    }


    pub fn inside(&self) -> Ref<Node<T>>{
        self.curr.borrow()
    }
//    pub fn inside_mut(&self) -> RefMut<Node<T>>{
//        self.curr.borrow_mut()
//    }
}
//impl <T>DerefMut for Explorer<T>{
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.curr.borrow_mut().data
//    }
//}
impl<T: Debug> Iterator for Explorer<T> {
    type Item = Explorer<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = Explorer::new(Rc::clone(&self.curr));
        let curr = Explorer::new(Rc::clone(&self.curr));
        match curr.child(0) {
            Ok(child) => {
                self.curr = child.curr;
                Some(ret)
            }
            Err(curr) => {
                match curr.next_sibling() {
                    Ok(next_sib) => {
                        self.curr = next_sib.curr;
                        Some(ret)
                    }
                    Err(curr) => {
                        match curr.parent().unwrap().next_sibling() {
                            Ok(parent_next_sib) => {
                                self.curr = parent_next_sib.curr;
                                Some(ret)
                            }
                            Err(_) => {
                                // Last Elem
                                None
                            }
                        }
                    }
                }
            }
        }
    }
}