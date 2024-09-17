use crate::RosterApp;

pub fn setup_tab(app: &mut RosterApp, ui: &mut egui::Ui) {
    egui::Grid::new("setup_grid").num_columns(2).show(ui, |ui| {
        ui.label("Blocks");
        let mut blocks_str = format!("{}", app.blocks);
        ui.text_edit_singleline(&mut blocks_str);
        if let Ok(c) = blocks_str.parse::<usize>() {
            app.blocks = c;
        }
        ui.end_row();

        ui.label("Stations");
        let mut stations_str = format!("{}", app.stations);
        ui.text_edit_singleline(&mut stations_str);
        if let Ok(c) = stations_str.parse::<usize>() {
            app.stations = c;
        }
        ui.end_row();
    });
}
