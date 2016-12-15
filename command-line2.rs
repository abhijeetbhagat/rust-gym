fn main(){
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);

    let n = n.trim().parse::<i32>().unwrap_or(0);

    let mut s = String::new();
    std::io::stdin().read_line(&mut s);

    println!("{}", n*2);
    println!("{}", s);
}
