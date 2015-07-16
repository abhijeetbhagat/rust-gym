mod singleton{
    static mut instance : Singleton = Singleton {x : 5};
    
    pub struct Singleton{
        x : u32
    }

    impl Singleton{
        
        pub fn get()->&'static mut Singleton{
            unsafe{
                &mut instance
            }
        }

        pub fn set_x(&mut self, v : u32){
            self.x = v;
        }

        pub fn get_x(&self)->u32{
            self.x
        }
    }
}


fn main(){
    let s = singleton::Singleton::get();
    assert_eq!(s.get_x(), 5);
    s.set_x(2);
    let s2 = singleton::Singleton::get();
    assert_eq!(s2.get_x(), 2);
}
