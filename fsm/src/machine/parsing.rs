use super::*;

impl Machine {
    pub fn parse_slow(&self, text: &str) -> Result<Vec<CapturedValue>, String> {
        fn insert_captured_value(
            captures: &mut Vec<CapturedValue>,
            record: PendingCapture,
            end_index: usize,
            text: &str,
        ) {
            let CapturePayload {
                capture_id,
                end_states: _,
                is_list: _,
            } = record.payload;

            let value = CapturedValue {
                capture_id,
                value: text[record.start_index..end_index].to_string(),
            };

            captures.push(value);
        }

        let mut captures = vec![];
        let mut pending_captures: HashSet<PendingCapture> = HashSet::new();
        let mut current_state = self.start_state;

        for (i, c) in text.chars().enumerate() {
            // -- Stop pending captures when ready --

            // Step 1: get captures that finished, and the rest
            let (to_be_stopped, _pending_captures) =
                pending_captures.into_iter().partition(|pending| {
                    pending
                        .payload
                        .end_states
                        .iter()
                        .any(|end_state| current_state > *end_state)
                }) as (HashSet<_>, HashSet<_>);

            // Step 2: stop them
            for record in to_be_stopped.clone() {
                insert_captured_value(&mut captures, record, i - 1, text);
            }

            // Step 3: update the list to remaining pendings
            pending_captures = _pending_captures;

            // -- Start capture when needed --
            if let Some(payload) = self.capture_table.get(&current_state) {
                pending_captures.insert(PendingCapture {
                    payload: (*payload).clone(),
                    start_index: i,
                });
            }

            // pull the next state
            if let Ok(state) = self.get_transition_at(&current_state, c) {
                if *state != ERROR_STATE {
                    current_state = *state;
                    continue;
                }
            }

            return Err(format!("invalid character '{}'", c));
        }

        if !self.final_states.contains(&current_state) {
            return Err("unexpected end of line".to_string());
        }

        // -- Collect the remaining pending captures, that finished just now --
        for record in pending_captures.iter().filter(|pending| {
            pending
                .payload
                .end_states
                .iter()
                .any(|end| current_state >= *end)
        }) {
            insert_captured_value(&mut captures, record.clone(), text.len() - 1, text);
        }

        Ok(captures)
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    fn err<T>(msg: &str) -> Result<T, String> {
        Err(msg.to_string())
    }

    #[test]
    fn string() -> Result<(), String> {
        let machine = Machine::from_path(Path::new().string("Hello").end())?;

        assert_eq!(machine.parse_slow("Hello"), Ok(vec![]));
        assert_eq!(machine.parse_slow("World"), err("invalid character 'W'"));
        assert_eq!(machine.parse_slow("Hell"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("Helloo"), err("invalid character 'o'"));
        assert_eq!(machine.parse_slow("ello"), err("invalid character 'e'"));
        assert_eq!(machine.parse_slow("Hellu"), err("invalid character 'u'"));

        Ok(())
    }

    #[test]
    fn one_of() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .one_of(vec![
                    Path::new().string("Hello"),
                    Path::new().string("Hellu"),
                    Path::new().string("World"),
                    Path::new().string("Wurld"),
                ])
                .end(),
        )?;
        assert_eq!(machine.parse_slow("Hello"), Ok(vec![]));
        assert_eq!(machine.parse_slow("Hellu"), Ok(vec![]));
        assert_eq!(machine.parse_slow("World"), Ok(vec![]));
        assert_eq!(machine.parse_slow("Wurld"), Ok(vec![]));
        assert_eq!(machine.parse_slow("Wurl"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("Wullo"), err("invalid character 'l'"));
        Ok(())
    }

    #[test]
    fn cycle() -> Result<(), String> {
        let machine = Machine::from_path(Path::new().cycle(Path::new().string("abc")).end())?;
        assert_eq!(machine.parse_slow(""), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("a"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("ab"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("ac"), err("invalid character 'c'"));
        assert_eq!(machine.parse_slow("abc"), Ok(vec![]));
        assert_eq!(machine.parse_slow("abcd"), err("invalid character 'd'"));
        assert_eq!(machine.parse_slow("abcc"), err("invalid character 'c'"));
        assert_eq!(machine.parse_slow("abcab"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("abcabc"), Ok(vec![]));
        Ok(())
    }

    #[test]
    fn optional() -> Result<(), String> {
        let machine = Machine::from_path(Path::new().optional(Path::new().string("abc")).end())?;
        assert_eq!(machine.parse_slow(""), Ok(vec![]));
        assert_eq!(machine.parse_slow("a"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("ab"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("abc"), Ok(vec![]));
        Ok(())
    }

    #[test]
    fn optional_and_cycle() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .cycle(Path::new().string("def"))
                .optional(Path::new().cycle(Path::new().string("abc")))
                .end(),
        )?;
        assert_eq!(machine.parse_slow(""), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("abc"), err("invalid character 'a'"));
        assert_eq!(machine.parse_slow("def"), Ok(vec![]));
        assert_eq!(machine.parse_slow("defdef"), Ok(vec![]));
        assert_eq!(machine.parse_slow("defdefa"), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("defdefabc"), Ok(vec![]));
        assert_eq!(machine.parse_slow("defdefabcabc"), Ok(vec![]));

        // TODO: this is failing:
        assert_eq!(
            machine.parse_slow("defdefabcabcdef"),
            err("invalid character 'd'")
        );
        Ok(())
    }

    // -- Captures

    #[test]
    fn capture_string2() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .string("Hello, ")
                .capture_text(String::from("who"), Path::new().string("World"))
                .string("!")
                .end(),
        )?;
        assert_eq!(machine.parse_slow(""), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("Hello, "), err("unexpected end of line"));
        assert_eq!(
            machine.parse_slow("Hello, World"),
            err("unexpected end of line")
        );

        assert_eq!(
            machine.parse_slow("Hello, World "),
            err("invalid character ' '")
        );

        assert_eq!(
            machine.parse_slow("Hello, World!"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "World".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, World! "),
            err("invalid character ' '")
        );

        Ok(())
    }

    #[test]
    fn capture_string1() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .string("Hello, ")
                .capture_text(String::from("who"), Path::new().string("World"))
                .end(),
        )?;
        assert_eq!(machine.parse_slow(""), err("unexpected end of line"));
        assert_eq!(machine.parse_slow("Hello, "), err("unexpected end of line"));
        assert_eq!(
            machine.parse_slow("Hello, World"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "World".to_string()
            }])
        );
        assert_eq!(
            machine.parse_slow("Hello, World "),
            err("invalid character ' '")
        );

