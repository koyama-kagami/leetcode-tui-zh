use color_eyre::Result;
use kdam::BarExt;
use leetcode_core::{GQLLeetcodeRequest, QuestionRequest};
use leetcode_tui_core::emit;
use leetcode_tui_db::DbQuestion;

pub async fn update_database_questions(runs_inside_tui: bool) -> Result<()> {
    if !runs_inside_tui && DbQuestion::get_total_questions()? > 0 {
        return Ok(());
    }
    let total = QuestionRequest::default()
        .send()
        .await?
        .get_total_questions()
        .max(0) as usize;
    let mut progress = kdam::tqdm!(total = total);
    let mut questions = Vec::with_capacity(total);
    while questions.len() < total {
        let page = QuestionRequest::new(100, questions.len() as i32)
            .send()
            .await?
            .get_questions();
        if page.is_empty() {
            color_eyre::eyre::bail!("题库同步提前结束，请稍后重试");
        }
        let count = page.len();
        for q in page {
            questions.push(DbQuestion::try_from(q)?);
        }
        if runs_inside_tui {
            emit!(ProgressUpdate(
                "正在同步题库…".into(),
                questions.len() as u32,
                total as u32
            ));
        } else {
            progress.update(count)?;
        }
    }
    DbQuestion::save_multiple_to_db(questions)?;
    Ok(())
}
