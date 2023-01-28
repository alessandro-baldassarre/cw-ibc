pub mod demo {
    use cosmwasm_schema::{cw_serde, QueryResponses};

    #[cw_serde]
    pub struct InstantiateMsg {
        /// Address of the current contract owner
        pub owner: String,
    }

    #[cw_serde]
    pub enum ExecuteMsg {}

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {}
}
