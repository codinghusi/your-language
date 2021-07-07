
#[macro_export]
macro_rules! spanned {
    ($buffer:expr, {body => {$($body:tt)*}, ($span:ident) => $return:expr}) => {
        {
            let start = $buffer.peek_span().start.clone();
            $($body)*
            let end = $buffer.span().end.clone();
            let $span = start..end;
            Ok($return)
        }
    };
}

