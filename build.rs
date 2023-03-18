const PAGES_DIR: &'static str = "src/pages";
const LIB_PATH: &'static str = "src/lib.rs";

use core::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct PageEntry {
    name: String,
    _tab_size: usize,
    children: Vec<PageEntry>,
}

fn main() {
    let pages = PathBuf::from(PAGES_DIR);
    let lib = PathBuf::from(LIB_PATH);

    let entries: PageEntry = PageEntry::from_walk_dir(pages, 0);
    let lib_str = format!("{}", entries);

    std::fs::write(lib, lib_str).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pages");
}

impl PageEntry {
    pub fn from_walk_dir(dir: PathBuf, tab: usize) -> PageEntry {
        let mut children: Vec<PageEntry> = Vec::new();

        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    children.push(PageEntry::from_walk_dir(entry.path(), tab + 1));
                } else if file_type.is_file() {
                    let file_name = entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .split('.')
                        .collect::<Vec<&str>>()[0]
                        .to_owned();

                    children.push(PageEntry {
                        name: file_name,
                        _tab_size: tab + 1,
                        children: vec![],
                    })
                }
            }
        }

        // WTF???
        let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();

        return PageEntry {
            name: dir_name,
            _tab_size: tab,
            children,
        };
    }
}

fn tab_size(size: usize) -> String {
    '\t'.to_string().repeat(size)
}

impl fmt::Display for PageEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}pub mod {}", tab_size(self._tab_size), self.name)?;

        if self.children.len() > 0 {
            write!(f, " {{ \n")?;
            for child in self.children.clone() {
                child.fmt(f)?;
            }
            write!(f, "{}}} \n", tab_size(self._tab_size))?;
        } else {
            write!(f, "; \n")?;
        }

        Ok(())
    }
}
