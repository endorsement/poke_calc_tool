use eframe::{egui, epi};
use super::poke_calc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    me : poke_calc::DmgCalcPokemonData,

    #[cfg_attr(feature = "persistence", serde(skip))]
    opponent : poke_calc::DmgCalcPokemonData,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            me: Default::default(),
            opponent: Default::default(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { label, value, me, opponent } = self;
        // let Self { label, value } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            ui.columns(3, |columns|{
                egui::ScrollArea::vertical()
                    .id_source("offense")
                    .show(&mut columns[0], |ui|
                        ui.group(|ui| {
                            ui.label("Offense");
                            pokemon_drawer(ui, me);
                        })
                    );
                egui::ScrollArea::vertical()
                    .id_source("defence")
                    .show(&mut columns[1], |ui|
                        ui.group(|ui| {
                            ui.label("Field");
                        })
                    );
                egui::ScrollArea::vertical()
                    .id_source("defence")
                    .show(&mut columns[2], |ui|
                        ui.group(|ui| {
                            ui.label("Defense");
                            pokemon_drawer(ui, opponent);
                        })
                    );
            });
            // ui.group(|ui| {
            //     ui.label("Offense");
            //     gui_drawer(ui, me);
            // });
            // ui.separator();
            // ui.group(|ui| {
            //     ui.label("Offense");
            //     gui_drawer(ui, me);
            // });
            egui::warn_if_debug_build(ui);
        });

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
        */

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn pokemon_drawer(ui: &mut eframe::egui::Ui, poke_data: &mut poke_calc::DmgCalcPokemonData) {
    ui.horizontal(|ui| {
        ui.label("HP");
        ui.add(egui::DragValue::new(& mut poke_data.hp).clamp_range(0..=255))
            .on_hover_text("Hp");
        ui.add(egui::DragValue::new(& mut poke_data.hp_iv).clamp_range(0..=31))
            .on_hover_text("Hp_iv");
        ui.add(egui::DragValue::new(& mut poke_data.hp_ev).clamp_range(0..=252))
            .on_hover_text("Hp_ev");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.label("ATK");
        ui.add(egui::DragValue::new(& mut poke_data.attack).clamp_range(0..=255))
            .on_hover_text("Attack");
        ui.add(egui::DragValue::new(& mut poke_data.attack_iv).clamp_range(0..=31))
            .on_hover_text("Attack_iv");
        ui.add(egui::DragValue::new(& mut poke_data.attack_ev).clamp_range(0..=252))
            .on_hover_text("Attack_ev");
        ui.add(egui::DragValue::new(& mut poke_data.attack_st).clamp_range(-6..=6))
            .on_hover_text("Attack_St");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.label("DEF");
        ui.add(egui::DragValue::new(& mut poke_data.defense).clamp_range(0..=255))
            .on_hover_text("Defense");
        ui.add(egui::DragValue::new(& mut poke_data.defense_iv).clamp_range(0..=31))
            .on_hover_text("Defense_iv");
        ui.add(egui::DragValue::new(& mut poke_data.defense_iv).clamp_range(0..=252))
            .on_hover_text("Defense_ev");
        ui.add(egui::DragValue::new(& mut poke_data.defense_st).clamp_range(-6..=6))
            .on_hover_text("Defense_St");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.label("SPA");
        ui.add(egui::DragValue::new(& mut poke_data.special_attack).clamp_range(0..=255))
            .on_hover_text("Special Atk");
        ui.add(egui::DragValue::new(& mut poke_data.special_attack_iv).clamp_range(0..=31))
            .on_hover_text("Special Atk_iv");
        ui.add(egui::DragValue::new(& mut poke_data.special_attack_ev).clamp_range(0..=252))
            .on_hover_text("Special Atk_ev");
        ui.add(egui::DragValue::new(& mut poke_data.special_attack_st).clamp_range(-6..=6))
            .on_hover_text("Special Atk_St");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.label("SP.DEF");
        ui.add(egui::DragValue::new(& mut poke_data.special_defense_iv).clamp_range(0..=255))
            .on_hover_text("Special Def");
        ui.add(egui::DragValue::new(& mut poke_data.special_defense_iv).clamp_range(0..=31))
            .on_hover_text("Special Def_iv");
        ui.add(egui::DragValue::new(& mut poke_data.special_defense_iv).clamp_range(0..=252))
            .on_hover_text("Special Def_ev");
        ui.add(egui::DragValue::new(& mut poke_data.special_defense_st).clamp_range(-6..=6))
            .on_hover_text("Special Def_St");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.label("SPD");
        ui.add(egui::DragValue::new(& mut poke_data.speed).clamp_range(0..=255))
            .on_hover_text("Speed");
        ui.add(egui::DragValue::new(& mut poke_data.speed_iv).clamp_range(0..=31))
            .on_hover_text("Speed_iv");
        ui.add(egui::DragValue::new(& mut poke_data.speed_iv).clamp_range(0..=252))
            .on_hover_text("Speed_ev");
        ui.add(egui::DragValue::new(& mut poke_data.speed_st).clamp_range(-6..=6))
            .on_hover_text("Speed_St");
    });
    ui.end_row();
}
fn field_drawer(ui: &mut eframe::egui::Ui, poke_data: &mut poke_calc::FieldCondition) {
}