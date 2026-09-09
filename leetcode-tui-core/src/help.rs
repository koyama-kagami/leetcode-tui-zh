use ratatui::widgets::*;

pub struct Help {
    state: TableState,
    items: Vec<Vec<&'static str>>,
    visible: bool,
}

impl Default for Help {
    fn default() -> Self {
        let mut help = Self {
            state: TableState::default(),
            items: vec![
                vec!["1", "Hot 100"],
                vec!["2", "面试经典 150 题"],
                vec!["t", "下一个分类"],
                vec!["T", "上一个分类"],
                vec!["Ctrl+s", "显示 / 隐藏分类统计"],
                vec!["j/Down", "下一道题"],
                vec!["k/Up", "上一道题"],
                vec!["r", "随机题目"],
                vec!["Enter", "阅读题面 / 确认选择"],
                vec!["e", "选择语言并打开编辑器"],
                vec!["R", "运行样例"],
                vec!["s", "提交解答"],
                vec!["/", "搜索"],
                vec!["c", "打开配置文件"],
                vec!["*", "同步题库"],
                vec!["d", "每日一题"],
                vec!["Esc", "关闭弹窗 / 搜索"],
                vec!["q / Ctrl+c", "退出"],
            ],
            visible: Default::default(),
        };
        if !help.items.is_empty() {
            help.state.select(Some(0));
        }
        help
    }
}

impl Help {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn get_mut_state(&mut self) -> &mut TableState {
        &mut self.state
    }
}

impl Help {
    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn get_items(&self) -> &Vec<Vec<&'static str>> {
        &self.items
    }

    pub fn next(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn previous(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn get_headers() {}
}
