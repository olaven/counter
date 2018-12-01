use iron::{Iron, Chain};
use router::Router; 

use handlers::increment_handler; 
use handlers::retrieve_handler; 

pub fn start() {
    let mut router = Router::new(); 
    
    router.post("/increment", increment_handler, "increment"); 
    router.get("/retrieve", retrieve_handler, "retrieve"); 

    let chain = Chain::new(router); 
    Iron::new(chain).http("localhost:8080").unwrap(); 
}

