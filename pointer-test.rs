struct Person {
    id : usize,
    name : String
}

fn main() {
    let mut p = Person {id : 12, name : "abhi".into()};
    let ptr = &mut p as *mut Person;
    unsafe {
        println!("{} {}", (*ptr).id, (*ptr).name);
    }
}
