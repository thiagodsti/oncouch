use std::fs::{DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use regex::Regex;

mod filemanager;

pub struct SimilarFile {
    pub movie: PathBuf,
    pub subtitle: PathBuf
}

impl SimilarFile {
    pub fn new(movie: PathBuf, subtitle: PathBuf) -> Self {
        SimilarFile {
            movie,
            subtitle,
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

    for movie in movies.iter() {
        let movie_name = String::from(movie.file_name().to_str().unwrap());
        let movie_chapter = find_chapter(movie_name.as_str());
        for (i_subtitle, subtitle) in subtitles.iter().enumerate() {
            let subtitle_name = subtitle.file_name();
            let subtitle_name = subtitle_name.to_str().unwrap();
            let subtitle_chapter = find_chapter(subtitle_name);
            if subtitle_chapter.is_some() && movie_chapter.is_some() && subtitle_chapter == movie_chapter {
                most_similar = 1.0;
                index_similar = i_subtitle;
                continue;
            }
            let similar = strsim::jaro(movie.file_name().to_str().unwrap(), subtitle_name);
            if similar > most_similar {
                most_similar = similar;
                index_similar = i_subtitle;
            }
        }
        similarities.push(SimilarFile::new(movie.path(), subtitles[index_similar].path()));
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
