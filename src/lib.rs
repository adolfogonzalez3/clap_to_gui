
extern crate iui;
extern crate yaml_rust;
use yaml_rust::Yaml;
use clap::App;
use iui::controls::{Button, Group, HorizontalBox, VerticalBox};
use iui::prelude::*;

pub mod widgets;
pub mod arguments;

use arguments::Argument;

pub fn run_gui<F: FnMut(clap::ArgMatches)>(docs: &Yaml, mut f: F) {
    let program_name: String = docs["name"].as_str().unwrap().into();
    let mut arguments = Argument::convert(&docs["args"]);
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    let mut args_widgets = Group::new(&ui, "Program Argument Inputs");
    let mut arg_view = VerticalBox::new(&ui);
    let mut window = Window::new(&ui, &program_name, 300, 150, WindowType::NoMenubar);
    arg_view.set_padded(&ui, true);

    for arg in arguments.iter_mut() {
        arg.create_widget(&ui, &window);
        let view = arg.widget_in_hbox(&ui);
        let label = arg.create_label(&ui);
        arg_view.append(&ui, label, LayoutStrategy::Compact);
        arg_view.append(&ui, view, LayoutStrategy::Compact);
    }
    let mut submit_button = Button::new(&ui, "Submit");
    let mut view = HorizontalBox::new(&ui);
    view.set_padded(&ui, true);
    view.append(&ui, submit_button.clone(), LayoutStrategy::Compact);
    arg_view.append(&ui, view, LayoutStrategy::Compact);
    args_widgets.set_child(&ui, arg_view);

    submit_button.on_clicked(&ui, {
        let arguments_0 = arguments.clone();
        let ui_0 = ui.clone();
        move |_b| {
            let mut cmdline_sort = arguments_0
                .iter()
                .map(|x| (x.position(), x.to_cmdline(&ui_0)))
                .collect::<Vec<(i64, String)>>();
            cmdline_sort.sort_unstable_by_key(|x| x.0);
            let cmdline = cmdline_sort
                .into_iter()
                .map(|(_, x)| x)
                .collect::<Vec<String>>();
            println!("{}", cmdline.join(" "));
            let matches_result = App::from(docs).get_matches_from_safe(cmdline);
            println!("{:?}", matches_result);
            if let Ok(matches) = matches_result {
                f(matches);
            }
        }
    });

    // The window allows all constituent components to be displayed.
    window.set_child(&ui, args_widgets);
    window.show(&ui);

    // These on_changed functions allow updating the application state when a
    // control changes its value.
   
    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    //let mut event_loop = ui.event_loop();
    ui.main();
}
