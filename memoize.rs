use std::collections::{HashMap};
struct StatefulMemoization{
    h : HashMap<String, bool>
}

impl StatefulMemoization{
    fn new()->Self{
        StatefulMemoization {
            h : HashMap::new()
        } 
    }
    fn memoize<'a, F : 'a + Fn(String)->bool>(&'a mut self, f : F) -> Box<FnMut(String)->bool + 'a>{
       Box::new(move
           |x:String|->bool{
               if self.h.contains_key(&x){
                   *self.h.get(&x).unwrap()
               }
               else{
                   let r = f(x.clone());
                   self.h.insert(x, r );
                   r 
               }
           }
           )
    }
}

fn main(){
    let mut s = StatefulMemoization::new();
    fn a (x : String) -> bool {println!("calculating");std::thread::sleep_ms(3000); true}
    println!("{}", s.memoize(a)(String::from("abhi")));
    println!("{}", s.memoize(a)(String::from("abhi")));
}
