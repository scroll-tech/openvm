[1;32mINFO    [0m prove halo2 verifier circuit [ 128s | 100.00% ] step: "static_verifier_prove"
[1;32mINFO    [0m ┝━ ｉ [info]: Poisoned rows after FlexGateConfig::configure 9 | log.target: "halo2_base::gates::flex_gate" | log.module_path: "halo2_base::gates::flex_gate" | log.file: "/.cargo/git/checkouts/halo2-lib-d11b5da38eeddd90/2fe813b/halo2-base/src/gates/flex_gate/mod.rs" | log.line: 136
[1;32mINFO    [0m ┕━ ｉ [info]: Poisoned rows after RangeConfig::configure 9 | log.target: "halo2_base::gates::range" | log.module_path: "halo2_base::gates::range" | log.file: "/.cargo/git/checkouts/halo2-lib-d11b5da38eeddd90/2fe813b/halo2-base/src/gates/range/mod.rs" | log.line: 101
test test_fibonacci_program_halo2_verify ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 357.63s


Commit: https://github.com/axiom-crypto/afs-prototype/commit/9e43903f7d272b6a21b5338ac9eda06405c9507f
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11210001762)