        Ok(())
    }

    #[test]
    fn capture_one_of() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .string("Hello, ")
                .capture_text(
                    String::from("who"),
                    Path::new().one_of(vec![
                        Path::new().string("World"),
                        Path::new().string("Mum"),
                        Path::new().string("Dad"),
                    ]),
                )
                .end(),
        )?;
        assert_eq!(machine.parse_slow("Hello, "), err("unexpected end of line"));
        assert_eq!(
            machine.parse_slow("Hello, World"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "World".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Mum"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "Mum".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Dad"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "Dad".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Foo"),
            err("invalid character 'F'")
        );

        assert_eq!(
            machine.parse_slow("Hello, World "),
            err("invalid character ' '")
        );
        Ok(())
    }

    #[test]
    fn capture_cycle() -> Result<(), String> {
        let machine = Machine::from_path(
            Path::new()
                .string("Hello, ")
                .capture_text(
                    String::from("who"),
                    Path::new().cycle(
                        Path::new()
                            .one_of_chars("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                    ),
                )
                .end(),
        )?;
        assert_eq!(machine.parse_slow("Hello, "), err("unexpected end of line"));
        assert_eq!(
            machine.parse_slow("Hello, World"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "World".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Mum"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "Mum".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Dad"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "Dad".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, Foo"),
            Ok(vec![CapturedValue {
                capture_id: 0,
                value: "Foo".to_string()
            }])
        );

        assert_eq!(
            machine.parse_slow("Hello, World "),
            err("invalid character ' '")
        );
        Ok(())
    }
}
