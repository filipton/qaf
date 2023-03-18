const PAGES_DIR: &'static str = "src/pages";
const MAIN_PATH: &'static str = "src/main.rs";
const MAIN_TEMPLATE_PATH: &'static str = "main.template.rs";

use core::fmt;
use std::path::PathBuf;

use syn::{Ident, ItemFn};

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

    let entries: PageEntry = PageEntry::generate(pages, 0);
    let lib_str = format!("{}", entries);
    let services_str = entries.generate_services(PathBuf::from("src"), "", true);

    let mut main_template_content = std::fs::read_to_string(main_template).unwrap();
    main_template_content = main_template_content.replace("//MOD_PAGES", &lib_str);
    main_template_content = main_template_content.replace("//SERVICES", &services_str);

    std::fs::write(main_file, main_template_content).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pages");
}

impl PageEntry {
    pub fn generate(dir: PathBuf, tab: usize) -> PageEntry {
        let mut children: Vec<PageEntry> = Vec::new();

        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    children.push(PageEntry::generate(entry.path(), tab + 1));
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

    pub fn generate_services(&self, path: PathBuf, use_path: &str, first: bool) -> String {
        let mut tmp = String::new();

        let use_path = format!("{}{}::", use_path, self.name);

        let mut path = path;
        path.push(&self.name);

        tmp += &format!(
            ".service(web::scope(\"{}\")\n",
            if first { "/" } else { &self.name }
        );

        for child in self.children.clone() {
            if child.children.len() > 0 {
                tmp += &child.generate_services(path.clone(), &use_path, false);
                continue;
            }

            let mut tmp_path = path.clone();
            tmp_path.push(format!("{}.rs", child.name));
            let tmp_use_path = format!("{}{}::", use_path, child.name);

            for endpoint in PageEntry::get_actix_endpoints(tmp_path).unwrap_or(vec![]) {
                tmp += &format!(".service({}{})\n", tmp_use_path, endpoint);
            }
        }

        tmp += ")";
        return tmp;
    }

    pub fn get_actix_endpoints(path: PathBuf) -> Result<Vec<String>, std::io::Error> {
        let file_content = std::fs::read_to_string(path)?;

        let syntax = syn::parse_file(&file_content).unwrap();
        let functions: Vec<String> = syntax
            .items
            .iter()
            .filter_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    if PageEntry::is_actix_attr(item_fn) {
                        return Some(item_fn.sig.ident.to_string());
                    }
                }

                None
            })
            .collect();

        return Ok(functions);
    }

    const ACTIX_MACROS: [&'static str; 7] =
        ["get", "post", "put", "delete", "head", "options", "patch"];

    fn is_actix_attr(item: &ItemFn) -> bool {
        for attr in item.attrs.clone() {
            for segment in attr.path.segments {
                let ident = segment.ident.to_string();
                if PageEntry::ACTIX_MACROS.contains(&ident.as_str()) {
                    return true;
                }
            }
        }

        return false;
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
