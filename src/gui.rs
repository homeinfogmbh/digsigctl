use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use gtk4::glib::{clone, timeout_future_seconds, MainContext, Priority};
use gtk4::{Align, Button};
use std::fs::read_to_string;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

const TIMEOUT_SECONDS: u32 = 15;
const ETC_HOSTNAME: &str = "/etc/hostname";

#[must_use]
pub fn spawn() -> SyncSender<String> {
    let (sender, receiver) = sync_channel::<String>(32);
    thread::spawn(move || {
        while matches!(
            receiver.recv().expect("could not receive message").as_str(),
            "show"
        ) {
            show_hostname(read_to_string(ETC_HOSTNAME).unwrap_or_else(|_| "N/A".to_string()));
        }
    });
    sender
}

fn show_hostname(hostname: String) {
    let application = Application::builder()
        .application_id("de.homeinfo.digsigctl")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .modal(true)
            .halign(Align::Center)
            .valign(Align::Center)
            .title(hostname.trim())
            .build();

        add_close_button(&window);
        make_window_close_channel(window.clone());
        window.show();
    });

    application.run_with_args::<&str>(&[]);
}

fn add_close_button(window: &ApplicationWindow) {
    let button = Button::with_label("Ok");
    let button_window = window.clone();
    button.connect_clicked(move |_| button_window.close());
    window.set_child(Some(&button));
}

fn make_window_close_channel(window: ApplicationWindow) {
    let (sender, receiver) = MainContext::channel(Priority::default());

    MainContext::default().spawn_local(clone!(@strong sender => async move {
        timeout_future_seconds(TIMEOUT_SECONDS).await;
        sender.send(window).expect("Could not send through channel");
    }));

    receiver.attach(None, move |window| {
        window.close();
        Continue(true)
    });
}
