use crate::rpc::beep::beep;
use crate::rpc::Result as RpcResult;

pub fn identify() -> RpcResult {
    beep(None)
}
