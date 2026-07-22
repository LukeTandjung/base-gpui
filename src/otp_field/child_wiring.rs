use crate::otp_field::{OTPFieldChild, OTPFieldContext};

pub struct WiredChildren {
    pub children: Vec<OTPFieldChild>,
    pub input_count: usize,
}

/// Attaches the context and assigns slot indices to `OTPFieldInput` children in
/// one pass; no index counters are threaded through parts.
pub fn wire_children(children: Vec<OTPFieldChild>, context: OTPFieldContext) -> WiredChildren {
    let mut slot_index = 0;
    let children = children
        .into_iter()
        .map(|child| match child {
            OTPFieldChild::Input(input) => {
                let wired = (*input)
                    .with_otp_field_context(context.clone())
                    .with_slot_index(slot_index);
                slot_index += 1;
                OTPFieldChild::Input(Box::new(wired))
            }
            other => other,
        })
        .collect();

    WiredChildren {
        children,
        input_count: slot_index,
    }
}
