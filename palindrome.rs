fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let input = input.trim().to_string();
    for (a,b) in input.chars().zip(input.chars().rev()){
        if a != b {println!("NO"); return;}
    }
    println!("YES");
}
