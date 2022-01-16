use crate::config::Config;
use crate::hasher::TrivialHasherBuilder;
use crate::pages::Pages;
use crate::visiter::TreeVisiter;
use crate::DatabaseError;
use crate::HashTreeVisiter;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::BufWriter;
use std::rc::Rc;

#[derive(Debug)]
pub struct Database<H, K, V>
where
    K: Ord,
{
    config: Rc<Config>,
    map: HashMap<H, Pages<K, V>, TrivialHasherBuilder>,
}

impl<H, K, V> Database<H, K, V>
where
    H: Eq + Hash + Serialize + DeserializeOwned + Debug,
    K: Ord + Clone + Default + Serialize + DeserializeOwned + Debug,
    V: Default + Serialize + DeserializeOwned + Debug,
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

    pub fn save(&mut self) -> Result<(), DatabaseError> {
        let path = self.config.storage_path().join("full.htdb");
        let file = File::create(path).map_err(DatabaseError::create_file_error)?;
        let mut writer = BufWriter::new(file);

        bincode::serialize_into(&mut writer, &self.map).map_err(DatabaseError::serialize_error)?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), DatabaseError> {
        let path = self.config.storage_path().join("full.htdb");
        let file = File::open(path).map_err(DatabaseError::open_file_error)?;
        let mut reader = BufReader::new(file);

        self.map.clear();

        let data: HashMap<H, Pages<K, V>> =
            bincode::deserialize_from(&mut reader).map_err(DatabaseError::serialize_error)?;

        self.map.extend(data.into_iter());

        Ok(())
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
