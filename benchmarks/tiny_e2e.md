| group | stark_prove_excluding_trace_time_ms | total_cells_used | trace_gen_time_ms |
| --- | --- | --- | --- |
| fibonacci_program_inner | 424.0 <span style="color: red">(+37.0 [+9.6%])</span> | 134,996 | 0.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | ByteXor | 65,536 |
| fibonacci_program_inner | Core | 6 |
| fibonacci_program_inner | FieldArithmetic | 90 |
| fibonacci_program_inner | Memory | 35 |
| fibonacci_program_inner | Program | 97 |
| fibonacci_program_inner | RangeChecker | 65,536 |

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

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | ProgramAir<BabyBear> | 4 | 1 | 1 |
| fibonacci_program_inner | CoreAir | 115 | 19 | 2 |
| fibonacci_program_inner | FieldArithmeticAir | 28 | 15 | 2 |
| fibonacci_program_inner | XorLookupAir<8> | 4 | 1 | 1 |
| fibonacci_program_inner | MemoryAuditAir | 21 | 6 | 2 |
| fibonacci_program_inner | VariableRangeCheckerAir | 4 | 1 | 1 |
| fibonacci_program_inner | VmConnectorAir | 4 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/tiny_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6cc64c16ae4da96af21aafae143baf9ef88f23c3
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11263954450)
