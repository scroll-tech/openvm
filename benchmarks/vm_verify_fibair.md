## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 222_380_192 | 53000.00 | 5410.00 | 348.00 | 6400.00 | 40842.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir<1>            | 1_048_576  | 66_060_288  | 0     | [51] | [12] |
| BabyBear>            | 16_384     | 147_456     | 7     | [1] | [8] |
| MemoryOfflineChecker | 2_097_152  | 142_606_336 | 0     | [48] | [20] |
| RangeCheckerGateAir  | 16         | 160         | 0     | [2] | [8] |
| FieldArithmeticAir   | 524_288    | 11_010_048  | 0     | [13] | [8] |
| FieldExtensionArithmeticAir | 16_384     | 868_352     | 0     | [37] | [16] |
| BabyBear>            | 4_096      | 1_687_552   | 0     | [380] | [32] |

### Custom metrics
| Name | Value |
|------|-------|
| cpu_cycles           | 580920     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 292579     |
| field_extension_ops  | 10898      |
| memory_chip_accesses | 1522930    |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 16         |

### Opcode counts
| Name | Count |
|------|-------|
| TERMINATE            | 1          |
| FDIV                 | 3          |
| HINT_BITS            | 104        |
| PERM_POS2            | 631        |
| BBE4INV              | 1239       |
| FE4ADD               | 1678       |
| COMP_POS2            | 2678       |
| FE4SUB               | 2891       |
| BEQ                  | 3429       |
| CT_END               | 3921       |
| CT_START             | 3921       |
| HINT_INPUT           | 4769       |
| BBE4MUL              | 5090       |
| FSUB                 | 7816       |
| JAL                  | 12562      |
| SHINTW               | 33232      |
| FMUL                 | 48656      |
| STOREW               | 62738      |
| LOADW                | 66793      |
| BNE                  | 82664      |
| FADD                 | 236104     |

### DSL counts - how many isa instructions each DSL instruction generates
| Name | Count |
|------|-------|
| Halt                 | 1          |
| MulFI                | 1          |
| AssertEqEI           | 4          |
| DivEIN               | 6          |
| DivFIN               | 6          |
| SubEI                | 8          |
| AssertEqVI           | 16         |
| HintBitsF            | 104        |
| MulEI                | 165        |
| AssertEqE            | 416        |
| IfNeI                | 618        |
| Poseidon2PermuteBabyBear | 631        |
| IfEq                 | 743        |
| SubVIN               | 824        |
| ImmE                 | 1656       |
| AddE                 | 1678       |
| MulF                 | 2038       |
| AddFI                | 2129       |
| DivE                 | 2476       |
| Poseidon2CompressBabyBear | 2678       |
| AddV                 | 2689       |
| IfNe                 | 2818       |
| SubE                 | 2891       |
| MulVI                | 3094       |
| MulV                 | 3430       |
| SubV                 | 3502       |
| AssertEqV            | 3640       |
| MulE                 | 3818       |
| SubVI                | 3902       |
| CycleTrackerEnd      | 3921       |
| CycleTrackerStart    | 3921       |
| HintInputVec         | 4769       |
| AssertEqF            | 5048       |
| AddEI                | 5420       |
| ImmF                 | 7034       |
| AddEFFI              | 8792       |
| IfEqI                | 13597      |
| ImmV                 | 13643      |
| StoreF               | 21625      |
| LoadF                | 21698      |
| LoadE                | 26610      |
| AddVI                | 27829      |
| StoreV               | 35880      |
| Alloc                | 39094      |
| StoreE               | 40412      |
| StoreHintWord        | 58471      |
| LoadV                | 64754      |
| For                  | 132417     |

### Opcode trace cells
| Name | Count |
|------|-------|
| FDIV                 | 717        |
| HINT_BITS            | 6032       |
| CT_END               | 227418     |
| CT_START             | 227418     |
| HINT_INPUT           | 276602     |
| BEQ                  | 498402     |
| BBE4INV              | 593481     |
| FE4ADD               | 1125938    |
| PERM_POS2            | 1306170    |
| JAL                  | 1331572    |
| FSUB                 | 1660952    |
| FE4SUB               | 1939861    |
| BBE4MUL              | 3415390    |
| COMP_POS2            | 4643652    |
| SHINTW               | 5117728    |
| FMUL                 | 9555760    |
| BNE                  | 11800688   |
| STOREW               | 12672980   |
| LOADW                | 13492186   |
| TERMINATE            | 27124106   |
| FADD                 | 49387064   |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/d3ea42b4dade3b458e2a87807a144e1e53d59f3f
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10275495810)
