use koce::{Path, PathNode};

lazy_static! {
    pub static ref ARGUMENT : Path = Path::new(vec![
        PathNode::Node("core".to_string()),
        PathNode::Node("handle".to_string()),
        PathNode::Node("Argument".to_string()),
    ]);
    pub static ref NEGATIVE : Path = Path::new(vec![
        PathNode::Node("core".to_string()),
        PathNode::Node("handle".to_string()),
        PathNode::Node("Negative".to_string()),
    ]);
}