use frizbee::{Config, Match, match_list, match_list_parallel};
use anyhow::anyhow;
use std::error::Error;
use std::{fs, fs::{DirEntry}};
use std::env;
use std::path::{Path, PathBuf};

const N_THREADS: usize = 4;

pub fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    let needle = argv.get(1).unwrap();

    let mut haystacks = Vec::new();
    ls_dirs_recurse(&PathBuf::from("."), &mut haystacks)?;
    let haystacks: Vec<String> = haystacks
        .iter()
        .map(|h| h.display().to_string()) // Don't ask
        .collect();

    let r#match = best_match(needle, &haystacks).unwrap();

    println!("{match}");

    Ok(())
}

pub fn best_match<'a>(needle: &str, haystacks: &'a [String]) -> Option<&'a str> {
    let config = Config {
        sort: true,
        ..Config::default()
    };
    let results = match_list_parallel(needle, &haystacks, &config, N_THREADS);
    haystacks.get(results[0].index as usize).map(|m| m.as_str())
}

pub fn ls_dirs_recurse(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
    let mut dirs: Vec<PathBuf> = entries
        .iter()
        .filter_map(|e| if e.file_type().ok()?.is_dir() {
            Some(e.path().clone()) 
        } else {
            None 
        })
        .collect();

    for dir in &dirs {
        ls_dirs_recurse(dir, out)?;
    }

    out.append(&mut dirs);

    Ok(())
}
