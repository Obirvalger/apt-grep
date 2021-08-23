use std::io::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::SearchQuery;

pub fn search(sq: &SearchQuery) -> Result<()> {
    let mut contents_indexes = Vec::with_capacity(sq.branches.len() * sq.arches.len());
    for branch in sq.branches {
        for arch in sq.arches {
            let contents_index = PathBuf::from(branch).join(arch);
            contents_indexes.push(contents_index);
        }
    }
    let outputs = sq.out_file.try_clone()?;
    let errors = outputs.try_clone()?;
    let mut rg = Command::new("rg");
    if sq.filename {
        rg.arg("--with-filename");
    } else {
        rg.arg("--no-filename");
    }
    if sq.max_count > 0 {
        rg.arg(format!("--max-count={}", sq.max_count));
    }
    rg.arg(sq.re)
        .args(&contents_indexes)
        .current_dir(&sq.contents_index_dir)
        .stdout(outputs)
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait()?;

    Ok(())
}
