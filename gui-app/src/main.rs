//! # MenuBar Sample
//!
//! This sample demonstrates how to use Menus/MenuBars and MenuItems in Windows.
//!
//! /!\ This is different from the system menu bar (which are preferred) available in `gio::Menu`!

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate meff;

use gio::prelude::*;
use glib::{clone, Receiver};
use gtk::prelude::*;
use gtk::{AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, Label, Menu, MenuBar, MenuItem, WindowPosition, FileChooserDialog, FileChooserAction, ResponseType};
use crate::util::{MEFFM};
use std::net::SocketAddr;

use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;

mod util;

// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
#headline {
    color: blue;
    font-weight: bold;
    font-size: 32px;
}
#subheadline {
    font-size: 18px;
}
#button {
    background: white;
    border-color: white;
}
#scrollview {
    padding: 10px;
}
#frame{
    padding: 10px;
}
#entry_red {
    border-color: red;
}
#entry_gray {
    border-color: #B8B8B8;
}";

fn build_startup(main_window: &gtk::ApplicationWindow, meff: Rc<RefCell<MEFFM>>) -> gtk::Window {
    let startup_window = gtk::Window::new(gtk::WindowType::Toplevel);
    startup_window.set_position(WindowPosition::Center);
    startup_window.set_size_request(550, 300);

    let header = gtk::HeaderBar::new();
    header.set_title(Some("Sign up"));
    startup_window.set_titlebar(Some(&header));

    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(400);

    let v_box_create = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let name_entry_create = gtk::Entry::new();
    let port_entry_create = gtk::Entry::new();

    let name_box = create_entry_with_label("Name", name_entry_create.clone());
    let port_box = create_entry_with_label("Port    ", port_entry_create.clone());

    v_box_create.pack_start(&name_box, true, true, 0);
    v_box_create.pack_start(&port_box, true, true, 0);
    v_box_create.set_margin_top(20);
    v_box_create.set_margin_bottom(20);

    let v_box_join = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let name_entry_join = gtk::Entry::new();
    let port_entry_join = gtk::Entry::new();
    let ip_entry_join = gtk::Entry::new();

    let name_box_join = create_entry_with_label("Name         ", name_entry_join.clone());
    let port_box_join = create_entry_with_label("Port            ", port_entry_join.clone());
    let ip_box_join = create_entry_with_label("IP Address", ip_entry_join.clone());

    v_box_join.pack_start(&name_box_join, true, true, 0);
    v_box_join.pack_start(&port_box_join, true, true, 0);
    v_box_join.pack_start(&ip_box_join, true, true, 0);

    stack.add_titled(&v_box_create, "create", "Create network");
    stack.add_titled(&v_box_join, "join", "Join network");

    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    let start_button = gtk::Button::new_with_label("Start");
    let cancel_button = gtk::Button::new_with_label("Cancel");

    let stack_clone = stack.clone();
    start_button.connect_clicked(clone!(@weak startup_window => move |_| {
        let current_stack = stack_clone.get_visible_child_name().unwrap().as_str().to_string().clone();

        if current_stack == "create" {
            let name = name_entry_create.get_text().unwrap().as_str().to_string().clone();
            let port = port_entry_create.get_text().unwrap().as_str().to_string().clone();
            set_entry_border(&name, &name_entry_create);
            set_entry_border(&port, &port_entry_create);

            if !name.is_empty() && !port.is_empty() {
                meff.borrow_mut().start(name, port, None);
                startup_window.destroy();
            }
        } else {
            let name = name_entry_join.get_text().unwrap().as_str().to_string().clone();
            let port = port_entry_join.get_text().unwrap().as_str().to_string().clone();
            let ip = ip_entry_join.get_text().unwrap().as_str().to_string().clone();

            set_entry_border(&name, &name_entry_join);
            set_entry_border(&port, &port_entry_join);
            set_entry_border(&ip, &ip_entry_join);

            if !name.is_empty() && !port.is_empty() && !ip.is_empty() {
                let addr = verify_ip(&ip);
                meff.borrow_mut().start(name, port, addr);
                startup_window.destroy();
            }
        }
    }));

    cancel_button.connect_clicked(clone!(@weak main_window => move |_| {
        main_window.destroy();
    }));

    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    h_box.pack_start(&start_button, true, true, 0);
    h_box.pack_start(&cancel_button, true, true, 0);
    h_box.set_halign(gtk::Align::Center);
    h_box.set_valign(gtk::Align::End);
    h_box.set_homogeneous(true);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    v_box.pack_start(&stack_switcher, true, true, 0);
    v_box.pack_start(&stack, true, true, 0);
    v_box.pack_start(&h_box, false, true, 10);
    v_box.set_halign(gtk::Align::Center);
    v_box.set_margin_top(10);

    startup_window.add(&v_box);
    startup_window
}

