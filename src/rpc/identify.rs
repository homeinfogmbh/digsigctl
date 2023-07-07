use crate::rpc::beep::beep;
use crate::rpc::Result;

pub fn identify() -> Result {
    beep(None)
}
