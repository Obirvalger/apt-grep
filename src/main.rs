use std::cmp::{max, min};
use std::fs::File;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;
use tempfile::tempfile;

use apt_grep::search_engine::ripgrep;
use apt_grep::SearchQuery;
use apt_grep::StrSet;
use apt_grep::{getenv, getenv_i64};

#[derive(Deserialize, Debug)]
struct Info {
    arches: String,
    branches: String,
    re: String,
    #[serde(default)]
    add_noarch: bool,
    #[serde(default = "default_lines")]
    lines: i64,
    #[serde(default)]
    filename: bool,
}

fn default_lines() -> i64 {
    getenv_i64("APT_GREP_DEFAULT_LINES", 20)
}

fn generate(info: &Info, out_file: &File) -> std::io::Result<()> {
    let contents_index_dir = getenv("APT_GREP_CONTENTS_INDEX_DIR", "contents_index_dir");
    let contents_index_dir = Path::new(&contents_index_dir);
    let branches = info.branches.split(',').collect::<StrSet>();
    let mut arches = info.arches.split(',').collect::<StrSet>();
    if info.add_noarch {
        arches.insert("noarch");
    }
    let max_lines = getenv_i64("APT_GREP_MAX_LINES", 100);
    let lines = max(min(info.lines, max_lines), 1);

    let sq = SearchQuery {
        re: &info.re,
        contents_index_dir,
        branches: &branches,
        arches: &arches,
        out_file,
        lines,
        filename: info.filename,
    };

    ripgrep::search(&sq)?;

    Ok(())
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> Result<NamedFile> {
    let out_file = tempfile()?;
    format!("Welcome {:?}!", info);
    generate(&info, &out_file)?;
    Ok(NamedFile::from_file(out_file, "result.txt")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = getenv("APT_GREP_HOST", "127.0.0.1");
    let port = getenv_i64("APT_GREP_PORT", 8080);
    let address = format!("{}:{}", host, port);
    HttpServer::new(|| App::new().service(index)).bind(address)?.run().await
}
