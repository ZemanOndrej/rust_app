use backyard::backyard::Vegetable;

mod backend;

fn main() {
   let vegetable = Vegetable::Cabbage;

   backend::service::run(vegetable);
}
