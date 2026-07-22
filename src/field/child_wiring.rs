use crate::{
    field::{
        FieldChild, FieldContext, FieldControl, FieldDescription, FieldError, FieldItem,
        FieldItemChild, FieldLabel, FieldValidity,
    },
    input::Input,
    number_field::NumberFieldRoot,
    otp_field::OTPFieldRoot,
};

pub trait FieldChildNode: Sized {
    fn with_field_context(self, context: FieldContext) -> Self;
}

pub fn wire_children(children: Vec<FieldChild>, context: FieldContext) -> Vec<FieldChild> {
    children
        .into_iter()
        .map(|child| child.with_field_context(context.clone()))
        .collect()
}

pub fn wire_item_children(
    children: Vec<FieldItemChild>,
    context: FieldContext,
) -> Vec<FieldItemChild> {
    children
        .into_iter()
        .map(|child| child.with_field_context(context.clone()))
        .collect()
}

impl FieldChildNode for FieldChild {
    fn with_field_context(self, context: FieldContext) -> Self {
        match self {
            Self::Item(item) => Self::Item(item.with_field_context(context)),
            Self::Label(label) => Self::Label(label.with_field_context(context)),
            Self::Control(control) => Self::Control(control.with_field_context(context)),
            Self::Input(input) => Self::Input(input.with_field_context(context)),
            Self::Description(description) => {
                Self::Description(description.with_field_context(context))
            }
            Self::Error(error) => Self::Error(error.with_field_context(context)),
            Self::Validity(validity) => Self::Validity(validity.with_field_context(context)),
            Self::NumberField(number_field) => {
                Self::NumberField(number_field.with_field_context(context))
            }
            Self::OTPField(otp_field) => Self::OTPField(otp_field.with_field_context(context)),
            Self::Slider(slider) => Self::Slider(slider.with_field_context(context)),
            Self::CheckboxGroup(checkbox_group) => Self::CheckboxGroup(checkbox_group),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl FieldChildNode for FieldItemChild {
    fn with_field_context(self, context: FieldContext) -> Self {
        match self {
            Self::Label(label) => Self::Label(label.with_field_context(context)),
            Self::Control(control) => Self::Control(control.with_field_context(context)),
            Self::Input(input) => Self::Input(input.with_field_context(context)),
            Self::Description(description) => {
                Self::Description(description.with_field_context(context))
            }
            Self::Error(error) => Self::Error(error.with_field_context(context)),
            Self::Validity(validity) => Self::Validity(validity.with_field_context(context)),
            Self::NumberField(number_field) => {
                Self::NumberField(number_field.with_field_context(context))
            }
            Self::OTPField(otp_field) => Self::OTPField(otp_field.with_field_context(context)),
            Self::Slider(slider) => Self::Slider(slider.with_field_context(context)),
            Self::CheckboxGroup(checkbox_group) => Self::CheckboxGroup(checkbox_group),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl FieldChildNode for FieldItem {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for FieldLabel {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for FieldControl {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for Input {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for FieldDescription {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for FieldError {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for FieldValidity {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for NumberFieldRoot {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}

impl FieldChildNode for OTPFieldRoot {
    fn with_field_context(self, context: FieldContext) -> Self {
        self.with_field_context(context)
    }
}
