struct Node{
    d : i32,
    l : Option<Box<Node>>,
    r : Option<Box<Node>>
}

impl Node{
    fn new(d : i32)->Self{
        Node {d : d, l : None, r : None}
    }
}

fn preorder(n : &Option<Box<Node>>){
    let _n = &*n.as_ref().unwrap();
    if _n.l.is_some(){
        preorder(&_n.l);
    }
    if _n.r.is_some(){
        preorder(&_n.r);
    }
    println!("{}", _n.d);
}
//this accepts an optional-boxed-node because when we recurse, we have the child node type as an
//optional-boxed-node
fn inorder(n : &Option<Box<Node>>){
    let _n = &*n.as_ref().unwrap(); //n is a borrowed ref to an optional-boxed-node. To unwrap, we use as_ref.
    //We still want to use a ref to this unwrapped value, because then it will mean a re-borrow
    //which the compiler will frown at. So using & will just calm down the compiler.
    if _n.l.is_some(){
        inorder(&_n.l);
    }
    println!("{}", _n.d);
    if _n.r.is_some(){
        inorder(&_n.r); 
    }
}

fn main(){
    let mut n = Node::new(0);
    let mut nl = Node::new(1);
    let mut nr = Node::new(2);
    n.l = Some(Box::new(nl)); 
    n.r = Some(Box::new(nr));
    let r = Some(Box::new(n));
    inorder(&r);
    preorder(&r);
}
