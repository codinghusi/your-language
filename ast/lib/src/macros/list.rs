
#[macro_export]
macro_rules! list {
    ($buffer:expr, $node:ty $(, $separator:pat)?) => {
        {
            let mut items = vec![];
            while let Ok(item) = lib::maybe_throw!(<$node>::parse($buffer)) {
                items.push(item);
                $(
                    if token!($buffer, $separator).is_err() {
                        break;
                    }
                )?
            }
            items
        }
    };
}