//! Shell state: selected destination + active theme, with a persistence hook
//! the composition root can attach.

use design_tokens_theme_definitions::DesignTokens;
use egui::Ui;

use crate::nav_destinations::NavDestination;

/// Callback invoked with the newly-selected theme's name whenever the user
/// switches themes; owned by the shell, implemented by the composition root.
pub type ThemeChangeHandler = Box<dyn Fn(&str)>;

/// Mutable state of the app shell, owned by the composition root's eframe
/// `App` and driven each frame via [`ShellState::shell_ui`].
///
/// The shell holds no store/bridge dependencies (layering); theme
/// persistence is delegated out through [`ShellState::set_theme_change_handler`],
/// which the composition root wires to its config file.
pub struct ShellState {
    selected_destination: NavDestination,
    active_theme: DesignTokens,
    available_theme_names: Vec<String>,
    theme_change_handler: Option<ThemeChangeHandler>,
}

impl ShellState {
    /// Builds the shell starting on the first rail destination with
    /// `initial_theme` applied.
    #[must_use]
    pub fn new(initial_theme: DesignTokens) -> Self {
        Self {
            selected_destination: NavDestination::ALL[0],
            available_theme_names: design_tokens_theme_definitions::all_themes()
                .into_iter()
                .map(|theme| theme.name)
                .collect(),
            active_theme: initial_theme,
            theme_change_handler: None,
        }
    }

    /// Attaches the callback invoked whenever the user switches themes.
    ///
    /// The composition root uses this to persist the choice into
    /// `AppConfig`; the shell itself never touches configuration.
    pub fn set_theme_change_handler(&mut self, handler: ThemeChangeHandler) {
        self.theme_change_handler = Some(handler);
    }

    /// Destination currently highlighted in the nav rail.
    #[must_use]
    pub const fn selected_destination(&self) -> NavDestination {
        self.selected_destination
    }

    /// Moves the selection highlight; Phase 0 destinations are placeholders,
    /// so this changes only what the content area displays.
    pub fn select_destination(&mut self, destination: NavDestination) {
        self.selected_destination = destination;
    }

    /// The currently applied theme tokens.
    #[must_use]
    pub fn active_theme(&self) -> &DesignTokens {
        &self.active_theme
    }

    /// Names offered by the theme-switcher dropdown (doc 08 §2 inventory).
    #[must_use]
    pub fn available_theme_names(&self) -> &[String] {
        &self.available_theme_names
    }

    /// Switches to the named shipped theme. Returns `false` (and applies
    /// nothing) for unknown names.
    pub fn switch_theme(&mut self, theme_name: &str) -> bool {
        let Some(tokens) = design_tokens_theme_definitions::theme_by_name(theme_name) else {
            return false;
        };
        self.active_theme = tokens;
        if let Some(handler) = &self.theme_change_handler {
            handler(self.active_theme.name.as_str());
        }
        true
    }

    /// Draws one full frame of the app shell into the root UI.
    ///
    /// Re-applies the active theme every frame so an egui context reset or
    /// mid-session switch both end up correct; the style build is cheap
    /// relative to frame rendering.
    pub fn shell_ui(&mut self, ui: &mut Ui) {
        ui_shared_widget_library::apply_theme(ui.ctx(), &self.active_theme);
        crate::shell_layout::draw_shell(ui, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn mocha() -> DesignTokens {
        design_tokens_theme_definitions::default_theme()
    }

    #[test]
    fn starts_on_first_destination_with_given_theme() {
        let state = ShellState::new(mocha());
        assert_eq!(state.selected_destination(), NavDestination::ALL[0]);
        assert_eq!(state.active_theme().name, "Catppuccin Mocha");
    }

    #[test]
    fn selection_follows_destination_clicks() {
        let mut state = ShellState::new(mocha());
        state.select_destination(NavDestination::VisualizerPreview);
        assert_eq!(
            state.selected_destination(),
            NavDestination::VisualizerPreview
        );
        state.select_destination(NavDestination::SettingsAndTheming);
        assert_eq!(
            state.selected_destination(),
            NavDestination::SettingsAndTheming
        );
    }

    #[test]
    fn theme_switch_updates_active_tokens() {
        let mut state = ShellState::new(mocha());
        assert!(state.switch_theme("Catppuccin Latte"));
        assert_eq!(state.active_theme().name, "Catppuccin Latte");
        assert!(!state.active_theme().is_dark);
    }

    #[test]
    fn unknown_theme_name_is_rejected_without_side_effects() {
        let mut state = ShellState::new(mocha());
        assert!(!state.switch_theme("Not A Theme"));
        assert_eq!(state.active_theme().name, "Catppuccin Mocha");
    }

    #[test]
    fn switching_theme_fires_persistence_handler_with_new_name() {
        let observed: Rc<RefCell<Vec<String>>> = Rc::default();
        let mut state = ShellState::new(mocha());
        let sink = Rc::clone(&observed);
        state.set_theme_change_handler(Box::new(move |name| {
            sink.borrow_mut().push(name.to_owned());
        }));

        assert!(state.switch_theme("Monokai"));
        assert_eq!(
            observed.borrow().as_slice(),
            ["Monokai".to_owned()],
            "handler should fire exactly once per successful switch"
        );

        // Unknown names must not fire the handler.
        assert!(!state.switch_theme("Bogus"));
        assert_eq!(observed.borrow().len(), 1);
    }
}
