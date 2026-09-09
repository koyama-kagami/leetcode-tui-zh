pub mod errors;
pub mod site;
pub use site::Site;
pub mod graphql;
pub mod types;
use errors::AppResult;
pub use graphql::client::GQLLeetcodeRequest;
pub use graphql::query::problemset_question_list::Query as QuestionRequest;
pub use graphql::query::question_content::Query as QuestionContentRequest;
pub use graphql::query::run_code::RunCodeRequest;
pub use graphql::query::submit_code::SubmitCodeRequest;
pub use graphql::query::EditorDataRequest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::sync::OnceLock;
pub use types::editor_data::QuestionData as EditorDataResponse;
pub use types::problemset_question_list::Root as QuestionResponse;

pub static REQ_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub async fn init(csrf: &str, sess: &str) -> AppResult<()> {
    init_for_site(csrf, sess, Site::Cn).await
}

pub async fn init_for_site(csrf: &str, sess: &str, site: Site) -> AppResult<()> {
    site::set(site)?;
    let client = build_reqwest_client(csrf, sess).await?;
    REQ_CLIENT.get_or_init(|| client);
    Ok(())
}

pub(crate) fn get_client() -> &'static Client {
    REQ_CLIENT.get().expect("Client not initialized")
}

pub async fn build_reqwest_client(csrf: &str, sess: &str) -> AppResult<Client> {
    let mut headers = HeaderMap::new();
    let header_k_v = [
        (
            "Cookie",
            format!("LEETCODE_SESSION={sess}; csrftoken={csrf}"),
        ),
        ("Content-Type", "application/json".to_string()),
        ("x-csrftoken", csrf.to_string()),
        ("Origin", site::current().origin().to_string()),
        ("Referer", site::current().origin().to_string()),
        ("Connection", "keep-alive".to_string()),
    ];

    for (key, value) in header_k_v {
        headers.append(key, HeaderValue::from_str(value.as_str())?);
    }

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Mozilla/5.0 leetui/0.5.2")
        .build()?;
    Ok(client)
}
