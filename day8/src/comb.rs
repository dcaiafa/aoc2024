use std::collections::VecDeque;

#[derive(Debug)]
struct StackItem {
    cur: Vec<usize>,
    avail: VecDeque<usize>,
}

#[derive(Debug)]
pub struct CombRaw {
    r: usize,
    stack: Vec<StackItem>,
}

impl std::iter::Iterator for CombRaw {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let i = self.stack.last_mut().unwrap();
            if i.cur.len() == self.r {
                let emit = self.stack.pop().unwrap().cur;
                return Some(emit);
            }
            if i.avail.len() < self.r - i.cur.len() {
                self.stack.pop();
                continue;
            }
            let next = i.avail.pop_front().unwrap();
            let mut cur = i.cur.clone();
            cur.push(next);
            let new = StackItem {
                cur,
                avail: i.avail.clone(),
            };
            self.stack.push(new);
        }
        return None;
    }
}

pub struct Comb<'a, T> {
    elems: &'a [T],
    iter: CombRaw,
}

impl<'a, T> std::iter::Iterator for Comb<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(raw_next) => Some(raw_next.into_iter().map(|i| &self.elems[i]).collect()),
            None => None,
        }
    }
}

pub fn comb<'a, T>(e: &'a [T], r: usize) -> Comb<'a, T> {
    Comb {
        elems: e,
        iter: CombRaw {
            r,
            stack: vec![StackItem {
                cur: vec![],
                avail: (0..e.len()).collect(),
            }],
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let e = vec![0, 1, 2, 3, 4];
        let res = comb(&e, 3).collect::<Vec<Vec<&i32>>>();
        let res: Vec<Vec<i32>> = res
            .iter()
            .map(|inner| inner.iter().copied().copied().collect())
            .collect();
        assert_eq!(
            res,
            vec![
                vec![0, 1, 2],
                vec![0, 1, 3],
                vec![0, 1, 4],
                vec![0, 2, 3],
                vec![0, 2, 4],
                vec![0, 3, 4],
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 3, 4],
                vec![2, 3, 4],
            ]
        );
    }
}
