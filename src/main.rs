pub mod filemanager;

use oncouch::{find_movies, find_similarities, find_subtitles};
use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct RenameSubtitle {
    #[tabled(rename = "Old Subtitle")]
    subtitle: String,
    #[tabled(rename = "New Subtitle")]
    new_subtitle: String
}

fn main() {
    let path = Path::new(".");
    let subtitle_files = find_subtitles(path).expect("find subtitles");

    let movie_files = find_movies(path).expect("find movies");

    let mut _similars = find_similarities(movie_files, subtitle_files);

    _similars.sort_by(|a, b| a.chapter.cmp(&b.chapter));

    let mut table_data = vec![];
    for similar in _similars.iter() {
        let _subtitle = similar
            .subtitle
            .file_name()
            .expect("get file name from subtitle")
            .to_str()
            .expect("Error");
        let mut _new_subtitle = PathBuf::from(similar.subtitle.as_path());
        _new_subtitle.set_file_name(similar.movie.file_name().expect("get file name from movie"));
        _new_subtitle.set_extension(
            similar
                .subtitle
                .extension()
                .expect("get extension from subtitle"),
        );

        let new_subtitle_name = similar
            .movie
            .file_name()
            .expect("get file name from movie")
            .to_str()
            .expect("not have error")
            .replace(
                similar
                    .movie
                    .extension()
                    .expect("get extension from movie")
                    .to_str()
                    .expect("not have error"),
                similar
                    .subtitle
                    .extension()
                    .expect("get extension from subtitle")
                    .to_str()
                    .expect("not have error"),
            );
        table_data.push(RenameSubtitle{
            subtitle: String::from(_subtitle),
            new_subtitle: String::from(new_subtitle_name)
        });
    }
    let table = Table::new(table_data).to_string();
    println!("{}", table);

    let mut approve = String::new();
    println!("Are these files ok? (y/N)");
    io::stdin()
        .read_line(&mut approve)
        .expect("to have a valid input");

    if approve.to_lowercase().trim() == "y" {
        for similar in _similars.iter() {
            let mut new_subtitle = PathBuf::from(similar.subtitle.as_path());
            let extension = similar.subtitle.extension().expect("get extension from subtitle");
            new_subtitle.set_file_name(similar.movie.file_name().expect("get filename from movie"));
            new_subtitle.set_extension(extension);
            println!(
                "Renaming {} to {}",
                similar.subtitle.as_os_str().to_str().unwrap(),
                new_subtitle.to_str().unwrap()
            );
            fs::rename(&similar.subtitle, new_subtitle).expect("rename files");
        }

        println!("All subtitles were renamed");
    }
}
