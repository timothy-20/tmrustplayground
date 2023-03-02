use tmrustplayground::cache;

#[test]
fn cache_test_1() {
    let values = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = values.iter().zip(values.iter().skip(1)).collect();

    eprintln!("result: {:?}", result);

    // let mut cache = Cache::new(|value| value - 1);
    // let result = cache.value(5);
    //
    // for x in cache {
    //     eprintln!("value: {}", x);
    // }

    assert!(false);
}