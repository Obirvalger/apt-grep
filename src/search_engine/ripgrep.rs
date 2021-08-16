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
    rg.arg(sq.re)
        .arg(format!("--max-count={}", sq.lines_in_file))
        .args(&contents_indexes)
        .current_dir(&sq.contents_index_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::from(errors));
    let mut rg_child = rg.spawn()?;
    if let Some(rg_output) = rg_child.stdout.take() {
        Command::new("head")
            .arg(format!("-n {}", sq.lines))
            .stdin(rg_output)
            .stdout(outputs)
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
    }

    Ok(())
}
