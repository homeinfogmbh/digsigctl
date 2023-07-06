use crate::rpc::beep::beep;
use crate::rpc::error::Error;
use crate::rpc::Result;
use std::sync::mpsc::SyncSender;

pub fn identify(sender: &SyncSender<String>) -> Result {
    let beep_result = beep(None);
    let show_error = sender.send("show".to_string()).map_or_else(
        |error| Result::Error(Error::from(error.to_string()).into()),
        |_| Result::Success(None),
    );

    beep_result + show_error
}
