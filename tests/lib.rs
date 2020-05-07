use oncouch;
use std::path::Path;

#[test]
fn find_subtitles() {
    let subtitles = oncouch::find_subtitles(Path::new("./tests/")).expect("Find subtitles");
    let total = subtitles.len();
    assert_eq!(5, total);
}