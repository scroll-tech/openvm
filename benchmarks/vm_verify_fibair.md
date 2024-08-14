## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 175_587_328 | 45100.00 | 5260.00 | 222.00 | 5070.00 | 34548.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir<1>            | 1_048_576  | 66_060_288  | 0     | [51] | [12] |
| BabyBear>            | 16_384     | 147_456     | 7     | [1] | [8] |
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
| cpu_cycles           | 563436     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 231945     |
| field_extension_ops  | 8831       |
| is_less_than_ops     | 0          |
| memory_chip_accesses | 1439222    |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 65536      |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 172167     | 31732833   |
| STOREW               | 95354      | 13480340   |
| LOADW                | 87226      | 13743928   |
| BNE                  | 75452      | 9371216    |
| FMUL                 | 50308      | 8657900    |
| SHINTW               | 33232      | 4320160    |
| JAL                  | 11943      | 1122642    |
| FSUB                 | 9467       | 1766569    |
| HINT_INPUT           | 4769       | 276602     |
| BBE4MUL              | 4676       | 2464252    |
| CT_END               | 3921       | 227418     |
| CT_START             | 3921       | 227418     |
| BEQ                  | 3429       | 423522     |
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
| For                  | 117374     |
| LoadV                | 64754      |
| StoreHintWord        | 58471      |
| StoreE               | 40412      |
| Alloc                | 39094      |
| StoreV               | 35876      |
| AddVI                | 34058      |
| LoadE                | 26610      |
| LoadF                | 21698      |
| StoreF               | 15029      |
| ImmV                 | 13643      |
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
| MulV                 | 3430       |
| MulE                 | 3404       |
| MulVI                | 3094       |
| IfNe                 | 2818       |
| AddV                 | 2689       |
| Poseidon2CompressBabyBear | 2678       |
| DivE                 | 2476       |
| AddFI                | 2129       |
| MulF                 | 2038       |
| AddE                 | 1678       |
| ImmE                 | 1656       |
| MulEF                | 1656       |
| SubE                 | 1238       |
| SubVIN               | 824        |
| IfEq                 | 743        |
| Poseidon2PermuteBabyBear | 631        |
| IfNeI                | 618        |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/d11b6c2b96d4faab6b369e0b522f43310e335876
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10380273223)