fn verify_ip(addr: &String) -> Option<SocketAddr> {
    match addr.parse::<SocketAddr>() {
        Ok(socket_addr) => Some(socket_addr),
        Err(_) => None
    }
}

fn set_entry_border(text: &str, entry: &gtk::Entry) {
    if text.is_empty() {
        gtk::WidgetExt::set_widget_name(entry, "entry_red");
    } else {
        gtk::WidgetExt::set_widget_name(entry, "entry_gray");
    }
}


fn create_entry_with_label(text: &str, entry: gtk::Entry) -> gtk::Box {
    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    let label = Label::new(Some(&text));

    h_box.pack_start(&label, false, true, 0);
    h_box.pack_end(&entry, false, true, 0);

    h_box
}

fn add_music_title(song_path: String, meff: Rc<RefCell<MEFFM>>) {
    let title_popup = gtk::Window::new(gtk::WindowType::Toplevel);
    title_popup.set_position(WindowPosition::Center);
    title_popup.set_size_request(400, 200);

    let header = gtk::HeaderBar::new();
    header.set_title(Some("Enter Title"));
    title_popup.set_titlebar(Some(&header));

    let entry = gtk::Entry::new();
    let box_label = create_entry_with_label("Title", entry.clone());
    box_label.set_halign(gtk::Align::Center);
    box_label.set_margin_top(10);

    let ok_button = gtk::Button::new_with_label("Upload");
    ok_button.set_halign(gtk::Align::Center);
    ok_button.set_valign(gtk::Align::End);

    ok_button.connect_clicked(clone!(@weak title_popup => move |_| {
        let title = entry.get_text().unwrap().as_str().to_string().clone();
        let song_path_clone = song_path.clone();

        if title.is_empty() {
            gtk::WidgetExt::set_widget_name(&entry, "entry_red");
        } else {
            //Push music to database
            gtk::WidgetExt::set_widget_name(&entry, "entry_gray");
            meff.borrow_mut().push(song_path_clone, title);
            title_popup.destroy();
        }
    }));

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    v_box.pack_start(&box_label, true, true, 0);
    v_box.pack_start(&ok_button, true, true, 20);

    title_popup.add(&v_box);
    title_popup.show_all();
}

fn add_song_to_list(song_name: Rc<String>, list_box: &gtk::ListBox, meff: Rc<RefCell<MEFFM>>) {
    let list_box_row = gtk::ListBoxRow::new();
    list_box_row.set_selectable(false);

    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let label_button = gtk::Button::new_with_label(&song_name);

    let trash_button = gtk::Button::new();
    gtk::WidgetExt::set_widget_name(&label_button, "button");

    let image_delete = gtk::Image::new_from_file("src/delete.png");
    trash_button.set_image(Some(&image_delete));

    let song_clone_1 = song_name.clone();
    let song_clone_2 = song_name.clone();
    let meff_clone = meff.clone();
    let meff_clone2 = meff_clone.clone();
    trash_button.connect_clicked(move |_| {
        meff_clone.borrow_mut().remove_title(song_clone_1.to_string());
    });

    let meff_clone_3 = meff_clone2.clone();
    label_button.connect_clicked( move |_| {
        meff_clone_3.borrow_mut().play(Some(song_clone_2.to_string()));
    });

    h_box.pack_start(&label_button, true, true, 0);
    h_box.pack_end(&trash_button, false, false, 0);

    list_box_row.add(&h_box);
    list_box_row.show_all();
    list_box.add(&list_box_row);
}

