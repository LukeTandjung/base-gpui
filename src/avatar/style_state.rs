/// The image loading status shared by every avatar part, mirroring Base UI's
/// `imageLoadingStatus` render state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarImageLoadingStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AvatarRootStyleState {
    pub image_loading_status: AvatarImageLoadingStatus,
}

impl AvatarRootStyleState {
    pub fn new(image_loading_status: AvatarImageLoadingStatus) -> Self {
        Self {
            image_loading_status,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AvatarImageStyleState {
    pub image_loading_status: AvatarImageLoadingStatus,
}

impl AvatarImageStyleState {
    pub fn new(image_loading_status: AvatarImageLoadingStatus) -> Self {
        Self {
            image_loading_status,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AvatarFallbackStyleState {
    pub image_loading_status: AvatarImageLoadingStatus,
}

impl AvatarFallbackStyleState {
    pub fn new(image_loading_status: AvatarImageLoadingStatus) -> Self {
        Self {
            image_loading_status,
        }
    }
}
