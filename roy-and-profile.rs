fn main(){
    let mut l = String::new();
    std::io::stdin().read_line(&mut l);
    let l = l.trim().parse::<u32>().unwrap_or(0);

    let mut n = String::new();
    std::io::stdin().read_line(&mut n);

    for i in 0.. n.trim().parse::<u32>().unwrap_or(0){
        let mut tup_str = String::new();
        std::io::stdin().read_line(&mut tup_str);

        let mut list_iter = tup_str.split_whitespace().map(|_str|_str.parse::<u32>().unwrap_or(0));
        let (y, x) = (list_iter.next().as_ref().unwrap_or(&0).clone(), *list_iter.next().as_ref().unwrap_or(&0));
        if y > l && x > l{
            println!("CROP IT");
        } else if y == l && x == l{
            println!("ACCEPTED");
        } else {
            println!("UPLOAD ANOTHER");
        }
    }

}
