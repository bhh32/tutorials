// Consolidated COSMIC/iced imports following the current applet patterns.
//
// Key changes from the pre-iced-0.14 version:
//   - Popup commands moved to: platform_specific::shell::wayland::commands::popup
//   - The `window` module comes from cosmic::iced::window (not iced_runtime::core)
//   - Task and Element are re-exported directly from the `cosmic` crate
//   - Element now carries a lifetime parameter: Element<'_, Message>
//   - The update/init return type is now app::Task<Self::Message>
use cosmic::{
    app::Core,
    iced::{
        self,
        platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup},
        window, Limits,
    },
    // Task and Element are re-exported at the cosmic crate root
    Element, Task, app,
    widget::{list_column, settings, text, toggler},
};

// Every COSMIC Application and Applet MUST have an ID.
const ID: &str = "com.example.BasicApplet";

/// Every COSMIC model must be a struct data type.
///
/// Mandatory fields for a COSMIC Applet are `core` and `popup`.
///   - `core` holds the core settings that allow the applet to interact with
///     the COSMIC desktop environment.
///   - `popup` tracks the window ID of the currently open popup so we can
///     open and close it on demand.
///
/// `is_enabled` is our custom field that we will manipulate based on the
/// messages we send.
#[derive(Default)]
pub struct Window {
    core: Core,
    popup: Option<window::Id>,
    is_enabled: bool,
}

/// Messages drive all state changes in a COSMIC applet (Elm architecture).
#[derive(Clone, Debug)]
pub enum Message {
    TogglePopup,         // Open or close the applet popup
    PopupClosed(window::Id), // The compositor notified us the popup was closed
    EnableDisable(bool), // Our custom message to flip the is_enabled toggle
}

impl cosmic::Application for Window {
    /// Executors control how async work is scheduled. SingleThreadExecutor is
    /// sufficient for applets that don't perform heavy background work.
    type Executor = cosmic::SingleThreadExecutor;

    /// Flags allow passing startup configuration into init(). An applet that
    /// needs no startup data uses the unit type.
    type Flags = ();

    /// Wire our Message enum as the application-level message type.
    type Message = Message;

    /// The unique application ID registered with the COSMIC desktop.
    const APP_ID: &'static str = ID;

    // --- Core accessors (required) -----------------------------------------

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    // --- Lifecycle ----------------------------------------------------------

    /// Initialize the applet. Returns the model and an optional startup Task.
    fn init(core: Core, _flags: Self::Flags) -> (Self, app::Task<Self::Message>) {
        let window = Window {
            core,
            is_enabled: false,
            ..Default::default()
        };

        (window, Task::none())
    }

    /// Called by the compositor when a window close is requested.
    /// Returning Some(msg) lets us handle it through update().
    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    // --- Update (message handler) ------------------------------------------

    /// Process incoming messages and return a Task for any async follow-up
    /// work. Most simple handlers return Task::none().
    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                // If a popup is already open, destroy it.
                return if let Some(popup_id) = self.popup.take() {
                    destroy_popup(popup_id)
                } else {
                    // Create a new popup with a unique window ID.
                    let new_id = window::Id::unique();
                    self.popup.replace(new_id);

                    // Build the popup settings from the applet core. The three
                    // Option parameters are: size hint, horizontal offset, and
                    // vertical offset relative to the panel icon.
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );

                    // Constrain the popup size so it looks reasonable on any
                    // display configuration.
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);

                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(popup_id) => {
                if self.popup.as_ref() == Some(&popup_id) {
                    self.popup = None;
                }
            }
            Message::EnableDisable(is_enabled) => {
                self.is_enabled = is_enabled;
            }
        }
        Task::none()
    }

    // --- Views --------------------------------------------------------------

    /// The panel icon view. This is what the user sees in the COSMIC panel
    /// before they click on the applet.
    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    /// The popup window that appears when the panel icon is clicked.
    fn view_window(&self, _id: window::Id) -> Element<'_, Self::Message> {
        let content_list = list_column()
            .padding(5)
            .spacing(0)
            .add(settings::item(
                "Is this enabled?",
                text(if self.is_enabled {
                    "It is enabled!"
                } else {
                    "It's not enabled!"
                }),
            ))
            .add(settings::item(
                "Enable/Disable",
                toggler(self.is_enabled).on_toggle(Message::EnableDisable),
            ));

        self.core.applet.popup_container(content_list).into()
    }

    /// Return the applet style so the panel renders our background correctly.
    fn style(&self) -> Option<iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
