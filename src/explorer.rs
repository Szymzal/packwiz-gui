use std::{error::Error, fmt::Display, fs::read_dir, path::PathBuf, vec};

use egui::{pos2, vec2, Id, Sense, TextStyle, Widget, WidgetInfo, WidgetText, WidgetType};

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

    fn create_item_ui(file_path: PathBuf, ui: &mut egui::Ui, _selected_files: &Vec<PathBuf>) {
        let Ok(explorer_item) = ExplorerItem::new(file_path) else {
            return;
        };

        ui.add(explorer_item);

        // let Some(file_name) = file_path.file_name() else {
        //     return;
        // };
        // let file_name = file_name.to_string_lossy();
        // let selected = selected_files.contains(&file_path);
        //
        // let label_value = if file_path.is_dir() { "> " } else { "  " };
        // let label_value = format!("{}{}", label_value, file_name);
        //
        // if ui
        //     .selectable_label(if selected { true } else { false }, label_value)
        //     .clicked()
        // {
        //     let mut selected_files = selected_files.clone();
        //
        //     if selected {
        //         selected_files.retain(|x| x != &file_path);
        //     } else {
        //         selected_files.push(file_path.clone());
        //     }
        //
        //     ui.ctx().memory_mut(|memory| {
        //         memory
        //             .data
        //             .insert_temp(Id::new("selected_files"), selected_files)
        //     });
        // }
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
            let selected_files: Option<Vec<PathBuf>> = ui
                .ctx()
                .memory(|memory| memory.data.get_temp(Id::new("selected_files")));
            let selected_files = if selected_files.is_some() {
                selected_files.unwrap()
            } else {
                Vec::new() as Vec<PathBuf>
            };

            for file in files {
                ExplorerTab::create_item_ui(file, ui, &selected_files);
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
    pub fn new(path: Option<PathBuf>) -> Self {
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

#[derive(Debug)]
pub enum ExplorerItemError {
    WrongFileName,
}

impl Error for ExplorerItemError {}

impl Display for ExplorerItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ExplorerItemError::WrongFileName => "File name is incorrect!",
        };

        f.write_str(msg)
    }
}

enum InternalExplorerItemType {
    Directory,
    PackwizFile,
    UnknownFile,
}

pub struct ExplorerItem {
    item_name: Box<str>,
    item_path: PathBuf,
    item_type: InternalExplorerItemType,
}

impl ExplorerItem {
    pub fn new(item_path: PathBuf) -> Result<Self, ExplorerItemError> {
        let Some(item_name) = item_path.file_name() else {
            return Err(ExplorerItemError::WrongFileName);
        };

        let item_type;
        if item_path.is_dir() {
            item_type = InternalExplorerItemType::Directory;
            // TODO: More checks if it is a packwiz file
        } else if item_path.extension().is_some() && item_path.extension().unwrap() == "toml" {
            item_type = InternalExplorerItemType::PackwizFile;
        } else {
            item_type = InternalExplorerItemType::UnknownFile;
        }

        Ok(Self {
            item_name: Box::from(item_name.to_string_lossy()),
            item_path,
            item_type,
        })
    }
}

// > [] directory_name
//   [] file_name.txt
//   |
// image
impl Widget for ExplorerItem {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let text: WidgetText = self.item_name.into_string().into();
        let text = text.into_galley(ui, Some(false), ui.available_width(), TextStyle::Button);
        let desired_size = vec2(ui.available_width(), text.size().y);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let cursor_x = rect.min.x + 2.0;
            let text_pos = pos2(cursor_x, rect.center().y - 0.5 * text.size().y);
            text.paint_with_visuals(ui.painter(), text_pos, visuals);
        }

        response
    }
}
