use super::*;

pub trait CustomDisplay {
    fn display(&self) -> String {
        self.get_display_lines().join("\n")
    }

    fn get_display_lines(&self) -> Vec<String>;
}

pub trait CodeSubmissionDisplay: CodeSubmissionCommons {
    fn display(&self) -> String {
        self.get_display_lines().join("\n")
    }

    fn get_display_lines(&self) -> Vec<String> {
        let mut lines = vec![
            self.get_heading(),
            self.get_memory_usage_string(),
            self.run_time_taken_string(),
        ];

        if let Some(rtp) = self.get_run_time_percentile_string() {
            lines.push(rtp)
        }
        if let Some(mp) = self.get_memory_percentile_string() {
            lines.push(mp)
        }
        lines
    }
}

pub trait CodeSubmissionCommons {
    fn get_heading(&self) -> String;

    fn get_memory_usage_string(&self) -> String {
        format!("内存占用：{}", self.get_memory_usage())
    }

    fn run_time_taken_string(&self) -> String {
        format!("运行耗时：{}", self.run_time_taken())
    }

    fn get_memory_usage(&self) -> &Memory;

    fn run_time_taken(&self) -> &str;

    fn get_run_time_percentile_string(&self) -> Option<String> {
        self.run_time_percentile()
            .map(|v| format!("运行速度超过 {}% 的提交", v))
    }

    fn get_memory_percentile_string(&self) -> Option<String> {
        self.memory_percentile()
            .map(|v| format!("内存表现超过 {}% 的提交", v))
    }

    fn run_time_percentile(&self) -> Option<f32> {
        None
    }

    fn memory_percentile(&self) -> Option<f32> {
        None
    }
}

impl CodeSubmissionCommons for RunAccepted {
    fn get_heading(&self) -> String {
        format!(
            "运行完成：通过 {}/{} 个用例",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

impl CodeSubmissionCommons for RunWrongAnswer {
    fn get_heading(&self) -> String {
        format!(
            "运行未通过：通过 {}/{} 个用例",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

impl CodeSubmissionCommons for SubmitAccepted {
    fn get_heading(&self) -> String {
        format!(
            "提交成功：通过 {}/{} 个用例",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }

    fn run_time_percentile(&self) -> Option<f32> {
        Some(self.runtime_percentile)
    }

    fn memory_percentile(&self) -> Option<f32> {
        Some(self.memory_percentile)
    }
}

impl CodeSubmissionCommons for SubmitWrongAnswer {
    fn get_heading(&self) -> String {
        format!(
            "通过 {}/{} 个用例",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

macro_rules! code_submission_display_impl {
    ($tr: ident, ($($ResponseType: ident),*)) => {
        $(
            impl $tr for $ResponseType {}
        )*
    };
}

code_submission_display_impl!(
    CodeSubmissionDisplay,
    (
        RunAccepted,
        RunWrongAnswer,
        SubmitAccepted,
        SubmitWrongAnswer
    )
);

impl CustomDisplay for Timeout {
    fn get_display_lines(&self) -> Vec<String> {
        vec!["请求超时".to_string()]
    }
}

impl CustomDisplay for CompileError {
    fn get_display_lines(&self) -> Vec<String> {
        vec![format!("编译错误：{}", self.full_compile_error)]
    }
}

impl CustomDisplay for RuntimeError {
    fn get_display_lines(&self) -> Vec<String> {
        vec![format!("运行错误：{}", self.full_runtime_error)]
    }
}

impl CustomDisplay for MemoryLimitExceeded {
    fn get_display_lines(&self) -> Vec<String> {
        vec![format!("超出内存限制：{}", self.memory)]
    }
}

impl CustomDisplay for InternalError {
    fn get_display_lines(&self) -> Vec<String> {
        vec![format!("力扣内部错误：状态码 {}", self.status_code)]
    }
}

impl CustomDisplay for TimeLimitExceeded {
    fn get_display_lines(&self) -> Vec<String> {
        vec![format!("超出时间限制：耗时 {}", self.elapsed_time)]
    }
}

impl CustomDisplay for OutputLimitExceed {
    fn get_display_lines(&self) -> Vec<String> {
        vec![
            format!("超出输出限制，最后用例：{:?}", self.last_testcase),
            format!("预期输出：{:?}", self.expected_output),
            format!("标准输出：{:?}", self.std_output),
            format!("实际输出：{:?}", self.code_output),
        ]
    }
}

macro_rules! display_impl {
    (($($ResponseType: ident),*)) => {
        $(
            impl std::fmt::Display for $ResponseType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.display())
                }
            }
        )*
    };
}

display_impl!((
    RunAccepted,
    RunWrongAnswer,
    SubmitAccepted,
    SubmitWrongAnswer,
    Timeout,
    CompileError,
    RuntimeError,
    MemoryLimitExceeded,
    InternalError,
    TimeLimitExceeded,
    OutputLimitExceed
));

impl std::fmt::Display for ParsedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = match self {
            ParsedResponse::RunAccepted(ra) => ra.to_string(),
            ParsedResponse::SubmitAccepted(sa) => sa.to_string(),
            ParsedResponse::RunWrongAnswer(rwa) => rwa.to_string(),
            ParsedResponse::SubmitWrongAnswer(swa) => swa.to_string(),
            ParsedResponse::Pending => "判题中…".to_string(),
            ParsedResponse::CompileError(ce) => ce.to_string(),
            ParsedResponse::RuntimeError(re) => re.to_string(),
            ParsedResponse::MemoryLimitExceeded(mle) => mle.to_string(),
            ParsedResponse::OutputLimitExceed(ole) => ole.to_string(),
            ParsedResponse::TimeLimitExceeded(tle) => tle.to_string(),
            ParsedResponse::InternalError(ie) => ie.to_string(),
            ParsedResponse::Unknown(e) => format!("未知错误，状态码：{e}"),
            ParsedResponse::TimeOut(to) => to.to_string(),
        };
        write!(f, "{res}")
    }
}

impl CustomDisplay for ParsedResponse {
    fn get_display_lines(&self) -> Vec<String> {
        match self {
            ParsedResponse::Pending => vec![self.to_string()],
            ParsedResponse::CompileError(r) => r.get_display_lines(),
            ParsedResponse::RuntimeError(r) => r.get_display_lines(),
            ParsedResponse::MemoryLimitExceeded(r) => r.get_display_lines(),
            ParsedResponse::OutputLimitExceed(r) => r.get_display_lines(),
            ParsedResponse::TimeLimitExceeded(r) => r.get_display_lines(),
            ParsedResponse::InternalError(r) => r.get_display_lines(),
            ParsedResponse::Unknown(_) => vec![self.to_string()],
            ParsedResponse::TimeOut(r) => r.get_display_lines(),
            ParsedResponse::RunAccepted(r) => r.get_display_lines(),
            ParsedResponse::SubmitAccepted(r) => r.get_display_lines(),
            ParsedResponse::RunWrongAnswer(r) => r.get_display_lines(),
            ParsedResponse::SubmitWrongAnswer(r) => r.get_display_lines(),
        }
    }
}
