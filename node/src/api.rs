use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::blockchain::Transaction;
use exonum::crypto::Hash;
use exonum::node::TransactionSend;

use schema::{Voter, PollServiceSchema};
use transactions::TxRegisterVoter;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVoterQuery {
    uid: Hash,
}

pub struct PollServiceApi;

impl PollServiceApi {
    fn add_voter(
        state: &ServiceApiState,
        tx: TxRegisterVoter) -> api::Result<Hash>
    {
        let tx_box: Box<dyn Transaction> = Box::new(tx);
        let tx_hash = tx_box.hash();
        match state.sender().send(tx_box) {
            Ok(_) => Ok(tx_hash),
            Err(err) => Err(api::Error::BadRequest(err.to_string())),
        }
    }

    fn get_voter(
        state: &ServiceApiState,
        query: GetVoterQuery,
    ) -> api::Result<Voter> {
        let schema = PollServiceSchema::new(state.snapshot());
        schema
            .get_voter(&query.uid)
            .ok_or_else(|| api::Error::NotFound("No such voter".to_string()))
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_mut("v1/voter", Self::add_voter)
            .endpoint("v1/voter", Self::get_voter);
    }
}
