use super::Machine;
use super::*;

impl Machine {
    pub fn export_xstatejs(&self) -> String {
        let mut states = self.transition_table.iter().collect::<Vec<_>>();
        states.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        let states = states
            .iter()
            .map(|(state, func)| {
                let transitions = func
                    .iter()
                    .enumerate()
                    .filter(|(_, target)| **target != ERROR_STATE)
                    .map(|(c, target)| {
                        format!(
                            "'{}': '{}'",
                            match c as u8 as char {
                                '\n' => "⤶".to_string(),
                                '\r' => "\\r".to_string(),
                                '\t' => "↹".to_string(),
                                ' ' => "␣".to_string(),
                                c => c.to_string(),
                            },
                            target
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let state_type = if self.final_states.contains(state) {
                    "type: 'final'"
                } else {
                    ""
                };
                format!(
                    "\t'{}': {{ on: {{ {} }}, {} }}",
                    state, transitions, state_type
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r"import {{ createMachine }} from 'xstate';

const machine = createMachine({{
  id: 'machine',
  initial: '1',
  states: {{
{}
  }}
}});",
            states
        )
    }
}
