
use super::SymbolType;
use std::cell::RefCell;
use std::rc::Rc;


pub enum Symbol{
    Root(Vec<Rc<RefCell<Symbol>>>, SymbolType, String, Type),
    // parent, children, type, name
    Node(Rc<RefCell<Symbol>>, Vec<Rc<RefCell<Symbol>>>, SymbolType, String, Type),
    Leaf(Rc<RefCell<Symbol>>, SymbolType, String, Type)
}
pub struct Definition{

}
pub struct Implementation{

}