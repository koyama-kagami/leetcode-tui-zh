use crate::SendError;

#[derive(Default)]
pub struct Input {
    pub visible: bool,
    current_text: Option<String>,
    sender: Option<super::UBStrSender>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chinese_input_backspace_removes_a_whole_character() {
        let mut input = Input::default();
        for c in "两数之和".chars() {
            input.char(c);
        }
        input.remove_char();
        assert_eq!(input.text().map(String::as_str), Some("两数之"));
    }

    #[test]
    fn paste_publishes_one_complete_query_in_order() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let mut input = Input::default();
        input.reset_with(tx, None);
        input.paste("两数之和\r\n");
        input.remove_char();
        assert_eq!(rx.try_recv().unwrap(), Some("两数之和".into()));
        assert_eq!(rx.try_recv().unwrap(), Some("两数之".into()));
        assert!(rx.try_recv().is_err());
    }
}

impl Input {
    pub fn text(&self) -> Option<&String> {
        self.current_text.as_ref()
    }
}

impl Input {
    pub fn close(&mut self) -> bool {
        self.current_text = None;
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(None).emit_if_error();
        }
        self.toggle()
    }

    pub fn char(&mut self, c: char) -> bool {
        if let Some(_text) = self.current_text.as_mut() {
            _text.push(c);
        } else {
            self.current_text = Some(c.into());
        }
        self.try_send();
        true
    }

    pub fn paste(&mut self, text: &str) -> bool {
        self.current_text
            .get_or_insert_with(String::new)
            .extend(text.chars().filter(|c| !c.is_control()));
        self.try_send();
        true
    }

    pub fn remove_char(&mut self) -> bool {
        if let Some(_text) = self.current_text.as_mut() {
            if !_text.is_empty() {
                _text.pop();
                self.try_send();
            }
        }
        true
    }

    pub fn try_send(&mut self) {
        let text = self.current_text.clone();
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(text).emit_if_error();
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn reset_with(&mut self, sender: super::UBStrSender, default_input: Option<String>) {
        self.sender = Some(sender);
        self.current_text = default_input;
    }
}
