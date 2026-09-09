use color_eyre::Result;
use leetcode_tui_config::CONFIG;
use leetcode_tui_db;
use leetcode_tui_rs::app::App;
use leetcode_tui_rs::utils::update_database_questions;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        println!("力扣终端中文版\n\n默认进入 Hot 100；按 1 / 2 切换 Hot 100 / 面试经典 150。\n按 e 选择 Python 3 等语言编写解答，R 运行样例，s 提交。\n按 / 搜索中文题名，? 查看全部快捷键，q 退出。\n\n配置文件：{}\nsite = cn 使用中国站；site = com 使用国际站。\n请配置相应站点的 lc_session 和 csrftoken。", leetcode_tui_config::utils::get_config_file_path().display());
        return Ok(());
    }
    leetcode_tui_config::init().await?;
    leetcode_tui_db::init(Some(&CONFIG.as_ref().db.path));
    leetcode_core::init_for_site(
        &CONFIG.as_ref().csrftoken,
        &CONFIG.as_ref().lc_session,
        CONFIG.as_ref().site,
    )
    .await?;
    leetcode_tui_core::init();
    update_database_questions(false).await?;
    App::run().await
}
