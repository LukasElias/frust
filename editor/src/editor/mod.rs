use eframe::egui;
use frust_core::Scene;

mod scene_hierarchy;
mod project_files;
mod properties;
mod game_preview;

#[derive(Clone, Copy)]
pub enum View {
    SceneHierarchy,
    ProjectFiles,
    Properties,
}

enum Panel {
    Left,
    Bottom,
    Right,
}

pub struct App {
    pub left_panel: Vec<View>,
    pub right_panel: Vec<View>,
    pub bottom_panel: Vec<View>,
    pub is_playing: bool,
    pub scene: frust_core::Scene,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            left_panel: vec![View::SceneHierarchy],
            right_panel: vec![View::Properties],
            bottom_panel: vec![View::ProjectFiles],
            is_playing: true,
            scene: Scene::new(),
        }
    }

    fn add_contents(self: &mut Self, ui: &mut egui::Ui, panel: Panel) {
        let panel = match panel {
            Panel::Left => &mut self.left_panel,
            Panel::Right => &mut self.right_panel,
            Panel::Bottom => &mut self.bottom_panel,
        };

        ui.horizontal(|ui| {
            for (index, view) in panel.to_owned().iter_mut().enumerate() {
                ui.add_space(5.0);

                let title: &str = match view {
                    View::SceneHierarchy => "Scene Hierarchy",
                    View::ProjectFiles => "Project Files",
                    View::Properties => "Properties",
                };

                let mut rich_text = egui::RichText::new(title);

                if index == 0 {
                    rich_text = rich_text.color(egui::Color32::from_rgb(255, 255, 255));
                }

                let label = ui.add(egui::Label::sense(egui::Label::new(rich_text), egui::Sense::click())).with_new_rect(egui::Rect {
                    min: egui::pos2(10.0, 20.0),
                    max: egui::pos2(200.0, 20.0),
                });

                if label.clicked() {
                    panel.remove(index);
                    panel.insert(0, view.to_owned());
                }

                ui.separator();
            }
        });

        ui.separator();

        match panel[0] {
            View::SceneHierarchy => scene_hierarchy::add_contents(ui),
            View::ProjectFiles => project_files::add_contents(ui),
            View::Properties => properties::add_contents(ui),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame {
            rounding: 1.0.into(),
            shadow: egui::epaint::Shadow::small_dark(),
            fill: egui::Color32::BLACK,
            ..Default::default()
        };

        egui::SidePanel::left(egui::Id::new("left panel"))
        .resizable(true)
        .frame(frame)
        .show(ctx, |ui| {
            self.add_contents(ui, Panel::Left);
        });
        
        egui::SidePanel::right(egui::Id::new("right panel"))
        .resizable(true)
        .frame(frame)
        .show(ctx, |ui| {
            self.add_contents(ui, Panel::Right);
        });
        
        egui::TopBottomPanel::bottom(egui::Id::new("bottom panel"))
        .height_range(egui::Rangef::new(100.0, 500.0))
        .resizable(true)
        .frame(frame)
        .show(ctx, |ui| {
            self.add_contents(ui, Panel::Bottom);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_playing {
                game_preview::add_contents(ctx, self, ui);
            }
        });
    }
}