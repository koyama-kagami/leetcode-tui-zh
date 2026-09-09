use leetcode_core::types::{problemset_question_list::Root, question_content::Data};
use leetcode_core::{GQLLeetcodeRequest, QuestionContentRequest, QuestionRequest};

#[test]
fn new_server_languages_do_not_break_python_templates() {
    let editor: leetcode_core::types::editor_data::QuestionData = serde_json::from_value(serde_json::json!({"data":{"question":{
        "questionId":"1", "questionFrontendId":"1", "titleSlug":"two-sum", "content":"", "enableRunCode":true,
        "codeSnippets":[{"lang":"Python3","langSlug":"python3","code":"class Solution: pass"}, {"lang":"Cangjie","langSlug":"cangjie","code":""}]
    }}})).unwrap();
    assert_eq!(editor.get_languages().len(), 1);
    assert_eq!(editor.get_languages()[0].to_string(), "python3");
}

#[test]
fn defaults_to_china_and_uses_china_question_list() {
    let q = QuestionRequest::default();
    assert_eq!(q.get_endpoint(), "https://leetcode.cn/graphql/");
    assert!(q.get_body()["query"].as_str().unwrap().contains("titleCn"));
    assert!(
        QuestionContentRequest::new("two-sum".into()).get_body()["query"]
            .as_str()
            .unwrap()
            .contains("translatedContent")
    );
}

#[test]
fn chinese_content_with_english_fallback() {
    for (translated, expected) in [
        (Some("<p>两数之和</p>"), "两数之和"),
        (None, "Two Sum"),
        (Some(""), "Two Sum"),
    ] {
        let data: Data = serde_json::from_value(serde_json::json!({"data":{"question":{
            "content":"<p>Two Sum</p>", "translatedContent":translated, "titleSlug":"two-sum"
        }}}))
        .unwrap();
        assert_eq!(data.data.question.html_to_text().trim(), expected);
    }
}

#[test]
fn china_titles_difficulties_and_status_are_normalized() {
    let root: Root = serde_json::from_value(serde_json::json!({"data":{"problemsetQuestionList":{
        "total":1,"questions":[{"frontendQuestionId":"LCR 001", "title":"Divide", "titleCn":"两数相除", "titleSlug":"divide", "difficulty":"EASY", "paidOnly":false, "status":"NOT_STARTED"}]
    }}})).unwrap();
    let q = root.get_questions().remove(0);
    assert_eq!(q.title, "两数相除");
    assert_eq!(q.difficulty, "Easy");
    assert_eq!(q.status, None);
}

#[test]
fn china_daily_response_accepts_anonymous_status_and_translated_title() {
    let daily: leetcode_core::types::daily_coding_challenge::IDailyCodingChallenge = serde_json::from_value(serde_json::json!({"data":{"todayRecord":[{
        "date":"2026-09-08", "userStatus":null, "question":{
            "frontendQuestionId":"3870", "title":"Count Commas in Range", "translatedTitle":"统计范围内的逗号", "titleSlug":"count-commas-in-range", "difficulty":"Easy", "paidOnly":false
        }
    }]}})).unwrap();
    assert_eq!(
        daily
            .data
            .active_daily_coding_challenge_question
            .question
            .localized()
            .title,
        "统计范围内的逗号"
    );
    assert!(serde_json::from_value::<
        leetcode_core::types::daily_coding_challenge::IDailyCodingChallenge,
    >(serde_json::json!({"data":{"todayRecord":[]}}))
    .is_err());
}
