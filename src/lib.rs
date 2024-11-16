use std::{borrow::Borrow, cmp, collections::VecDeque, rc::Rc};

#[derive(Default, Debug)]
pub struct MinQueue<T> {
    v: VecDeque<(Rc<T>, Rc<T>)>,
    border: usize,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        return Self {
            v: VecDeque::new(),
            border: 0,
        };
    }

    pub fn push(&mut self, val: T) {
        let rc_val = Rc::new(val);
        let rc_val_clone = rc_val.clone();
        self.v.push_back({
            if self.len() == self.border {
                (rc_val, rc_val_clone)
            } else {
                (
                    rc_val,
                    cmp::min(rc_val_clone, self.v.back().unwrap().1.clone()),
                )
            }
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.border != 0 {
            self.border = self.border.saturating_sub(1);
            return self.v.pop_front().map(|x| (*x.0).clone());
        }
        while self.border != self.v.len() {
            let mut val_pair = self.v.pop_back().unwrap();
            if self.border == 0 {
                val_pair.1 = val_pair.0.clone()
            } else {
                val_pair.1 = cmp::min(val_pair.0.clone(), self.v.front().unwrap().1.clone())
            };
            self.v.push_front(val_pair);
            self.border += 1;
        }
        self.border = self.border.saturating_sub(1);
        self.v.pop_front().map(|x| (*x.0).clone())
    }

    pub fn front(&self) -> Option<&T> {
        return self.v.front().map(|x| x.0.borrow());
    }

    pub fn min(&self) -> Option<&T> {
        let back = self.v.back().map(|x| x.1.borrow());
        let front = self.v.front().map(|x| x.1.borrow());
        cmp::min(back.or(front), front)
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
