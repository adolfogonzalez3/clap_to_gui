extern crate iui;
extern crate yaml_rust;
use crate::widgets::FileChooser;
use iui::controls::{Checkbox, Entry, HorizontalBox, Label};
use iui::prelude::*;
// All enums have in common the
// name, help, and required attribute

#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub help: String,
    takes_value: bool,
    pub index: i64,
    widget: Option<ArgumentWidget>,
}

#[derive(Clone)]
pub enum ArgumentWidget {
    FileEntry(FileChooser),
    Checkbox(Checkbox),
    TextEntry(Entry),
}

impl Argument {
    pub fn new(name: &str) -> Argument {
        Argument {
            name: name.into(),
            help: "".into(),
            takes_value: false,
            index: -1,
            widget: None,
        }
    }

    pub fn convert(yaml: &yaml_rust::yaml::Yaml) -> Vec<Argument> {
        let mut arguments = Vec::new();
        for arg in yaml.as_vec().unwrap() {
            for (k, it) in arg.as_hash().unwrap() {
                let mut argument = Argument::new(k.as_str().unwrap());
                argument.set_help(it["help"].as_str().unwrap_or(""));
                argument.set_takes_value(it["takes_value"].as_bool().unwrap_or(false));
                argument.set_index(it["index"].as_i64().unwrap_or(-1));
                arguments.push(argument);
            }
        }
        arguments
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn position(&self) -> i64 {
        self.index
    }

    pub fn set_help(&mut self, text: &str) {
        self.help = text.into();
    }

    pub fn set_takes_value(&mut self, value: bool) {
        self.takes_value = value;
    }

    pub fn set_index(&mut self, value: i64) {
        self.index = value;
    }

    pub fn create_widget(&mut self, ui: &UI, window: &Window) -> ArgumentWidget {
        let is_entry = {
            let help_lower = self.help.to_lowercase();
            help_lower.contains("file") || help_lower.contains("path")
        };
        let widget = if is_entry {
            ArgumentWidget::FileEntry(FileChooser::new(&ui, &window))
        } else if !self.takes_value {
            ArgumentWidget::Checkbox(Checkbox::new(&ui, "Enabled"))
        } else {
            ArgumentWidget::TextEntry(Entry::new(&ui))
        };
        self.widget = Some(widget.clone());
        widget
    }

    pub fn to_cmdline(&self, ui: &UI) -> String {
        match  self.widget.as_ref().unwrap() {
            ArgumentWidget::TextEntry(entry) => {
                if self.index != -1 {
                    format!("{}", entry.value(&ui))
                } else {
                    format!("--{} {}", self.name, entry.value(&ui))
                }
            },
            ArgumentWidget::FileEntry(chooser) => {
                let text = chooser.get_text();
                if self.index != -1 {
                    format!("{}", text.value(&ui))
                } else {
                    format!("--{} {}", self.name, text.value(&ui))
                }
            },
            ArgumentWidget::Checkbox(check) => {
                if check.checked(&ui) {
                    format!("--{}", self.name)
                } else {
                    "".into()
                }
                    
            }
        }
    }

    pub fn create_label(&self, ui: &UI) -> Label {
        Label::new(&ui, &format!("{}: {}", self.name, self.help))
    }

    pub fn widget_in_hbox(&self, ui: &UI) -> HorizontalBox {
        self.widget.as_ref().unwrap().with_hbox(&ui)
    }
}

impl ArgumentWidget {
    pub fn with_hbox(&self, ui: &UI) -> HorizontalBox {
        let mut view = HorizontalBox::new(&ui);
        view.set_padded(&ui, true);
        match &self {
            ArgumentWidget::Checkbox(check) => {
                view.append(&ui, check.clone(), LayoutStrategy::Compact);
            }
            ArgumentWidget::FileEntry(e) => {
                view.append(&ui, e.text.clone(), LayoutStrategy::Compact);
                view.append(&ui, e.submit.clone(), LayoutStrategy::Compact);
            }
            ArgumentWidget::TextEntry(e) => {
                view.append(&ui, e.clone(), LayoutStrategy::Compact);
            }
        }
        view
    }
}
