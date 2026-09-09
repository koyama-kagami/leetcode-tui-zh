use leetcode_core::{
    EditorDataRequest, GQLLeetcodeRequest, QuestionContentRequest, QuestionRequest, Site,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let site = if std::env::args().any(|arg| arg == "com") {
        Site::Com
    } else {
        Site::Cn
    };
    leetcode_core::init_for_site("", "", site).await?;
    let list = QuestionRequest::new(2, 0).send().await?;
    println!("题库总数：{}", list.get_total_questions());
    for q in list.get_questions() {
        println!("{} {} {}", q.frontend_question_id, q.title, q.difficulty);
    }
    let content = QuestionContentRequest::new("two-sum".into()).send().await?;
    let text = content.data.question.html_to_text();
    assert!(!text.is_empty());
    if site == Site::Cn {
        assert!(text.contains("整数"));
    }
    let editor = EditorDataRequest::new("two-sum".into()).send().await?;
    assert!(editor
        .get_languages()
        .iter()
        .any(|lang| lang.to_string() == "python3"));
    let daily = leetcode_core::graphql::query::daily_coding_challenge::Query::new()
        .send()
        .await?;
    println!(
        "每日一题：{}",
        daily
            .data
            .active_daily_coding_challenge_question
            .question
            .localized()
            .title
    );
    println!("题面、Python 3 模板、每日一题：通过");
    Ok(())
}
