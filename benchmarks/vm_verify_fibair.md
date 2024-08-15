## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 176_029_696 | 48900.00 | 5350.00 | 232.00 | 5330.00 | 37988.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir<1>            | 1_048_576  | 66_060_288  | 0     | [51] | [12] |
| BabyBear>            | 65_536     | 589_824     | 7     | [1] | [8] |
| MemoryOfflineChecker | 2_097_152  | 100_663_296 | 0     | [36] | [12] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
| FieldArithmeticAir   | 262_144    | 5_505_024   | 0     | [13] | [8] |
| FieldExtensionArithmeticAir | 16_384     | 868_352     | 0     | [37] | [16] |
| BabyBear>            | 4_096      | 1_687_552   | 0     | [380] | [32] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|-------|
| cpu_cycles           | 562695     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 231318     |
| field_extension_ops  | 8831       |
| is_less_than_ops     | 0          |
| memory_chip_accesses | 1436916    |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 65536      |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 171746     | 31640170   |
| STOREW               | 95350      | 13479964   |
| LOADW                | 87223      | 13743466   |
| BNE                  | 75347      | 9357566    |
| FMUL                 | 50102      | 8616082    |
| SHINTW               | 33232      | 4320160    |
| JAL                  | 11941      | 1122454    |
| FSUB                 | 9467       | 1766569    |
| HINT_INPUT           | 4769       | 276602     |
| BBE4MUL              | 4676       | 2464252    |
| CT_END               | 3921       | 227418     |
| CT_START             | 3921       | 227418     |
| BEQ                  | 3429       | 423486     |
| COMP_POS2            | 2678       | 3775980    |
| FE4ADD               | 1678       | 884306     |
| BBE4INV              | 1239       | 474537     |
| FE4SUB               | 1238       | 652426     |
| PERM_POS2            | 631        | 1048722    |
| HINT_BITS            | 104        | 6032       |
| FDIV                 | 3          | 609        |
| TERMINATE            | 1          | 58         |

### DSL counts
How many opcodes each DSL instruction generates:
| Name | Count |
|------|-------|
| For                  | 117162     |
| LoadV                | 64752      |
| StoreHintWord        | 58471      |
| StoreE               | 40412      |
| Alloc                | 39094      |
| StoreV               | 35464      |
| AddVI                | 34417      |
| LoadE                | 26610      |
| LoadF                | 21698      |
| StoreF               | 15029      |
| ImmV                 | 13640      |
| IfEqI                | 13597      |
| ImmF                 | 7034       |
| SubEF                | 6612       |
| AddEI                | 5420       |
| AssertEqF            | 5048       |
| HintInputVec         | 4769       |
| CycleTrackerEnd      | 3921       |
| CycleTrackerStart    | 3921       |
| SubVI                | 3900       |
| AssertEqV            | 3640       |
| SubV                 | 3502       |
| MulE                 | 3404       |
| MulVI                | 3300       |
| MulV                 | 3224       |
| IfNe                 | 2817       |
| Poseidon2CompressBabyBear | 2678       |
| DivE                 | 2476       |
| AddV                 | 2274       |
| AddFI                | 2073       |
| MulF                 | 2038       |
| AddE                 | 1678       |
| ImmE                 | 1656       |
| MulEF                | 1656       |
| SubE                 | 1238       |
| SubVIN               | 824        |
| IfEq                 | 743        |
| Poseidon2PermuteBabyBear | 631        |
| IfNeI                | 619        |
| AddEFFI              | 524        |
| AssertEqE            | 416        |
| MulEI                | 165        |
| HintBitsF            | 104        |
| AssertEqVI           | 16         |
| SubEI                | 8          |
| DivEIN               | 6          |
| DivFIN               | 6          |
| AssertEqEI           | 4          |
| Halt                 | 1          |
| MulFI                | 1          |
</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/ac563852200f21e9a45aadd4b176dcd8ca6fbe6b
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10403465156)
