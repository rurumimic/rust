macro_rules! my_vec {
    // first matcher
    () => [ // first transcriber
        Vec::new()
    ]; // end of transcriber
    (make an empty vec) => ( // second matcher and transcriber
        Vec::new()
    );

    // one expression bind to $x
    {$x:expr} => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };

    // multiple expressions with comma separated
    [$($x:expr),+] => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    )
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec!(make an empty vec);
    println!("{:?}", also_empty);

    let one_number = my_vec![1];
    println!("{:?}", one_number);

    let three_numbers = my_vec![1, 2, 3];
    println!("{:?}", three_numbers);

    /***/

    let empty: Vec<i32> = my_vec!();
    println!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec![make an empty vec];
    println!("{:?}", also_empty);

    let one_number = my_vec!{1};
    println!("{:?}", one_number);

    let three_numbers = my_vec!(1, 2, 3);
    println!("{:?}", three_numbers);
}
