#[macro_use]
extern crate serde_json;
extern crate exonum;
extern crate exonum_testkit;
extern crate exopoll;

use exonum::api::Error;
use exonum::crypto::{self, Hash};
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

use exopoll::api::GetBallotQuery;
use exopoll::schema::{Ballot, PollInfo};
use exopoll::service::PollService;

#[test]
fn test_register_ballot() -> Result<(), Error> {
    let mut api = Api::new();
    let poll = api.poll_info()?;
    assert_eq!(poll.ballots_count, 0, "no ballots at the start");

    let ballot = crypto::hash(&[1, 2, 3]);
    api.register_ballot(&ballot).expect("register a ballot");
    let poll = api.poll_info()?;
    assert_eq!(poll.ballots_count, 1, "one ballot registered");

    // - get ballot by hash
    let res = api
        .get_ballot(&GetBallotQuery {
            hash: Some(ballot),
            id: None,
        })
        .expect("get ballot by hash");
    assert_eq!(res.index, 0, "get ballot index");
    assert_eq!(res.hash, ballot, "get ballot index");
    // - get ballot by id
    // assert_eq!(poll.ballots_count, ballot, "get ballot by index");
    Ok(())
}

// adding exisiting ballot should result in failed transaction

#[test]
fn test_registration_close() {
    // add ballot
    // close registration
    // add ballot
    // check poll status
}

#[test]
fn test_merkle_paths() {
    // add number of ballots
    // finalize registration
    // get their paths
    // check path validity
}

struct Api {
    api: TestKitApi,
    testkit: TestKit,
}

type ValueResult = Result<serde_json::Value, Error>;

impl Api {
    pub fn new() -> Self {
        let testkit = TestKitBuilder::validator()
            .with_service(PollService)
            .create();
        let api = testkit.api();
        Api { api, testkit }
    }

    pub fn poll_info(&mut self) -> Result<PollInfo, Error> {
        self.testkit.create_block();
        self.api.public(ApiKind::Service("poll")).get("v1/poll")
    }

    pub fn register_ballot(&mut self, ballot: &Hash) -> ValueResult {
        self.testkit.create_block();
        self.api
            .public(ApiKind::Service("poll"))
            .query(ballot)
            .post("v1/ballot")
    }

    pub fn get_ballot(&mut self, q: &GetBallotQuery) -> Result<Ballot, Error> {
        self.testkit.create_block();
        self.api
            .public(ApiKind::Service("poll"))
            .query(q)
            .get("v1/ballot")
    }
}
