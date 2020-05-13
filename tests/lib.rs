use oncouch;
use std::path::Path;

#[test]
fn find_subtitles() {
    let subtitles = oncouch::find_subtitles(Path::new("./tests/")).expect("Find subtitles");
    let total = subtitles.len();
    assert_eq!(5, total);
}

#[test]
fn find_similarities() {
    let movies = oncouch::find_movies(Path::new("./tests/")).expect("Find movies");
    let subtitles = oncouch::find_subtitles(Path::new("./tests/")).expect("Find subtitles");

    let similars = oncouch::find_similarities(movies, subtitles);

    for similar in similars.iter() {
        println!("{:?} - {:?}", similar.subtitle, similar.movie);
    }

    assert_eq!(5, similars.len());
}

#[test]
fn find_similarities_without_movies() {
    let movies = oncouch::find_movies(Path::new("./tests/movies-wrong-extension/")).expect("Find movies");
    let subtitles = oncouch::find_subtitles(Path::new("./tests/movie-wrong-extension/")).expect("Find subtitles");

    let similars = oncouch::find_similarities(movies, subtitles);

    for similar in similars.iter() {
        println!("{:?} - {:?}", similar.subtitle, similar.movie);
    }

    assert_eq!(0, similars.len());
}