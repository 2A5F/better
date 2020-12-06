use better::*;

#[better]
#[test]
fn test1() {
    let a = 1;
    let a = **"value: {a}";
    println!("{}", a);
    assert_eq!(a, "value: 1");
}
