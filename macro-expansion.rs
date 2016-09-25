trait Foo{
    fn bar(&self);
    fn foo(&self)->i32;
}

struct Binding;

impl Binding{
    fn serialize(&self){println!("serialize called");}
}

macro_rules! def_impl{

    ($tr: ty, $(($mname:ident, $ret:ty)),+) => {
        struct Proxy{
            binding : Binding
        }

        impl Proxy{
            fn new(b : Binding)->Self{
                Proxy{
                    binding : b
                }
            }
        }
        impl $tr for Proxy{
            $(def_method!($ret, $mname);)* 
        }
    }
}

macro_rules! def_method{
    ($ret : ty, $name : ident) => {
        fn $name(&self)->$ret{

            println!("serialization done");
            <$ret> ::default()
        }
    };

    ($name : ident) => {
        fn $name(&self){

        }
    };
}

def_impl!(Foo, (bar, ()), (foo, i32));
//def_method!(foo);
//def_method!(i32, bar);
fn create_proxy()->Box<Foo>{
    Box::new(
        Proxy{
            binding : Binding 
        }
    )
}

fn main(){
    let f : Box<Foo> = create_proxy();
    assert!(f.bar() == ());
    
}
