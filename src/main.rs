use cuckush::{HashMod, CuckooTable};

fn main(){
    let h1 = HashMod::new(13);
    let h2 = HashMod::new(11);
    let mut cuckoo_table = CuckooTable::new(Box::new(h1), Box::new(h2), 17);
    // insert
    let insert_set = vec![1,144,287];//vec![14,17,19,21,44];
    for item in insert_set{
        match cuckoo_table.insert(item){
            Ok(_) => continue,
            Err(e) => {println!("Error: {}",e); break;}
        }
    }
    // test look up
    println!("{}",cuckoo_table.lookup(19));
    println!("{}",cuckoo_table.lookup(20));
}
