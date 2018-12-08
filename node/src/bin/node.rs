extern crate exonum;
extern crate exopoll;

use exonum::{crypto, blockchain, helpers, helpers::fabric};


#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        exopoll::service::SERVICE_NAME
    }

    fn make_service(&mut self, _: &fabric::Context) -> Box<dyn blockchain::Service> {
        Box::new(exopoll::service::PollService)
    }
}

fn main() {
    crypto::init();
    helpers::init_logger().unwrap();

    fabric::NodeBuilder::new()
        .with_service(Box::new(ServiceFactory))
        .run();
}
