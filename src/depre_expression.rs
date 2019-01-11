
//
//

//
//named!(pub koce_constant<CompleteStr, OpExpr>, alt!(koce_rawbytes | koce_integer | koce_float | koce_literal));
//
//fn koce_opexpr_variable(data : CompleteStr) -> OpExpr {
//    return OpExpr::Identifiers(data.0.to_owned())
//}
//fn is_name_body(c : char) -> bool{
//    match c {
//        c if c.is_alphanumeric() => true,
//        '_' => true,
//        _ => false,
//    }
//}
//named!(pub koce_name<CompleteStr, OpExpr>,map!(
//    recognize!(do_parse!(
//        alpha1 >>
//        take_while!(is_name_body) >>
//        ()
//    )),
//    koce_opexpr_variable
//));
//named!(pub koce_variable<CompleteStr, OpExpr>,map!(
//    recognize!(do_parse!(
//        alpha1 >>
//        take_while!(is_name_body) >>
//        ()
//    )),
//    koce_opexpr_variable
//));
//
//#[derive(Debug)]
//struct BinaryOperator{
//    operator : char,
//    left_opexpr : OpExpr,
//    right_opexpr : OpExpr,
//}
//fn p_koce_opexpr_bin(data : BinaryOperator) -> Result<OpExpr, char>{
//    match data.operator {
//        '+' => Ok(OpExpr::Add(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '-' => Ok(OpExpr::Sub(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '*' => Ok(OpExpr::Mul(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '/' => Ok(OpExpr::Div(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '%' => Ok(OpExpr::Mod(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '&' => Ok(OpExpr::And(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '|' => Ok(OpExpr::Or(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        '^' => Ok(OpExpr::Xor(Box::new(data.left_opexpr), Box::new(data.right_opexpr))),
//        _ => Err(data.operator),
//    }
//}
//
//
//
//named!(pub koce_opexpr_bin<CompleteStr, OpExpr>,
//map_res!(
//    do_parse!(
//        left_opexpr : koce_opexpr_bin_left >>
//        multispace0 >>
//        operator:one_of!("+-*/%=&|^") >>
//        multispace0 >>
//        right_opexpr: koce_opexpr_bin_right >>
//        (BinaryOperator{left_opexpr : left_opexpr, operator : operator, right_opexpr: right_opexpr})
//    ),
//    p_koce_opexpr_bin
//));
//named!(koce_opexpr_bin_left<CompleteStr, OpExpr>,
//alt!(
//    koce_opexpr_unary | koce_value
//));
//named!(koce_opexpr_bin_right<CompleteStr, OpExpr>,
//alt!(
//    koce_opexpr_bin | koce_opexpr_unary | koce_value
//));
//
//#[derive(Debug)]
//struct UnaryOperator{
//    operator : char,
//    opexpr : OpExpr,
//}
//fn p_koce_opexpr_unary(data : UnaryOperator) -> Result<OpExpr, char>{
//    match data.operator {
//        '+' => Ok(OpExpr::Positive(Box::new(data.opexpr))),
//        '-' => Ok(OpExpr::Negative(Box::new(data.opexpr))),
//        '!' => Ok(OpExpr::Not(Box::new(data.opexpr))),
//        _ => Err(data.operator),
//    }
//}
//named!(pub koce_opexpr_unary<CompleteStr, OpExpr>,
//map_res!(
//    do_parse!(
//        operator:one_of!("+-!") >>
//        multispace0 >>
//        opexpr : koce_value >>
//        (UnaryOperator{opexpr : opexpr, operator : operator})
//    ),
//    p_koce_opexpr_unary
//));
//
//named!(pub koce_opexpr_call<CompleteStr, OpExpr>,
//    do_parse!(
//        cmd : koce_opexpr_words >>
//        args : koce_opexpr_vec_tuple >>
//        (OpExpr::Call(Box::new(cmd), args))
//    )
//);
//
//
//named!(pub koce_value<CompleteStr, OpExpr>,
//    alt!(koce_variable | koce_constant | koce_opexpr_tuple)
//);
//
//named!(pub koce_opexpr_words<CompleteStr, OpExpr>,
//alt!(
//    koce_opexpr_unary | koce_opexpr_bin | koce_value
//));
