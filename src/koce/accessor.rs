use nom::types::CompleteStr;
#[derive(Debug)]
pub enum Accessor {
    Public,
    Package,
    Private,
}



named!(pub parse_accessor<CompleteStr, Accessor>,
    alt!(
        map!(tag!("pub"), |_|Accessor::Public)
        | map!(tag!("pkg"), |_|Accessor::Package)
        | map!(tag!("pri"), |_|Accessor::Private)
    )
);
