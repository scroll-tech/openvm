| group | app.proof_time_ms | app.cycles | app.cells_used | leaf.proof_time_ms | leaf.cycles | leaf.cells_used |
| -- | -- | -- | -- | -- | -- | -- |
| [verify_fibair](https://github.com/openvm-org/openvm/blob/benchmark-results/benchmarks-dispatch/refs/heads/feat/optimize-for-loop/verify_fibair-24155b5d18d2f00285398a1aba4f49379380ccd4.md) |<span style='color: green'>(-128 [-3.3%])</span> 3,709 | <span style='color: red'>(+63261 [+8.5%])</span> 809,358 | <span style='color: green'>(-955553 [-3.2%])</span> 29,053,721 |- | - | - |
| [fibonacci_program](https://github.com/openvm-org/openvm/blob/benchmark-results/benchmarks-dispatch/refs/heads/feat/optimize-for-loop/fibonacci-24155b5d18d2f00285398a1aba4f49379380ccd4.md) |<span style='color: red'>(+30 [+0.5%])</span> 6,059 |  1,500,137 |  51,505,102 |<span style='color: green'>(-30 [-0.2%])</span> 14,765 | <span style='color: red'>(+301635 [+9.5%])</span> 3,473,649 | <span style='color: green'>(-2234103 [-1.7%])</span> 126,631,384 |
| [regex_program](https://github.com/openvm-org/openvm/blob/benchmark-results/benchmarks-dispatch/refs/heads/feat/optimize-for-loop/regex-24155b5d18d2f00285398a1aba4f49379380ccd4.md) |<span style='color: green'>(-23 [-0.1%])</span> 18,491 |  4,190,904 |  165,028,173 |<span style='color: red'>(+926 [+3.0%])</span> 31,318 | <span style='color: red'>(+191528 [+2.9%])</span> 6,714,321 | <span style='color: green'>(-14843309 [-5.1%])</span> 276,453,830 |
| [fib_e2e](https://github.com/openvm-org/openvm/blob/benchmark-results/benchmarks-dispatch/refs/heads/feat/optimize-for-loop/fib_e2e-24155b5d18d2f00285398a1aba4f49379380ccd4.md) | 43,140 |  12,000,137 |  410,820,430 | 89,465 |  20,855,361 |  756,878,809 |
| [ecrecover_program](https://github.com/openvm-org/openvm/blob/benchmark-results/benchmarks-dispatch/refs/heads/feat/optimize-for-loop/ecrecover-24155b5d18d2f00285398a1aba4f49379380ccd4.md) | 2,627 |  285,401 |  15,092,297 |<span style='color: red'>(+11830 [+28.1%])</span> 53,902 | <span style='color: red'>(+76551 [+0.8%])</span> 9,739,837 | <span style='color: green'>(-27530505 [-6.2%])</span> 413,026,711 |


Commit: https://github.com/openvm-org/openvm/commit/24155b5d18d2f00285398a1aba4f49379380ccd4

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12685715218)
