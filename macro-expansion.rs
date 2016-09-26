#![feature(trace_macros)]
trait Service{
    fn bar(&self, a:i32, b:i32,);
}

struct Binding;

impl Binding{
    fn serialize(&self){println!("serialize called");}
}

macro_rules! expand_args{
    () => ();
    ($arghead:ident:$head:ty) => ($arghead:$head,);
    ($arghead:ident:$head:ty;$($argtail:ident:$tail:ty);*) => ($arghead:$head, expand_args!($($argtail:$tail);*));
}

macro_rules! def_impl{
    ($tr: ty, $(($mname:ident, $ret:ty => $($argname:ident:$argtype:ty);*)),+) => {
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
            $(def_method!($ret, $mname, $($argname:$argtype);*);)* 
        }
    }
}

macro_rules! def_method{
    ($ret : ty, $name : ident, $($argname:ident:$argtype:ty);*) => {
        fn $name(&self, expand_args!($($argname:$argtype);*);)->$ret{

            println!("serialization done");
            <$ret> ::default()
        }
    };
}

def_impl!(Service, (bar, () => a:i32;b:i32));

fn main(){
   trace_macros!(true) ;
}
