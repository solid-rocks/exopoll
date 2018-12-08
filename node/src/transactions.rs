use exonum::blockchain::{ExecutionResult, Transaction};
use exonum::crypto::Hash;
use exonum::storage::Fork;

use schema::PollServiceSchema;
use service;

transactions! {
    pub PollServiceTransactions {
        const SERVICE_ID = service::SERVICE_ID;

        struct TxRegisterVoter {
            uid: &Hash,
            ballot: &Hash,
        }
    }
}

impl Transaction for TxRegisterVoter {
    fn verify(&self) -> bool {
        // FIXME: only registered authotites are able to register voters
        // FIXME: check if `uid` or `ballot` are already registered
        true
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = PollServiceSchema::new(view);
        schema.add_voter(self.uid(), self.ballot());
        Ok(())
    }
}
