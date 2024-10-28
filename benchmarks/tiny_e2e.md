| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | <span style="color: green">(-3.0 [-0.5%])</span> <div style='text-align: right'>633.0</div>  | <div style='text-align: right'>1,192,274</div>  | <div style='text-align: right'>134,436</div>  | <span style="color: green">(-4.0 [-0.6%])</span> <div style='text-align: right'>650.0</div>  | <span style="color: green">(-1.0 [-5.6%])</span> <div style='text-align: right'>17.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | FieldArithmetic | <div style='text-align: right'>90</div>  |
| fibonacci_program_inner | Jal | <div style='text-align: right'>1</div>  |
| fibonacci_program_inner | LoadStore | <div style='text-align: right'>4</div>  |
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
| fibonacci_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| fibonacci_program_inner | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| fibonacci_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| fibonacci_program_inner | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| fibonacci_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>900</div>  |
| fibonacci_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>1,800</div>  |
| fibonacci_program_inner | Boundary | AddFI | ADD | <div style='text-align: right'>330</div>  |
| fibonacci_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>82</div>  |
| fibonacci_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>22</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | ProgramAir | <div style='text-align: right'>2,304</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program_inner | VolatileBoundaryAir | <div style='text-align: right'>1,472</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| fibonacci_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>340</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4</div>  |
| fibonacci_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>30</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| fibonacci_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>8,448</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/828a339aeb06e531f0a67f22b47403924c469924/tiny_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/828a339aeb06e531f0a67f22b47403924c469924
AWS Instance Type: [64cpu-linux-arm64](https://instances.vantage.sh/aws/ec2/64cpu-linux-arm64)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11560343759)
