pub fn is_string_empty(s: String) -> bool {
    s.is_empty()
}

pub fn is_string_empty_by_ref(s: &String) -> bool {
    s.is_empty()
}

pub fn demonstrate_ownership() {
    { // start scope
        let some_var: String = String::from("oh, a string");
        println!("{}", some_var); // `some_var` is valid

        println!("Is string empty? {}", is_string_empty_by_ref(&some_var)); // `some_var` is borrowed, ownership is maintained in local scope
        println!("Is string empty? {}", is_string_empty(some_var)); // `some_var` is moved, ownership is transferred to the new scope
    } // end scope
    // println!("{}", some_var); // `some_var` is not valid
}