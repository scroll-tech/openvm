## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 84_328_448 | 16700.00 | 2840.00 | 144.00 | 2390.00 | 11326.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 56_098_816  | 0     | [87] | [20] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| MemoryAuditAir       | 131_072    | 4_587_520   | 0     | [23] | [12] |
| FieldArithmeticAir   | 262_144    | 15_990_784  | 0     | [45] | [16] |
| FieldExtensionArithmeticAir | 16_384     | 3_047_424   | 0     | [138] | [48] |
| Poseidon2VmAir       | 4_096      | 3_358_720   | 0     | [696] | [124] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|-------|

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 134354     | 6125924    |
| BNE                  | 75347      | 0          |
| STOREW               | 74001      | 1034011    |
| LOADW                | 49216      | 89815      |
| LOADW2               | 38007      | 4048       |
| SHINTW               | 33232      | 764336     |
| STOREW2              | 21346      | 338606     |
| FMUL                 | 20715      | 1005361    |
| JAL                  | 12839      | 23         |
| FSUB                 | 9467       | 501432     |
| BBE4MUL              | 4998       | 692852     |
| HINT_INPUT           | 4769       | 0          |
| CT_END               | 3921       | 0          |
| CT_START             | 3921       | 0          |
| BEQ                  | 3429       | 0          |
| COMP_POS2            | 2678       | 1863888    |
| FE4ADD               | 1678       | 232668     |
| BBE4INV              | 1239       | 170982     |
| FE4SUB               | 1238       | 171028     |
| PERM_POS2            | 631        | 439176     |
| HINT_BITS            | 104        | 0          |
| FDIV                 | 3          | 135        |
| TERMINATE            | 1          | 0          |

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
| IfEqI                | 14495      |
| StoreV               | 13846      |
| ImmV                 | 13022      |
| StoreF               | 10959      |
| ImmF                 | 7240       |
| SubEF                | 6612       |
| AddEI                | 6244       |
| AssertEqF            | 5048       |
| HintInputVec         | 4769       |
| CycleTrackerEnd      | 3921       |
| CycleTrackerStart    | 3921       |
| SubVI                | 3900       |
| MulE                 | 3726       |
| AssertEqV            | 3640       |
| SubV                 | 3502       |
| AddFI                | 3309       |
| MulVI                | 3300       |
| MulV                 | 3224       |
| IfNe                 | 2817       |
| MulF                 | 2682       |
| Poseidon2CompressBabyBear | 2678       |
| DivE                 | 2476       |
| AddV                 | 2274       |
| ImmE                 | 2068       |
| AddE                 | 1678       |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/62f2e990f10d4885fd553e5606c9178e200d2bbc
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10567916062)
