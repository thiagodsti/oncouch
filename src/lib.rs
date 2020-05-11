use std::fs::{DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use regex::Regex;

mod filemanager;

pub struct SimilarFile {
    pub movie: PathBuf,
    pub subtitle: PathBuf,
    pub chapter: Option<String>
}

impl SimilarFile {
    pub fn new(movie: PathBuf, subtitle: PathBuf, chapter: Option<String>) -> Self {
        SimilarFile {
            movie,
            subtitle,
            chapter
        }
    }
}

impl PartialEq for SimilarFile {
    fn eq(&self, other: &Self) -> bool {
        self.subtitle == other.subtitle && self.movie == other.movie
    }
}


pub fn find_similarities<'a>(movies: Vec<DirEntry>, subtitles: Vec<DirEntry>) -> Vec<SimilarFile> {
    let mut similarities = Vec::new();
    let mut most_similar = 0.0;
    let mut index_similar = 0;
    let mut chapter = Option::None;

    for subtitle in subtitles.iter() {
        let subtitle_name = String::from(subtitle.file_name().to_str().unwrap());
        let subtitle_chapter = find_chapter(subtitle_name.as_str());
        for (i_movie, movie) in movies.iter().enumerate() {
            let movie_name = movie.file_name();
            let movie_name = movie_name.to_str().unwrap();
            let movie_chapter = find_chapter(movie_name);
            if subtitle_chapter.is_some() && movie_chapter.is_some() && subtitle_chapter == movie_chapter {
                most_similar = 1.0;
                index_similar = i_movie;
                chapter = subtitle_chapter.clone();
                continue;
            }
            let similar = strsim::jaro(subtitle.file_name().to_str().unwrap(), movie_name);
            if similar > most_similar {
                most_similar = similar;
                index_similar = i_movie;
            }
        }
        similarities.push(SimilarFile::new(movies[index_similar].path(), subtitle.path(), chapter.clone()));
        most_similar = 0.0;
    }
    return similarities;
}

pub fn find_chapter(input: &str) -> Option<String> {
    let re : Regex = Regex::new(r"(?P<chapter>[s|S]\d+[e|E]\d+)").unwrap();
    re.captures(input).and_then(|cap| {
        cap.name("chapter").map(|chapter| chapter.as_str().to_lowercase())
    })
}

pub fn find_subtitles(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut extensions = Vec::new();
    extensions.push("srt");
    return filemanager::find_files(path, extensions);
}

pub fn find_movies(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut extensions = Vec::new();
    extensions.push("mkv");
    extensions.push("avi");
    return filemanager::find_files(path, extensions);
}
