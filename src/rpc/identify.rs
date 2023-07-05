use crate::rpc::Result as RpcResult;
use beep_evdev::Melody;

pub fn identify() -> RpcResult {
    let acoustic_result = Melody::default().play();
    let visual_result = identify_visually();

    match (acoustic_result, visual_result) {
        (Err(acoustic_error), Err(visual_error)) => {
            RpcResult::Error(format!("{acoustic_error} + {visual_error}"))
        }
        (Err(acoustic_error), _) => RpcResult::Error(format!("{acoustic_error}")),
        (_, Err(visual_error)) => RpcResult::Error(format!("{visual_error}")),
        _ => RpcResult::Success(None),
    }
}

fn identify_visually() -> Result<(), std::io::Error> {
    todo!("Show an identification message on-screen.")
}
