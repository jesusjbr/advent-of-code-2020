/*
This day I took a rogue route instead of implementing a solution myself.
I grabbed the crate `eval` and modified the priority of the operators sum and mul.

```Rust
#[test]
fn part1_same_priority() {
    let input = include_str!("../inputs/day18.txt");
    let res : u64 = input.lines().map(|l| match eval(l).unwrap() {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_u64().unwrap(),
        Value::String(_) => 0,
        Value::Array(_) => 0,
        Value::Object(_) => 0,
    }).sum();
    assert_eq!(res, 3885386961962);
}

#[test]
fn part2_sum_higher_priority() {
    let input = include_str!("../inputs/day18.txt");
    let res : u64 = input.lines().map(|l| match eval(l).unwrap() {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_u64().unwrap(),
        Value::String(_) => 0,
        Value::Array(_) => 0,
        Value::Object(_) => 0,
    }).sum();
    assert_eq!(res, 112899558798666);
}
*/