fn compose<F1, F2>(f1 : F1, f2 : F2) -> Box<Fn(i32)->i32> where F1 : 'static + Fn(i32)->i32, F2 : 'static + Fn(i32)->i32{
    Box::new(move|x : i32| -> i32{
        f1(f2(x))
    }
    )
}

fn main(){
    fn a(x:i32)->i32{x}
    fn b(x:i32)->i32{x}
    assert_eq!(compose(a, b)(21), 21);
}
