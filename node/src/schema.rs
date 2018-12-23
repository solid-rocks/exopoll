use exonum::crypto::Hash;
use exonum::storage::{
    Entry, Fork, ListProof, MapIndex, ProofListIndex, Snapshot,
};

const BALLOTS_DB_KEY: &str = "service.exopoll.ballots";
const BALLOTS_MAP_DB_KEY: &str = "service.exopoll.ballots_map";
const STATUS_DB_KEY: &str = "service.exopoll.status";

#[derive(Debug, Serialize, Deserialize)]
pub struct Ballot {
    pub index: u64,
    pub hash: Hash,
    pub path: ListProof<Hash>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PollStatus {
    RegistrationOpen = 0,
    RegistrationClosed = 1,
    VotingStarted = 2,
    VotingFinished = 3,
    ___Unknown,
}

// FIXME: Sadly, I don't know how to serialize enums with protobufs
impl PollStatus {
    fn from_u8(x: u8) -> PollStatus {
        match x {
            0 => PollStatus::RegistrationOpen,
            1 => PollStatus::RegistrationClosed,
            2 => PollStatus::VotingStarted,
            3 => PollStatus::VotingFinished,
            _ => PollStatus::___Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollInfo {
    // poll_id: u64
    pub status: PollStatus,
    pub ballots_count: u64,
    pub ballots_root: Hash,
}

pub struct PollSchema<T> {
    view: T,
}

impl<T: AsRef<dyn Snapshot>> PollSchema<T> {
    pub fn new(view: T) -> Self {
        PollSchema { view }
    }

    fn poll_status(&self) -> Entry<&dyn Snapshot, u8> {
        Entry::new(STATUS_DB_KEY, self.view.as_ref())
    }

    fn ballots(&self) -> ProofListIndex<&dyn Snapshot, Hash> {
        ProofListIndex::new(BALLOTS_DB_KEY, self.view.as_ref())
    }

    fn ballots_map(&self) -> MapIndex<&dyn Snapshot, Hash, u64> {
        MapIndex::new(BALLOTS_MAP_DB_KEY, self.view.as_ref())
    }

    pub fn poll_info(&self) -> PollInfo {
        let ballots = self.ballots();
        // FIXME: this is a bit ugly. We need to initialize `poll_status` entry
        // somehow before anybody is able to call this.
        let status = match self.poll_status().get() {
            Some(st) => PollStatus::from_u8(st),
            None => PollStatus::RegistrationOpen,
        };
        PollInfo {
            status,
            ballots_count: ballots.len(),
            ballots_root: ballots.merkle_root(),
        }
    }

    pub fn get_ballot_by_hash(&self, &hash: &Hash) -> Option<Ballot> {
        self.ballots_map().get(&hash).map(|index| Ballot {
            hash,
            index,
            path: self.ballots().get_proof(index),
        })
    }

    pub fn get_ballot_by_index(&self, index: u64) -> Option<Ballot> {
        let ballots = self.ballots();
        ballots.get(index).map(|hash| Ballot {
            index,
            hash,
            path: ballots.get_proof(index),
        })
    }
}

impl<'a> PollSchema<&'a mut Fork> {
    pub fn create_poll() {
        // self.get_ballots().len();
    }

    // pub fn freeze_ballots

    pub fn add_ballot(&mut self, hash: &Hash) {
        if self.get_ballot_by_hash(hash).is_none() {
            let index;
            {
                let mut ballots: ProofListIndex<&mut Fork, Hash> =
                    ProofListIndex::new(BALLOTS_DB_KEY, &mut self.view);
                ballots.push(*hash);
                index = ballots.len() - 1;
            }

            let mut ballots_map: MapIndex<&mut Fork, Hash, u64> =
                MapIndex::new(BALLOTS_MAP_DB_KEY, &mut self.view);
            ballots_map.put(&hash, index);
        }
        // else: TODO: report duplicate ballot registration attempt?
    }
}
