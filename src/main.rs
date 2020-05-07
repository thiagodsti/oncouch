pub mod filemanager;
#[macro_use] extern crate prettytable;

use std::io;
use std::path::{Path, PathBuf};
use std::fs::{self};
use prettytable::{Table};
use oncouch::{find_subtitles, find_movies, find_similarities};

fn main() {
    let path = Path::new(".");
    let subtitle_files = find_subtitles(path).unwrap();

    let movie_files = find_movies(path).unwrap();

    let _similars = find_similarities(movie_files, subtitle_files);


    let mut table = Table::new();
    table.add_row(row!["Movie", "New Subtitle", "Old Subtitle"]);
    for similar in _similars.iter() {
        let _movie = similar.movie.file_name().unwrap().to_str().unwrap();
        let _subtitle = similar.subtitle.file_name().unwrap().to_str().unwrap();
        let mut _new_subtitle = PathBuf::from(similar.subtitle.as_path());
        _new_subtitle.set_file_name(similar.movie.file_name().unwrap());
        _new_subtitle.set_extension(similar.subtitle.extension().unwrap());

        let new_subtitle_name = similar.movie.file_name().unwrap().to_str().unwrap().replace(similar.movie.extension().unwrap().to_str().unwrap(), similar.subtitle.extension().unwrap().to_str().unwrap());
        table.add_row(row![_movie, new_subtitle_name, _subtitle]);
    }
    table.printstd();

    let mut approve = String::new();
    println!("Are these files ok? (Y/n)");
    io::stdin().read_line(&mut approve).expect("Error to read the input");

    let yes: String = String::from("y");
    if approve.to_lowercase().trim() == yes || approve.to_lowercase().eq(&String::from("\n")) || approve.to_lowercase().eq(&String::from("\n\r")) {
        for similar in _similars.iter() {
            let mut new_subtitle = PathBuf::from(similar.subtitle.as_path());
            let extension = similar.subtitle.extension().unwrap();
            new_subtitle.set_file_name(similar.movie.file_name().unwrap());
            new_subtitle.set_extension(extension);
            println!("Renaming {} to {}", similar.subtitle.as_os_str().to_str().unwrap(), new_subtitle.to_str().unwrap());
            fs::rename(&similar.subtitle, new_subtitle).expect("Error renaming files");
        }

        println!("All subtitles was renamed");
    }


}