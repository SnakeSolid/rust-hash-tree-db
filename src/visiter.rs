use std::fmt::Debug;

pub trait HashTreeVisiter<H, K, V> {
    fn visit_hash_before(&mut self, _hash: &H) {}
    fn visit_hash_after(&mut self, _hash: &H) {}
}

pub trait TreeVisiter<K, V> {
    fn visit_page_before(&mut self, _index: usize, _range_start: &K, _range_end: &K) {}
    fn visit_value(&mut self, _key: &K, _value: &V) {}
    fn visit_page_after(&mut self, _index: usize, _range_start: &K, _range_end: &K) {}
}

#[derive(Default)]
pub struct PrintVisiter {}

impl<H, K, V> HashTreeVisiter<H, K, V> for PrintVisiter
where
    H: Debug,
    K: Debug,
    V: Debug,
{
    fn visit_hash_before(&mut self, hash: &H) {
        println!("partition {:?}:", hash);
    }
}

impl<K, V> TreeVisiter<K, V> for PrintVisiter
where
    K: Debug,
    V: Debug,
{
    fn visit_page_before(&mut self, index: usize, range_start: &K, range_end: &K) {
        print!(
            "    page #{} [{:?} .. {:?}]: {{ ",
            index, range_start, range_end
        );
    }

    fn visit_value(&mut self, key: &K, value: &V) {
        print!("{:?} => {:?}, ", key, value);
    }

    fn visit_page_after(&mut self, _index: usize, _range_start: &K, _range_end: &K) {
        println!("}},");
    }
}
