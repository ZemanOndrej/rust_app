use yard::backyard::entities::Vegetable;

mod backend;
use log::{error, info, warn};
use log4rs;

fn main() {
    let vegetable = Vegetable::Onion;
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");
    backend::service::run(vegetable);
    backend::service::run(Vegetable::Cabbage);
    backend::service::run(Vegetable::Carrot);
	backend::repository::user_repository::get_users();
	info!("shutting down");
}
