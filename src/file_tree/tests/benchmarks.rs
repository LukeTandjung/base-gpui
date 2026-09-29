use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use crate::file_tree::{
    mutation::FileTreeBatchOperation, runtime::FileTreeRuntime, FileTreeInitialExpansion,
    FileTreePreparedInput,
};

#[test]
#[ignore = "performance harness; run with --release --ignored --nocapture"]
fn file_tree_performance_harness() {
    let sizes = if std::env::var_os("FILE_TREE_BENCH_FULL").is_some() {
        vec![10_000, 100_000, 300_000]
    } else {
        vec![10_000]
    };

    println!("file-tree benchmark sizes: {sizes:?}");
    for size in sizes {
        benchmark_size(size);
    }
    benchmark_wide_directory(if std::env::var_os("FILE_TREE_BENCH_FULL").is_some() {
        100_000
    } else {
        10_000
    });
}

fn benchmark_size(size: usize) {
    let paths = representative_paths(size);
    let presorted = FileTreePreparedInput::presorted(paths.clone()).unwrap();
    let prepared = FileTreePreparedInput::prepare(paths.clone()).unwrap();

    measure(size, "construction/raw", 1, || {
        let input = FileTreePreparedInput::prepare(paths.clone()).unwrap();
        FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[])
    });
    measure(size, "construction/prepared", 3, || {
        FileTreeRuntime::new(&prepared, FileTreeInitialExpansion::Open, &[])
    });
    measure(size, "construction/presorted", 3, || {
        FileTreeRuntime::new(&presorted, FileTreeInitialExpansion::Open, &[])
    });

    let mut runtime = FileTreeRuntime::new(&presorted, FileTreeInitialExpansion::Open, &[]);
    measure(size, "projection/visible_count", 100_000, || {
        runtime.visible_count()
    });
    let middle = runtime.visible_count() / 2;
    measure(size, "projection/materialize_64", 1_000, || {
        runtime.visible_rows(middle..middle.saturating_add(64))
    });

    measure(size, "expansion/collapse_expand", 1_000, || {
        runtime.collapse("workspace/");
        runtime.expand("workspace/")
    });

    measure(size, "search/query_cycle", 20, || {
        runtime.set_search(Some("module-042"));
        runtime.set_search(Some("generated"));
        runtime.set_search(Some("file-0007"));
        runtime.set_search(None)
    });

    let additions = (0..100)
        .map(|index| FileTreeBatchOperation::add(format!("workspace/bench-{index:03}.tmp")))
        .collect::<Vec<_>>();
    let removals = (0..100)
        .map(|index| {
            FileTreeBatchOperation::remove(
                format!("workspace/bench-{index:03}.tmp"),
                Default::default(),
            )
        })
        .collect::<Vec<_>>();
    measure(size, "mutation/add_remove_batches_100", 3, || {
        runtime.apply_batch(&additions).unwrap();
        runtime.apply_batch(&removals).unwrap()
    });
}

fn benchmark_wide_directory(size: usize) {
    let paths = (0..size)
        .map(|index| format!("wide/file-{index:07}.rs"))
        .collect::<Vec<_>>();
    let input = FileTreePreparedInput::presorted(paths).unwrap();
    let mut runtime = FileTreeRuntime::new(&input, FileTreeInitialExpansion::Open, &[]);
    let last = runtime.visible_count().saturating_sub(64);

    measure(size, "wide/visible_count", 100_000, || {
        runtime.visible_count()
    });
    measure(size, "wide/materialize_last_64", 1_000, || {
        runtime.visible_rows(last..last.saturating_add(64))
    });
    measure(size, "wide/collapse_expand", 1_000, || {
        runtime.collapse("wide/");
        runtime.expand("wide/")
    });
}

fn representative_paths(size: usize) -> Vec<String> {
    (0..size)
        .map(|index| {
            let package = index / 1_000;
            let module = (index / 100) % 10;
            let file = index % 100;
            format!("workspace/package-{package:04}/src/module-{module:03}/file-{file:04}.rs")
        })
        .collect()
}

fn measure<R>(size: usize, label: &str, iterations: usize, mut operation: impl FnMut() -> R) {
    let started = Instant::now();
    for _ in 0..iterations {
        black_box(operation());
    }
    print_measurement(size, label, iterations, started.elapsed());
}

fn print_measurement(size: usize, label: &str, iterations: usize, elapsed: Duration) {
    let nanos_per_iteration = elapsed.as_secs_f64() * 1_000_000_000. / iterations.max(1) as f64;
    println!(
        "{label:36} paths={size:>7} iterations={iterations:>7} total={elapsed:?} ns/iter={nanos_per_iteration:.2}"
    );
}
