## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 78_409_728 | 15500.00 | 2940.00 | 97.60 | 2280.00 | 10182.40 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 61_341_696  | 0     | [97] | [20] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| MemoryAuditAir       | 131_072    | 4_718_592   | 0     | [24] | [12] |
| FieldArithmeticAir   | 262_144    | 4_718_592   | 0     | [10] | [8] |
| FieldExtensionArithmeticAir | 16_384     | 3_031_040   | 0     | [137] | [48] |
| Poseidon2VmAir       | 4_096      | 3_354_624   | 0     | [695] | [124] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|-------|
| cpu_cycles           | 493209     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 161835     |
| field_extension_ops  | 8831       |
| is_less_than_ops     | 0          |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 65536      |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 132294     | 15346104   |
| BNE                  | 75347      | 7986782    |
| STOREW               | 74001      | 7844106    |
| LOADW                | 49216      | 5216896    |
| LOADW2               | 38007      | 4028742    |
| SHINTW               | 33232      | 3522592    |
| STOREW2              | 21346      | 2262676    |
| FMUL                 | 20071      | 2328236    |
| JAL                  | 11941      | 1265746    |
| FSUB                 | 9467       | 1098172    |
| HINT_INPUT           | 4769       | 505514     |
| BBE4MUL              | 4676       | 1136268    |
| CT_END               | 3921       | 415626     |
| CT_START             | 3921       | 415626     |
| BEQ                  | 3429       | 363474     |
| COMP_POS2            | 2678       | 2145078    |
| FE4ADD               | 1678       | 407754     |
| BBE4INV              | 1239       | 301077     |
| FE4SUB               | 1238       | 300834     |
| PERM_POS2            | 631        | 505431     |
| HINT_BITS            | 104        | 11024      |
| FDIV                 | 3          | 348        |
| TERMINATE            | 1          | 106        |

### DSL counts
How many opcodes each DSL instruction generates:
| Name | Count |
|------|-------|
| For                  | 117162     |
| StoreHintWord        | 58471      |
| AddVI                | 39783      |
| Alloc                | 39094      |
| StoreE               | 37932      |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/3699bcab0712d159aa585f1bb92dd8d771d07667
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10495671605)
