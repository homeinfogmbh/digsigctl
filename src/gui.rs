use gtk4::glib::{clone, timeout_future_seconds, MainContext, Priority};
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Button};
use std::fs::read_to_string;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

const TIMEOUT_SECONDS: u32 = 15;
const ETC_HOSTNAME: &str = "/etc/hostname";

#[must_use]
pub fn spawn() -> SyncSender<&'static str> {
    let (sender, receiver) = sync_channel::<&'static str>(32);
    thread::spawn(move || {
        while matches!(receiver.recv().expect("could not receive message"), "show") {
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

        let button = Button::with_label("Ok");
        button.connect_clicked(|button| {
            if let Some(parent) = button.parent() {
                if let Ok(window) = parent.downcast::<ApplicationWindow>() {
                    window.close();
                }
            }
        });

        window.set_child(Some(&button));
        set_timeout(window.clone(), TIMEOUT_SECONDS);

        window.show();
    });

    application.run_with_args::<&str>(&[]);
}

fn set_timeout(window: ApplicationWindow, seconds: u32) {
    let (sender, receiver) = MainContext::channel(Priority::default());

    receiver.attach(None, move |window: ApplicationWindow| {
        window.close();
        Continue(true)
    });

    MainContext::default().spawn_local(clone!(@strong sender => async move {
        timeout_future_seconds(seconds).await;
        sender.send(window).expect("Could not send through channel");
    }));
}
