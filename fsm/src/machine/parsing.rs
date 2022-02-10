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

        println!("Capture Table: {:?}", self.capture_table);
        println!("Result: {:?}", captures);

        Ok(captures)
    }
}
