fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = String::from("xyz");

        // let result = longest(string1.as_str(), string2);
        result = longest(string1.as_str(), string2.as_str()); // borrowed value does not live long enough
    } // `string2` dropped here while still borrowed

    println!("The longest string is {}", result); // borrow later used her
}

// fn dangling_references() {
//     // `x` does not live long enough
//     let r;

//     {
//         let x = 5;
//         r = &x;
//         //  ^^ borrowed value does not live long enough
//     } // `x` dropped here while still borrowed

//     println!("r: {}", r);
//     //                - borrow later used here
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // error[E0106]: missing lifetime specifier
    // fn longest(x: &str, y: &str) -> &str {
    //               ----     ----     ^ expected named lifetime parameter

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
