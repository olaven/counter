extern crate iron;
extern crate iron_cors;
extern crate router;
// extern crate iron_cors;

mod controller; 

fn main() {
    controller::start();
}