| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | <span style="color: red">(+50.0 [+0.5%])</span> <div style='text-align: right'>9,538.0</div>  | <div style='text-align: right'>20,067,876</div>  | <div style='text-align: right'>2,166,425</div>  | <span style="color: red">(+51.0 [+0.5%])</span> <div style='text-align: right'>9,606.0</div>  | <span style="color: red">(+1.0 [+1.5%])</span> <div style='text-align: right'>68.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| fibonacci_program_inner | Core | <div style='text-align: right'>6</div>  |
| fibonacci_program_inner | FieldArithmetic | <div style='text-align: right'>90</div>  |
| fibonacci_program_inner | Memory Boundary | <div style='text-align: right'>35</div>  |
| fibonacci_program_inner | ProgramChip | <div style='text-align: right'>97</div>  |
| fibonacci_program_inner | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| fibonacci_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| fibonacci_program_inner | AddF | ADD | <div style='text-align: right'>30</div>  |
| fibonacci_program_inner | AddFI | ADD | <div style='text-align: right'>60</div>  |
| fibonacci_program_inner | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| fibonacci_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| fibonacci_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| fibonacci_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| fibonacci_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| fibonacci_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>900</div>  |
| fibonacci_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>1,800</div>  |
| fibonacci_program_inner | Boundary | AddFI | ADD | <div style='text-align: right'>570</div>  |
| fibonacci_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| fibonacci_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| fibonacci_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>124</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | ProgramAir | <div style='text-align: right'>2,304</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | CoreAir | <div style='text-align: right'>848</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| fibonacci_program_inner | VmAirWrapper<NativeAdapterAir, FieldArithmeticCoreAir> | <div style='text-align: right'>8,448</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| fibonacci_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program_inner | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| fibonacci_program_inner | VolatileBoundaryAir | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| fibonacci_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ad2a6a965a4a4b5920701eef08d49bef90cdfc01/tiny_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/ad2a6a965a4a4b5920701eef08d49bef90cdfc01
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11356528357)
