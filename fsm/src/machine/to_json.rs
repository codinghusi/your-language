use super::Machine;
use super::*;

impl Machine {
    pub fn result_to_json(&self, result: &Vec<CapturedValue>) -> String {
        self.result_to_json_intern(&mut result.iter().peekable(), &self.capture_structure_root)
            .unwrap_or_else(|| "null".to_string())
    }

    fn result_to_json_intern<'a, I>(
        &self,
        result: &mut Peekable<I>,
        entrypoint: &HashMap<String, CaptureValue>,
    ) -> Option<String>
    where
        I: Iterator<Item = &'a CapturedValue> + 'a,
    {
        let mut is_null = true;
        let mut ret = String::new();
        ret += "{";
        ret += &entrypoint
            .iter()
            .map(|(key, value)| {
                let raw = self.result_to_json_value(result, value);
                let value = if let Some(value) = raw {
                    is_null = false;
                    value
                } else {
                    "null".to_string()
                };
                format!("\"{}\": {}", key, value)
            })
            .collect::<Vec<_>>()
            .join(",");
        ret += "}";

        if is_null {
            None
        } else {
            Some(ret)
        }
    }

    fn result_to_json_value<'a, I>(
        &self,
        result: &mut Peekable<I>,
        capture_value: &CaptureValue,
    ) -> Option<String>
    where
        I: Iterator<Item = &'a CapturedValue> + 'a,
    {
        if let None = result.peek() {
            return None;
        }
        match capture_value {
            CaptureValue::String(capture_id) => {
                if let Some(CapturedValue {
                    capture_id: c_id,
                    value,
                }) = result.peek()
                {
                    if c_id == capture_id {
                        result.next();
                        Some(format!("\"{}\"", value))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            // FIXME: nesting not working
            CaptureValue::List(list_item) => {
                let mut list = vec![];
                loop {
                    let value = self.result_to_json_value(result, &*list_item);
                    if let Some(value) = value {
                        list.push(value);
                    } else {
                        break;
                    }
                }
                Some(format!("[{}]", list.join(", ")))
            }

            CaptureValue::Map(map) => self.result_to_json_intern(result, map),
        }
    }
}
