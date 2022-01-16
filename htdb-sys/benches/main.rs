#[macro_use]
extern crate bencher;

use bencher::black_box;
use bencher::Bencher;
use hash_tree_db::Config;
use hash_tree_db::Database;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const N: usize = 10_000;

fn crate_database() -> Database<String, String, String> {
    let config = Config::default().set_max_page_size(128);

    Database::new(config)
}

fn generate_strings<R>(rng: &mut R, prefix: &str) -> Vec<String>
where
    R: Rng,
{
    (0..N)
        .map(|index| {
            let value: usize = rng.gen_range(0..1000);
            let width: usize = rng.gen_range(5..10);

            format!(
                "{prefix}-{value:width$}-{index}",
                prefix = prefix,
                value = value,
                width = width,
                index = index
            )
        })
        .collect()
}

fn insert_only(bench: &mut Bencher) {
    let mut database = crate_database();
    let mut rng = StdRng::from_seed([0; 32]);
    let hashes = generate_strings(&mut rng, "hash");
    let keys = generate_strings(&mut rng, "tree");
    let values = generate_strings(&mut rng, "value");

    bench.iter(|| {
        for ((hash, key), value) in hashes.iter().zip(&keys).zip(&values) {
            black_box(database.put(hash.clone(), key.clone(), value.clone())).ok();
        }
    })
}

fn insert_and_select(bench: &mut Bencher) {
    let mut database = crate_database();
    let mut rng = StdRng::from_seed([0; 32]);
    let hashes = generate_strings(&mut rng, "hash");
    let keys = generate_strings(&mut rng, "tree");
    let values = generate_strings(&mut rng, "value");

    bench.iter(|| {
        for ((hash, key), value) in hashes.iter().zip(&keys).zip(&values) {
            black_box(database.put(hash.clone(), key.clone(), value.clone())).ok();
        }

        for (hash, key) in hashes.iter().zip(&keys) {
            black_box(database.get(hash, key)).ok();
        }
    })
}

fn insert_and_delete(bench: &mut Bencher) {
    let mut database = crate_database();
    let mut rng = StdRng::from_seed([0; 32]);
    let hashes = generate_strings(&mut rng, "hash");
    let keys = generate_strings(&mut rng, "tree");
    let values = generate_strings(&mut rng, "value");

    bench.iter(|| {
        for ((hash, key), value) in hashes.iter().zip(&keys).zip(&values) {
            black_box(database.put(hash.clone(), key.clone(), value.clone()).ok());
        }

        for (hash, key) in hashes.iter().zip(&keys) {
            black_box(database.delete(hash, key)).ok();
        }
    })
}

fn insert_select_delete(bench: &mut Bencher) {
    let mut database = crate_database();
    let mut rng = StdRng::from_seed([0; 32]);
    let hashes = generate_strings(&mut rng, "hash");
    let keys = generate_strings(&mut rng, "tree");
    let values = generate_strings(&mut rng, "value");

    bench.iter(|| {
        for ((hash, key), value) in hashes.iter().zip(&keys).zip(&values) {
            black_box(database.put(hash.clone(), key.clone(), value.clone()).ok());
        }

        for (hash, key) in hashes.iter().zip(&keys) {
            black_box(database.get(hash, key)).ok();
        }

        for (hash, key) in hashes.iter().zip(&keys) {
            black_box(database.delete(hash, key)).ok();
        }
    })
}

benchmark_group!(
    benches,
    insert_only,
    insert_and_select,
    insert_and_delete,
    insert_select_delete
);
benchmark_main!(benches);
