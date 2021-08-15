use std::fs::{File, OpenOptions};
use std::io::Result;
use std::process::{Command, Stdio};

use crate::SearchQuery;

pub fn search(sq: &SearchQuery) -> Result<()> {
    File::create(sq.out_file)?;
    for branch in sq.branches {
        for arch in sq.arches {
            let outputs = OpenOptions::new().append(true).open(sq.out_file)?;
            let errors = outputs.try_clone()?;
            let contents_index =
                sq.contents_index_dir.join(branch).join(arch).join("contents_index");
            Command::new("rg")
                .arg(sq.re)
                .arg(contents_index)
                .stdout(Stdio::from(outputs))
                .stderr(Stdio::from(errors))
                .spawn()?
                .wait()?
                .success();
        }
    }

    Ok(())
}
