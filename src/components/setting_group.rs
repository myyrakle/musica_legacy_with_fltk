use fltk::{
    button::Button,
    dialog::{FileDialog, FileDialogType},
    group::Group,
    prelude::*,
};

use crate::types::state::SharedState;

pub fn create_setting_group(state: SharedState, window_width: i32, window_height: i32) -> Group {
    let group_top_margin = 30;

    let setting_group = Group::new(0, group_top_margin, window_width, window_height, "Setting");

    let button_top_margin = 15;
    let button_left_margin = 15;

    let browse_button_width = 120;
    let browse_button_height = 40;
    let mut browse_button = Button::new(
        button_left_margin,
        group_top_margin + button_top_margin,
        browse_button_width,
        browse_button_height,
        "Choose Folder",
    );

    browse_button.set_callback(move |_| {
        let mut file_dialog = FileDialog::new(FileDialogType::BrowseDir);
        file_dialog.show();
        let path = file_dialog.filename();

        let mut state = state.lock().unwrap();
        state.set_directory_path(path);
        state.write_to_config_file();
        state.read_music_list();
    });

    setting_group.end();
    setting_group
}
