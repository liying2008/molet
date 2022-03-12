#![windows_subsystem = "windows"]

use molet_app::{init_db, load_config, load_ui};

fn main() {
    // 加载配置
    let config = load_config();
    // 初始化数据库
    init_db(&config);
    // 加载UI
    load_ui()
}
