const PAGES_DIR: &'static str = "src/pages";
const MAIN_PATH: &'static str = "src/main.rs";
const MAIN_TEMPLATE_PATH: &'static str = "main.template.rs";

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
    let main_file = PathBuf::from(MAIN_PATH);
    let main_template = PathBuf::from(MAIN_TEMPLATE_PATH);

    let entries: PageEntry = PageEntry::from_walk_dir(pages, 0);
    let lib_str = format!("{}", entries);

    //panic!("{:#?}", entries);

    let test = PathBuf::from("src/pages/test.rs");
    let dsa = PageEntry::get_actix_endpoints(test).unwrap();
    panic!("{:?}", dsa);

    let mut main_template_content = std::fs::read_to_string(main_template).unwrap();
    main_template_content = main_template_content.replace("//MOD_PAGES", &lib_str);

    std::fs::write(main_file, main_template_content).unwrap();

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

    //pub fn generate_services(&self, tab: usize) -> String {}

    pub fn get_actix_endpoints(path: PathBuf) -> Result<Vec<String>, std::io::Error> {
        let mut output: Vec<String> = Vec::new();
        let file_content = std::fs::read_to_string(path)?;
        let file_lines: Vec<&str> = file_content.lines().map(|x| x).collect();

        let mut actix_macro = false;
        for i in 0..file_lines.len() {
            let line = file_lines[i];

            if line.starts_with("#[post(")
                || line.starts_with("#[get(")
                || line.starts_with("#[put(")
                || line.starts_with("#[delete(")
            {
                actix_macro = true;
                continue;
            }

            if actix_macro && line.contains("fn ") {
                let fn_name = line.split("fn ").nth(1).unwrap().split("(").nth(0).unwrap();
                output.push(String::from(fn_name));
            }
            actix_macro = false;
        }

        return Ok(output);
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
