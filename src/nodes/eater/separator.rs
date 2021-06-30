use crate::nodes::eater::Eater;

pub enum Whitespace {
    Optional,
    Required,
    NotAllowed
}

// FIXME: Names are to similar (SeparatedEater and SeperatorEater)
pub struct SeparatedEater {
    separator_before: SeparationEater,
    eater: Eater
}

pub struct SeparationEater {
    whitespace: Whitespace,
    require_following_eater: bool
}

impl SeparationEater {
    pub fn fromRaw(raw: &str) -> SeparationEater {
        match raw {
            "->" => Self {
                whitespace: Whitespace::Optional,
                require_following_eater: true
            },

            "->>" => Self {
                whitespace: Whitespace::Required,
                require_following_eater: true
            },

            "-!>" => Self {
                whitespace: Whitespace::NotAllowed,
                require_following_eater: true
            },

            "~>" => Self {
                whitespace: Whitespace::Optional,
                require_following_eater: false
            },

            "~>>" => Self {
                whitespace: Whitespace::Required,
                require_following_eater: false
            },

            "~!>" => Self {
                whitespace: Whitespace::NotAllowed,
                require_following_eater: false
            },


            _ => panic!(format!("{} is not a separator eater", raw))
        }
    }
}