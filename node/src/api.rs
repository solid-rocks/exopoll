use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::crypto::Hash;
use exonum::node::ExternalMessage;

use schema::{PollSchema, Voter};
use transactions::TxRegisterVoter;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVoterQuery {
    uid: Hash,
}

pub struct PollServiceApi;

impl PollServiceApi {
    // FIXME: this one must accept only signed transactions as only authorized participants
    // should be able to register voters.
    #[allow(clippy::needless_pass_by_value)]
    fn add_voter(state: &ServiceApiState, voter: Voter) -> api::Result<Hash> {
        let tx = TxRegisterVoter::new(&voter.uid, &voter.ballot);
        let signed_tx = tx.sign(state.public_key(), state.secret_key());
        let tx_hash = signed_tx.hash();
        let msg = ExternalMessage::Transaction(signed_tx);
        match state.sender().send_external_message(msg) {
            Ok(_) => Ok(tx_hash),
            Err(err) => Err(api::Error::BadRequest(err.to_string())),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn get_voter(
        state: &ServiceApiState,
        query: GetVoterQuery,
    ) -> api::Result<Voter> {
        let schema = PollSchema::new(state.snapshot());
        schema
            .get_voter(&query.uid)
            .ok_or_else(|| api::Error::NotFound("No such voter".to_string()))
    }

    fn get_all_voters(
        state: &ServiceApiState,
        _query: (),
    ) -> api::Result<Vec<Voter>> {
        let schema = PollSchema::new(state.snapshot());
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
