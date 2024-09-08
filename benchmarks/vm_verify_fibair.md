## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 70_144_000 | 15800.00 | 2030.00 | 124.00 | 2190.00 | 11456.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 47_710_208  | 0     | [71] | [20] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| FieldArithmeticAir   | 262_144    | 13_107_200  | 0     | [34] | [16] |
| FieldExtensionArithmeticAir | 8_192      | 1_048_576   | 0     | [80] | [48] |
| Poseidon2VmAir       | 4_096      | 2_707_456   | 0     | [537] | [124] |
| MemoryAuditAir       | 131_072    | 4_325_376   | 0     | [21] | [12] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|------:|
| Cpu                  | `        495_895` |
| FieldArithmetic      | `        164_539` |
| FieldExtension       | `          7_914` |
| Memory               | `        107_120` |
| Poseidon2            | `          3_309` |
| Program              | `         54_724` |
| RangeChecker         | `         65_536` |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|------:|-----:|
| FADD                 | `        134_354` | `     14_180_208` |
| BNE                  | `         75_347` | `      5_349_637` |
| STOREW               | `         74_001` | `      6_198_084` |
| LOADW                | `         49_216` | `      3_576_341` |
| LOADW2               | `         38_007` | `      2_702_193` |
| SHINTW               | `         33_232` | `      3_057_344` |
| STOREW2              | `         21_346` | `      1_824_728` |
| FMUL                 | `         20_715` | `      2_241_897` |
| JAL                  | `         12_839` | `        911_590` |
| FSUB                 | `          9_467` | `      1_062_894` |
| HINT_INPUT           | `          4_769` | `        338_599` |
| CT_END               | `          3_921` | `        278_391` |
| CT_START             | `          3_921` | `        278_391` |
| BBE4MUL              | `          3_759` | `        570_381` |
| BEQ                  | `          3_429` | `        243_459` |
| COMP_POS2            | `          2_678` | `      1_628_224` |
| FE4ADD               | `          1_678` | `        254_386` |
| BBE4DIV              | `          1_239` | `        187_173` |
| FE4SUB               | `          1_238` | `        187_106` |
| PERM_POS2            | `            631` | `        383_648` |
| HINT_BITS            | `            104` | `          7_384` |
| FDIV                 | `              3` | `            315` |
| TERMINATE            | `              1` | `             71` |

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
| IfEqI                | `         14_495` |
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
| AddFI                | `          3_309` |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/142ceae47d66cdbc32f37f928e5f122866c8bb2c
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10763789010)
