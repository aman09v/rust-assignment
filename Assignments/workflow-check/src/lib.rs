#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::test_workflow;
    #[test]
    assert_eq!(test_workflow(1, 2), 5);
}
#[allow(dead_code)]
fn test_workflow(a: i32, b: i32) -> i32 {
    a + b
}
#[allow(dead_code)]
fn test_workflow1(a: i32, b: i32) -> i32 {
    a + b
}
