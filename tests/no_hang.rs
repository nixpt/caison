//! Regression tests for CAISON-5: the parser must always advance or error.
//!
//! Each input is parsed on its own thread with a hard timeout, so a
//! no-progress loop fails the test instead of stalling the suite.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use caison::CaisonParser;

/// Parse `input` on a worker thread; panic if it does not return in time.
fn parse_bounded(input: &str) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let owned = input.to_string();
    thread::spawn(move || {
        let r = CaisonParser::new(&owned).parse().map(|_| ());
        let _ = tx.send(r);
    });
    rx.recv_timeout(Duration::from_secs(3))
        .unwrap_or_else(|_| panic!("parser hung (no result in 3s) on {input:?}"))
}

fn assert_errors(input: &str) {
    let err = parse_bounded(input).expect_err(&format!("{input:?} must be a parse error"));
    assert!(
        err.contains("line") || err.contains("col"),
        "error for {input:?} should carry a position, got {err:?}"
    );
}

#[test]
fn object_key_position_open_bracket() {
    assert_errors("d: {[\n");
    assert_errors("d: { a: 1\n[x]\n");
    assert_errors("[e0001]\nd: { a: 1,\n[e0002]\nk: 1\n");
}

#[test]
fn object_key_position_close_bracket() {
    assert_errors("d: { a: 1 ]\n");
}

#[test]
fn object_key_position_at_sign() {
    assert_errors("d: { @x\n a: 1 }\n");
}

#[test]
fn array_element_position_stray_close_brace() {
    assert_errors("xs: [1, }\n");
}

#[test]
fn controls_still_error() {
    assert_errors("d: { a: 1\nb: 2\n");
    assert_errors("xs: [1, 2\n[e]\n");
}
