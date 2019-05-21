use std::fmt::{Display, Error, Formatter, Debug};
use std::ops::Drop;
use std::ptr;
use std::ptr::null_mut;

mod code;
use self::code::*;
#[derive(Debug)]
pub struct TreeGraph<T> {
    pub data: T,
    ptr_parent: *mut TreeGraph<T>,
    ptr_children: Vec<*mut TreeGraph<T>, >,
}

impl<T> TreeGraph<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            ptr_parent: ptr::null_mut(),
            ptr_children: Vec::new(),
        }
    }
    unsafe fn new_root(data: T) -> Self {
        Self {
            data,
            ptr_parent: null_mut(),
            ptr_children: Vec::new(),
        }
    }
    pub fn append<'a, 'b>(&'a mut self, data: T) -> &'b mut Self {
        unsafe {
            let temp = Box::into_raw(Box::new(unsafe { Self::new_root(data) }));
            (*temp).ptr_parent = self;
            self.ptr_children.push(temp);
            &mut *temp
        }
    }
    pub fn is_root(&self) -> bool {
        self.ptr_parent == null_mut()
    }
    pub fn root(&self) -> & Self {
        match self.parent() {
            None => {
                self
            },
            Some(some) => {
                some.root()
            },
        }
    }
    pub fn root_mut(&mut self) -> & mut Self {
        match self.parent_mut() {
            None => {
                self
            },
            Some(some) => {
                some.root_mut()
            },
        }
    }
    pub fn is_parent(&self) -> bool {
        self.ptr_parent != null_mut()
    }

    pub fn parent<'a, 'b>(&'a self) -> Option<&'b Self> {
        if self.is_parent() {
            Some(unsafe { &*self.ptr_parent })
        } else {
            None
        }
    }
    pub fn parent_mut<'a, 'b>(&'a self) -> Option<&'b mut Self> {
        if self.is_parent() {
            Some(unsafe { &mut *self.ptr_parent })
        } else {
            None
        }
    }

    pub fn len_children(&self) -> usize {
        self.ptr_children.len()
    }
    pub fn child<'a, 'b>(&'a self, nth: usize) -> Option<&'b Self> {
        self.ptr_children.get(nth).map(|&x| {
            unsafe { &*x }
        })
    }
    pub fn child_mut<'a, 'b>(&'a mut self, nth: usize) -> Option<&'b mut Self> {
        self.ptr_children.get(nth).map(|&x| {
            unsafe { &mut *x }
        })
    }
    pub fn children<'a, 'b>(&'a self) -> Vec<&'b Self> {
        self.ptr_children.iter().map(|&x| unsafe { &*x }).collect()
    }
    pub fn children_mut<'a, 'b>(&'a mut self) -> Vec<&'b mut Self> {
        self.ptr_children.iter().map(|&x| unsafe { &mut *x }).collect()
    }
    pub fn remove(&mut self) {
        unsafe {
            (*self.ptr_parent).ptr_children.retain(|&x| {
                !ptr::eq(x, self)
            });
            Box::from_raw(self);
        }
    }
    pub fn depth(&self) -> usize {
        match self.parent() {
            None => {
                0
            },
            Some(parent) => {
                1 + parent.depth()
            },
        }
    }
}

impl<T> Drop for TreeGraph<T> {
    fn drop(&mut self) {
        for x in &self.ptr_children {
            unsafe {
                Box::from_raw(*x);
            }
        }
    }
}

impl<T: Display> Display for TreeGraph<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn inner<T: Display>(s : &TreeGraph<T>, f: &mut Formatter, from : usize)  -> Result<(), Error>{
            for _ in 0..s.depth() - from{
                f.write_str("    ")?;
            }
            f.write_fmt(format_args!("{}\n", s.data))?;
            for x in s.children() {
                inner(x, f, from)?;
            }
            Ok(())
        }
        inner(self, f, self.depth())
    }
}