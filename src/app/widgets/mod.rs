mod input_list;
mod roster_tab;
mod setup;
pub use roster_tab::roster_tab;
pub use setup::setup_tab;

pub struct DropdownGrid<'a> {
    pub data: &'a mut Vec<Vec<String>>,
    pub options: &'a Vec<String>,
    pub column_titles: &'a Vec<String>,
    pub row_titles: &'a Vec<String>,
}

impl<'a> egui::Widget for DropdownGrid<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let key = ui.id().with("dropdown_grid");
        let response = egui::Grid::new(key)
            .num_columns(self.column_titles.len() + 1)
            .show(ui, |ui| {
                ui.label("Station");
                self.column_titles.iter().for_each(|title| {
                    ui.label(title.clone());
                });
                ui.end_row();

                self.data.iter_mut().enumerate().for_each(|(row_id, row)| {
                    ui.label(self.row_titles[row_id].clone());

                    row.iter_mut().enumerate().for_each(|(col_id, elem)| {
                        egui::ComboBox::from_id_source(key.with(row_id).with(col_id))
                            .selected_text(elem.clone())
                            .show_ui(ui, |ui| {
                                for option in self.options {
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

pub fn enter_pressed(response: egui::Response) -> bool {
    response.lost_focus()
        && response.ctx.input(|i| i.key_pressed(egui::Key::Enter))
}
