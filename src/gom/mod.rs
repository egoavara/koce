use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt::{Debug, Display, Error, Formatter};
use std::io::Cursor;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};

mod iterfn;
pub use self::iterfn::*;
use std::path::{Path, Component};
use std::ffi::OsStr;

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
            Some(ref x) => Some(Explorer::new(Rc::clone(x))),
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

pub trait PathMatcher{
    fn is_matched(&self, test : &OsStr)-> bool;
}

impl<T> Explorer<T> {
    pub fn new(curr: Handle<T>) -> Self {
        Explorer {
            curr,
        }
    }
    pub fn root(self) -> Self {
        let a = self.curr.borrow().parent.upgrade();
        match a {
            None => self,
            Some(some) => Explorer::new(some).root(),
        }
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
    pub fn follow_fn<P : AsRef<Path>, F : Fn(&T, &OsStr) -> bool>(self, p : P, matcher : F) -> Result<Self, Self>{
        p.as_ref().components().fold(Ok(self), |x, comp|{
            match comp {
                Component::RootDir => {
                    match x {
                        Ok(ok) => {Ok(ok.root())},
                        Err(err) => {Err(err)},
                    }
                },
                Component::Prefix(_) => {x},
                Component::CurDir => {x},
                Component::ParentDir => {
                    match x {
                        Ok(ok) => {ok.parent()},
                        Err(err) => {Err(err)},
                    }
                },
                Component::Normal(name) => {
                    match x {
                        Ok(ok) => {
                            ok.find_child(|x|matcher(x, name))
                        },
                        Err(err) => {Err(err)},
                    }
                },
            }
        })
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
    pub fn inside(&self) -> Ref<Node<T>> {
        self.curr.borrow()
    }
    pub fn inside_mut(&self) -> RefMut<Node<T>> {
        self.curr.borrow_mut()
    }
    pub fn depth(&self) -> usize {
        Self::util_depth(&self.curr)
    }
    pub fn util_depth(h: &Handle<T>)-> usize {
        match RefCell::borrow(h).parent.upgrade() {
            None => 0,
            Some(parent) => Self::util_depth(&parent) + 1,
        }
    }
}
impl<T : PathMatcher> Explorer<T> {
    pub fn follow<P : AsRef<Path>>(self, p : P) -> Result<Self, Self>{
        p.as_ref().components().fold(Ok(self), |x, comp|{
            match comp {
                Component::RootDir => {
                    match x {
                        Ok(ok) => {Ok(ok.root())},
                        Err(err) => {Err(err)},
                    }
                },
                Component::Prefix(_) => {x},
                Component::CurDir => {x},
                Component::ParentDir => {
                    match x {
                        Ok(ok) => {ok.parent()},
                        Err(err) => {Err(err)},
                    }
                },
                Component::Normal(name) => {
                    match x {
                        Ok(ok) => {
                            ok.find_child(|x|x.is_matched(name))
                        },
                        Err(err) => {Err(err)},
                    }
                },
            }
        })
    }
}
impl <T> Clone for Explorer<T>{
    fn clone(&self) -> Self {
        Self{
            curr : Rc::clone(&self.curr)
        }
    }
}
//impl<T> Iterator for Explorer<T> {
//    type Item = Handle<T>;
//    fn next(&mut self) -> Option<Self::Item> {
//        match &self.root {
//            None => {
//                self.root = Some(Rc::clone(&self.curr));
//                Some(Rc::clone(&self.curr))
//            },
//            Some(root) => {
//                let curr = Explorer::new(Rc::clone(&self.curr));
//                match curr.child(0) {
//                    Ok(child) => {
//                        self.curr = Rc::clone(&child.curr);
//                        Some(child.curr)
//                    }
//                    Err(curr) => {
//                        match curr.next_sibling() {
//                            Ok(next_sib) => {
//                                self.curr = Rc::clone(&next_sib.curr);
//                                Some(next_sib.curr)
//                            }
//                            Err(curr) => {
//                                match curr.parent() {
//                                    Ok(parent) => {
//                                        if Rc::ptr_eq(&parent.curr, root) {
//                                            self.root = None;
//                                            None
//                                        } else {
//                                            match parent.next_sibling() {
//                                                Ok(parent_next_sib) => {
//                                                    self.curr = Rc::clone(&parent_next_sib.curr);
//                                                    Some(parent_next_sib.curr)
//                                                }
//                                                Err(_) => {
//                                                    // Last Elem
//                                                    self.root = None;
//                                                    None
//                                                }
//                                            }
//                                        }
//                                    }
//                                    Err(_) => {
//                                        self.root = None;
//                                        None
//                                    }
//                                }
//                            }
//                        }
//                    }
//                }
//            },
//        }
//    }
//}

impl<T: Debug + Display> Display for Explorer<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let root_depth = self.depth();
        let mut a = Self::new(Rc::clone(&self.curr));
        for elem in a.iter(IterRule::Walk) {
            for _ in 0..Self::util_depth(&elem) - root_depth {
                f.write_str("    ")?;
            }
            Display::fmt(&elem.borrow().data, f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}