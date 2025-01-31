use anyhow::Result;
pub trait HashFunction{
    fn hash(&self, x: usize)-> usize;
}

pub struct HashMod{
    pub modulus: usize,
}

impl HashMod{
    pub fn new(lmod: usize)->Self{
        Self { modulus: lmod }
    }
}

impl HashFunction for HashMod {
    fn hash(&self, x: usize)-> usize {
        x % self.modulus
    }
}

pub struct CuckooTable{
    pub hash1: Box<dyn HashFunction>,
    pub hash2: Box<dyn HashFunction>,
    pub table1: Vec<Option<usize>>,
    pub table2: Vec<Option<usize>>,
    pub size: usize
}

impl CuckooTable{
    pub fn new(h1: Box<dyn HashFunction>, h2: Box<dyn HashFunction>, sz: usize) -> Self{
        let table1 =  vec![None; sz];//Vec::with_capacity(sz);
        let table2 = vec![None; sz];
        Self {hash1: h1, hash2: h2, size:sz, table1, table2 }
    }
    
    pub fn insert(&mut self, xi: usize) -> Result<(), &str>{
        // insert element x into either table
        let mut x = xi;
        loop {
            // loop until we face a cycle or until we find the last position to insert
            let index1 = self.hash1.hash(x) % self.size;
            if let Some(y) = self.table1[index1] {
                self.table1[index1] = Some(x); 
                let index2 = self.hash2.hash(y) % self.size;
                if let Some(z) = self.table2[index2]{
                    if z==xi{
                        return Err("cycle, rehash");
                    }else{
                       self.table2[index2] = Some(y); // kick z out of the table
                       x = z; // then we continue the insertion to the first table
                       continue;
                    } 
                }
                else{
                    self.table2[index2] = Some(y);
                    break;
                }
            } else {
                self.table1[index1] = Some(x);
                break;
            }
        }
        Ok(())
    } 

    pub fn lookup(&self, x: usize) -> bool{
        let index1 = self.hash1.hash(x) % self.size;
        let index2 = self.hash2.hash(x) % self.size;
        match (self.table1[index1], self.table2[index2]) {
            (Some(val1), _) if val1 == x => true,
            (_, Some(val2)) if val2 == x => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    //test hash funciton
    #[test]
    fn hashmod(){
        let h1 = HashMod::new(13);
        let x = 24;
        assert_eq!(h1.hash(x), 11);
    }

    #[test]
    fn init_test(){
        let h1 = HashMod::new(13);
        let h2 = HashMod::new(11);
        let tab= CuckooTable::new(Box::new(h1), Box::new(h2), 13);
        assert_eq!(tab.table1[10], None);
    }

    #[test]
    fn insert_test1(){
        let h1 = HashMod::new(13);
        let h2 = HashMod::new(11);
        let mut tab= CuckooTable::new(Box::new(h1), Box::new(h2), 13);
        let x = 10;
        assert_eq!(tab.insert(x),Ok(()));
        assert_eq!(tab.table1[10], Some(x));
    }

    #[test]
    //#[should_panic]
    fn cycle_test(){
        let h1 = HashMod::new(13);
        let h2 = HashMod::new(11);
        let mut tab= CuckooTable::new(Box::new(h1), Box::new(h2), 13);
        assert_eq!(tab.insert(1),Ok(()));
        assert_eq!(tab.insert(144),Ok(()));
        assert_eq!(tab.insert(287),Err("cycle, rehash"));
    }

    #[test]
    fn lookup_test(){
        let h1 = HashMod::new(13);
        let h2 = HashMod::new(11);
        let mut tab= CuckooTable::new(Box::new(h1), Box::new(h2), 13);
        assert_eq!(tab.insert(222), Ok(()));
        assert_eq!(tab.insert(111), Ok(()));
        assert!(tab.lookup(222) && tab.lookup(111));
        assert!(!tab.lookup(123));
    }
}

