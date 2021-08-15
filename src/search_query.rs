use std::fs::File;
use std::path::Path;

use crate::StrSet;

pub struct SearchQuery<'a> {
    pub re: &'a str,
    pub contents_index_dir: &'a Path,
    pub branches: &'a StrSet<'a>,
    pub arches: &'a StrSet<'a>,
    pub out_file: &'a File,
}
