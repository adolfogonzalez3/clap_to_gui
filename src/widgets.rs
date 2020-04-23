
extern crate iui;
use iui::controls::{Button, Entry, HorizontalBox};
use iui::prelude::*;

#[derive(Clone)]
pub struct FileChooser {
    pub text: Entry,
    pub submit: Button,
}

impl FileChooser {
    pub fn new(_ctx: &UI, win: &Window) -> FileChooser {
        let entry = Entry::new(&_ctx);
        let mut passed_entry = entry.clone();
        let mut btn = Button::new(&_ctx, "Browse");

        btn.on_clicked(&_ctx, move |_b| {
            let path = win.open_file(&_ctx).unwrap();
            passed_entry.set_value(&_ctx, path.to_str().unwrap());
        });
        let mut chooser = FileChooser {
            text: entry,
            submit: btn,
        };
        chooser.on_clicked(_ctx, win, |_e, _b| {});
        chooser
    }

    pub fn value(&self, _ctx: &UI) -> String {
        self.text.value(&_ctx)
    }

    pub fn get_text(&self) -> Entry {
        self.text.clone()
    }

    pub fn on_clicked<F: FnMut(&mut Entry, &mut Button)>(
        &mut self,
        _ctx: &UI,
        win: &Window,
        mut callback: F,
    ) {
        let mut passed_entry = self.text.clone();
        let passed_window = win.clone();
        self.submit.on_clicked(&_ctx, move |mut b| {
            let path = passed_window.open_file(&_ctx).unwrap_or("".into());
            passed_entry.set_value(
                &_ctx, path.to_str().unwrap_or("Can't be represented.".into()));
            callback(&mut passed_entry, &mut b);
        });
    }

    pub fn get_hview(&self, _ctx: &UI) -> HorizontalBox {
        let mut view = HorizontalBox::new(&_ctx);
        view.set_padded(&_ctx, true);
        view.append(&_ctx, self.text.clone(), LayoutStrategy::Compact);
        view.append(&_ctx, self.submit.clone(), LayoutStrategy::Compact);
        view
    }
}