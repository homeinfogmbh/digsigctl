use crate::rpc::Result as RpcResult;
use beep_evdev::Melody;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use gtk4::glib::{self, clone, timeout_future_seconds, MainContext, Priority};
use gtk4::{Align, Button};
use std::fs::read_to_string;
use std::thread;

const GUI_TIMEOUT_SECONDS: u32 = 15;
const HOSTNAME: &str = "/etc/hostname";

pub fn identify() -> RpcResult {
    let acoustic_result = Melody::default().play();
    let visual_result = identify_visually();

    match (acoustic_result, visual_result) {
        (Err(acoustic_error), Err(visual_error)) => RpcResult::Error(
            [
                acoustic_error.to_string().into(),
                visual_error.to_string().into(),
            ]
            .as_slice()
            .into(),
        ),
        (Err(acoustic_error), _) => RpcResult::Error(acoustic_error.to_string().into()),
        (_, Err(visual_error)) => RpcResult::Error(visual_error.to_string().into()),
        _ => RpcResult::Success(None),
    }
}

fn identify_visually() -> Result<(), std::io::Error> {
    let hostname = read_to_string(HOSTNAME)?;
    thread::spawn(|| show_gui(hostname));
    Ok(())
}

fn show_gui(hostname: String) {
    let application = Application::builder()
        .application_id("de.homeinfo.digsigctl")
        .build();

    let (sender, receiver) = MainContext::channel(Priority::default());

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .modal(true)
            .halign(Align::Center)
            .valign(Align::Center)
            .title(hostname.trim())
            .build();

        add_close_button(&window);

        let window_clone = window.clone();
        MainContext::default().spawn_local(clone!(@strong sender => async move {
            timeout_future_seconds(GUI_TIMEOUT_SECONDS).await;
            sender.send(window_clone).expect("Could not send through channel");
        }));

        window.show();
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak application => @default-return Continue(false),
                    move |window| {
                        window.close();
                        Continue(true)
                    }
        ),
    );

    application.run_with_args::<&str>(&[]);
}

fn add_close_button(window: &ApplicationWindow) {
    let button = Button::with_label("Ok");
    let button_window = window.clone();
    button.connect_clicked(move |_| button_window.close());
    window.set_child(Some(&button));
}
