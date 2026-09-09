use std::num::ParseIntError;

use native_db::db_type;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErr {
    #[error("本地题库错误：{0}")]
    NativeDbError(#[from] db_type::Error),

    #[error("无法创建分类：{0}")]
    TopicCreateError(String),

    #[error("FrontEndQuestionIdParseError: {0}")]
    FrontEndQuestionIdParseError(#[from] ParseIntError),

    #[error("未找到题目：{0}")]
    QuestionsNotFoundInDb(String),

    #[error("未找到分类：{0}")]
    TopicsNotFoundInDb(String),
}

pub type DBResult<T> = Result<T, DbErr>;
