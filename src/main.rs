// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data_processer::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    {
        // 为数据模型 Backend 添加回调
        data_processer::logic::impl_logic_for_backend(ui_weak.clone());
    }

    {
        // 启动事件，加载配置
        data_processer::startup::init(ui_weak.clone());
    }

    {
        // 窗口关闭时的事件，保存页面状态
        data_processer::close::close_windows_event(ui_weak.clone());
    }

    ui.run()?;

    Ok(())
}
