use std::collections::{HashMap};
fn memoize<'a, F>(f:F, h : &'a mut HashMap<String, bool>)->Box<FnMut(String)->bool + 'a> where F:'a+Fn(String)->bool
{
    Box::new(move |x:String|->bool{
        if h.contains_key(&x){
            *h.get(&x).unwrap()
        }
        else{
           let r = f(x.clone());  
           h.insert(x.clone(), r);
           r
        }
        })
}
fn main() {
    let mut h  = HashMap::new();
    fn a(x:String)->bool{println!("found");true}
    println!("{}", memoize(a, &mut h)(String::from("abhi"))); 
    println!("{}", memoize(a, &mut h)(String::from("abhi"))); 
    println!("{}", memoize(a, &mut h)(String::from("rust"))); 
}
