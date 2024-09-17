use crate::RosterApp;
use crate::app::widgets::DropdownGrid;

pub fn roster_tab(app: &mut RosterApp, ui: &mut egui::Ui) {
    let key = ui.id().with("grid_one");
    let mut data = ui
        .data(|map| map.get_temp::<Vec<Vec<String>>>(key))
        .unwrap_or(vec![vec![String::from(""); app.blocks]; app.stations]);
    ui.add(DropdownGrid {
        data: &mut data,
        options: vec![String::from(""), String::from("A"), String::from("B")],
    });
    ui.data_mut(|map| map.insert_temp::<Vec<Vec<String>>>(key, data));
}
