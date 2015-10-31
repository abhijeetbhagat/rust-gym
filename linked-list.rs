struct Node{
    d : i32,
    n : Option<Box<Node>>
}

impl Node{
    fn new(d : i32) -> Self{
        Node{d : d, n : None}
    }
}

fn traverse(h : &Node){
    let mut p = h;
    while !p.n.is_none(){
        println!("{}", p.d);
        p = p.n.as_ref().unwrap();
    }
    println!("{}", p.d);
}
fn main(){
    let mut n = Node::new(12);
    let mut n2 = Node::new(13);
    n.n = Some(Box::new(n2));
    traverse(&n);
    println!("{}", n.d);
}
