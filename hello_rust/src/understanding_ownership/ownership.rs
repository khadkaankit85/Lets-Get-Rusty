#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ownership_fun() {
    /*
        let aarati = String::from("Angkit Khadka");
        let new_girl = aarati;
        println!("{}", new_girl);
        let aarati = new_girl;
        println!("{} ", aarati);
    */
    //example of giving the ownerhip
    /*
            let name: String = String::from("Angkit");
            let lastname: String = String::from("Khadka");
            let (fullname, lastname) = join_name(name, lastname);
            println!("{}", fullname);
            //println!("{}", lastname); //this is wrong because the new owner of fullname is the function
            println!("{}", lastname); //this is wrong because the new owner of fullname is the function
        }

            //if we want to return lastname to the previous owner, we have to return it like this
        //fn join_name(firstname: String, lastname: String) -> (String, String) {
        let fullname = firstname + &lastname;
        (fullname, lastname)
    }
        */

    let name: String = String::from("Angkit");
    let lastname: String = String::from("Khadka");

    let fullname = join_name(&name, &lastname);
    println!("{}{}", name, lastname);
    println!("full name is {}", fullname);
}

fn join_name(firstname: &str, lastname: &str) -> String {
    let fullname = format!("{} {}", firstname, lastname);
    fullname
}
