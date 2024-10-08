| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | 392.0 <span style="color: green">(-319.0 [-44.9%])</span> | 1,192,484 <span style="color: green">(-589,824 [-33.1%])</span> | 134,996 <span style="color: green">(-65,536 [-32.7%])</span> | 0.0 <span style="color: green">(-1.0 [-100.0%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | ByteXor | 65,536 |
| fibonacci_program_inner | Core | 6 |
| fibonacci_program_inner | FieldArithmetic | 90 |
| fibonacci_program_inner | Memory | 35 |
| fibonacci_program_inner | Program | 97 |
| fibonacci_program_inner | RangeChecker | 65,536 <span style="color: green">(-65,536 [-50.0%])</span> |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner |  | JAL | 1 |
| fibonacci_program_inner |  | STOREW | 2 |
| fibonacci_program_inner | AddF | ADD | 30 |
| fibonacci_program_inner | AddFI | ADD | 60 |
| fibonacci_program_inner | Halt | TERMINATE | 1 |
| fibonacci_program_inner | ImmF | STOREW | 2 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | Audit |  | JAL | 19 |
| fibonacci_program_inner | CoreAir |  | JAL | 62 |
| fibonacci_program_inner | Audit |  | STOREW | 38 |
| fibonacci_program_inner | CoreAir |  | STOREW | 124 |
| fibonacci_program_inner | FieldArithmeticAir | AddF | ADD | 930 |
| fibonacci_program_inner | Audit | AddFI | ADD | 570 |
| fibonacci_program_inner | FieldArithmeticAir | AddFI | ADD | 1,860 |
| fibonacci_program_inner | CoreAir | Halt | TERMINATE | 62 |
| fibonacci_program_inner | Audit | ImmF | STOREW | 38 |
| fibonacci_program_inner | CoreAir | ImmF | STOREW | 124 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | ProgramAir<BabyBear> | 1,152 | 4 | 1 | 1 | 8 | 9 | 1 | 128 |
| fibonacci_program_inner | CoreAir | 848 | 115 | 19 | 62 | 44 |  | 2 | 8 |
| fibonacci_program_inner | FieldArithmeticAir | 8,576 | 28 | 15 | 31 | 36 |  | 2 | 128 |
| fibonacci_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| fibonacci_program_inner | MemoryAuditAir | 2,240 | 21 | 6 | 19 | 16 |  | 2 | 64 |
| fibonacci_program_inner | VariableRangeCheckerAir | 589,824 <span style="color: green">(-589,824 [-50.0%])</span> | 4 | 1 | 1 | 8 | 2 | 1 | 65,536 <span style="color: green">(-65,536 [-50.0%])</span> |
| fibonacci_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a/tiny_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/21140b5f3cca2c72375fe4c9b3d2e1a28dddf36a
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11236326343)
