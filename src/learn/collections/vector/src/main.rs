fn main() {
    // Init Vector
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3]; // macro

    // Update a vector
    let mut v = Vec::new(); // infers type <i32> from the data
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Access to a element
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100); // == None

    // cannot borrow `v` as mutable because it is also borrowed as immutable
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
                       // v.push(6); // mutable borrow occurs here
    println!("The first element is: {}", first); // immutable borrow later used here

    // iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * dereference operator
    }
    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
