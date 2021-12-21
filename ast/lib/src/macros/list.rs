#[macro_export]
macro_rules! list {
    ($buffer:expr, $node:ty $(, $separator:pat)?) => {
        {
            let mut items = vec![];
            loop {
                match <$node>::parse($buffer) {
                    Ok(item) => {
                        items.push(item);
                        $(
                            if token!($buffer, $separator).is_err() {
                                break Ok(items);
                            }
                        )?
                    },
                    Err(err) => {
                        if items.len() == 0 {
                            break Err(err);
                        } else {
                            break Ok(items);
                        }
                    }
                }
            }
        }
    };
}
