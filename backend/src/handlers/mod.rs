use iron::{Response, IronError, Request, status}; 
use iron::headers::AccessControlAllowOrigin; 

use storage::get_stored_count; 
use storage::increment_stored_count; 

pub fn increment_handler(_request : &mut Request) -> Result<Response, IronError> {
    increment_stored_count();  

    let mut response = Response::new();
    allow_all_origin(&mut response); 

    Ok(response)
}

pub fn retrieve_handler(_request : &mut Request) -> Result<Response, IronError> {
    let stored_count = get_stored_count(); 
    let count_as_string : String = stored_count.to_string(); 

    let mut response = Response::new(); 
    allow_all_origin(&mut response);  

    response.body = Some(Box::new(count_as_string)); 
    Ok(response)
}

fn allow_all_origin(response : &mut Response) {
    response.headers.set(AccessControlAllowOrigin::Any); 
    response.status = Some(status::Ok); 
}
