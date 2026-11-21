use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Entry,Orientation};

fn print_text(entry: &Entry){
    println!("{}", entry.text());
}

fn build_ui(app: &Application){
    let entry = Entry::builder()
        .placeholder_text("Type something here...")
        .build();
    let input = Entry::builder()
        .placeholder_text("Enter the bakcup path")
        .build();

    let button = Button::with_label("Save");

    //clicked input
    button.connect_clicked(glib::clone!(
        #[weak]
        entry,
        move |_| {
            let text = entry.text(); 
            println!("Button clicked! User entered: {}", text);
        }
    ));
    button.connect_clicked(glib::clone!(
        #[weak]
        input,
        move |_|{
            println!("{}", input.text());
        }
    ));
    //enter input
    entry.connect_activate(|entry| {
        let text = entry.text();
        println!("Enter pressed! User entered: {}", text);
    });
    input.connect_activate(print_text);

    let container = Box::new(Orientation::Vertical, 10);
    container.append(&entry);
    container.append(&input);
    container.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("emu-save")
        .child(&container)
        .build();
    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.autosave.emulator")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

