
struct SourceText{
    text : String, 
    vec: Vec<char>,
    line_no : u32
}

impl SourceText{
    fn new()->Self{
        let s = "asd".to_string();
        let mut v : Vec<char> = Vec::new();
        for b in s.as_bytes(){
            v.push(*b as char);
        }
        SourceText {text : s, vec: v, line_no : 2}
    }
}
struct SourceTextIter{
    text : String,
    vec: Vec<char>,
    pos : usize
}

impl Iterator for SourceTextIter{
    type Item = char;
    
    fn next(&mut self)->Option<Self::Item>{
        if self.pos == self.text.len(){
            None
        }
        else{
            self.pos += 1;
            Some(self.text.as_bytes()[self.pos-1] as char)
        }
    }
}


impl IntoIterator for SourceText{
    type Item = char;
    type IntoIter = SourceTextIter;
    
    fn into_iter(self)->Self::IntoIter{
        SourceTextIter {text : self.text, vec: Vec::new(),  pos : 0}
    }
}

impl<'a> IntoIterator for &'a SourceText{
    type Item =  &'a char;
    type IntoIter = std::slice::Iter<'a, char>;

    fn into_iter(self)-> Self::IntoIter{
        self.vec.iter()
    }
}

fn main(){
    let src = SourceText::new();
    // for c in src{ error: moved value
    //     println!("{}", c);
    // } 

    for c in &src{
    }
}
