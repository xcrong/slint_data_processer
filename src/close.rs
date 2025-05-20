

use crate::AppWindow;
use slint::{CloseRequestResponse, ComponentHandle, Weak};

pub fn close_windows_event(ui_weak: Weak<AppWindow>) {
    if let Some(ui_strong) = ui_weak.upgrade() {
        ui_strong
            .window()
            .on_close_requested({ move || CloseRequestResponse::HideWindow });
    }
}
