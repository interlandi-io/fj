use anyhow::{anyhow, Error};
use frizbee::{Config, Scoring, match_list_parallel};
use std::env;
use std::fs;
use std::path::MAIN_SEPARATOR_STR;
use std::path::{Path, PathBuf};

const N_THREADS: usize = 4;
const SKIP_DIRS: &[&str] = &["node_modules", ".git", "target", "dist", ".next", "build"];
const PUSHD_PREFIX: &'static str = "__pushd__";

fn main() -> Result<(), Error> {
    let argv: Vec<String> = env::args().collect();
    let needle = argv.get(1).unwrap();

    let mut haystacks = Vec::new();
    ls_dirs_recurse(&PathBuf::from("."), &mut haystacks)?;

    let out = match best_match(needle, &haystacks) {
        Some(dir) => [PUSHD_PREFIX, &dir].join(""),
        None => {
            return Err(anyhow!(format!("No match found for {needle}")))
        },
    };

    println!("{out}");

    Ok(())
}

fn best_match<'a>(needle: &str, haystacks: &'a [PathBuf]) -> Option<String> {
    let haystacks: Vec<String> = haystacks
        .iter()
        .map(|p| p.display().to_string()) // Note: panics if path is not UTF-8
        .collect();

    let config = Config {
        scoring: Scoring {
            exact_match_bonus: 999,
            gap_open_penalty: 6,
            gap_extend_penalty: 3,
            ..Scoring::default()
        },
        ..Config::default()
    };
    let mut results = match_list_parallel(needle, &haystacks, &config, N_THREADS);
    for result in &mut results {
        let string = haystacks.get(result.index as usize).unwrap();
        let depth_penalty = string.matches(MAIN_SEPARATOR_STR).count() as u16 * 3;
        result.score = result.score.saturating_sub(depth_penalty);
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));

    results.get(0).map(|m| haystacks.get(m.index as usize).unwrap().clone())
}

fn ls_dirs_recurse(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if SKIP_DIRS.contains(&&*name) || name.starts_with('.') {
            continue;
        }
        ls_dirs_recurse(&path, out)?;
        out.push(path);
    }
    Ok(())
}
