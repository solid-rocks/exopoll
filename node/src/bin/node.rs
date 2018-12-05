extern crate clap;
extern crate exonum;
extern crate exopoll;

use clap::{App, AppSettings, Arg};
use exonum::helpers::config::ConfigFile;
use exonum::node::Node;
use exonum::storage::MemoryDB;

use exopoll::service;

fn main() {
    let app = App::new("Exopoll node")
        .setting(AppSettings::ArgRequiredElseHelp)
        .about("Run node with a specified config.")
        .arg(
            Arg::with_name("CONFIG")
                .help("Node's config file")
                .required(true),
        ).get_matches();

    let config_file = app.value_of("CONFIG").unwrap();
    let config = ConfigFile::load(config_file).expect("invalid config");

    exonum::helpers::init_logger().unwrap();

    let node = Node::new(
        MemoryDB::new(),
        vec![Box::new(service::PollService)],
        config,
        None, // config path
    );
    node.run().unwrap();
}
