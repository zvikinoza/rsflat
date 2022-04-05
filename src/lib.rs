
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
    where
        I: IntoIterator,
        I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    outer: O,
    front_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_inner: None,
            back_inner: None,
        }
    }
}

impl <O> Iterator for Flatten<O>
    where
        O:Iterator,
        O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_inner) = self.front_inner {
                if let Some(i) = front_inner.next() {
                    return Some(i);
                }
                self.front_inner = None;
            }
            if let Some(out) = self.outer.next() {
                self.front_inner = Some(out.into_iter());
            } else {
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
    where
        O: Iterator + DoubleEndedIterator,
        O::Item: IntoIterator,
        <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_inner) = self.back_inner {
                if let Some(i) = back_inner.next_back() {
                    return Some(i);
                }
                self.back_inner = None;
            }
            if let Some(out) = self.outer.next_back() {
                self.back_inner = Some(out.into_iter());
            } else {
                return self.front_inner.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn twice() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn twice_deep() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn empty_deep() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_deep() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]);
    }

    #[test]
    fn next_and_next_back() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn infinite_iter() {
        let mut iter = flatten((0..).map(|i| { 0..i }));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn we_need_to_go_deeper() {
        let deep = vec![vec![vec![vec![vec![1, 2]], vec![vec![3, 4]]]]];
        assert_eq!(flatten(flatten(flatten(flatten(deep)))).count(), 4);
    }
}