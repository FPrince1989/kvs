use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use tempfile::TempDir;

use kvs::{KvsEngine, SharedKvStore};

fn bench_kvs_set(c: &mut Criterion) {
    let mut rng = thread_rng();
    let temp_dir = TempDir::new().unwrap();
    let mut kvs = SharedKvStore::open(temp_dir.path()).unwrap();
    // let mut kvs = KvStore::open(&current_dir().unwrap()).unwrap();
    c.bench_function("kvs_write", move |b| {
        b.iter(|| {
            let key = rng.gen_range(1, 1_000_000);
            let value = rng.gen_range(1, 1_000_000);
            kvs.set(format!("key{}", key), format!("value{}", value))
                .unwrap();
        })
    });
}

criterion_group!(benches, bench_kvs_set);

criterion_main!(benches);
