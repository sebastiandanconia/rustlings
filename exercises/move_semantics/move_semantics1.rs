// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // Either ^ the "mut" in the function prototype above
    // or v the "mut" in the redeclaration below is sufficient for a successful compile.
    let mut vec = vec;

    vec.push(88);

    vec
}
