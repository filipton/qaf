use anyhow::{anyhow, Result};
use std::path::PathBuf;
use syn::{ItemFn, LitStr};

#[derive(Debug, Clone)]
pub struct PageEntry {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<PageEntry>,
}

impl PageEntry {
    pub fn generate(dir: &PathBuf) -> Result<PageEntry> {
        let mut children: Vec<PageEntry> = Vec::new();

        // TODO: Refactor this
        for entry in dir.read_dir()? {
            if let Ok(entry) = entry {
                let file_type = entry.file_type()?;

                if file_type.is_dir() {
                    children.push(PageEntry::generate(&entry.path())?);
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
                        is_dir: entry.file_type()?.is_dir(),
                        children: vec![],
                    })
                }
            }
        }

        // WTF???
        let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
        children.sort_by_key(|k| k.is_dir);

        return Ok(PageEntry {
            name: dir_name,
            is_dir: true,
            children,
        });
    }

    pub fn get_mods_string(&self) -> Result<String> {
        let mut out = String::new();
        if self.is_dir && self.children.len() == 0 {
            return Ok(out);
        }

        if self.name.contains(":") || self.name.contains("{") || self.name.contains("}") {
            out += &format!("#[path = \"{}\"]\n", self.name);
        }
        out += &format!(
            "pub mod {}",
            self.name
                .replace(":", "_")
                .replace("{", "_")
                .replace("}", "_")
        );

        if self.children.len() > 0 {
            out += "{ \n";

            for child in self.children.clone() {
                out += &child.get_mods_string()?;
            }

            out += "} \n";
        } else {
            out += "; \n";
        }

        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionRoute {
    pub function: String,
    pub route_type: RouteType,
    pub route: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RouteType {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Any,
    On,
}

impl RouteType {
    pub fn from_str(s: &str) -> Result<RouteType> {
        match s {
            "get" => Ok(RouteType::Get),
            "post" => Ok(RouteType::Post),
            "put" => Ok(RouteType::Put),
            "delete" => Ok(RouteType::Delete),
            "patch" => Ok(RouteType::Patch),
            "head" => Ok(RouteType::Head),
            "options" => Ok(RouteType::Options),
            "any" => Ok(RouteType::Any),
            "on" => Ok(RouteType::On),
            _ => Err(anyhow!("Invalid route type")),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            RouteType::Get => "get",
            RouteType::Post => "post",
            RouteType::Put => "put",
            RouteType::Delete => "delete",
            RouteType::Patch => "patch",
            RouteType::Head => "head",
            RouteType::Options => "options",
            RouteType::Any => "any",
            RouteType::On => "on",
        }
    }
}

pub fn get_file_routes(path: PathBuf) -> Result<Vec<FunctionRoute>> {
    let file_content = std::fs::read_to_string(path)?;

    let syntax = syn::parse_file(&file_content).unwrap();
    let functions: Vec<FunctionRoute> = syntax
        .items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Fn(item_fn) = item {
                return get_macro_attr(item_fn);
            }

            None
        })
        .collect();

    return Ok(functions);
}

const MACROS: [&'static str; 10] = [
    "get", "post", "put", "delete", "patch", "head", "options", "trace", "any", "on",
];
fn get_macro_attr(item: &ItemFn) -> Option<FunctionRoute> {
    for attr in item.attrs.clone() {
        for segment in attr.path().segments.clone() {
            let ident = segment.ident.to_string();
            if MACROS.contains(&ident.as_str()) {
                let route: Option<String> = attr.parse_args::<LitStr>().ok().map(|x| x.value());

                return Some(FunctionRoute {
                    function: item.sig.ident.to_string(),
                    route_type: RouteType::from_str(&ident).unwrap(),
                    route,
                });
            }
        }
    }

    return None;
}
