mod page;

pub use self::page::Page;
use crate::config::Config;
use crate::visiter::TreeVisiter;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pages<K, V>
where
    K: Ord,
{
    #[serde(skip)]
    config: Rc<Config>,
    pages: Vec<Page<K, V>>,
}

impl<K, V> Pages<K, V>
where
    K: Ord + Clone + Debug,
    V: Debug,
{
    pub fn new(config: Rc<Config>) -> Pages<K, V> {
        Pages {
            config,
            pages: Vec::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.pages.is_empty() {
            return None;
        }

        match self
            .pages
            .partition_point(|page| page.range_start() <= &key)
        {
            0 => None,
            index => self.pages[index - 1].get(key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.pages.is_empty() {
            let page = Page::from_key_value(key, value);

            self.pages.push(page);

            return true;
        }

        match self
            .pages
            .partition_point(|page| page.range_start() <= &key)
        {
            0 => {
                let page = Page::from_key_value(key, value);

                self.pages.insert(0, page);

                true
            }
            index => {
                let page = &mut self.pages[index - 1];

                if page.range_end() < &key {
                    page.set_range_end(key.clone());
                }

                let result = page.insert(key, value);

                if page.size() > self.config.max_page_size() {
                    let next = page.split();

                    self.pages.insert(index, next);
                }

                result
            }
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        if self.pages.is_empty() {
            return false;
        }

        match self
            .pages
            .partition_point(|page| page.range_start() <= &key)
        {
            0 => false,
            index => self.pages[index - 1].contains(key),
        }
    }

    pub fn remove(&mut self, key: &K) -> bool {
        if self.pages.is_empty() {
            return false;
        }

        match self
            .pages
            .partition_point(|page| page.range_start() <= &key)
        {
            0 => false,
            index => {
                let index = index - 1;
                let page = &mut self.pages[index];
                let result = page.remove(key);

                if result && page.size() == 0 {
                    self.pages.remove(index);
                }

                result
            }
        }
    }

    pub fn size(&self) -> usize {
        self.pages.iter().map(|page| page.size()).sum()
    }

    pub fn visit<T>(&self, visiter: &mut T)
    where
        T: TreeVisiter<K, V>,
    {
        for (index, page) in self.pages.iter().enumerate() {
            page.visit(index, visiter);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pages::Pages;
    use crate::visiter::TreeVisiter;
    use crate::Config;
    use std::rc::Rc;

    #[derive(Default)]
    struct CollectVisiter<K, V> {
        pages: Vec<Vec<(K, V)>>,
        page: Vec<(K, V)>,
    }

    impl<K, V> TreeVisiter<K, V> for CollectVisiter<K, V>
    where
        K: Clone,
        V: Clone,
    {
        fn visit_value(&mut self, key: &K, value: &V) {
            self.page.push((key.clone(), value.clone()));
        }

        fn visit_page_after(&mut self, _index: usize, _range_start: &K, _range_end: &K) {
            self.pages.push(self.page.clone());
            self.page.clear();
        }
    }

    #[test]
    fn get_must_return_none_if_pages_empty() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(None, pages.get(&10));
    }

    #[test]
    fn get_must_return_none_if_value_not_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(None, pages.get(&9));
        assert_eq!(None, pages.get(&11));
        assert_eq!(None, pages.get(&17));
        assert_eq!(None, pages.get(&25));
    }

    #[test]
    fn get_must_return_value_if_value_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(Some(&0), pages.get(&10));
        assert_eq!(Some(&0), pages.get(&12));
        assert_eq!(Some(&0), pages.get(&18));
        assert_eq!(Some(&0), pages.get(&20));
    }

    #[test]
    fn insert_must_add_value_if_empty() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(true, pages.insert(10, 10));
    }

    #[test]
    fn insert_must_replace_value_if_empty() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(true, pages.insert(10, 10));
        assert_eq!(false, pages.insert(10, 20));
        assert_eq!(Some(&20), pages.get(&10));
    }

    #[test]
    fn contains_must_return_false_if_pages_empty() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(false, pages.contains(&10));
    }

    #[test]
    fn contains_must_return_false_if_value_not_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(false, pages.contains(&9));
        assert_eq!(false, pages.contains(&11));
        assert_eq!(false, pages.contains(&17));
        assert_eq!(false, pages.contains(&25));
    }

    #[test]
    fn contains_must_return_true_if_value_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(true, pages.contains(&10));
        assert_eq!(true, pages.contains(&12));
        assert_eq!(true, pages.contains(&18));
        assert_eq!(true, pages.contains(&20));
    }

    #[test]
    fn remove_must_return_false_if_pages_empty() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(false, pages.remove(&10));
    }

    #[test]
    fn remove_must_return_false_if_value_not_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(false, pages.remove(&9));
        assert_eq!(false, pages.remove(&11));
        assert_eq!(false, pages.remove(&17));
        assert_eq!(false, pages.remove(&25));
    }

    #[test]
    fn remove_must_return_true_if_value_exists() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        for index in (10..=20).step_by(2) {
            pages.insert(index, 0);
        }

        assert_eq!(true, pages.remove(&10));
        assert_eq!(true, pages.remove(&12));
        assert_eq!(true, pages.remove(&18));
        assert_eq!(true, pages.remove(&20));
        assert_eq!(2, pages.size());
    }

    #[test]
    fn size_must_return_number_of_elements() {
        let config = Rc::new(Config::default().set_max_page_size(3));
        let mut pages: Pages<usize, usize> = Pages::new(config);

        assert_eq!(0, pages.size());

        for index in 0..10 {
            pages.insert(index, index);
            assert_eq!(index + 1, pages.size());
        }

        for index in 0..10 {
            pages.insert(index, index);
            assert_eq!(10, pages.size());
        }
    }
}
