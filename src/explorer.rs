use std::{fs::read_dir, path::PathBuf, vec};

use egui::Id;

use crate::tab::PackwizTab;

pub struct ExplorerTab {
    explorer: Explorer,
}

impl ExplorerTab {
    pub fn new(path: Option<PathBuf>) -> Self {
        let path = match path {
            Some(some_path) => Some(some_path.into()),
            None => None,
        };
        Self {
            explorer: Explorer::new(path),
        }
    }
}

impl PackwizTab for ExplorerTab {
    fn title(&self) -> &'static str {
        "Explorer"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let path = ui
            .ctx()
            .memory(|memory| memory.data.get_temp::<PathBuf>(Id::new("root_dir")));

        if self.explorer.path != path {
            self.explorer.update(path);
        }

        if self.explorer.path.is_none() {
            ui.label("Open project!");
            return;
        }

        let files = self.explorer.files();

        if files.len() > 0 {
            for file in files {
                if let Some(file_name) = file.file_name() {
                    ui.label(format!("File: {:?}", file_name));
                }
            }

            return;
        }

        ui.label("Empty");
    }
}

pub struct Explorer {
    path: Option<PathBuf>,
}

impl Explorer {
    pub fn new(path: Option<PathBuf>) -> Explorer {
        Self { path }
    }

    pub fn update(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    pub fn files(&self) -> Vec<PathBuf> {
        if let Some(path) = &self.path {
            let paths = read_dir(path);
            if let Ok(paths) = paths {
                let mut files: Vec<PathBuf> = vec![];
                for path in paths {
                    if let Ok(entry) = path {
                        files.push(entry.path());
                    }
                }

                return files;
            }
        }

        return vec![];
    }
}
