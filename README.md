# rstest-log

Wrapper for the [test_log](https://github.com/d-e-s-o/test-log) crate to support
[rstest](https://github.com/la10736/rstest).

## Usage

```rust
use rstest_log::rstest;

#[rstest(tokio::test)]
#[case(1)]
#[case(2)]
async fn test(#[case] val: i64) {
    assert_eq!(val, val);
}
```
