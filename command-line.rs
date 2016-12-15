fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let nums : Vec<u32> = input.split_whitespace().map(|str_num| str_num.parse::<u32>().unwrap_or(0)).collect();
    println!("{}", input);
    println!("{:?}", nums);
}