fn show_status(meff: Rc<RefCell<MEFFM>>) {
    let status_window = gtk::Window::new(gtk::WindowType::Toplevel);
    status_window.set_position(WindowPosition::Center);
    status_window.set_size_request(400, 400);

    let header = gtk::HeaderBar::new();
    header.set_title(Some("Status"));
    status_window.set_titlebar(Some(&header));

    let list = meff.borrow_mut().status();
    let list_box = gtk::ListBox::new();

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    gtk::WidgetExt::set_widget_name(&scrolled_window, "scrollview");
    scrolled_window.add(&list_box);

    for (name, addr) in list {
        println!("New element");
        let row = gtk::ListBoxRow::new();
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let label_ip = gtk::Label::new(Some(&addr.to_string()));
        let label_name = gtk::Label::new(Some(&name));
        h_box.pack_start(&label_name, true, true, 0);
        h_box.pack_end(&label_ip, true, true, 0);
        row.add(&h_box);
        row.show_all();
        list_box.add(&row);
    }

    let close_button = gtk::Button::new_with_label("Close");
    close_button.set_halign(gtk::Align::Center);
    close_button.set_valign(gtk::Align::End);

    close_button.connect_clicked(clone!(@weak status_window => move |_| {
        status_window.destroy();
    }));

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    v_box.pack_start(&scrolled_window, true, true, 0);
    v_box.pack_start(&close_button, true, true, 20);

    status_window.add(&v_box);
    status_window.show_all();
}

