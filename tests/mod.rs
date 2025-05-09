// Copyright (C) 2019-2024 Daniel Mueller <deso@posteo.net>
//   and David Morrison <drmorr@appliedcomputing.io>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use tokio::runtime::Builder;
use tracing::{
    debug,
    error,
    info,
    instrument,
};


mod something {
    pub type Error = String;
}

use something::Error;


#[rstest_log::rstest]
fn without_return_type() {
    assert_eq!(2 + 2, 4);
}

#[rstest_log::rstest]
fn with_return_type() -> Result<(), Error> {
    Ok(())
}

#[rstest_log::rstest]
#[should_panic(expected = "success")]
fn with_panic() {
    panic!("success")
}

#[rstest_log::rstest(tokio::test)]
async fn with_inner_test_attribute_and_async() {
    assert_eq!(async { 42 }.await, 42)
}

#[rstest_log::rstest]
#[case(-2, -4)]
fn with_inner_test_attribute_and_test_args(#[case] x: i8, #[case] y: i8) {
    assert_eq!(x, -2);
    assert_eq!(y, -4);
}

#[rstest_log::rstest]
#[case::my_test_name(-2, -4)]
fn with_inner_test_attribute_and_test_args_and_name(#[case] x: i8, #[case] y: i8) {
    assert_eq!(x, -2);
    assert_eq!(y, -4);
}

#[should_panic]
#[rstest_log::rstest]
#[case(-2, -4)]
fn with_inner_test_attribute_and_test_args_and_panic(#[case] x: i8, #[case] _y: i8) {
    assert_eq!(x, 0);
}

#[rstest_log::rstest(tokio::test)]
#[case(-2, 4)]
async fn async_case(#[case] x: i8, #[case] y: i8) {
    assert_eq!(x, -2);
    assert_eq!(y, 4);
}

#[instrument]
async fn instrumented(input: usize) -> usize {
    info!("input = {}", input);
    if input == 0 || input == 4 {
        error!("here we go");
    }
    let result = input + 1;
    info!("result = {}", result);
    result
}

/// To run the tracing tests and manually verify the output, run with
/// the `trace` feature:
/// ```sh
/// cargo test --features trace trace_with_custom_runtime -- --nocapture
/// ```
///
/// Log level can be configured via the `RUST_LOG` env variable and span
/// events for `#[instrumented]` can be configured via the
/// `RUST_LOG_SPAN_EVENTS` env variable:
/// ```sh
/// RUST_LOG=debug RUST_LOG_SPAN_EVENTS=full \
///   cargo test --features trace trace_with_custom_runtime -- --nocapture
/// ```
#[rstest_log::rstest]
fn trace_with_custom_runtime() {
    let rt = Builder::new_current_thread().build().unwrap();

    rt.block_on(async {
        instrumented(0).await;
        instrumented(1).await;
        debug!("done");
    })
}

#[rstest_log::rstest(tokio::test)]
async fn trace_with_tokio_attribute() {
    instrumented(6).await;
    instrumented(4).await;
    debug!("done");
}


#[rstest_log::rstest(tokio::test(flavor = "multi_thread", worker_threads = 1))]
async fn trace_with_tokio_attribute_with_arguments() {
    instrumented(6).await;
    instrumented(4).await;
    debug!("done");
}


/// A module used for testing the `test` attribute after importing it
/// via `use` instead of using fuller qualified syntax.
mod local {
    use rstest_log::rstest;

    use super::Error;

    #[rstest]
    fn without_return_type() {
        assert_eq!(2 + 2, 4);
    }

    #[rstest]
    fn with_return_type() -> Result<(), Error> {
        Ok(())
    }

    #[rstest]
    #[should_panic(expected = "success")]
    fn with_panic() {
        panic!("success")
    }

    #[rstest(tokio::test)]
    async fn with_inner_test_attribute_and_async() {
        assert_eq!(async { 42 }.await, 42)
    }

    #[rstest]
    #[case(-2, -4)]
    fn with_inner_test_attribute_and_test_args(#[case] x: i8, #[case] y: i8) {
        assert_eq!(x, -2);
        assert_eq!(y, -4);
    }

    #[should_panic]
    #[rstest]
    #[case(-2, -4)]
    fn with_inner_test_attribute_and_test_args_and_panic(#[case] x: i8, #[case] _y: i8) {
        assert_eq!(x, 0);
    }
}
