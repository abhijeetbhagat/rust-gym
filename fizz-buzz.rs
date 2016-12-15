fn main(){
    let mut input = String::new();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let n = n.trim().parse::<u32>().unwrap_or(0); 

    std::io::stdin().read_line(&mut input);
    let input : Vec<u32> = input.split_whitespace().map(|str_num| str_num.parse::<u32>().unwrap_or(0)).collect();

    assert_eq!(n , input.len() as u32);

    let v : Vec<()> = input.iter().map(|limit|{
        for i in 1..limit+1{
            if i%3 == 0 && i%5 == 0{
                println!("fizbuzz");
            } else if  i%3 == 0 {
                println!("fizz"); 
            } else if i%5 == 0 {
                println!("buzz"); 
            } else {
                println!("{}", i);
            } 
        }
        println!(""); 
    }).collect();
}
