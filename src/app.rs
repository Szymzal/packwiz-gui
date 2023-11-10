use egui::{menu, Button, Id, TopBottomPanel};
use egui_dock::{DockArea, DockState, Style};
use log::error;
use native_dialog::FileDialog;

use crate::{
    explorer::ExplorerTab,
    tab::{Tab, TabViewer},
};

pub struct App {
    tree: DockState<Tab>,
}

impl Default for App {
    fn default() -> Self {
        let tab = Tab::new(ExplorerTab::new(Some(
            "D:\\Games\\Minecraft\\ModPacks\\HMMinecraft\\S1".into(),
        )));
        let tree = DockState::new(vec![tab]);

        // let [a, b] = tree.main_surface_mut().split_left(
        //     NodeIndex::root(),
        //     0.3,
        //     vec![Tab {
        //         value: "tab3".to_string(),
        //     }],
        // );
        //
        // let [_, _] = tree.main_surface_mut().split_below(
        //     a,
        //     0.7,
        //     vec![Tab {
        //         value: "tab4".to_string(),
        //     }],
        // );
        // let [_, _] = tree.main_surface_mut().split_below(
        //     b,
        //     0.5,
        //     vec![Tab {
        //         value: "tab5".to_string(),
        //     }],
        // );

        Self { tree }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.add(Button::new("Open")).clicked() {
                        let path = FileDialog::new()
                            .set_location("D:\\Games\\Minecraft\\ModPacks\\HMMinecraft\\S1")
                            .show_open_single_dir();

                        match path {
                            Ok(path) => {
                                if let Some(path) = path {
                                    ui.ctx().memory_mut(|memory| {
                                        memory.data.insert_temp(Id::new("root_dir"), path)
                                    });
                                }
                            }
                            Err(err) => error!("Error: {}", err),
                        }

                        ui.close_menu();
                    }
                });
            });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }
}
