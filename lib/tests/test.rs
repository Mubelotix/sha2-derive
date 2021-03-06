use sha2_derive::*;

#[derive(Hashable)]
pub(crate) struct Test {
    test: String,
    test2: u32, // Comment
    pub(in crate) test3: Vec<i16>,
    test4: Vec<(i32, i64)>,
    /// Doc comment
    test5: &'static [u8],
    test6: std::collections::HashMap<String, Vec<u16>>,
}

#[test]
fn test() {
    let test = Test {
        test: "test".to_string(),
        test2: 42,
        test3: vec![1, 2, 3],
        test4: vec![(1, 2), (3, 4)],
        test5: b"test",
        test6: {
            let mut map = std::collections::HashMap::new();
            map.insert("test".to_string(), vec![1, 2, 3]);
            map
        },
    };

    let hash = test.hash();
    assert_eq!(hash.len(), 32);
}
