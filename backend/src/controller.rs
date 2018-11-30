/**
 * I believe that this file should be separated into 
 * 3 files, at least. 
 * 1. storage 
 * 2. request-handlers 
 * 3. mapping paths/server-stuff (the remains of this file)
 */


use iron::{Iron, Request, Response, Chain, IronError, headers, IronResult, Handler, AroundMiddleware};
// use iron::method::Method;
use router::Router; 
use iron::headers::AccessControlAllowOrigin; 
use iron::status; 
use iron_cors::CorsMiddleware; 

use std::fs::File;
use std::io::Read;
use std::fs; 

pub fn start() {
    let mut router = Router::new(); 
    
    router.post("/increment", increment_handler, "increment"); 
    router.get("/retrieve", retrieve_handler, "retrieve"); 

    let mut chain = Chain::new(router); 

    // let cors = CorsMiddleware::with_allow_any(); 
    //chain.link_around(cors); 

    Iron::new(chain).http("localhost:8080").unwrap(); 
}

fn increment_handler(_request : &mut Request) -> Result<Response, IronError> {
    //counter = increment(counter); 
    //get count stored 
    // set stored count to old + 1

    increment_stored_count();  

    //let mut response = Response::new(); 
    //response.headers.set(headers::AccessControlAllowOrigin::Any); 

    // let cors = CorsMiddleware::with_allow_any(); 
    let mut response = Response::new();
    response.headers.set(AccessControlAllowOrigin::Any); 
    response.status = Some(status::Ok); 
    
    Ok(response)
}

fn retrieve_handler(_request : &mut Request) -> Result<Response, IronError> {
    let counter = get_stored_count(); 
    let counter_string : String = counter.to_string(); 

    let mut response = Response::new(); 
    response.headers.set(AccessControlAllowOrigin::Any); 
    response.status = Some(status::Ok);  

    response.body = Some(Box::new(counter_string)); 
    
    Ok(response)
}




pub fn increment_stored_count() {
    let stored_count = get_stored_count();

    set_stored_count(stored_count + 1); 
}


pub fn get_stored_count() -> u32 {
    //NOTE: Lot of duplication 
    let mut file = File::open("count.txt")
        .expect("file not found."); 

    let mut count_in_file = String::new(); 
    file.read_to_string(&mut count_in_file)
        .expect("something wrong when reading file."); 

    let count_as_int = count_in_file.parse::<u32>()
        .expect("error parsing stored count"); 

    count_as_int 
}


fn set_stored_count(count : u32) {
    let mut file = File::open("count.txt") 
        .expect("file not found"); 

    let count_as_string = count.to_string(); 
    fs::write("count.txt", count_as_string).expect("something wrong"); 
}