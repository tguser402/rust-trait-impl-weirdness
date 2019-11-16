fn _f(x: usize, y: i64) -> bool {
    use std::convert::TryInto;
    if let Ok(y) = y.try_into() {
    // Error: ^ cannot infer type
        x == y
    } else {
        false
    }
}

fn main() {
    // Commenting this resolves the error
    use foo_lib::useful_func;
}
