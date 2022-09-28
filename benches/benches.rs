use criterion::{black_box, criterion_group};

lazy_static::lazy_static! {
    static ref COMPLEX_JSON_1: serde_json::Value =
        serde_json::from_slice(include_bytes!("sample.json")).unwrap();
}

fn configure_group<M>(group: &mut criterion::BenchmarkGroup<M>)
where
    M: criterion::measurement::Measurement,
{
    group.sample_size(1000);
    group.sampling_mode(criterion::SamplingMode::Flat);
}

fn bench_iter_recursive(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("iter/recursive");
    configure_group(&mut group);
    let value = &*COMPLEX_JSON_1;
    group.bench_function("dfs/sequential", |b| {
        b.iter(|| {
            use serde_json_merge::{Dfs, Iter};
            black_box(value.clone().iter_recursive::<Dfs>().count());
        });
    });
}

#[cfg(feature = "merge")]
fn bench_merge_recursive(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("merge/recursive");
    configure_group(&mut group);
    let value = &*COMPLEX_JSON_1;
    group.bench_function("dfs/sequential", |b| {
        b.iter(|| {
            use serde_json_merge::{Dfs, Merge};
            black_box(value.clone()).merge_recursive::<Dfs>(value);
        });
    });
}

#[cfg(feature = "sort")]
fn bench_sort_recursive(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("sort/recursive");
    configure_group(&mut group);
    let value = &*COMPLEX_JSON_1;
    group.bench_function("dfs/sequential", |b| {
        b.iter(|| {
            use serde_json_merge::{Dfs, Sort};
            black_box(value.clone()).sort_recursive::<Dfs>();
        });
    });

    // #[cfg(feature = "rayon")]
    // group.bench_function(
    //     format!("dfs/parallel ({} threads)", rayon::current_num_threads()),
    //     |b| {
    //         b.iter(|| {
    //             use rayon::iter::{ParallelBridge, ParallelIterator};
    //             use serde_json_merge::{Dfs, Sort, iter::Traverser};
    //             let traverser = Dfs::new();
    //             traverser.set_depth(None);
    //             traverser.set_limit(None);
    //             traverser.into_par_iter().map(|test: String| {
    //                 dbg!(test);
    //             });
    //             // black_box(value.clone().sorted_recursive::<Dfs>());
    //             // self.mutate_recursive::<T>()
    //             //     .for_each(|idx: &IndexPath, val: &mut Value| {
    //             //         val.sort_keys_by(&mut |ak, av, bk, bv| {
    //             //             let ak = idx.clone().join(ak);
    //             //             let bk = idx.clone().join(bk);
    //             //             cmp(&ak, av, &bk, bv)
    //             //         });
    //             //     });

    //             // iter.clone().par_bridge().count()
    //         })
    //     },
    // );

    // #[cfg(feature = "rayon")]
    // group.bench_function(
    //     format!("parallel bridge ({} threads)", rayon::current_num_threads()),
    //     |b| {
    //         b.iter(|| {
    //             use rayon::iter::{ParallelBridge, ParallelIterator};
    //             iter.clone().par_bridge().count()
    //         })
    //     },
    // );
}

criterion_group!(bench_iter, bench_iter_recursive);

#[cfg(feature = "merge")]
criterion_group!(bench_merge, bench_merge_recursive);

#[cfg(feature = "sort")]
criterion_group!(bench_sort, bench_sort_recursive);

fn main() {
    bench_iter();

    #[cfg(feature = "sort")]
    bench_sort();
    #[cfg(feature = "merge")]
    bench_merge();

    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
