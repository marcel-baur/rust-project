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


// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
#headline {
    color: blue;
    font-weight: bold;
    font-size: 32px;
}
#subheadline {
    font-size: 12px;
}
#scrollview {
    padding: 10px;
}";


fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("MEFF");
    window.set_position(WindowPosition::Center);
    window.set_size_request(600, 600);

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

    let label = Label::new(Some("MEFF"));
    let label2 = Label::new(Some("Music Entertainment For Friends"));

    gtk::WidgetExt::set_widget_name(&label, "headline");
    gtk::WidgetExt::set_widget_name(&label2, "subheadline");

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

    let list_box = gtk::ListBox::new();

    for x in 0..100 {
        let mut list_box_row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Abba"));
        hbox.pack_start(&label, false, false, 5);
        list_box_row.add(&hbox);
        list_box_row.show_all();
        list_box.add(&list_box_row);
    }

    h_box.pack_start(&h_box_label, false, true, 0);
    h_box.pack_start(&textbox, false, true, 0);

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&label, false, true, 0);
    v_box.pack_start(&label2, false, true, 0);
    v_box.pack_start(&h_box, false, true, 0);
    v_box.pack_start(&button, false, true, 0);
    v_box.pack_start(&search_button, false, true, 0);
    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    gtk::WidgetExt::set_widget_name(&scrolled_window, "scrollview");
    scrolled_window.add(&list_box);
    scrolled_window.set_size_request(200, 200);
    scrolled_window.set_valign(gtk::Align::Start);
    v_box.pack_start(&scrolled_window, false, false, 10);
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
        Some("com.meef"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}