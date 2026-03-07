use crate::core::config::ConfigManager;
use crate::core::output;

pub fn handle_init() -> bool {
    let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let cfg = ConfigManager::new(&root);
    let _ = cfg.ensure_config();
    cfg.ensure_gitignore();
    output::ok("初始化完成，.aide/ 与默认配置已准备就绪");
    true
}
