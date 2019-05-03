use std::collections::HashMap;
use super::Path;

pub enum Permission {
    Open,
    Exclusive,
    Internal,
    Private,
}