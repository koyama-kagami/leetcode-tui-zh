use crate::emit;
pub(crate) mod string_ops;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn daily_selection_uses_absolute_index_and_search_can_be_empty() {
        let mut list = Paginate::new((0..100).collect());
        list.set_element_by_index(73, 10);
        assert_eq!(list.hovered(), Some(&73));
        assert!(list.window(10).contains(&73));
        list.update_list(vec![]);
        assert!(list.window(10).is_empty());
        assert!(list.window(0).is_empty());
    }
}

use rand::{thread_rng, Rng};
use std::ops::Range;

pub struct Paginate<T> {
    list: Vec<T>,
    nth_window: usize,
    cursor_range: Range<usize>,
    cursor: usize,
    hovered: Option<T>,
}

impl<T> Paginate<T>
where
    T: Clone,
{
    pub fn new(list: Vec<T>) -> Self {
        let hovered = list.first().cloned();
        Self {
            list,
            nth_window: Default::default(),
            cursor_range: Default::default(),
            cursor: Default::default(),
            hovered,
        }
    }

    pub fn update_list(&mut self, list: Vec<T>) {
        *self = Self::new(list)
    }
}

impl<T> Paginate<T>
where
    T: Clone,
{
    pub fn next_elem(&mut self, wid_height: usize) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["暂无题目".into()]));
            return true;
        }
        self.set_cursor_range(wid_height);
        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        if self.cursor >= self.cursor_range.end {
            self.nth_window = self.nth_window.saturating_add(1).min(
                self.list
                    .windows(self.cursor_upper_bound(wid_height))
                    .count()
                    - 1,
            );
        } else {
            self.cursor = (self.cursor + 1).min(self.cursor_upper_bound(wid_height) - 1);
        }
        self.hovered = self.window(wid_height).get(self.cursor).cloned();
        self.cursor != old_cursor || self.nth_window != old_window
    }

    pub fn prev_elem(&mut self, wid_height: usize) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["暂无题目".into()]));
            return true;
        }
        self.set_cursor_range(wid_height);
        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        if self.cursor < self.cursor_range.start {
            self.nth_window = self.nth_window.saturating_sub(1);
        } else {
            self.cursor = self.cursor.saturating_sub(1);
        }

        self.hovered = self.window(wid_height).get(self.cursor).cloned();
        self.cursor != old_cursor || self.nth_window != old_window
    }

    pub fn rand_elem(&mut self, wid_height: usize) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["暂无题目".into()]));
            return true;
        }
        self.set_cursor_range(wid_height);
        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        self.cursor = thread_rng().gen_range(0..self.cursor_upper_bound(wid_height));

        let upper = self
            .list
            .windows(self.cursor_upper_bound(wid_height))
            .count()
            - 1;

        if upper > 0 {
            self.nth_window = thread_rng().gen_range(0..upper);
        }

        self.hovered = self.window(wid_height).get(self.cursor).cloned();
        self.cursor != old_cursor || self.nth_window != old_window
    }

    fn set_cursor_range(&mut self, wid_height: usize) {
        let mut b = self.cursor_upper_bound(wid_height);
        if self.cursor_upper_bound(wid_height) >= wid_height {
            b = self.cursor_upper_bound(wid_height).saturating_sub(3)
        }
        if self.nth_window == 0 {
            //first window
            self.cursor_range = 0..b;
        } else if self.nth_window
            == (self
                .list
                .windows(self.cursor_upper_bound(wid_height))
                .count()
                - 1)
        {
            // last_window
            self.cursor_range = 3..self.cursor_upper_bound(wid_height);
        } else {
            self.cursor_range = 3..self.cursor_upper_bound(wid_height).saturating_sub(3);
        }
    }

    pub fn set_element_by_index(&mut self, index: usize, wid_height: usize) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["暂无题目".into()]));
            return false;
        }

        if index >= self.list.len() {
            emit!(Popup(vec!["选择超出范围".into()]));
            return false;
        }

        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        self.nth_window = index.saturating_sub(self.cursor_upper_bound(wid_height) - 1);
        self.cursor = index - self.nth_window;

        self.set_cursor_range(wid_height);
        self.hovered = self.window(wid_height).get(self.cursor).cloned();

        self.cursor != old_cursor || self.nth_window != old_window
    }

    fn cursor_upper_bound(&self, wid_height: usize) -> usize {
        wid_height.min(self.list.len()).max(1)
    }

    pub fn window(&self, wid_height: usize) -> &[T] {
        self.list
            .windows(self.cursor_upper_bound(wid_height))
            .nth(self.nth_window)
            .unwrap_or(&[])
    }
    pub fn hovered(&self) -> Option<&T> {
        self.hovered.as_ref()
    }
}
