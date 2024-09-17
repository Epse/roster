use crate::app::widgets::DropdownGrid;
use crate::RosterApp;

fn get_empty(app: &RosterApp) -> Vec<Vec<String>> {
    vec![vec![String::from(""); app.blocks.len()]; app.stations.len()]
}

pub fn roster_tab(app: &mut RosterApp, ui: &mut egui::Ui) {
    let key = ui.id().with("grid_one");
    let mut data = ui
        .data(|map| map.get_temp::<Vec<Vec<String>>>(key))
        .unwrap_or(get_empty(app));
    if ui.button("Reset").clicked() {
        data = get_empty(app);
    }
    ui.add(DropdownGrid {
        data: &mut data,
        options: &app.controllers,
        column_titles: &app.blocks,
        row_titles: &app.stations,
    });
    ui.data_mut(|map| map.insert_temp::<Vec<Vec<String>>>(key, data));
}
