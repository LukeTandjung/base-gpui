use crate::avatar::{AvatarChild, AvatarContext};

pub struct WiredAvatarChildren {
    pub image_key: Option<u64>,
    pub children: Vec<AvatarChild>,
}

pub trait AvatarChildNode: Sized {
    fn with_avatar_context(self, context: AvatarContext) -> Self;

    fn wire_avatar_child(self, _wiring: &mut AvatarChildWiring) -> Self {
        self
    }
}

pub struct AvatarChildWiring {
    image_key: Option<u64>,
}

impl AvatarChildWiring {
    fn new() -> Self {
        Self { image_key: None }
    }

    pub fn register_image(&mut self, source_key: u64) {
        self.image_key = Some(source_key);
    }

    fn finish(self, children: Vec<AvatarChild>) -> WiredAvatarChildren {
        WiredAvatarChildren {
            image_key: self.image_key,
            children,
        }
    }
}

pub fn wire_children(children: Vec<AvatarChild>, context: AvatarContext) -> WiredAvatarChildren {
    let mut wiring = AvatarChildWiring::new();
    let children = children
        .into_iter()
        .map(|child| child.wire_avatar_child(&mut wiring))
        .map(|child| child.with_avatar_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl AvatarChildNode for AvatarChild {
    fn with_avatar_context(self, context: AvatarContext) -> Self {
        match self {
            Self::Image(image) => Self::Image(image.with_avatar_context(context)),
            Self::Fallback(fallback) => Self::Fallback(fallback.with_avatar_context(context)),
            Self::Any(element) => Self::Any(element),
        }
    }

    fn wire_avatar_child(self, wiring: &mut AvatarChildWiring) -> Self {
        match self {
            Self::Image(image) => Self::Image(image.wire_avatar_child(wiring)),
            other => other,
        }
    }
}
