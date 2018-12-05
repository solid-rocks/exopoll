use exonum::api::ServiceApiBuilder;
use exonum::blockchain::{Service, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding::Error as EncodingError;
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;

use api::PollServiceApi;
use transactions::PollServiceTransactions;

pub const SERVICE_ID: u16 = 0x5741;
pub const SERVICE_NAME: &str = "poll";

pub struct PollService;

impl Service for PollService {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn state_hash(&self, _view: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn tx_from_raw(&self, raw: RawTransaction)
        -> Result<Box<dyn Transaction>, EncodingError>
    {
        let tx = PollServiceTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PollServiceApi::wire(builder);
    }
}
