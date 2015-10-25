use std::ascii::AsciiExt;

fn is_isogram(s : &str)->Result<Option<bool>, &str> {
    let mut m = 0u32;    
    for b in s.to_ascii_lowercase().as_bytes(){
        if *b >= 97 && *b <= 122{
            let s = 1 << b % 97;
            if m & s == 0{
                m = m | s;
            }
            else{
                return Ok(Some(false)) 
            }
        }
        else{
            return Err("Invalid char found")
        }
    }
    Ok(Some(true))
}

fn main(){
    assert_eq!(is_isogram("abhi").unwrap().unwrap(), true);
    assert_eq!(is_isogram("rojo").unwrap().unwrap(), false); 
    assert_eq!(is_isogram("1rojo1").is_err(), true); 
}
