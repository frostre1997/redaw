use std::fmt;

#[derive(Debug)]
pub enum redawError {
    Audio(String),
    Plugin(String),
    File(String),
    State(String),
}

impl fmt::Display for redawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            redawError::Audio(msg) => write!(f, "Audio error: {}", msg),
            redawError::Plugin(msg) => write!(f, "Plugin error: {}", msg),
            redawError::File(msg) => write!(f, "File error: {}", msg),
            redawError::State(msg) => write!(f, "State error: {}", msg),
        }
    }
}

impl std::error::Error for redawError {}

pub type Result<T> = std::result::Result<T, redawError>;

// Add user-friendly message generation
impl redawError {
    /// Get a user-friendly title for this error
    pub fn title(&self) -> &str {
        match self {
            redawError::Audio(_) => "Audio Error",
            redawError::Plugin(_) => "Plugin Error",
            redawError::File(_) => "File Error",
            redawError::State(_) => "Application Error",
        }
    }

    /// Get a user-friendly hint for resolving this error
    pub fn hint(&self) -> Option<&str> {
        match self {
            redawError::Audio(_) => Some("Check your audio device settings"),
            redawError::Plugin(_) => Some("Try bypassing the plugin or checking for updates"),
            redawError::File(_) => Some("Check file permissions and disk space"),
            redawError::State(_) => Some("Try restarting the application"),
        }
    }

    /// Get the detailed error message
    pub fn details(&self) -> &str {
        match self {
            redawError::Audio(msg)
            | redawError::Plugin(msg)
            | redawError::File(msg)
            | redawError::State(msg) => msg,
        }
    }

    /// Format for user display
    pub fn user_message(&self) -> String {
        if let Some(hint) = self.hint() {
            format!("{}: {}\n\nHint: {}", self.title(), self.details(), hint)
        } else {
            format!("{}: {}", self.title(), self.details())
        }
    }

    /// Format for logging
    pub fn log_message(&self) -> String {
        format!("{}", self) // Uses Display impl
    }
}

// Conversion helpers
impl From<std::io::Error> for redawError {
    fn from(err: std::io::Error) -> Self {
        redawError::File(err.to_string())
    }
}

impl From<anyhow::Error> for redawError {
    fn from(err: anyhow::Error) -> Self {
        redawError::Plugin(err.to_string())
    }
}

// Builder pattern for creating detailed errors
impl redawError {
    pub fn audio(msg: impl Into<String>) -> Self {
        redawError::Audio(msg.into())
    }

    pub fn plugin(msg: impl Into<String>) -> Self {
        redawError::Plugin(msg.into())
    }

    pub fn file(msg: impl Into<String>) -> Self {
        redawError::File(msg.into())
    }

    pub fn state(msg: impl Into<String>) -> Self {
        redawError::State(msg.into())
    }
}

// Extension trait for Result types
pub trait ResultExt<T> {
    /// Show error to user via dialog
    fn notify_user(self, dialogs: &mut impl UserNotification) -> Option<T>;

    /// Log error to console
    fn log_error(self) -> Option<T>;

    /// Both notify and log
    fn handle_error(self, dialogs: &mut impl UserNotification) -> Option<T>;
}

impl<T> ResultExt<T> for Result<T> {
    fn notify_user(self, dialogs: &mut impl UserNotification) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                dialogs.show_error(&e.user_message());
                None
            }
        }
    }

    fn log_error(self) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                eprintln!("{}", e.log_message());
                None
            }
        }
    }

    fn handle_error(self, dialogs: &mut impl UserNotification) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                eprintln!("{}", e.log_message());
                dialogs.show_error(&e.user_message());
                None
            }
        }
    }
}

// Trait for dialog notification (to avoid circular dependency)
pub trait UserNotification {
    fn show_error(&mut self, message: &str);
    fn show_success(&mut self, message: &str);
    fn show_warning(&mut self, message: &str);
    fn show_info(&mut self, message: &str);
}

// Common error creation helpers
pub mod common {
    use super::redawError;

    pub fn project_save_failed(e: impl std::fmt::Display) -> redawError {
        redawError::file(format!("Failed to save project: {}", e))
    }

    pub fn project_load_failed(e: impl std::fmt::Display) -> redawError {
        redawError::file(format!("Failed to load project: {}", e))
    }

    pub fn audio_import_failed(path: &std::path::Path, e: impl std::fmt::Display) -> redawError {
        redawError::file(format!("Failed to import {}: {}", path.display(), e))
    }

    pub fn plugin_load_failed(name: &str, e: impl std::fmt::Display) -> redawError {
        redawError::plugin(format!("Failed to load plugin '{}': {}", name, e))
    }

    pub fn plugin_process_error(name: &str, e: impl std::fmt::Display) -> redawError {
        redawError::plugin(format!("Plugin '{}' processing error: {}", name, e))
    }

    pub fn recording_error(e: impl std::fmt::Display) -> redawError {
        redawError::audio(format!("Recording error: {}", e))
    }

    pub fn playback_error(e: impl std::fmt::Display) -> redawError {
        redawError::audio(format!("Playback error: {}", e))
    }
}
