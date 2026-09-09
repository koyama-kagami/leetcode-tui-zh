use thiserror::Error;

#[derive(Error, Debug)]
pub enum LcAppError {
    #[error("站点已初始化，请重启后切换站点")]
    SiteAlreadyInitialized,
    #[error("力扣接口返回错误：{0}")]
    GraphqlError(String),
    #[error("登录已失效或访问被拒绝，请更新当前站点 config.toml 中的 lc_session 和 csrftoken。")]
    CookiesExpiredError,

    #[error("数据解析失败：{0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("网络请求失败，请检查网络和站点设置。")]
    RequestError(#[from] reqwest::Error),

    #[error("HTTP 状态 {code:?}：{contents:?}")]
    StatusCodeError { code: String, contents: String },

    #[error("初始化网络客户端失败：{0}")]
    ClientBuildError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("题目 {0} 不支持该语言")]
    LanguageDoesNotExistError(String),
}

pub type AppResult<T> = Result<T, LcAppError>;
