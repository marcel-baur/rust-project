//! # MenuBar Sample
//!
//! This sample demonstrates how to use Menus/MenuBars and MenuItems in Windows.
//!
//! /!\ This is different from the system menu bar (which are preferred) available in `gio::Menu`!

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, Label, Menu, MenuBar, MenuItem, WindowPosition, FileChooserDialog, FileChooserAction, ResponseType};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("FEFM Database Music");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");

    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    quit.connect_activate(clone!(@weak window => move |_| {
        window.destroy();
    }));

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    // It isn't available directly through gdk::ModifierType, since it has
    // different values on different platforms.
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let label = Label::new(Some("FEFM Database Music"));
    let button = gtk::Button::new_with_label("Choose file");
    //FOR CSS
    gtk::WidgetExt::set_widget_name(&button, "button1");

    let dialog = FileChooserDialog::new(Some("Open File"), Some(&window), FileChooserAction::Open);
    dialog.add_button("_Cancel", ResponseType::Cancel);
    dialog.add_button("_Open", ResponseType::Accept);

    button.connect_clicked(move |_| {
        dialog.run();
        let file = dialog.get_filename();
        match file {
            Some(file) =>  {
                println!("{}", file.into_os_string().into_string().unwrap())
            },
            _ => {},
        }
        dialog.hide();
    });

    let search_button = gtk::Button::new_with_label("search for music");


    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let textbox = gtk::Entry::new();
    let h_box_label = Label::new(Some("Titel"));

    h_box.pack_start(&h_box_label, false, true, 0);
    h_box.pack_start(&textbox, false, true, 0);

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&label, false, true, 0);
    v_box.pack_start(&h_box, false, true, 0);
    v_box.pack_start(&button, false, true, 0);
    v_box.pack_start(&search_button, false, true, 0);
    window.add(&v_box);
    window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["gtk-rs developers"]);
        p.set_website_label(Some("gtk-rs"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}