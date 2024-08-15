## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 148_242_432 | 40500.00 | 4310.00 | 170.00 | 5570.00 | 30450.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir<1>            | 524_288    | 38_273_024  | 0     | [61] | [12] |
| BabyBear>            | 65_536     | 589_824     | 9     | [1] | [8] |
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
| cpu_cycles           | 487843     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 156469     |
| field_extension_ops  | 8831       |
| is_less_than_ops     | 0          |
| memory_chip_accesses | 1301747    |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 65536      |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 126928     | 24065252   |
| BNE                  | 75347      | 10261730   |
| STOREW               | 74001      | 10824258   |
| LOADW                | 49216      | 8024896    |
| LOADW2               | 38007      | 8133498    |
| SHINTW               | 33232      | 4718944    |
| STOREW2              | 21346      | 4568044    |
| FMUL                 | 20071      | 3841757    |
| JAL                  | 11941      | 1265746    |
| FSUB                 | 9467       | 1880173    |
| HINT_INPUT           | 4769       | 333830     |
| BBE4MUL              | 4676       | 2520364    |
| CT_END               | 3921       | 274470     |
| CT_START             | 3921       | 274470     |
| BEQ                  | 3429       | 464634     |
| COMP_POS2            | 2678       | 3808116    |
| FE4ADD               | 1678       | 904442     |
| BBE4INV              | 1239       | 489405     |
| FE4SUB               | 1238       | 667282     |
| PERM_POS2            | 631        | 1056294    |
| HINT_BITS            | 104        | 7280       |
| FDIV                 | 3          | 537        |
| TERMINATE            | 1          | 70         |

### DSL counts
How many opcodes each DSL instruction generates:
| Name | Count |
|------|-------|
| For                  | 117162     |
| StoreHintWord        | 58471      |
| Alloc                | 39094      |
| StoreE               | 37932      |
| AddVI                | 34417      |
| LoadV                | 30112      |
| LoadE                | 19400      |
| LoadF                | 17279      |
| StoreV               | 13846      |
| ImmV                 | 13640      |
| IfEqI                | 13597      |
| StoreF               | 10959      |
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
| IfEq                 | 743        |
| Poseidon2PermuteBabyBear | 631        |
| IfNeI                | 619        |
| AddEFFI              | 524        |
| AssertEqE            | 416        |
| SubVIN               | 412        |
| MulEI                | 165        |
| HintBitsF            | 104        |
| AssertEqVI           | 16         |
| SubEI                | 8          |
| DivEIN               | 6          |
| AssertEqEI           | 4          |
| DivFIN               | 3          |
| Halt                 | 1          |
| MulFI                | 1          |
</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/9ac9a5e81b5f537b32d4d03b33c4253e4b79d87a
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10410721154)
