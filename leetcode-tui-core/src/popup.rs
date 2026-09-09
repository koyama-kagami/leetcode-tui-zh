use ratatui::widgets::{ListState, ScrollbarState};
use std::fmt::Display;

use crate::emit;

#[cfg(test)]
mod chinese_tests {
    use super::*;
    #[test]
    fn scrolls_all_wrapped_rows_and_clamps_after_resize() {
        let mut popup = Popup::new(vec!["中文长段落".repeat(20)]);
        popup.set_rendered_height(20, 5);
        for _ in 0..50 {
            popup.scroll_down();
        }
        assert_eq!(popup.v_scroll, 15);
        popup.set_rendered_height(2, 5);
        assert_eq!(popup.v_scroll, 0);
        assert!(!popup.scroll_down());
    }
}

#[derive(Default)]
pub struct Popup {
    pub visible: bool,
    lines: Vec<String>,
    pub v_scroll_state: ScrollbarState,
    pub v_scroll: usize,
    max_scroll: usize,
    title: Option<String>,
}

impl Popup {
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

impl Popup {
    pub fn new(lines: Vec<String>) -> Self {
        let mut p = Popup {
            lines,
            ..Default::default()
        };
        p.v_scroll_state = p.v_scroll_state.content_length(p.lines.len());
        p
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn reset(&mut self, title: Option<String>, lines: Vec<String>) {
        let mut p = Self::new(lines);
        p.visible = self.visible;
        p.title = title;
        *self = p;
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    /// Scrolling follows rendered rows, including wrapped CJK paragraphs.
    pub fn set_rendered_height(&mut self, rows: usize, viewport: usize) {
        self.max_scroll = rows.saturating_sub(viewport).min(u16::MAX as usize);
        self.v_scroll = self.v_scroll.min(self.max_scroll);
        self.v_scroll_state = ScrollbarState::new(rows)
            .viewport_content_length(viewport)
            .position(self.v_scroll);
    }

    pub fn scroll_down(&mut self) -> bool {
        if self.v_scroll >= self.max_scroll {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_add(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }

    pub fn scroll_up(&mut self) -> bool {
        if self.v_scroll == 0 {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_sub(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }
}

#[derive(Default)]
pub struct SelectPopup<T: Display> {
    pub visible: bool,
    pub state: ListState,
    items: Vec<T>,
    sender: Option<tokio::sync::oneshot::Sender<Option<usize>>>,
    title: Option<String>,
}

impl<T: Display> SelectPopup<T> {
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

impl<T: Display> SelectPopup<T> {
    pub fn with_items(
        &mut self,
        maybe_title: Option<String>,
        items: Vec<T>,
        sender: tokio::sync::oneshot::Sender<Option<usize>>,
    ) {
        *self = SelectPopup {
            visible: self.visible,
            state: ListState::default(),
            items,
            sender: Some(sender),
            title: maybe_title,
        };
        if !self.items.is_empty() {
            self.state.select(Some(0))
        }
    }

    pub fn get_lines(&self) -> &Vec<T> {
        &self.items
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn close_unselected(&mut self) -> bool {
        if let Some(sender) = self.sender.take() {
            if let Err(e) = sender.send(None) {
                emit!(Error(format!("无法发送选择结果：{e:?}")));
            } else {
                self.toggle();
            }
        }
        true
    }

    pub fn close(&mut self) -> bool {
        let mut error_message = None;
        if let Some(sender) = self.sender.take() {
            let k = sender.send(self.state.selected());
            if let Err(e) = k {
                error_message = Some(format!(
                    "index: {:?} could not be sent through the channel",
                    e
                ));
            };
        } else {
            error_message = Some("选择窗口已关闭，无法发送结果".to_string());
        }
        if let Some(em) = error_message {
            emit!(Error(em));
        }
        self.toggle();
        true
    }

    pub fn next_item(&mut self) -> bool {
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

    pub fn prev_item(&mut self) -> bool {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
