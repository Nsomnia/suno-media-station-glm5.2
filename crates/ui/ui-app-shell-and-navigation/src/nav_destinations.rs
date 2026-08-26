//! The fixed set of top-level destinations shown in the navigation rail.
//!
//! Names mirror doc 02's `ui-screen-*` crates one-to-one so adding real
//! screens later is a mechanical substitution, not a redesign. Entries are
//! non-functional placeholders in Phase 0: they carry selected-state only.

/// One persistent top-level section of the app (doc 02 screen-crate tree).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavDestination {
    /// Suno cloud library browser (`ui-screen-remote-library-browser`).
    RemoteLibrary,
    /// Local downloads browser/player (`ui-screen-local-library-browser`).
    LocalLibrary,
    /// Karaoke timing review/edit (`ui-screen-lyrics-editor`).
    LyricsEditor,
    /// Live visualizer preview + render controls (`ui-screen-visualizer-preview`).
    VisualizerPreview,
    /// Freeform canvas scene editor (`ui-screen-canvas-scene-editor`).
    CanvasSceneEditor,
    /// Pipeline authoring + run monitor (`ui-screen-automation-pipeline-builder`).
    AutomationPipelineBuilder,
    /// Basic capture UI (`ui-screen-recording-studio`).
    RecordingStudio,
    /// Generation submission + job monitor (`ui-screen-creation-studio`).
    CreationStudio,
    /// Add/switch/remove Suno accounts (`ui-screen-account-management`).
    AccountManagement,
    /// Theme picker, provider keys, prefs (`ui-screen-settings-and-theming`).
    SettingsAndTheming,
}

impl NavDestination {
    /// Every rail entry in display order (doc 08 §5: nav-first layout).
    pub const ALL: [NavDestination; 10] = [
        Self::RemoteLibrary,
        Self::LocalLibrary,
        Self::LyricsEditor,
        Self::VisualizerPreview,
        Self::CanvasSceneEditor,
        Self::AutomationPipelineBuilder,
        Self::RecordingStudio,
        Self::CreationStudio,
        Self::AccountManagement,
        Self::SettingsAndTheming,
    ];

    /// Human-facing label drawn in the navigation rail.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::RemoteLibrary => "Library — Remote",
            Self::LocalLibrary => "Library — Local",
            Self::LyricsEditor => "Lyrics",
            Self::VisualizerPreview => "Visualizer",
            Self::CanvasSceneEditor => "Canvas",
            Self::AutomationPipelineBuilder => "Automation",
            Self::RecordingStudio => "Recording Studio",
            Self::CreationStudio => "Creation Studio",
            Self::AccountManagement => "Accounts",
            Self::SettingsAndTheming => "Settings",
        }
    }

    /// Placeholder copy shown in the content area while the destination's
    /// real `ui-screen-*` crate has not shipped yet.
    #[must_use]
    pub const fn placeholder_description(self) -> &'static str {
        match self {
            Self::RemoteLibrary => "Search and browse your Suno cloud library.",
            Self::LocalLibrary => "Browse and play downloaded tracks.",
            Self::LyricsEditor => "Review and edit karaoke lyric timing.",
            Self::VisualizerPreview => "Live projectM preview and render controls.",
            Self::CanvasSceneEditor => "Freeform overlay placement and keyframes.",
            Self::AutomationPipelineBuilder => "Author and monitor batch pipelines.",
            Self::RecordingStudio => "Capture microphone takes locally.",
            Self::CreationStudio => "Submit Suno generations and watch jobs.",
            Self::AccountManagement => "Add, switch, and remove Suno accounts.",
            Self::SettingsAndTheming => "Theme picker, provider keys, preferences.",
        }
    }
}
