use crate::core::config::ConfigManager;
use crate::core::output;
use crate::core::project::find_project_root;

pub fn handle_config_get(key: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    match cfg.get_value(key) {
        Some(value) => {
            output::info(&format!("{key} = {value}"));
            true
        }
        None => {
            output::warn(&format!("未找到配置项 {key}"));
            false
        }
    }
}

pub fn handle_config_set(key: &str, value: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    cfg.set_value(key, value);
    true
}
