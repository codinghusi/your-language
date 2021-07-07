#[macro_export]
macro_rules! maybe_unwrap {
    ($value:expr, $match:pat => $return:expr) => {
        match $value {
            $match => Some($return),
            _ => None
        }
    };
}