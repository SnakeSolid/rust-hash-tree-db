use crate::config::Config;
use crate::hasher::TrivialHasherBuilder;
use crate::pages::Pages;
use crate::visiter::TreeVisiter;
use crate::DatabaseError;
use crate::HashTreeVisiter;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct Database<H, K, V> {
    config: Rc<Config>,
    map: HashMap<H, Pages<K, V>, TrivialHasherBuilder>,
}

impl<H, K, V> Database<H, K, V>
where
    H: Eq + Hash + Debug,
    K: Ord + Clone + Default + Debug,
    V: Default + Debug,
{
    pub fn new(config: Config) -> Database<H, K, V> {
        Database {
            config: Rc::new(config),
            map: HashMap::default(),
        }
    }

    pub fn get(&mut self, hash_key: &H, tree_key: &K) -> Result<Option<&V>, DatabaseError> {
        if let Some(pages) = self.map.get(hash_key) {
            Ok(pages.get(tree_key))
        } else {
            Ok(None)
        }
    }

    pub fn put(&mut self, hash_key: H, tree_key: K, data: V) -> Result<bool, DatabaseError> {
        let pages = self
            .map
            .entry(hash_key)
            .or_insert_with(|| Pages::new(self.config.clone()));

        let replaced = pages.insert(tree_key, data);

        Ok(replaced)
    }

    pub fn contains(&mut self, hash_key: &H, tree_key: &K) -> Result<bool, DatabaseError> {
        if let Some(pages) = self.map.get(hash_key) {
            Ok(pages.contains(tree_key))
        } else {
            Ok(false)
        }
    }

    pub fn delete(&mut self, hash_key: &H, tree_key: &K) -> Result<bool, DatabaseError> {
        if let Some(pages) = self.map.get_mut(hash_key) {
            Ok(pages.remove(tree_key))
        } else {
            Ok(false)
        }
    }

    pub fn count(&mut self) -> Result<usize, DatabaseError> {
        Ok(self.map.values().map(Pages::size).sum())
    }

    pub fn visit<T>(&self, visiter: &mut T)
    where
        T: HashTreeVisiter<H, K, V> + TreeVisiter<K, V>,
    {
        for (hash, page) in &self.map {
            visiter.visit_hash_before(hash);
            page.visit(visiter);
            visiter.visit_hash_after(hash);
        }
    }
}
