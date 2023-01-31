mod callback;
mod checks;
mod msg;

use cosmwasm_std::IbcOrder;

pub use crate::callback::ReceiveIcaResponseMsg;
pub use crate::checks::{check_order, check_version, SimpleIcaError};
pub use crate::msg::{
    BalancesResponse, DispatchResponse, IbcQueryResponse, PacketMsg, StdAck, WhoAmIResponse,
};

pub const IBC_APP_VERSION: &str = "cw-ibc-v1";
pub const APP_ORDER: IbcOrder = IbcOrder::Unordered;
