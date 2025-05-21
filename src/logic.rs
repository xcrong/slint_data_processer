use crate::{AppWindow, ProcessAdapter};
use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel, Weak};
use std::rc::Rc;

pub fn impl_logic_for_process_asdapter(ui_weak: Weak<AppWindow>) {
    if let Some(ui_strong) = ui_weak.clone().upgrade() {
        let process_adapter = ui_strong.global::<ProcessAdapter>();

        process_adapter.on_choose_file(|| {
            println!("choose file");
            "fake_path".into()
        });

        process_adapter.on_process_data(|| {
            println!("process data");
            // ModelRc<ModelRc<StandardListViewItem>>
            let table_data: Rc<VecModel<ModelRc<StandardListViewItem>>> =
                Rc::new(VecModel::default());
            let row = Rc::new(VecModel::default());
            table_data.push(row.into());

            table_data.into()
        });

        process_adapter.on_copy_result(|table_data, start_row| {
            println!("copy result");
        });

        process_adapter.on_del_by_starts_with(|table_data, stats_with| {
            println!("del by starts with");

            let table_data: Rc<VecModel<ModelRc<StandardListViewItem>>> =
                Rc::new(VecModel::default());
            let row = Rc::new(VecModel::default());
            table_data.push(row.into());

            table_data.into()
        });
    }
}
