use super::theme::Theme;
use crate::utils::{get_config_file_path, get_default_database_file_path, get_solutions_dir_path};
use color_eyre::Result;
use leetcode_tui_shared::RoCell;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::{fs::create_dir_all, path::PathBuf};
pub static CONFIG: RoCell<Config> = RoCell::new();

#[cfg(test)]
mod chinese_tests {
    use super::*;
    #[test]
    fn old_config_defaults_to_china_and_site_data_is_isolated() {
        let mut cn: Config = toml::from_str("csrftoken = ''\nlc_session = ''").unwrap();
        let mut com: Config =
            toml::from_str("site = 'com'\ncsrftoken = ''\nlc_session = ''").unwrap();
        assert_eq!(cn.site, leetcode_core::Site::Cn);
        cn.apply_site_paths();
        com.apply_site_paths();
        assert_ne!(cn.db.path, com.db.path);
        assert_ne!(cn.solutions_dir, com.solutions_dir);
        assert!(cn.db.path.to_string_lossy().contains("-cn-v2.db"));
    }
}

pub fn init() -> Result<()> {
    CONFIG.init({
        let config_file = get_config_file_path();
        if !config_file.exists() {
            Config::create_default_config(&config_file);
            Config::create_default_solution_dir();
            println!(
                "请将当前站点 Cookie 中的 LEETCODE_SESSION 填入 lc_session，将 csrftoken 填入同名字段。\n配置文件：{}",
                config_file.display()
            );
            std::process::exit(0);
        }

        let contents = std::fs::read_to_string(&config_file)?;
        let mut parsed_config: Config = toml::from_str(&contents)?;

        if parsed_config.db.path.to_str() == Some("") {
            println!(
                "请设置有效的 db.path，或删除 db 配置以使用默认路径。\n配置文件：{}",
                config_file.display()
            );
            std::process::exit(0);
        }

        if parsed_config.solutions_dir.to_str() == Some("") {
            println!(
                "请设置有效的 solutions_dir，或删除该配置以使用默认路径。\n配置文件：{}",
                config_file.display()
            );
            std::process::exit(0);
        }

        parsed_config.apply_site_paths();

        if !parsed_config.solutions_dir.exists() {
            create_dir_all(parsed_config.solutions_dir.clone())?;
        }

        if !parsed_config.db.path.exists() {
            if let Some(parent) = parsed_config.db.path.parent() { create_dir_all(parent)?; }
        }

        parsed_config
    });
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub site: leetcode_core::Site,
    pub csrftoken: String,
    pub lc_session: String,
    #[serde(default, skip_serializing)]
    pub db: Database,
    #[serde(default = "get_solutions_dir_path", skip_serializing)]
    pub solutions_dir: PathBuf,
    #[serde(default, skip_serializing)]
    pub theme: Theme,
}

impl Config {
    fn apply_site_paths(&mut self) {
        let stem = self
            .db
            .path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        self.db.path = self
            .db
            .path
            .with_file_name(format!("{}-{}-v2.db", stem, self.site.key()));
        self.solutions_dir = self.solutions_dir.join(self.site.key());
    }

    fn create_default_solution_dir() {
        create_dir_all(get_solutions_dir_path()).unwrap();
    }

    fn create_default_config(config_file: &PathBuf) {
        let config_dir = config_file.as_path().parent().expect(&format!(
            "无法获取配置文件的父目录：{}",
            config_file.display()
        ));
        create_dir_all(config_dir)
            .expect(format!("无法创建配置目录：{}", config_dir.display()).as_str());
        let default_config = Self::default();
        let default_config_str = toml::to_string(&default_config).expect("无法生成默认配置");
        let mut file = File::create(&config_file)
            .expect(format!("无法创建文件：{}", config_file.display()).as_str());
        file.write_all(default_config_str.as_bytes())
            .expect(format!("无法写入配置文件：{}", config_file.display()).as_str());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: get_default_database_file_path(),
        }
    }
}
