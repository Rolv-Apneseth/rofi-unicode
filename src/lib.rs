use glib::markup_escape_text;
use rofi_mode::{Action, Event, Style};
use std::fmt::Write;
use tracing::error;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod unicode;
mod utils;

use unicode::{UNICODE, Unicode};
use utils::{copy_to_clipboard, insert};

struct Mode<'rofi> {
    entries: &'rofi [Unicode],
}

// ROFI MODE
impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "unicode\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        if api.display_name().is_none() {
            api.set_display_name("unicode");
        };

        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .init();

        let entries = UNICODE;

        Ok(Mode { entries })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];

        let mut s = format!(
            "<b>{}</b> <small>{}</small>",
            markup_escape_text(entry.char),
            markup_escape_text(entry.name)
        );
        if !entry.alt_name.is_empty() {
            let _ = &write!(s, "<small>, {}</small>", markup_escape_text(entry.alt_name));
        }

        rofi_mode::format!("{}", s)
    }

    fn entry_style(&self, _line: usize) -> Style {
        Style::MARKUP
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            // User accepted an option from the list
            Event::Ok { alt, selected } => {
                let selected_entry = &self.entries[selected];

                // User selected entry with alternative binding, attempt to simulate
                // typing unicode                if alt {
                if alt {
                    if let Err(e) = insert(selected_entry.char) {
                        error!("Error with typing out unicode: {e:?}")
                    };
                // User selected entry regularly, copy the unicode to the system clipboard
                } else if let Err(e) = copy_to_clipboard(selected_entry.char) {
                    error!("Error with copying unicode to clipboard: {e:?}")
                }
            }

            // User cancelled selection i.e. pressed `Esc`
            Event::Cancel { selected: _ } => {}

            // All other events are unsupported
            _ => {
                error!("Unsupported input event: {event:?}")
            }
        }

        Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(self.entries[line].name) || matcher.matches(self.entries[line].alt_name)
    }

    fn entry_attributes(&self, _line: usize) -> rofi_mode::Attributes {
        rofi_mode::Attributes::new()
    }

    fn completed(&self, line: usize) -> rofi_mode::String {
        self.entry_content(line)
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        rofi_mode::String::new()
    }
}

rofi_mode::export_mode!(Mode);
