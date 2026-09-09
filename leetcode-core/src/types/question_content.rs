use html2md::parse_html;
#[derive(Debug, serde::Deserialize)]
pub struct QueryQuestionContent {
    pub question: QuestionContent,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionContent {
    #[serde(
        default,
        deserialize_with = "crate::types::problemset_question_list::null_string"
    )]
    pub content: String,
    #[serde(default)]
    pub translated_content: Option<String>,
    pub title_slug: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub data: QueryQuestionContent,
}

impl QuestionContent {
    pub fn html_to_text(&self) -> String {
        let string = self
            .translated_content
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&self.content);
        let s: String = parse_html(string);
        s.lines().collect::<Vec<&str>>().join("\n")
    }
}
