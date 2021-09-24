use crate::tab::{Row, Tab};
use std::rc::Rc;

pub type SharedRow = Rc<Box<Row>>;

pub enum Diff {
    Left { left: SharedRow },
    Right { right: SharedRow },
    Both { left: SharedRow, right: SharedRow },
}

impl Diff {
    pub fn new_left(left: SharedRow) -> Diff {
        Diff::Left { left: left.clone() }
    }

    pub fn new_right(right: SharedRow) -> Diff {
        Diff::Right { right: right.clone() }
    }

    pub fn new_both(left: SharedRow, right: SharedRow) -> Diff {
        Diff::Both {
            left: left.clone(),
            right: right.clone(),
        }
    }

    pub fn left(&self) -> Option<SharedRow> {
        match self {
            Diff::Left { left } => Option::Some(left.clone()),
            Diff::Right { .. } => Option::None,
            Diff::Both { left, .. } => Option::Some(left.clone()),
        }
    }

    pub fn right(&self) -> Option<SharedRow> {
        match self {
            Diff::Left { .. } => Option::None,
            Diff::Right { right } => Option::Some(right.clone()),
            Diff::Both { right, .. } => Option::Some(right.clone()),
        }
    }

    pub fn create_diffs(left: Tab, right: Tab) -> Vec<Diff> {
        let sl = left.rows.len();
        let sr = right.rows.len();
        let mut il: usize = 0;
        let mut ir: usize = 0;
        let mut rl = &left.rows[il];
        let mut rr = &right.rows[ir];
        let mut d: Diff;
        let mut diffs: Vec<Diff> = vec!();
        while il < sl && ir < sr {
            rl = &left.rows[il];
            rr = &right.rows[ir];
            if rl == rr {
                d = Diff::new_both(rl.clone(), rr.clone());
                il += 1;
                ir += 1;
            } else if rl < rr {
                d = Diff::new_left(rl.clone());
                il += 1;
            } else { // if rl > rr
                d = Diff::new_both(rl.clone(), rr.clone());
                ir += 1;
            }
            diffs.push(d);
        }
        for i in il .. left.rows.len() {
            diffs.push(Diff::new_left(left.rows[i].clone()))
        }
        for i in ir .. right.rows.len() {
            diffs.push(Diff::new_right(right.rows[i].clone()))
        }
        diffs
    }
}
