fn main(){
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let n = n.trim().parse::<u32>();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let list: Vec<u32> = input.split_whitespace().map(|str_n|str_n.parse::<u32>().unwrap_or(0)).collect();

    assert_eq!(n.unwrap_or(0) as usize, list.len());
    println!("{}", list.iter().fold(1, |acc, &x| acc * x));
}
