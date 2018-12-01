
use std::fs::File;
use std::io::Read;
use std::fs; 


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
    let _file = File::open("count.txt") 
        .expect("file not found"); 

    let count_as_string = count.to_string(); 
    fs::write("count.txt", count_as_string).expect("something wrong"); 
}