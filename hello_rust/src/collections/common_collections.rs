#[allow(dead_code)]
pub fn vector_collections() {
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(6);
    //println!("{:?}", nums);
    let mut myv = vec![String::from("one"), String::from("two")];
    /*
         let third = &myv[0];
     let nth = myv.get(0);
      match nth {
         None => println!("None"),
         Some(value) => println!("{}", value),
      }
         println!("{:?} {:?}", myv, third);
        println!("{:?}", nth);
    */
    //let first = &myv[0]; //this gives an error
    let first = myv[0].clone();
    myv.push(String::from("four"));
    println!("first: {}", &first);
    for mut s in myv.clone() {
        s += "Number";
        println!("{}", s);
    }
    println!("{:?}", &myv);
}
#[allow(dead_code)]
pub fn string_collections() {
    let str1 = "Hello".to_string();
    let str2 = " Aaru".to_string();
    let result = format!("{}{}", str1, str2);
    println!("{}{}{}", result, str1, str2);
}
