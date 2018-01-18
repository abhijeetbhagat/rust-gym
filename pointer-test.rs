struct Person {
    id : usize,
    name : String
}

fn main() {
    let mut ptr : *mut Person = std::ptr::null_mut();//= &mut p;// as *mut Person;
    {
        let mut p = Person {id : 12, name : "abhi".into()};
        ptr = &mut p; //implicit coercion from mut ref to mut ptr
    }
    unsafe {
        println!("{} {}", (*ptr).id, (*ptr).name); //prints garbage
    }
}
