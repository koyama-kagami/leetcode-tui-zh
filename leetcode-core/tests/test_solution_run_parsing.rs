use leetcode_core::types::run_submit_response::{ParsedResponse, RunSubmitResult};
use serde_json::{self, Value};

const JSONS_STR: &str = include_str!("./test_solution_run_parsing.json");

pub(crate) fn get_parsed_response(key_name: &str) -> ParsedResponse {
    let parsed: Value = serde_json::from_str(JSONS_STR).unwrap();
    let run_wrong_value = &parsed[key_name];
    let parsed_response: RunSubmitResult = serde_json::from_value(run_wrong_value.clone()).unwrap();
    parsed_response.to_parsed_response().unwrap()
}

#[test]
fn test_should_parse_run_correct_response_correctly() {
    assert!(matches!(
        get_parsed_response("run_correct"),
        ParsedResponse::RunAccepted { .. }
    ))
}

#[test]
fn test_should_parse_run_wrong_response_correctly() {
    assert!(matches!(
        get_parsed_response("run_wrong"),
        ParsedResponse::RunWrongAnswer { .. }
    ))
}

#[test]
fn test_should_parse_submit_correct_response_correctly() {
    assert!(matches!(
        get_parsed_response("submit_correct"),
        ParsedResponse::SubmitAccepted { .. }
    ))
}

#[test]
fn test_should_parse_submit_wrong_response_correctly() {
    assert!(matches!(
        get_parsed_response("submit_wrong"),
        ParsedResponse::SubmitWrongAnswer { .. }
    ))
}

#[test]
fn test_run_correct_response_output() {
    let parsed_response = get_parsed_response("run_correct");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "运行完成：通过 2/2 个用例",
            "内存占用：2.00 MB",
            "运行耗时：0 ms",
        ]
        .join("\n")
    )
}

#[test]
fn test_run_wrong_response_output() {
    let parsed_response = get_parsed_response("run_wrong");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "运行未通过：通过 0/3 个用例",
            "内存占用：16.39 MB",
            "运行耗时：82 ms",
        ]
        .join("\n")
    )
}

#[test]
fn test_submit_correct_response_output() {
    let parsed_response = get_parsed_response("submit_correct");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "提交成功：通过 57/57 个用例",
            "内存占用：2.35 MB",
            "运行耗时：2 ms",
            "运行速度超过 83.9281% 的提交",
            "内存表现超过 39.7233% 的提交",
        ]
        .join("\n")
    )
}

#[test]
fn test_submit_wrong_response_output() {
    let parsed_response = get_parsed_response("submit_wrong");
    assert_eq!(
        parsed_response.to_string(),
        vec!["通过 3/80 个用例", "内存占用：2.32 MB", "运行耗时：N/A",].join("\n")
    )
}

#[test]
fn test_memory_limit_exceeded_response_output() {
    let parsed_response = get_parsed_response("memory_limit_exceeded");
    assert_eq!(
        parsed_response.to_string(),
        vec!["超出内存限制：976.69 MB"].join("\n")
    )
}
#[test]
fn test_output_limit_response_output() {
    let parsed_response = get_parsed_response("output_limit");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            r#"超出输出限制，最后用例："maybe long testcase""#,
            r#"预期输出："true""#,
            r#"标准输出："some_long_string""#,
            r#"实际输出："""#,
        ]
        .join("\n")
    )
}
