use exonum::blockchain::{ExecutionResult, Transaction, TransactionContext};
use exonum::crypto::Hash;

use proto;
use schema::PollSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxRegisterBallot")]
pub struct TxRegisterBallot {
    hash: Hash,
}

impl Transaction for TxRegisterBallot {
    // FIXME: only registered authotites are able to register voters
    // FIXME: check if `uid` or `ballot` are already registered
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let view = context.fork();
        let mut schema = PollSchema::new(view);
        schema.add_ballot(&self.hash);
        Ok(())
    }
}

impl TxRegisterBallot {
    pub fn new(&hash: &Hash) -> Self {
        TxRegisterBallot { hash }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxCloseRegistration")]
pub struct TxCloseRegistration;
impl Transaction for TxCloseRegistration {
    // FIXME: only registered authotity are able to close registration
    fn execute(&self, mut _context: TransactionContext) -> ExecutionResult {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum PollTransactions {
    RegisterBallot(TxRegisterBallot),
    CloseRegistration(TxCloseRegistration),
}
