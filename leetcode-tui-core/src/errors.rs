use std::{num::ParseIntError, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("解答文件名格式不正确：{0}")]
    FileNameFormatDoesNotMatch(PathBuf),

    #[error("无法解析语言编号：{0}")]
    LangIdParseError(#[from] ParseIntError),

    #[error("路径中没有文件名：{0}")]
    FileNameDoesNotExistError(PathBuf),

    #[error("文件名不是有效 UTF-8：{0}")]
    Utf8ValidityError(PathBuf),

    #[error("题目 {0} 尚无解答，请先按 e 选择语言并编写代码")]
    QuestionIdDoesNotExist(String),
}

pub type CoreResult<T> = Result<T, CoreError>;
