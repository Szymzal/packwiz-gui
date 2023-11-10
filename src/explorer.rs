use std::{
    error::Error,
    fmt::Display,
    fs::read_dir,
    path::PathBuf,
    sync::{Arc, Mutex},
    vec,
};

use egui::{
    pos2, vec2, Id, Rounding, Sense, TextStyle, Widget, WidgetInfo, WidgetText, WidgetType,
};
use log::{error, info};

use crate::tab::PackwizTab;

pub struct ExplorerTab {
    explorer: Explorer,
}

impl ExplorerTab {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            explorer: Explorer::new(path),
        }
    }

    fn create_item_ui(
        file_path: PathBuf,
        ui: &mut egui::Ui,
        selected_files: Arc<Mutex<Vec<PathBuf>>>,
    ) {
        let mut selected_files = match selected_files.lock() {
            Ok(selected_files) => selected_files,
            Err(err) => {
                error!("Failed to get selected files: {err}");
                return;
            }
        };

        let selected = selected_files.contains(&file_path);

        let Ok(explorer_item) = ExplorerItem::new(file_path.clone(), selected) else {
            return;
        };

        if ui.add(explorer_item).clicked() && !selected {
            selected_files.push(file_path);
        }

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

        if !files.is_empty() {
            let selected_files_id = Id::new("selected_files");
            let selected_files: Option<Arc<Mutex<Vec<PathBuf>>>> = ui
                .ctx()
                .memory(|memory| memory.data.get_temp(selected_files_id));
            let selected_files = if let Some(selected_files) = selected_files {
                selected_files
            } else {
                Arc::new(Mutex::new(Vec::new()))
            };

            for file in files {
                ExplorerTab::create_item_ui(file, ui, selected_files.clone());
            }

            ui.ctx()
                .memory_mut(|memory| memory.data.insert_temp(selected_files_id, selected_files));

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
                paths.for_each(|path| {
                    if let Ok(entry) = path {
                        files.push(entry.path());
                    }
                });

                return files;
            }
        }

        vec![]
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
    selected: bool,
    item_name: Box<str>,
    item_path: PathBuf,
    item_type: InternalExplorerItemType,
}

impl ExplorerItem {
    pub fn new(item_path: PathBuf, selected: bool) -> Result<Self, ExplorerItemError> {
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
            selected,
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
        // TODO: customizable?
        let padding = 2.0;
        // TODO: customizable?
        let mut margin = 2.0;
        let window_stroke_width = 2.0 * ui.visuals().window_stroke().width;

        let text: WidgetText = self.item_name.into_string().into();
        let text = text.into_galley(ui, Some(false), ui.available_width(), TextStyle::Button);
        let mut desired_size = vec2(ui.available_width(), text.size().y);
        margin += window_stroke_width;
        margin += padding;
        desired_size += vec2(0.0, margin);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::SelectableLabel, self.selected, text.text())
        });

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact_selectable(&response, self.selected);

            if response.hovered() {
                ui.painter().rect(
                    rect,
                    Rounding::same(2.0),
                    visuals.bg_fill,
                    visuals.bg_stroke,
                );
            }

            if self.selected {
                ui.painter().rect(
                    rect,
                    Rounding::same(2.0),
                    visuals.bg_fill,
                    visuals.bg_stroke,
                );
            }

            let cursor_x = rect.min.x + padding;
            let text_pos = pos2(cursor_x, rect.center().y - 0.5 * text.size().y);
            text.paint_with_visuals(ui.painter(), text_pos, &visuals);
        }

        response
    }
}
