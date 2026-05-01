use std::fmt;
use std::io;
use std::io::stdout;

use codex_terminal_detection::Multiplexer;
use codex_terminal_detection::terminal_info;
use crossterm::Command;
use ratatui::crossterm::execute;

#[derive(Debug)]
pub struct BelBackend {
    dcs_passthrough: bool,
}

impl Default for BelBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl BelBackend {
    pub fn new() -> Self {
        Self {
            dcs_passthrough: matches!(terminal_info().multiplexer, Some(Multiplexer::Tmux { .. })),
        }
    }

    pub fn notify(&mut self, _message: &str) -> io::Result<()> {
        execute!(
            stdout(),
            PostNotification {
                dcs_passthrough: self.dcs_passthrough,
            }
        )
    }
}

/// Command that emits a BEL desktop notification.
#[derive(Debug, Clone)]
pub struct PostNotification {
    pub dcs_passthrough: bool,
}

impl Command for PostNotification {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if self.dcs_passthrough {
            write!(f, "\x1bPtmux;\x07\x1b\\")
        } else {
            write!(f, "\x07")
        }
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        Err(std::io::Error::other(
            "tried to execute PostNotification using WinAPI; use ANSI instead",
        ))
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crossterm::Command;
    use pretty_assertions::assert_eq;

    use super::PostNotification;

    #[test]
    fn post_notification_writes_bel_sequence() {
        let mut ansi = String::new();
        PostNotification {
            dcs_passthrough: false,
        }
        .write_ansi(&mut ansi)
        .expect("BEL command should format");

        assert_eq!(ansi, "\u{7}");
    }

    #[test]
    fn post_notification_wraps_bel_for_tmux_passthrough() {
        let mut ansi = String::new();
        PostNotification {
            dcs_passthrough: true,
        }
        .write_ansi(&mut ansi)
        .expect("BEL command should format");

        assert_eq!(ansi, "\u{1b}Ptmux;\u{7}\u{1b}\\");
    }
}
