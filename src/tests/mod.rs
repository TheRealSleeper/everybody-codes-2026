use super::*;

fn test(func: fn(&str) -> Answer, answer: Answer, path: &str) {
    assert_eq!(
        func(&std::fs::read_to_string(path).expect("Unable to read file")),
        answer
    );
}

#[test]
fn test1() {
    test(part1, todo!(), "sample1.txt");
}

#[test]
fn test2() {
    test(part2, todo!(), "sample1.txt");
}