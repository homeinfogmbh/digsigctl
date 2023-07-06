use crate::create;
use crate::rpc::Result as RpcResult;
use beep_evdev::Melody;
use gtk4::prelude::ApplicationExtManual;
use std::fs::read_to_string;

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
    create(read_to_string(HOSTNAME)?).run_with_args::<&str>(&[]);
    Ok(())
}
