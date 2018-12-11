use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::blockchain::Transaction;
use exonum::crypto::Hash;
use exonum::node::TransactionSend;

use schema::{PollServiceSchema, Voter};
use transactions::TxRegisterVoter;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVoterQuery {
    uid: Hash,
}

pub struct PollServiceApi;

impl PollServiceApi {
    // FIXME: this one must accept only signed transactions as only authorized participants
    // should be able to register voters.
    fn add_voter(state: &ServiceApiState, voter: Voter) -> api::Result<Hash> {
        let tx = TxRegisterVoter::new(
            voter.uid(),
            voter.ballot(),
            state.secret_key(),
        );

        let tx_box: Box<Transaction> = tx.into();
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

    fn get_all_voters(
        state: &ServiceApiState,
        _query: (),
    ) -> api::Result<Vec<Voter>> {
        let schema = PollServiceSchema::new(state.snapshot());
        Ok(schema.get_voters())
    }

    // fn prepare_vote()
    // fn inspect_vote()
    // fn commit_vote()

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_mut("v1/voter", Self::add_voter)
            .endpoint("v1/voter", Self::get_voter)
            .endpoint("v1/voters", Self::get_all_voters);
    }
}
