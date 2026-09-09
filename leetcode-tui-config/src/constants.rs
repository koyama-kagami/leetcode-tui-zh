use std::env;
use std::sync::OnceLock;

pub static PROJECT_NAME: OnceLock<String> = OnceLock::new();
pub static GIT_COMMIT_HASH: OnceLock<String> = OnceLock::new();
pub static LOG_ENV: OnceLock<String> = OnceLock::new();
pub static LOG_FILE: OnceLock<String> = OnceLock::new();
pub static EDITOR: OnceLock<String> = OnceLock::new();

pub(crate) fn init() {
    let project_name = env!("CARGO_CRATE_NAME").to_uppercase().to_string();

    PROJECT_NAME.get_or_init(|| project_name.clone());

    GIT_COMMIT_HASH.get_or_init(|| {
        std::env::var(format!("{}_GIT_INFO", project_name.clone()))
            .unwrap_or_else(|_| String::from("UNKNOWN"))
    });

    LOG_ENV.get_or_init(|| format!("{}_LOGLEVEL", project_name.clone()));

    LOG_FILE.get_or_init(|| format!("{}.log", env!("CARGO_PKG_NAME")));

    EDITOR.get_or_init(|| {
        if let Ok(env_editor) = std::env::var("EDITOR") {
            env_editor
        } else if is_executable_in_path("nvim") {
            "nvim".into()
        } else if is_executable_in_path("vim") {
            "vim".into()
        } else if is_executable_in_path("nano") {
            "nano".into()
        } else if cfg!(windows) {
            "notepad.exe".into()
        } else {
            "vi".into()
        }
    });
}

fn is_executable_in_path(executable_name: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| {
        std::env::split_paths(&paths).any(|dir| {
            dir.join(executable_name).is_file()
                || (cfg!(windows) && dir.join(format!("{executable_name}.exe")).is_file())
        })
    })
}
