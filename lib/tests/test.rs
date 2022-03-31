use sha2_derive::*;

#[derive(Hashable)]
struct Test {
    test: String,
    test2: u32,
    test3: Vec<i16>,
    test4: &'static [u8],
}

#[test]
fn test() {
    let test = Test {
        test: "test".to_string(),
        test2: 42,
        test3: vec![1, 2, 3],
        test4: b"test",
    };

    let hash = test.hash();
    assert_eq!(hash.len(), 32);
}
