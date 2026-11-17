// Integration test for SigmaOS-screen-recording
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use screen_recording as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