fn build_ui(application: &gtk::Application, meff: Rc<RefCell<MEFFM>>, receiver: Receiver<(String, String)>) {
    let main_window = ApplicationWindow::new(application);
    let meff_clone = Rc::clone(&meff);
    let meff_clone_l = Rc::clone(&meff);
    let meff_clone_play = Rc::clone(&meff);
    let meff_clone_pause = Rc::clone(&meff);
    let meff_clone_stop = Rc::clone(&meff);
    let meff_clone_continue = Rc::clone(&meff);
    let meff_clone_remove = Rc::clone(&meff);
    let meff_clone_quit = Rc::clone(&meff);
    let meff_clone_status = Rc::clone(&meff);
    let meff_clone_stream = Rc::clone(&meff);
    let meff_clone_download = Rc::clone(&meff);

    let startup_window = build_startup(&main_window, meff_clone);

    main_window.set_position(WindowPosition::Center);
    main_window.set_size_request(600, 600);

    let header = gtk::HeaderBar::new();
    header.set_title(Some("MEFF"));
    main_window.set_titlebar(Some(&header));

    let v_box_window = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let h_box_window = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let v_box2 = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    main_window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");
    let status = MenuItem::new_with_label("Status");

    menu.append(&status);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    status.connect_activate(move |_| {
        let meff_status = Rc::clone(&meff_clone_status);
        show_status(meff_status);
    });


    quit.connect_activate(clone!(@weak main_window => move |_| {
        let meff_quit = Rc::clone(&meff_clone_quit);
        meff_quit.borrow_mut().quit();
        main_window.destroy();
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

    label.set_margin_top(25);

    let upload_button = gtk::Button::new_with_label("Upload music");
    upload_button.set_margin_start(40);
    upload_button.set_margin_end(40);

    let dialog = FileChooserDialog::new(Some("Open File"), Some(&main_window), FileChooserAction::Open);
    dialog.add_button("_Cancel", ResponseType::Cancel);
    dialog.add_button("_Open", ResponseType::Accept);

    upload_button
        .connect_clicked(move |_| {
            let meff_clone2 = Rc::clone(&meff);
            let result = dialog.run();
        match result {
            ResponseType::Cancel => {
            }
            ResponseType::Accept => {
                let file = dialog.get_filename();
                match file {
                    Some(file) =>  {
                        let file_path = file.into_os_string().into_string().unwrap().clone();
                        add_music_title(file_path, meff_clone2);
                    },
                    _ => {},
                }
            }
            _ => {}
        }
            dialog.hide();
        });

    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let textbox = gtk::Entry::new();
    let h_box_label = Label::new(Some("Title"));

    let download_button = gtk::Button::new_with_label("Download");
    let stream_button = gtk::Button::new_with_label("Stream");
    let textbox_clone_stream = textbox.clone();
    let textbox_clone_button = textbox.clone();
    stream_button.connect_clicked(move |button| {
        let title = textbox_clone_stream.get_text().unwrap().as_str().to_string();
        meff_clone_stream.borrow_mut().stream(title);
    });

    download_button.connect_clicked(move |_| {
        let title = textbox_clone_button.get_text().unwrap().as_str().to_string();
        meff_clone_download.borrow_mut().download(title);
    });

    let list_box = gtk::ListBoxBuilder::new().activate_on_single_click(true).build();
    let l_b_clone = list_box.clone();

    receiver.attach(None, move |(text, instr)| {
        if instr == "New" {
            let meff_clone_3 = Rc::clone(&meff_clone_l);
            let clone = text.clone();
            let text_clone = Rc::new(clone);
            add_song_to_list(text_clone, &l_b_clone, meff_clone_3);
        }
        if instr == "Delete" {
            let text_clone_2 = text.clone();
            for element in l_b_clone.get_children() {
                let element_clone = element.clone().downcast::<gtk::ListBoxRow>().unwrap();
                let h_box = element_clone.get_child().clone().unwrap().downcast::<gtk::Box>().unwrap();
                let button = h_box.get_children()[0].clone().downcast::<gtk::Button>().unwrap();
                let title = button.get_label().unwrap().as_str().to_string();
                if title == text_clone_2 {
                    l_b_clone.remove(&element);
                }
            }
        }
        glib::Continue(true)

    });

    let title_db = Label::new(Some("Your Music"));
    let controller_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    title_db.set_margin_top(10);

    let play_music = gtk::Button::new();
    let pause_music = gtk::Button::new();
    let stop_music = gtk::Button::new();

    let image_play = gtk::Image::new_from_file("src/play.png");
    let image_pause = gtk::Image::new_from_file("src/pause.png");
    let image_stop = gtk::Image::new_from_file("src/stop.png");

    play_music.set_image(Some(&image_play));
    pause_music.set_image(Some(&image_pause));
    stop_music.set_image(Some(&image_stop));

    play_music.connect_clicked(move |_| {
        let meff_clone_4 = Rc::clone(&meff_clone_play);
        meff_clone_4.borrow_mut().play(None);
    });

    pause_music.connect_clicked(move |_| {
        let meff_clone_5 = Rc::clone(&meff_clone_pause);
        meff_clone_5.borrow_mut().pause();
    });

    stop_music.connect_clicked(move |_| {
        let meff_clone_6 = Rc::clone(&meff_clone_stop);
        meff_clone_6.borrow_mut().stop();
    });

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    gtk::WidgetExt::set_widget_name(&scrolled_window, "scrollview");
    scrolled_window.add(&list_box);
    scrolled_window.set_size_request(100, 200);
    scrolled_window.set_valign(gtk::Align::Start);

    let frame = gtk::Frame::new(Option::from("Music Control"));
    gtk::WidgetExt::set_widget_name(&frame, "frame");
    let middle_sep = gtk::Separator::new(gtk::Orientation::Vertical);

    controller_box.pack_start(&play_music, false, true, 0);
    controller_box.pack_start(&pause_music, false, true, 0);
    controller_box.pack_start(&stop_music, false, true, 0);
    controller_box.set_halign(gtk::Align::Center);
    controller_box.set_valign(gtk::Align::End);

    v_box2.pack_start(&title_db, false, true, 0);
    v_box2.pack_start(&scrolled_window, false, true, 0);
    v_box2.pack_start(&upload_button, true, false, 10);

    h_box.pack_start(&h_box_label, false, true, 0);
    h_box.pack_start(&textbox, false, true, 0);
    h_box.set_margin_top(20);
    h_box.set_margin_bottom(10);


    v_box.pack_start(&h_box, false, true, 0);
    v_box.pack_start(&download_button, false, false, 0);
    v_box.pack_start(&stream_button, false, false, 0);

    h_box_window.pack_start(&v_box, false, false, 10);
    h_box_window.pack_start(&middle_sep, false, false, 10);
    h_box_window.pack_start(&v_box2, true, true, 10);

    frame.add(&h_box_window);

    header.pack_start(&menu_bar);

    v_box_window.pack_start(&label, false, true, 0);
    v_box_window.pack_start(&label2, false, true, 0);
    v_box_window.pack_start(&frame, true, true, 0);
    v_box_window.pack_start(&controller_box, true, true, 10);

    main_window.add(&v_box_window);
    main_window.show_all();
    startup_window.set_modal(true);
    startup_window.set_transient_for(Some(&main_window));
    startup_window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Fabian Frey, Marcel Baur, Elena Liebl, Franziska Lang"]);
        p.set_website_label(Some("MEFF - Music Entertainment For Friends "));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_title("About");
        p.set_transient_for(Some(&main_window));
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
        // @TODO check if it is okay to create our application model here
        let meff = Rc::new(RefCell::new(MEFFM::new()));
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        meff.borrow_mut().set_sender(tx);
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
        build_ui(app, meff, rx);
    });

    application.run(&args().collect::<Vec<_>>());
    print!("run");
}
