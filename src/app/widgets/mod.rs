mod roster_tab;
mod setup;
pub use roster_tab::roster_tab;
pub use setup::setup_tab;

pub struct DropdownGrid<'a> {
    pub options: Vec<String>,
    pub data: &'a mut Vec<Vec<String>>,
}

impl<'a> egui::Widget for DropdownGrid<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let key = ui.id().with("dropdown_grid");
        let response = egui::Grid::new(key).show(ui, |ui| {
            self.data.iter_mut().enumerate().for_each(|(row_id, row)| {
                row.iter_mut().enumerate().for_each(|(col_id, elem)| {
                    egui::ComboBox::from_id_source(key.with(row_id).with(col_id))
                        .selected_text(elem.clone())
                        .show_ui(ui, |ui| {
                            for option in &self.options {
                                ui.selectable_value(elem, option.clone(), option.clone());
                            }
                        });
                });
                ui.end_row();
            });
        });
        response.response
    }
}