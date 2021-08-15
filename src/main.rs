use std::fs::File;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;
use tempfile::tempfile;

use apt_grep::search_engine::ripgrep;
use apt_grep::SearchQuery;
use apt_grep::StrSet;

#[derive(Deserialize, Debug)]
struct Info {
    arches: String,
    branches: String,
    re: String,
    #[serde(default)]
    add_noarch: bool,
}

fn generate(info: &Info, out_file: &File) -> std::io::Result<()> {
    let contents_index_dir = Path::new("contents_index_dir");
    let branches = info.branches.split(',').collect::<StrSet>();
    let mut arches = info.arches.split(',').collect::<StrSet>();
    if info.add_noarch {
        arches.insert("noarch");
    }

    let sq = SearchQuery {
        re: &info.re,
        contents_index_dir: &contents_index_dir,
        branches: &branches,
        arches: &arches,
        out_file: &out_file,
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
    HttpServer::new(|| App::new().service(index)).bind("127.0.0.1:8080")?.run().await
}
