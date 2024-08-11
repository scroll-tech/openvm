## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 181_092_352 | 46500.00 | 5330.00 | 226.00 | 5030.00 | 35914.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir<1>            | 1_048_576  | 66_060_288  | 0     | [51] | [12] |
| BabyBear>            | 16_384     | 147_456     | 7     | [1] | [8] |
| MemoryOfflineChecker | 2_097_152  | 100_663_296 | 0     | [36] | [12] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
| FieldArithmeticAir   | 524_288    | 11_010_048  | 0     | [13] | [8] |
| FieldExtensionArithmeticAir | 16_384     | 868_352     | 0     | [37] | [16] |
| BabyBear>            | 4_096      | 1_687_552   | 0     | [380] | [32] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|-------|
| cpu_cycles           | 565503     |
| cpu_timestamp        | 0          |
| field_arithmetic_ops | 284994     |
| field_extension_ops  | 10898      |
| is_less_than_ops     | 0          |
| memory_chip_accesses | 1493333    |
| poseidon2_chip_rows  | 3309       |
| range_checker_count  | 65536      |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|-------|-----|
| FADD                 | 228525     | 41144619   |
| BNE                  | 75452      | 9371216    |
| LOADW                | 66793      | 11087638   |
| STOREW               | 62738      | 10414436   |
| FMUL                 | 48652      | 8321732    |
| SHINTW               | 33232      | 4320160    |
| JAL                  | 11943      | 1122642    |
| FSUB                 | 7814       | 1431010    |
| BBE4MUL              | 5090       | 2682430    |
| HINT_INPUT           | 4769       | 276602     |
| CT_END               | 3921       | 227418     |
| CT_START             | 3921       | 227418     |
| BEQ                  | 3429       | 423522     |
| FE4SUB               | 2891       | 1523557    |
| COMP_POS2            | 2678       | 3775980    |
| FE4ADD               | 1678       | 884306     |
| BBE4INV              | 1239       | 474537     |
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
| AddEFFI              | 8792       |
| ImmF                 | 7034       |
| AddEI                | 5420       |
| AssertEqF            | 5048       |
| HintInputVec         | 4769       |
| CycleTrackerEnd      | 3921       |
| CycleTrackerStart    | 3921       |
| SubVI                | 3900       |
| MulE                 | 3818       |
| AssertEqV            | 3640       |
| SubV                 | 3502       |
| MulV                 | 3430       |
| MulVI                | 3094       |
| SubE                 | 2891       |
| IfNe                 | 2818       |
| AddV                 | 2689       |
| Poseidon2CompressBabyBear | 2678       |
| DivE                 | 2476       |
| AddFI                | 2129       |
| MulF                 | 2038       |
| AddE                 | 1678       |
| ImmE                 | 1656       |
| SubVIN               | 824        |
| IfEq                 | 743        |
| Poseidon2PermuteBabyBear | 631        |
| IfNeI                | 618        |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/c50053c8d22b924abc4fe7d4effc37a626e42ee1
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10341304880)
