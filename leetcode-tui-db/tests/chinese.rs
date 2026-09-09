use leetcode_tui_db::DbQuestion;

#[test]
fn accepts_chinese_series_question_ids() {
    let question: leetcode_core::types::problemset_question_list::Question = serde_json::from_value(serde_json::json!({
        "frontendQuestionId":"面试题 01.01", "title":"判定字符是否唯一", "titleSlug":"is-unique-lcci", "difficulty":"Easy", "paidOnly":false
    })).unwrap();
    let question = DbQuestion::try_from(question).unwrap();
    assert!(question.to_string().contains("面试题 01.01"));
}

#[test]
fn priority_plans_keep_official_order_and_shared_completion() {
    use leetcode_tui_db::{
        study_plans::{HOT_100, INTERVIEW_150},
        DbTopic,
    };
    leetcode_tui_db::init(None);
    let mut unique = std::collections::BTreeSet::new();
    unique.extend(HOT_100.iter().copied());
    unique.extend(INTERVIEW_150.iter().copied());
    let questions = unique
        .into_iter()
        .enumerate()
        .map(|(i, slug)| {
            DbQuestion::new(
                i + 1,
                slug,
                slug,
                "Easy".into(),
                false,
                if slug == "two-sum" {
                    Some("ac".into())
                } else {
                    None
                },
            )
        })
        .collect();
    DbQuestion::save_multiple_to_db(questions).unwrap();
    for (plan, expected) in [("hot-100", HOT_100), ("interview-150", INTERVIEW_150)] {
        let questions = DbTopic::new(plan).fetch_questions().unwrap();
        assert_eq!(
            questions
                .iter()
                .map(|q| q.title_slug.as_str())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            questions
                .iter()
                .find(|q| q.title_slug == "two-sum")
                .unwrap()
                .status
                .as_deref(),
            Some("ac")
        );
    }
}
