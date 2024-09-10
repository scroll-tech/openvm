## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 64_495_616 | 15300.00 | 2030.00 | 124.00 | 1990.00 | 11156.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 43_515_904  | 0     | [67] | [16] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| FieldArithmeticAir   | 262_144    | 12_320_768  | 0     | [31] | [16] |
| FieldExtensionArithmeticAir | 8_192      | 884_736     | 0     | [68] | [40] |
| Poseidon2VmAir       | 4_096      | 2_465_792   | 0     | [502] | [100] |
| MemoryAuditAir       | 131_072    | 3_538_944   | 0     | [19] | [8] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|------:|
| Cpu                  | `        496_026` |
| FieldArithmetic      | `        164_600` |
| FieldExtension       | `          7_914` |
| Memory               | `        107_120` |
| Poseidon2            | `          3_309` |
| Program              | `         54_724` |
| RangeChecker         | `        131_072` |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|------:|-----:|
| FADD                 | `        134_415` | `     13_238_752` |
| BNE                  | `         75_347` | `      5_048_249` |
| STOREW               | `         74_001` | `      5_812_174` |
| LOADW                | `         49_216` | `      3_371_667` |
| LOADW2               | `         38_007` | `      2_549_813` |
| SHINTW               | `         33_232` | `      2_857_952` |
| STOREW2              | `         21_346` | `      1_709_900` |
| FMUL                 | `         20_715` | `      2_090_528` |
| JAL                  | `         12_909` | `        864_922` |
| FSUB                 | `          9_467` | `        990_067` |
| HINT_INPUT           | `          4_769` | `        319_523` |
| CT_END               | `          3_921` | `        262_707` |
| CT_START             | `          3_921` | `        262_707` |
| BBE4MUL              | `          3_759` | `        509_973` |
| BEQ                  | `          3_429` | `        229_743` |
| COMP_POS2            | `          2_678` | `      1_523_782` |
| FE4ADD               | `          1_678` | `        227_442` |
| BBE4DIV              | `          1_239` | `        167_341` |
| FE4SUB               | `          1_238` | `        167_282` |
| PERM_POS2            | `            631` | `        359_039` |
| HINT_BITS            | `            104` | `          6_968` |
| FDIV                 | `              3` | `            294` |
| TERMINATE            | `              1` | `             67` |

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
| IfEqI                | `         14_565` |
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
| AddFI                | `          3_370` |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/776ed0763cd6897dcf280d584cc022d8c8ed364b
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10796637561)
