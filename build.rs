const PAGES_DIR: &'static str = "src/pages";
const LIB_PATH: &'static str = "src/lib.rs";

use core::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct PageEntry {
    name: String,
    children: Vec<PageEntry>,
}

fn main() {
    let pages = PathBuf::from(PAGES_DIR);
    let lib = PathBuf::from(LIB_PATH);

    let entries: PageEntry = PageEntry::from_walk_dir(pages);

    panic!("{}", entries);

    println!("cargo:rerun-if-changed=build.rs");
}

impl PageEntry {
    pub fn from_walk_dir(dir: PathBuf) -> PageEntry {
        let mut children: Vec<PageEntry> = Vec::new();

        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    children.push(PageEntry::from_walk_dir(entry.path()));
                } else if file_type.is_file() {
                    let file_name = entry.file_name().to_str().unwrap().to_owned();

                    children.push(PageEntry {
                        name: file_name,
                        children: vec![],
                    })
                }
            }
        }

        // WTF???
        let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();

        return PageEntry {
            name: dir_name,
            children,
        };
    }
}

impl fmt::Display for PageEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pub mod {}", self.name)?;

        if self.children.len() > 0 {
            write!(f, " {{ \n")?;
            for child in self.children.clone() {
                child.fmt(f)?;
            }
            write!(f, "}} \n")?;
        } else {
            write!(f, "; \n")?;
        }

        Ok(())
    }
}
