use crate::core::config::ConfigManager;
use crate::core::output;
use crate::core::project::find_project_root;
use std::fs;
use std::io::{self, Write};

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

pub fn handle_config_reset(force: bool) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);

    if cfg.config_path.exists() && !force {
        output::warn("此操作将重置配置到默认值，现有配置将备份。是否继续？[y/N]");
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            output::info("已取消");
            return false;
        }
    }

    if cfg.config_path.exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = cfg.backups_dir.join(format!("config.toml.{}", timestamp));
        let _ = cfg.ensure_base_dirs();

        if let Err(e) = fs::copy(&cfg.config_path, &backup_path) {
            output::err(&format!("备份失败: {}", e));
            return false;
        }

        output::ok(&format!("已备份配置到 .aide/backups/config.toml.{}", timestamp));
    }

    let _ = fs::write(&cfg.config_path, crate::core::config::DEFAULT_CONFIG);
    cfg.generate_config_md();

    output::ok("配置已重置为默认值");
    true
}

pub fn handle_config_update() -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);

    let config = cfg.load_config();
    let current_schema = crate::core::config::walk_get(&config, "meta.schema_version")
        .and_then(|v| v.as_integer())
        .unwrap_or(0);

    if current_schema >= crate::core::config::CURRENT_SCHEMA_VERSION {
        output::ok(&format!("配置已是最新版本（schema v{}）", current_schema));
        return true;
    }

    output::info(&format!(
        "检测到配置版本差异：当前 schema v{}，最新 schema v{}",
        current_schema,
        crate::core::config::CURRENT_SCHEMA_VERSION
    ));

    let _ = fs::write(&cfg.config_path, crate::core::config::DEFAULT_CONFIG);
    cfg.generate_config_md();

    output::ok(&format!("配置已更新到 schema v{}", crate::core::config::CURRENT_SCHEMA_VERSION));
    output::ok("已更新配置说明 .aide/config.md");
    true
}
