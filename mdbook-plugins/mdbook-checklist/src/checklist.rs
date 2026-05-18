use mdbook::book::Chapter;
use std::path::PathBuf;
use toml::{value::Table, Value};

pub struct Checklist {
    title: String,
    data: Vec<(String, PathBuf, Vec<CheckEntry>)>,
}

impl Checklist {
    pub fn new() -> Self {
        Checklist {
            title: "Checklist".to_string(),
            data: Vec::new(),
        }
    }

    pub fn update_config(&mut self, config: &Table) {
        if let Some(Value::String(title)) = config.get("title") {
            self.title = title.clone();
        }
    }

    pub fn insert(&mut self, chap_name: &str, chap_path: &PathBuf, name: String, desc: String) {
        match self.data.iter_mut().find(|(_, c, _)| c == chap_path) {
            None => {
                self.data.push((
                    chap_name.to_string(),
                    chap_path.clone(),
                    vec![CheckEntry { name, desc }],
                ));
            }
            Some((_, _, ref mut v)) => v.push(CheckEntry { name, desc }),
        }
    }

    pub fn generate_chapter(self) -> Chapter {
        let mut content = String::new();

        content.push_str(&format!("# {}\n\n", self.title));

        for (chap_name, _, entries) in &self.data {
            content.push_str(&format!("\n - {}:\n", chap_name,));
            for entry in entries {
                content.push_str(&format!("   - [ ] {} ([{}])\n", entry.desc, entry.name,));
            }
        }

        content.push_str("\n\n");
        for (_, chap_path, entries) in &self.data {
            for entry in entries {
                content.push_str(&format!(
                    "[{}]: {}#{}\n",
                    entry.name,
                    chap_path.to_str().unwrap(),
                    entry.name,
                ));
            }
        }

        Chapter::new(&self.title, content, "checklist.md", vec![])
    }
}

struct CheckEntry {
    name: String,
    desc: String,
}
