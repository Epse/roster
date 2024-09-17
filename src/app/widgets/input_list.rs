use super::enter_pressed;

pub fn input_list(data: &mut Vec<String>, ui: &mut egui::Ui, title: &str) {
    let key = ui.id().with(title);
    egui::Grid::new(key)
        .striped(true)
        .num_columns(2)
        .show(ui, |ui| {
            data.retain(|elem| {
                let mut keep = true;
                ui.label(elem.clone());
                if ui.button("X").clicked() {
                    keep = false;
                }
                ui.end_row();
                keep
            });

            let mut text = ui
                .data(|map| map.get_temp::<String>(key.with("tmp_input")))
                .clone()
                .unwrap_or(String::from(""));
            let input_id = key.with("tmp_input");
            let input = ui.add(egui::TextEdit::singleline(&mut text).id(input_id));
            if ui.button("+").clicked() || enter_pressed(input) {
                data.push(text.clone());
                text = String::from("");
                ui.memory_mut(|m| m.request_focus(input_id));
            }
            ui.data_mut(|map| map.insert_temp::<String>(key.with("tmp_input"), text));
            ui.end_row();
        });
}
