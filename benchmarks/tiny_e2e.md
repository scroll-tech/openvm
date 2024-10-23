| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | <span style="color: red">(+2.0 [+0.3%])</span> <div style='text-align: right'>623.0</div>  | <span style="color: red">(+4 [+0.0%])</span> <div style='text-align: right'>1,193,504</div>  | <span style="color: green">(-66 [-1.8%])</span> <div style='text-align: right'>3,665</div>  | <span style="color: red">(+2.0 [+0.3%])</span> <div style='text-align: right'>644.0</div>  | <div style='text-align: right'>21.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | Core | <div style='text-align: right'>5</div>  |
| fibonacci_program_inner | FieldArithmetic | <div style='text-align: right'>90</div>  |
| fibonacci_program_inner | Memory Boundary | <div style='text-align: right'>35</div>  |
| fibonacci_program_inner | ProgramChip | <div style='text-align: right'>97</div>  |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| fibonacci_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| fibonacci_program_inner | AddF | ADD | <div style='text-align: right'>30</div>  |
| fibonacci_program_inner | AddFI | ADD | <div style='text-align: right'>60</div>  |
| fibonacci_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| fibonacci_program_inner | CoreAir |  | JAL | <div style='text-align: right'>60</div>  |
| fibonacci_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| fibonacci_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>120</div>  |
| fibonacci_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>900</div>  |
| fibonacci_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>1,800</div>  |
| fibonacci_program_inner | Boundary | AddFI | ADD | <div style='text-align: right'>570</div>  |
| fibonacci_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| fibonacci_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>120</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | ProgramAir | <div style='text-align: right'>2,304</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | VmConnectorAir | <span style="color: red">(+12 [+60.0%])</span> <div style='text-align: right'>32</div>  | <span style="color: red">(+2 [+33.3%])</span> <div style='text-align: right'>8</div>  | <span style="color: red">(+1 [+50.0%])</span> <div style='text-align: right'>3</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: red">(+4 [+50.0%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program_inner | CoreAir | <span style="color: green">(-8 [-1.0%])</span> <div style='text-align: right'>832</div>  | <span style="color: green">(-3 [-2.8%])</span> <div style='text-align: right'>104</div>  | <div style='text-align: right'>19</div>  | <span style="color: green">(-1 [-1.6%])</span> <div style='text-align: right'>60</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| fibonacci_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>8,448</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | VolatileBoundaryAir | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| fibonacci_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e00fee050cac003d83db7521aef51ca5b309fbbd/tiny_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/e00fee050cac003d83db7521aef51ca5b309fbbd
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11470408296)
