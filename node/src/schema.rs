use exonum::crypto::Hash;
use exonum::storage::{Fork, MapIndex, Snapshot};

use proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Voter")]
pub struct Voter {
    /// Voter's uinque identifier
    pub uid: Hash,
    /// Hash of voter's secret ballot
    pub ballot: Hash,
}

impl Voter {
    pub fn new(&uid: &Hash, &ballot: &Hash) -> Self {
        Voter { uid, ballot }
    }
}

const VOTERS_DB_KEY: &str = "service.exopoll.voters";

pub struct PollSchema<T> {
    view: T,
}

impl<T: AsRef<dyn Snapshot>> PollSchema<T> {
    pub fn new(view: T) -> Self {
        PollSchema { view }
    }

    pub fn get_voter(&self, uid: &Hash) -> Option<Voter> {
        let voters: MapIndex<_, Hash, Voter> =
            MapIndex::new(VOTERS_DB_KEY, self.view.as_ref());
        voters.get(&uid)
    }

    pub fn get_voters(&self) -> Vec<Voter> {
        let voters: MapIndex<_, Hash, Voter> =
            MapIndex::new(VOTERS_DB_KEY, self.view.as_ref());
        let vec = voters.values();
        vec.collect()
    }
}

impl<'a> PollSchema<&'a mut Fork> {
    pub fn add_voter(&mut self, uid: &Hash, ballot: &Hash) {
        if self.get_voter(&uid).is_none() {
            let mut voters: MapIndex<&mut Fork, Hash, Voter> =
                MapIndex::new(VOTERS_DB_KEY, &mut self.view);
            let voter = Voter::new(uid, ballot);
            voters.put(&uid, voter);
        }
    }
}
