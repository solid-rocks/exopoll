use exonum::crypto::Hash;
use exonum::storage::{Fork, MapIndex, Snapshot};

encoding_struct! {
    struct Voter {
        /// Voter's uinque identifier
        uid: &Hash,
        /// Hash of voter's secret ballot
        ballot: &Hash,
    }
}

const VOTERS_DB_KEY: &str = "service.exopoll.voters";

pub struct PollServiceSchema<T> {
    view: T,
}

impl<T: AsRef<dyn Snapshot>> PollServiceSchema<T> {
    pub fn new(view: T) -> Self {
        PollServiceSchema { view }
    }

    pub fn get_voter(&self, uid: &Hash) -> Option<Voter> {
        let voters: MapIndex<_, Hash, Voter> = MapIndex::new(
            VOTERS_DB_KEY,
            self.view.as_ref());
        voters.get(&uid)
    }

    pub fn get_voters(&self) -> Vec<Voter> {
        let voters: MapIndex<_, Hash, Voter> = MapIndex::new(
            VOTERS_DB_KEY,
            self.view.as_ref());
        let vec = voters.values();
        vec.collect()
    }
}

impl<'a> PollServiceSchema<&'a mut Fork> {
    pub fn add_voter(&mut self, uid: &Hash, ballot: &Hash) {
        if None == self.get_voter(&uid) {
            let mut voters: MapIndex<&mut Fork, Hash, Voter> =
                MapIndex::new(VOTERS_DB_KEY, &mut self.view);
            let voter = Voter::new(uid, ballot);
            voters.put(&uid, voter);
        }
    }
}
