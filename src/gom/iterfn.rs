use gom::{Explorer, Handle};
use std::rc::Rc;

impl<T> Explorer<T>{
    pub fn iter(&self, rule : IterRule) ->Iter<T>{
        Iter{
            curr : Rc::clone(&self.curr),
            work: None,
            rule,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IterRule{
    Walk,
    Hierarchy,
    Parents,
    Children,
    Siblings,
}
pub struct Iter<T>{
    rule : IterRule,
    curr: Handle<T>,
    work : Option<Handle<T>>,
}

impl <T>Iter<T> {
    pub fn rule(&self) -> IterRule{
        self.rule
    }
    fn next_siblings(&mut self) -> Option<<Iter<T> as Iterator>::Item>{
        match self.work {
            None => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.parent() {
                    Ok(parent) => {
                        self.work = Some(Rc::clone(&self.curr));
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.work = None;
                        None
                    },
                }
            },
            Some(ref revert) => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.next_sibling() {
                    Ok(sib) => {
                        self.curr = sib.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.curr = Rc::clone(revert);
                        self.work = None;
                        None
                    },
                }
            },
        }
    }
    fn next_hierarchy(&mut self) -> Option<<Iter<T> as Iterator>::Item>{
        match self.work {
            None => {
                self.work = Some(Rc::clone(&self.curr));
                Some(Rc::clone(&self.curr))
            },
            Some(ref revert) => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.parent() {
                    Ok(parent) => {
                        self.curr = parent.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.curr = Rc::clone(revert);
                        self.work = None;
                        None
                    },
                }
            },
        }
    }
    fn next_parent(&mut self) -> Option<<Iter<T> as Iterator>::Item>{
        match self.work {
            None => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.parent() {
                    Ok(parent) => {
                        self.work = Some(Rc::clone(&self.curr));
                        self.curr = parent.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.work = None;
                        None
                    },
                }
            },
            Some(ref revert) => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.parent() {
                    Ok(parent) => {
                        self.curr = parent.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.curr = Rc::clone(revert);
                        self.work = None;
                        None
                    },
                }
            },
        }
    }
    fn next_children(&mut self) -> Option<<Iter<T> as Iterator>::Item>{
        match self.work {
            None => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.child(0) {
                    Ok(child) => {
                        self.work = Some(Rc::clone(&self.curr));
                        self.curr = child.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.work = None;
                        None
                    },
                }
            },
            Some(ref revert) => {
                let temp = Explorer::new(Rc::clone(&self.curr));
                match temp.next_sibling() {
                    Ok(next) => {
                        self.curr = next.curr;
                        Some(Rc::clone(&self.curr))
                    },
                    Err(_) => {
                        self.curr = Rc::clone(revert);
                        self.work = None;
                        None
                    },
                }
            },
        }
    }
    fn next_walk(&mut self) -> Option<<Iter<T> as Iterator>::Item> {
        match &self.work {
            None => {
                self.work = Some(Rc::clone(&self.curr));
                Some(Rc::clone(&self.curr))
            },
            Some(root) => {
                let curr = Explorer::new(Rc::clone(&self.curr));
                match curr.child(0) {
                    Ok(child) => {
                        self.curr = Rc::clone(&child.curr);
                        Some(child.curr)
                    }
                    Err(curr) => {
                        match curr.next_sibling() {
                            Ok(next_sib) => {
                                self.curr = Rc::clone(&next_sib.curr);
                                Some(next_sib.curr)
                            }
                            Err(curr) => {
                                match curr.parent() {
                                    Ok(parent) => {
                                        if Rc::ptr_eq(&parent.curr, root) {
                                            self.curr = Rc::clone(root);
                                            self.work = None;
                                            None
                                        } else {
                                            match parent.next_sibling() {
                                                Ok(parent_next_sib) => {
                                                    self.curr = Rc::clone(&parent_next_sib.curr);
                                                    Some(parent_next_sib.curr)
                                                }
                                                Err(_) => {
                                                    // Last Elem
                                                    self.curr = Rc::clone(root);
                                                    self.work = None;
                                                    None
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        self.curr = Rc::clone(root);
                                        self.work = None;
                                        None
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

impl <T>Iterator for Iter<T>{
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rule {
            IterRule::Walk => self.next_walk(),
            IterRule::Hierarchy => self.next_hierarchy(),
            IterRule::Parents => self.next_parent(),
            IterRule::Children => self.next_children(),
            IterRule::Siblings => self.next_siblings(),
        }
    }
}