use super::*;
use crate as mulver;

fn bad_result() -> mulver::Result<i32> {
    Err(MulverError::Unspecified)
}

#[test]
fn test_mulver_result() {
    assert!(match bad_result().unwrap_err() {
        MulverError::Unspecified => true,
    });
}
