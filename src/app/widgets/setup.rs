use super::input_list::input_list;
use crate::RosterApp;

pub fn setup_tab(app: &mut RosterApp, ui: &mut egui::Ui) {
    ui.heading("Blocks");
    input_list(&mut app.blocks, ui, "Blocks");

    ui.heading("Stations");
    input_list(&mut app.stations, ui, "Stations");

    ui.heading("Controllers");
    input_list(&mut app.controllers, ui, "Controllers");
    // TODO some notes would be neat
}
