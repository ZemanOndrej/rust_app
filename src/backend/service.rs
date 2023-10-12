use log::{error, info, warn};
use yard::backyard::entities::Vegetable;
use yard::frontyard::Flowers;

pub fn run(vegetable: Vegetable) {
    info!("service run");
    match vegetable {
        Vegetable::Onion => info!("vegetable is onion"),
        Vegetable::Carrot => error!("vegetable is carrot"),
        _ => warn!("vegetable is not onion"),
    }
    info!(target:"backend", "flowers: {:?}", Flowers::Rose);
	work();
}

fn work() {
	info!("service work");
}