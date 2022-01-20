use crate::visiter::TreeVisiter;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Bound;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<K, V>
where
    K: Ord,
{
    range_start: K,
    range_end: K,
    tree: BTreeMap<K, V>,
}

impl<K, V> Page<K, V>
where
    K: Ord + Clone + Debug,
    V: Debug,
{
    #[cfg(test)]
    pub fn from_range(range_start: K, range_end: K) -> Page<K, V> {
        Page {
            range_start,
            range_end,
            tree: BTreeMap::new(),
        }
    }

    pub fn from_key_value(key: K, value: V) -> Page<K, V> {
        Page {
            range_start: key.clone(),
            range_end: key.clone(),
            tree: BTreeMap::from([(key, value)]),
        }
    }

    pub fn range_start(&self) -> &K {
        &self.range_start
    }

    pub fn range_end(&self) -> &K {
        &self.range_end
    }

    pub fn set_range_end(&mut self, range_end: K) {
        self.range_end = range_end;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.tree.insert(key, value).is_none()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.tree.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.tree.remove(key).is_some()
    }

    /// Returns `true` if next page must be processed, otherwise returns `false`.
    pub fn range<F>(&self, key_first: &K, key_last: &K, mut callback: F) -> bool
    where
        F: FnMut(&K, &V) -> bool,
    {
        for (key, value) in self.tree.range(key_first..=key_last) {
            if !callback(key, value) {
                return false;
            }
        }

        true
    }

    pub fn succ(&self, key: &K) -> Option<(&K, &V)> {
        self.tree
            .range((Bound::Excluded(key), Bound::Unbounded))
            .next()
    }

    pub fn pred(&self, key: &K) -> Option<(&K, &V)> {
        self.tree
            .range((Bound::Unbounded, Bound::Excluded(key)))
            .rev()
            .next()
    }

    pub fn size(&self) -> usize {
        self.tree.len()
    }

    pub fn split(&mut self) -> Page<K, V> {
        // TODO: Replace with get root.
        let middle = self.tree.len() / 2;

        if let Some(key) = self.tree.keys().nth(middle).cloned() {
            let tree = self.tree.split_off(&key);
            let next = Page {
                range_start: key,
                range_end: self.range_end.clone(),
                tree,
            };

            if let Some(key) = self.tree.keys().rev().next() {
                self.range_end = key.clone();
            }

            next
        } else {
            unreachable!()
        }
    }

    pub fn visit<T>(&self, index: usize, visiter: &mut T)
    where
        T: TreeVisiter<K, V>,
    {
        visiter.visit_page_before(index, &self.range_start, &self.range_end);

        for (key, value) in &self.tree {
            visiter.visit_value(key, value);
        }

        visiter.visit_page_after(index, &self.range_start, &self.range_end);
    }
}

#[cfg(test)]
mod tests {
    use crate::pages::Page;

    #[test]
    fn from_key_value_must_create_page() {
        let page: Page<_, usize> = Page::from_key_value(10, 20);

        assert_eq!(10, *page.range_start());
        assert_eq!(10, *page.range_end());
        assert_eq!(Some(&20), page.get(&10));
    }

    #[test]
    fn must_change_end_range() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        assert_eq!(20, *page.range_end());

        page.set_range_end(15);

        assert_eq!(15, *page.range_end());
    }

    #[test]
    fn insert_must_add_value() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        assert_eq!(0, page.size());

        page.insert(15, 150);

        assert_eq!(1, page.size());
    }

    #[test]
    fn insert_must_replace_value() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        assert_eq!(0, page.size());
        assert_eq!(true, page.insert(15, 150));
        assert_eq!(1, page.size());
        assert_eq!(false, page.insert(15, 150));
        assert_eq!(1, page.size());
    }

    #[test]
    fn get_must_return_value_if_exists() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(Some(&150), page.get(&15));
    }

    #[test]
    fn get_must_return_none_if_missing() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(None, page.get(&16));
    }

    #[test]
    fn contains_must_return_true_if_exists() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(true, page.contains(&15));
    }

    #[test]
    fn contains_must_return_false_if_missing() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(false, page.contains(&16));
    }

    #[test]
    fn remove_must_return_true_if_exists() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(1, page.size());
        assert_eq!(true, page.remove(&15));
        assert_eq!(0, page.size());
    }

    #[test]
    fn remove_must_return_false_if_missing() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(15, 150);
        assert_eq!(1, page.size());
        assert_eq!(false, page.remove(&16));
        assert_eq!(1, page.size());
    }

    #[test]
    fn size_must_return_page_size() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        assert_eq!(0, page.size());
        page.insert(15, 150);
        assert_eq!(1, page.size());
        page.insert(16, 160);
        assert_eq!(2, page.size());
        page.insert(15, 150);
        assert_eq!(2, page.size());
    }

    #[test]
    fn split_must_split_page() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        for index in 0..4 {
            page.insert(index, index);
        }

        assert_eq!(4, page.size());

        let next = page.split();

        assert_eq!(true, page.contains(&0));
        assert_eq!(true, page.contains(&1));
        assert_eq!(false, page.contains(&2));
        assert_eq!(false, page.contains(&3));
        assert_eq!(2, page.size());

        assert_eq!(false, next.contains(&0));
        assert_eq!(false, next.contains(&1));
        assert_eq!(true, next.contains(&2));
        assert_eq!(true, next.contains(&3));
        assert_eq!(2, next.size());
    }

    #[test]
    fn range_must_select_one_value() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);
        let mut result = Vec::new();

        page.insert(10, 100);
        page.insert(20, 200);
        page.insert(30, 300);

        assert_eq!(
            true,
            page.range(&15, &25, |&k, &v| {
                result.push((k, v));

                true
            })
        );
        assert_eq!(vec![(20, 200)], result);
    }

    #[test]
    fn range_must_select_one_when_breaked() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);
        let mut result = Vec::new();

        page.insert(10, 100);
        page.insert(20, 200);

        assert_eq!(
            false,
            page.range(&15, &25, |&k, &v| {
                result.push((k, v));

                false
            })
        );
        assert_eq!(vec![(20, 200)], result);
    }

    #[test]
    fn range_must_select_all_values() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);
        let mut result = Vec::new();

        page.insert(1, 10);
        page.insert(2, 20);
        page.insert(3, 30);
        page.insert(4, 40);
        page.insert(5, 50);

        assert_eq!(
            true,
            page.range(&2, &4, |&k, &v| {
                result.push((k, v));

                true
            })
        );
        assert_eq!(vec![(2, 20), (3, 30), (4, 40)], result);
    }

    #[test]
    fn succ_must_select_next_value() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(1, 10);
        page.insert(2, 20);

        assert_eq!(Some((&2, &20)), page.succ(&1));
    }

    #[test]
    fn succ_must_select_none() {
        let mut page: Page<_, usize> = Page::from_range(10, 20);

        page.insert(1, 10);
        page.insert(2, 20);

        assert_eq!(None, page.succ(&2));
    }
}
