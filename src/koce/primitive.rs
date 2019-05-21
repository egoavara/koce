

#[derive(Debug)]
pub enum Type {
    // padding
    Void(usize),
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Pointer,
    Complex(Vec<Type>),
    Array(Vec<Type>),
}

#[derive(Debug, Clone)]
pub enum Raw {
    Void,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Pointer(usize),
    Complex(Vec<Raw>),
    Array(Vec<Raw>),
}