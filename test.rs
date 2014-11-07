enum JSONType{
    JNum(int),
    JBool(bool),
    JString(String),
    JObject(Vec<(String,  JSONType)>),
    JArray(Vec<JSONType>)
}

fn main(){
    let v = JObject(vec![("id".to_string(), JNum(1)),
                                              ("KIA".to_string(), JBool(true)),
                                              ("weapons".to_string(), JArray(vec![JNum(12), JNum(43)])),
                                              ("dog_tag_desc".to_string(), JString("abhi".to_string()))
                                            ]);
    pretty_print(&v);
}

fn pretty_print( o : &JSONType){
    match o{
        &JNum(ref n) => {println!("{},", n)},
        &JBool(ref b) => {println!("{},", b )},
        &JString(ref s) => {println!("\"{}\"'", s)},
        &JArray(ref a) => {
            println!("[");
            for i in a.iter(){ //since we are not 'destructing', we already have a ref
                pretty_print(i); //already a ref, no  need to add &
            }
            println!("],");
        },
        &JObject(ref v) => {
            println!("{{");
            for &(ref s, ref t) in v.iter(){ //since we are 'destructing', we need ref to catch
                //let v : (String, JSONType) = i.val1();
                print!("{}:", s);
                pretty_print(t); // //already a ref, no  need to add &
            }
            println!("}}");
        },
    }
}

fn foo(vec : &Vec<JSONType>){
}
