use ax_stark_sdk::p3_baby_bear::BabyBear;
use axvm_benchmarks::utils::build_bench_program;
use axvm_circuit::arch::{instructions::exe::AxVmExe, VmExecutor};
use axvm_keccak256_circuit::Keccak256Rv32Config;
use axvm_keccak256_transpiler::Keccak256TranspilerExtension;
use axvm_rv32im_transpiler::{
    Rv32ITranspilerExtension, Rv32IoTranspilerExtension, Rv32MTranspilerExtension,
};
use axvm_sdk::StdIn;
use axvm_transpiler::{transpiler::Transpiler, FromElf};
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn benchmark_function(c: &mut Criterion) {
    let elf = build_bench_program("regex").unwrap();
    let exe = AxVmExe::from_elf(
        elf,
        Transpiler::<BabyBear>::default()
            .with_extension(Rv32ITranspilerExtension)
            .with_extension(Rv32MTranspilerExtension)
            .with_extension(Rv32IoTranspilerExtension)
            .with_extension(Keccak256TranspilerExtension),
    )
    .unwrap();

    let mut group = c.benchmark_group("regex");
    group.sample_size(10);
    let config = Keccak256Rv32Config::default();
    let executor = VmExecutor::<BabyBear, Keccak256Rv32Config>::new(config);

    let data = include_str!("../programs/regex/regex_email.txt");

    let fe_bytes = data.to_owned().into_bytes();
    group.bench_function("execute", |b| {
        b.iter(|| {
            executor
                .execute(exe.clone(), StdIn::from_bytes(&fe_bytes))
                .unwrap();
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(10, Output::Flamegraph(None)));
    targets = benchmark_function
}
criterion_main!(benches);
