use std::time::Duration;

use crate::avatar::{
    AvatarFallbackStyleState, AvatarImageLoadingStatus, AvatarImageStyleState, AvatarRootStyleState,
};

/// The outcome of reporting an image loading status: whether the runtime's
/// status actually transitioned, so the reporting part knows to fire the user
/// callback exactly once per distinct transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AvatarStatusOutcome {
    changed: bool,
    status: AvatarImageLoadingStatus,
}

impl AvatarStatusOutcome {
    fn new(changed: bool, status: AvatarImageLoadingStatus) -> Self {
        Self { changed, status }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn status(&self) -> AvatarImageLoadingStatus {
        self.status
    }
}

/// Owns the shared image loading status and the fallback show-delay state for
/// one avatar. All status transitions are computed here, once.
#[derive(Clone, Debug)]
pub struct AvatarRuntime {
    status: AvatarImageLoadingStatus,
    image_key: Option<u64>,
    fallback_delay_armed: bool,
    fallback_delay_passed: bool,
}

impl Default for AvatarRuntime {
    fn default() -> Self {
        Self {
            status: AvatarImageLoadingStatus::Idle,
            image_key: None,
            fallback_delay_armed: false,
            fallback_delay_passed: true,
        }
    }
}

impl AvatarRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reconciles the identity of the mounted image source. A changed source
    /// (or a removed image part) resets the status machine to `Idle` so the
    /// status re-derives for the new source.
    pub fn sync_image(&mut self, image_key: Option<u64>) {
        if self.image_key != image_key {
            self.image_key = image_key;
            self.status = AvatarImageLoadingStatus::Idle;
        }
    }

    /// Records the loading status observed by the image part. Returns whether
    /// the status transitioned so the part can fire `on_loading_status_change`
    /// at most once per distinct transition.
    pub fn report_image_status(&mut self, status: AvatarImageLoadingStatus) -> AvatarStatusOutcome {
        if self.status == status {
            return AvatarStatusOutcome::new(false, status);
        }

        self.status = status;
        AvatarStatusOutcome::new(true, status)
    }

    /// Arms the fallback show-delay on first fallback mount. Returns the delay
    /// to wait when a timer should be started; `None` when the fallback may
    /// show immediately or a timer is already pending.
    pub fn arm_fallback_delay(&mut self, delay: Option<Duration>) -> Option<Duration> {
        if self.fallback_delay_armed {
            return None;
        }

        self.fallback_delay_armed = true;

        match delay {
            Some(delay) => {
                self.fallback_delay_passed = false;
                Some(delay)
            }
            None => {
                self.fallback_delay_passed = true;
                None
            }
        }
    }

    /// Records that the fallback show-delay has elapsed.
    pub fn fallback_delay_elapsed(&mut self) {
        self.fallback_delay_passed = true;
    }

    /// Answers whether the image part should render its image content.
    pub fn image_visible(&self) -> bool {
        self.status == AvatarImageLoadingStatus::Loaded
    }

    /// Answers whether the fallback part should render its content.
    pub fn fallback_visible(&self) -> bool {
        self.status != AvatarImageLoadingStatus::Loaded && self.fallback_delay_passed
    }

    /// Returns the style state for `AvatarRoot`.
    pub fn root_state(&self) -> AvatarRootStyleState {
        AvatarRootStyleState::new(self.status)
    }

    /// Returns the style state for `AvatarImage`.
    pub fn image_state(&self) -> AvatarImageStyleState {
        AvatarImageStyleState::new(self.status)
    }

    /// Returns the style state for `AvatarFallback`.
    pub fn fallback_state(&self) -> AvatarFallbackStyleState {
        AvatarFallbackStyleState::new(self.status)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::AvatarRuntime;
    use crate::avatar::AvatarImageLoadingStatus;

    #[test]
    fn status_transitions_from_idle_through_loading_to_loaded() {
        let mut runtime = AvatarRuntime::new();
        assert_eq!(
            runtime.root_state().image_loading_status,
            AvatarImageLoadingStatus::Idle
        );

        let outcome = runtime.report_image_status(AvatarImageLoadingStatus::Loading);
        assert!(outcome.changed());
        assert_eq!(
            runtime.image_state().image_loading_status,
            AvatarImageLoadingStatus::Loading
        );

        let outcome = runtime.report_image_status(AvatarImageLoadingStatus::Loaded);
        assert!(outcome.changed());
        assert!(runtime.image_visible());
        assert!(!runtime.fallback_visible());
    }

    #[test]
    fn status_transitions_from_idle_through_loading_to_error() {
        let mut runtime = AvatarRuntime::new();

        runtime.report_image_status(AvatarImageLoadingStatus::Loading);
        let outcome = runtime.report_image_status(AvatarImageLoadingStatus::Error);

        assert!(outcome.changed());
        assert!(!runtime.image_visible());
        assert!(runtime.fallback_visible());
        assert_eq!(
            runtime.fallback_state().image_loading_status,
            AvatarImageLoadingStatus::Error
        );
    }

    #[test]
    fn missing_source_reports_error() {
        let mut runtime = AvatarRuntime::new();
        runtime.sync_image(Some(0));

        let outcome = runtime.report_image_status(AvatarImageLoadingStatus::Error);

        assert!(outcome.changed());
        assert_eq!(
            runtime.root_state().image_loading_status,
            AvatarImageLoadingStatus::Error
        );
    }

    #[test]
    fn fallback_visibility_follows_status_and_delay() {
        let runtime = AvatarRuntime::new();
        assert!(runtime.fallback_visible());

        let mut delayed = AvatarRuntime::new();
        let timer = delayed.arm_fallback_delay(Some(Duration::from_millis(500)));
        assert_eq!(timer, Some(Duration::from_millis(500)));
        assert!(!delayed.fallback_visible());

        delayed.fallback_delay_elapsed();
        assert!(delayed.fallback_visible());

        delayed.report_image_status(AvatarImageLoadingStatus::Loaded);
        assert!(!delayed.fallback_visible());
    }

    #[test]
    fn unchanged_status_report_produces_no_outcome() {
        let mut runtime = AvatarRuntime::new();

        assert!(runtime
            .report_image_status(AvatarImageLoadingStatus::Loading)
            .changed());
        assert!(!runtime
            .report_image_status(AvatarImageLoadingStatus::Loading)
            .changed());
    }

    #[test]
    fn source_change_resets_status_derivation() {
        let mut runtime = AvatarRuntime::new();
        runtime.sync_image(Some(1));
        runtime.report_image_status(AvatarImageLoadingStatus::Loaded);

        runtime.sync_image(Some(2));
        assert_eq!(
            runtime.root_state().image_loading_status,
            AvatarImageLoadingStatus::Idle
        );

        runtime.sync_image(Some(2));
        assert_eq!(
            runtime.root_state().image_loading_status,
            AvatarImageLoadingStatus::Idle
        );
    }

    #[test]
    fn arming_without_delay_shows_fallback_immediately() {
        let mut runtime = AvatarRuntime::new();

        assert_eq!(runtime.arm_fallback_delay(None), None);
        assert!(runtime.fallback_visible());
        assert_eq!(
            runtime.arm_fallback_delay(Some(Duration::from_secs(1))),
            None
        );
        assert!(runtime.fallback_visible());
    }
}
