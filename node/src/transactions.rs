use exonum::blockchain::{ExecutionResult, Transaction, TransactionContext};
use exonum::crypto::{Hash, PublicKey, SecretKey};
use exonum::messages::{Message, RawTransaction, Signed};

use proto;
use schema::PollSchema;
use service;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxRegisterVoter")]
pub struct TxRegisterVoter {
    pub uid: Hash,
    pub ballot: Hash,
}

impl Transaction for TxRegisterVoter {
    // FIXME: only registered authotites are able to register voters
    // FIXME: check if `uid` or `ballot` are already registered
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let view = context.fork();
        let mut schema = PollSchema::new(view);
        schema.add_voter(&self.uid, &self.ballot);
        Ok(())
    }
}

impl TxRegisterVoter {
    pub fn new(&uid: &Hash, &ballot: &Hash) -> Self {
        TxRegisterVoter { uid, ballot }
    }

    pub fn sign(
        self,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(self, service::SERVICE_ID, *pk, sk)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum PollTransactions {
    RegisterVoter(TxRegisterVoter),
}
