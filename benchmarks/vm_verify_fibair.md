## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 77_418_496 | 16100.00 | 2250.00 | 153.00 | 2190.00 | 11507.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 52_428_800  | 0     | [80] | [20] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| FieldArithmeticAir   | 262_144    | 14_942_208  | 0     | [41] | [16] |
| FieldExtensionArithmeticAir | 8_192      | 1_261_568   | 0     | [106] | [48] |
| Poseidon2VmAir       | 4_096      | 3_215_360   | 0     | [661] | [124] |
| MemoryAuditAir       | 131_072    | 4_325_376   | 0     | [21] | [12] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|------:|
| Cpu                  | `        496_082` |
| FieldArithmetic      | `        164_584` |
| FieldExtension       | `          7_914` |
| Memory               | `        107_120` |
| Poseidon2            | `          3_309` |
| Program              | `         54_724` |
| RangeChecker         | `         65_536` |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|------:|-----:|
| FADD                 | `        134_399` | `     16_335_317` |
| BNE                  | `         75_347` | `      6_027_760` |
| STOREW               | `         74_001` | `      6_864_093` |
| LOADW                | `         49_216` | `      4_019_285` |
| LOADW2               | `         38_007` | `      3_044_256` |
| SHINTW               | `         33_232` | `      3_356_432` |
| STOREW2              | `         21_346` | `      2_016_842` |
| FMUL                 | `         20_715` | `      2_573_337` |
| JAL                  | `         12_981` | `      1_038_501` |
| FSUB                 | `          9_467` | `      1_214_366` |
| HINT_INPUT           | `          4_769` | `        381_520` |
| CT_END               | `          3_921` | `        313_680` |
| CT_START             | `          3_921` | `        313_680` |
| BBE4MUL              | `          3_759` | `        701_946` |
| BEQ                  | `          3_429` | `        274_320` |
| COMP_POS2            | `          2_678` | `      1_984_398` |
| FE4ADD               | `          1_678` | `        313_116` |
| BBE4DIV              | `          1_239` | `        230_538` |
| FE4SUB               | `          1_238` | `        230_436` |
| PERM_POS2            | `            631` | `        467_571` |
| HINT_BITS            | `            104` | `          8_320` |
| FDIV                 | `              3` | `            363` |
| TERMINATE            | `              1` | `             80` |

### DSL counts
How many opcodes each DSL instruction generates:
| Name | Count |
|------|------:|
| For                  | `        117_162` |
| StoreHintWord        | `         58_471` |
| AddVI                | `         39_783` |
| Alloc                | `         39_094` |
| StoreE               | `         37_932` |
| LoadV                | `         30_112` |
| LoadE                | `         19_400` |
| LoadF                | `         17_279` |
| IfEqI                | `         14_637` |
| StoreV               | `         13_846` |
| ImmV                 | `         13_022` |
| StoreF               | `         10_959` |
| ImmF                 | `          7_240` |
| SubEF                | `          6_612` |
| AddEI                | `          6_244` |
| AssertEqF            | `          5_048` |
| HintInputVec         | `          4_769` |
| CycleTrackerEnd      | `          3_921` |
| CycleTrackerStart    | `          3_921` |
| SubVI                | `          3_900` |
| MulE                 | `          3_726` |
| AssertEqV            | `          3_640` |
| SubV                 | `          3_502` |
| AddFI                | `          3_354` |
| MulVI                | `          3_300` |
| MulV                 | `          3_224` |
| IfNe                 | `          2_817` |
| MulF                 | `          2_682` |
| Poseidon2CompressBabyBear | `          2_678` |
| AddV                 | `          2_274` |
| ImmE                 | `          2_068` |
| AddE                 | `          1_678` |
| MulEF                | `          1_656` |
| DivE                 | `          1_238` |
| SubE                 | `          1_238` |
| IfEq                 | `            743` |
| Poseidon2PermuteBabyBear | `            631` |
| IfNeI                | `            619` |
| AddEFFI              | `            524` |
| AssertEqE            | `            416` |
| SubVIN               | `            412` |
| MulEI                | `            165` |
| HintBitsF            | `            104` |
| AssertEqVI           | `             16` |
| SubEI                | `              8` |
| DivEIN               | `              5` |
| AssertEqEI           | `              4` |
| DivFIN               | `              3` |
| Halt                 | `              1` |
| MulFI                | `              1` |
</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/4aec0b72303d3fc6e328f26ea0fee0e5e8a4610e
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10726896219)
