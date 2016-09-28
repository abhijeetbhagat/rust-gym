


#[derive(Clone)]
struct CC<T:Clone>{
   d : Box<T>
}

impl<T:Clone> CC<T>{
  fn new(d : T)->Self{
    CC{d:Box::new(d)}
  }
  
  fn create(&self)->Box<T>{
    Box::new(*self.d.clone())         
  }
}

trait S{
  fn add(&self, a:i32, b:i32) -> i32;
}

#[derive(Clone)]
struct C;


impl S for C{
 fn add(&self, a:i32, b:i32) -> i32{
    a+b
 }
}


fn main(){
 let c = C;
  let c : CC<&S> = CC::new(&c);
  let s : &S = *c.create();
  assert!(s.add(1,1)==2);
}
