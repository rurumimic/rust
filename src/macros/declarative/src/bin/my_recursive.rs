#![feature(trace_macros)]
#![recursion_limit = "3"]

macro_rules! count {
    ($val: expr) => {
        if $val == 1 {
            1
        } else {
            count!($val - 1)
        }
    };
}

fn main() {
    trace_macros!(true);
    count!(3);
    trace_macros!(false);
}
