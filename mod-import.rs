
use std::default::Default;
mod foo{
    pub trait Foo<T:Default>{
        fn a()->Box<T> ;
    }
}

mod bar{
    use foo::Foo;
    struct Bar<T:Default>{
        t : T
    }
    impl<T> Foo<T> for Bar<T> where T:Default{
        fn a()->Box<T>{Box::new(T::default())}
    }
}

fn main(){
}
