pub mod service {
    use backyard::backyard::Vegetable;

    pub fn run(vegetable: Vegetable) {
        println!("service run");
		println!("vegetable: {:?}", vegetable);
    }
}
