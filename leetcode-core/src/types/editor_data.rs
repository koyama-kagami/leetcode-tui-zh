use super::language::Language;
use crate::errors::{AppResult, LcAppError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub lang: String,
    pub lang_slug: Language,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    pub question_frontend_id: String,
    #[serde(deserialize_with = "supported_snippets")]
    pub code_snippets: Vec<CodeSnippet>,
    pub title_slug: String,
    pub enable_run_code: bool,
    #[serde(
        default,
        deserialize_with = "crate::types::problemset_question_list::null_string"
    )]
    pub content: String,
    #[serde(default)]
    pub translated_content: Option<String>,
}

// New server-side languages must not prevent opening a Python/C++ solution.
fn supported_snippets<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<CodeSnippet>, D::Error> {
    let raw = Vec::<serde_json::Value>::deserialize(deserializer)?;
    let mut snippets = Vec::new();
    for value in raw {
        let lang = value
            .get("langSlug")
            .ok_or_else(|| serde::de::Error::custom("代码模板缺少 langSlug"))?;
        if serde_json::from_value::<Language>(lang.clone()).is_err() {
            continue;
        }
        snippets.push(serde_json::from_value(value).map_err(serde::de::Error::custom)?);
    }
    Ok(snippets)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionEditorData {
    pub question: Question,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionData {
    pub data: QuestionEditorData,
}

impl QuestionData {
    pub fn get_languages(&self) -> Vec<&Language> {
        self.data
            .question
            .code_snippets
            .iter()
            .map(|snippet| &snippet.lang_slug)
            .collect()
    }

    pub fn get_editor_data_by_language(&self, lang: &Language) -> Option<&str> {
        let mut filtered = self
            .data
            .question
            .code_snippets
            .iter()
            .filter(|cs| &cs.lang_slug == lang)
            .collect::<Vec<_>>();
        filtered.pop().map(|v| v.code.as_str())
    }

    pub fn get_filename(&self, for_lang: &Language) -> AppResult<String> {
        if self.get_languages().contains(&for_lang) {
            let frontend_id = &self.data.question.question_frontend_id;
            let slug = &self.data.question.title_slug;
            let lang_id: u32 = for_lang.clone().into();
            let extension = for_lang.get_extension();
            let id = frontend_id
                .parse::<u32>()
                .map(|n| format!("{n:04}"))
                .unwrap_or_else(|_| frontend_id.clone());
            let encoded = format!("{id}_{slug}_{lang_id}.{extension}");
            Ok(encoded)
        } else {
            Err(LcAppError::LanguageDoesNotExistError(
                self.data.question.title_slug.to_string(),
            ))
        }
    }
}

// impl TryFrom<&str> for Question {
//     type Error = LcAppError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {}
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_filename_encoding() {
//         let qed = QuestionData {
//             data: QuestionEditorData {
//                 question: Question {
//                     question_id: "1".into(),
//                     question_frontend_id: "1".into(),
//                     code_snippets: vec![Language::Python3.into()],
//                     title_slug: "two-sum".into(),
//                     enable_run_code: false,
//                 },
//             },
//         };
//         assert_eq!(
//             qed.get_filename(&Language::Python3).unwrap(),
//             "0001_two-sum_11.py"
//         );
//     }

//     #[test]
//     fn test_filename_decoding() -> AppResult<()> {
//         let test_data = Question {
//             question_id: "22".into(),
//             question_frontend_id: "22".into(),
//             code_snippets: vec![Language::Kotlin.into()],
//             title_slug: "three-sum".into(),
//             enable_run_code: false,
//         };
//         let target: Question = ("0022_three-sum_11.kt").try_into()?;
//         assert_eq!(target.question_id, test_data.question_id);
//         assert_eq!(target.title_slug, test_data.title_slug);
//         Ok(())
//     }
// }
