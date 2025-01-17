pub mod contract;
mod error;
pub mod msg;
mod state;

pub use crate::error::ContractError;
pub use crate::state::Badge721Contract;
use cosmwasm_std::Empty;
use cw721_base::Extension;

pub type ExecuteMsg = badge721::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub mod entry {
    use super::*;
    use crate::{msg::QueryMsg, state::Badge721Contract};

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
    use cw2::set_contract_version;
    use cw721_base::Extension;
    use badge721::InstantiateMsg;
    use badge_std::Response;

    // version info for migration info
    pub const CONTRACT_NAME: &str = "crates.io:sg721-base";
    pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let res = Badge721Contract::<Extension>::default().instantiate(deps, env, info, msg)?;

        Ok(res
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Badge721Contract::<Extension>::default().execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Badge721Contract::<Extension>::default().query(deps, env, msg)
    }
}