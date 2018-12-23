use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::crypto::Hash;
use exonum::messages::{Message, ServiceTransaction};
use exonum::node::ExternalMessage;

use schema::{self, PollSchema};
use service;
use transactions::{TxCloseRegistration, TxRegisterBallot};

pub struct PollServiceApi;

impl PollServiceApi {
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint_mut("v1/ballot", register_ballot)
            .endpoint("v1/ballot", get_ballot)
            .endpoint_mut("v1/close_registration", close_ballot_registration)
            .endpoint("v1/poll", get_poll_info);
    }
}

// NB. For testing purposes we allow anyone to register ballots, in real
// system only trusted authority should do this.
#[allow(clippy::needless_pass_by_value)]
fn register_ballot(state: &ServiceApiState, ballot: Hash) -> api::Result<Hash> {
    let tx = TxRegisterBallot::new(&ballot);
    send_self_signed_transaction(&state, tx)
}

#[allow(clippy::needless_pass_by_value)]
fn close_ballot_registration(
    state: &ServiceApiState,
    _query: (),
) -> api::Result<Hash> {
    send_self_signed_transaction(&state, TxCloseRegistration {})
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBallotQuery {
    pub id: Option<u64>,
    pub hash: Option<Hash>,
}

#[allow(clippy::needless_pass_by_value)]
fn get_ballot(
    state: &ServiceApiState,
    query: GetBallotQuery,
) -> api::Result<schema::Ballot> {
    let schema = PollSchema::new(state.snapshot());
    match query {
        GetBallotQuery {
            id: None,
            hash: Some(hash),
        } => schema
            .get_ballot_by_hash(&hash)
            .ok_or_else(|| api::Error::NotFound("No such ballot".to_string())),
        GetBallotQuery {
            id: Some(id),
            hash: None,
        } => schema
            .get_ballot_by_index(id)
            .ok_or_else(|| api::Error::NotFound("No such ballot".to_string())),
        _ => Err(api::Error::BadRequest(
            "(id | hash) must be provided".to_string(),
        )),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn get_poll_info(
    state: &ServiceApiState,
    _query: (),
) -> api::Result<schema::PollInfo> {
    let schema = PollSchema::new(state.snapshot());
    Ok(schema.poll_info())
}

fn send_self_signed_transaction<T: Into<ServiceTransaction>>(
    state: &ServiceApiState,
    tx: T,
) -> api::Result<Hash> {
    let signed_tx = Message::sign_transaction(
        tx,
        service::SERVICE_ID,
        *state.public_key(),
        state.secret_key(),
    );
    let tx_hash = signed_tx.hash();
    let msg = ExternalMessage::Transaction(signed_tx);
    match state.sender().send_external_message(msg) {
        Ok(_) => Ok(tx_hash),
        Err(err) => Err(api::Error::BadRequest(err.to_string())),
    }
}
