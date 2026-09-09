use leetcode_core::types::language::Language;
use leetcode_core::{
    GQLLeetcodeRequest, QuestionContentRequest, QuestionRequest, RunCodeRequest, Site,
    SubmitCodeRequest,
};

#[tokio::test]
async fn international_site_keeps_its_queries_and_all_judge_endpoints() {
    leetcode_core::init_for_site("", "", Site::Com)
        .await
        .unwrap();
    assert_eq!(
        QuestionRequest::default().get_endpoint(),
        "https://leetcode.com/graphql/"
    );
    assert!(!QuestionRequest::default().get_body()["query"]
        .as_str()
        .unwrap()
        .contains("titleCn"));
    assert!(
        !QuestionContentRequest::new("two-sum".into()).get_body()["query"]
            .as_str()
            .unwrap()
            .contains("translatedContent")
    );
    let run = RunCodeRequest::new(
        Language::Python3,
        None,
        "1".into(),
        "pass".into(),
        "two-sum".into(),
    );
    let submit = SubmitCodeRequest::new(
        Language::Python3,
        "1".into(),
        "pass".into(),
        "two-sum".into(),
    );
    assert_eq!(
        run.get_endpoint(),
        "https://leetcode.com/problems/two-sum/interpret_solution/"
    );
    assert_eq!(
        submit.get_endpoint(),
        "https://leetcode.com/problems/two-sum/submit/"
    );
    assert!(leetcode_core::init_for_site("", "", Site::Cn)
        .await
        .is_err());
}
