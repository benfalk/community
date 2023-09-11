pub(crate) trait SplitWhileTrait {
    type Item: Sized;
    fn split_while<P>(&mut self, predicate: P) -> Vec<Self::Item>
    where
        P: Fn(&Self::Item) -> bool;
}

impl<T: Sized> SplitWhileTrait for Vec<T> {
    type Item = T;
    fn split_while<P>(&mut self, predicate: P) -> Vec<Self::Item>
    where
        P: Fn(&Self::Item) -> bool,
    {
        self.split_off(
            self.len() - self.iter().rev().take_while(|i| predicate(i)).count()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_while_empty() {
        let mut items: Vec<i32> = vec![];
        let empty: Vec<i32> = vec![];
        let popped = items.split_while(|i| *i == 1);
        assert_eq!(popped, empty);
        assert_eq!(items, empty);
    }

    #[test]
    fn split_while_simple() {
        let empty: Vec<i32> = vec![];
        let mut items = vec![1, 1, 1, 2, 2];
        let twos = items.split_while(|i| *i == 2);
        assert_eq!(items, vec![1, 1, 1]);
        assert_eq!(twos, vec![2, 2]);

        let zeros = items.split_while(|i| *i == 0);
        assert_eq!(zeros, empty);
        assert_eq!(items, vec![1, 1, 1]);

        let ones = items.split_while(|i| *i == 1);
        assert_eq!(ones, vec![1, 1, 1]);
        assert_eq!(items, empty);
    }
}
