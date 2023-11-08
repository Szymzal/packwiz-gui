pub trait PackwizTab {
    fn title(&self) -> &'static str;
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct TabViewer {}

pub struct Tab {
    inner_tab: Box<dyn PackwizTab>,
}

impl Tab {
    pub fn new(tab: impl PackwizTab + 'static) -> Self {
        Tab {
            inner_tab: Box::from(tab),
        }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.inner_tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.inner_tab.ui(ui);
    }
}
