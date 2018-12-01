use iron::{Response, IronError, Request, status}; 
use iron::headers::AccessControlAllowOrigin; 

use storage::get_stored_count; 
use storage::increment_stored_count; 

pub fn increment_handler(_request : &mut Request) -> Result<Response, IronError> {
    increment_stored_count();  

    let mut response = Response::new();
    response.headers.set(AccessControlAllowOrigin::Any); 
    response.status = Some(status::Ok); 
    
    Ok(response)
}

pub fn retrieve_handler(_request : &mut Request) -> Result<Response, IronError> {
    let counter = get_stored_count(); 
    let counter_string : String = counter.to_string(); 

    let mut response = Response::new(); 
    response.headers.set(AccessControlAllowOrigin::Any); 
    response.status = Some(status::Ok);  

    response.body = Some(Box::new(counter_string)); 
    
    Ok(response)
}
