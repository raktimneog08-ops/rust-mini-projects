use fizzbuzz_cli::{fizzbuzz, fizzbuzz_custom, fizzbuzz_range};

#[test]
fn test_standard_fizzbuzz_boundaries() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(98), "98");
    assert_eq!(fizzbuzz(99), "Fizz");
    assert_eq!(fizzbuzz(100), "Buzz");
}

#[test]
fn test_large_numbers() {
    assert_eq!(fizzbuzz(300_000), "FizzBuzz");
    assert_eq!(fizzbuzz(300_001), "300001");
    assert_eq!(fizzbuzz(300_003), "Fizz");
    assert_eq!(fizzbuzz(300_005), "Buzz");
}

#[test]
fn test_custom_three_rule_fizzbuzz() {
    let custom_rules = [(3, "Fizz"), (5, "Buzz"), (7, "Bazz")];
    assert_eq!(fizzbuzz_custom(21, &custom_rules), "FizzBazz");
    assert_eq!(fizzbuzz_custom(35, &custom_rules), "BuzzBazz");
    assert_eq!(fizzbuzz_custom(105, &custom_rules), "FizzBuzzBazz");
    assert_eq!(fizzbuzz_custom(11, &custom_rules), "11");
}

#[test]
fn test_range_length_and_order() {
    let range = fizzbuzz_range(10, 20);
    assert_eq!(range.len(), 11);
    assert_eq!(range[0], "Buzz"); // 10
    assert_eq!(range[1], "11"); // 11
    assert_eq!(range[2], "Fizz"); // 12
    assert_eq!(range[3], "13"); // 13
    assert_eq!(range[4], "14"); // 14
    assert_eq!(range[5], "FizzBuzz"); // 15
    assert_eq!(range[10], "Buzz"); // 20
}
