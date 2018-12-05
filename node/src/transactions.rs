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
        // TODO: only registered authotites are able to register voters
        true
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = PollServiceSchema::new(view);
        schema.add_voter(self.uid(), self.ballot());
        Ok(())
    }
}
