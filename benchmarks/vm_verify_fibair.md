## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 76_894_208 | 16100.00 | 2290.00 | 133.00 | 2230.00 | 11447.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CpuAir               | 524_288    | 51_904_512  | 0     | [79] | [20] |
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
| Cpu                  | `        496_026` |
| FieldArithmetic      | `        164_600` |
| FieldExtension       | `          7_914` |
| Memory               | `        107_120` |
| Poseidon2            | `          3_309` |
| Program              | `         54_724` |
| RangeChecker         | `         65_536` |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|------:|-----:|
| FADD                 | `        134_415` | `     16_202_838` |
| BNE                  | `         75_347` | `      5_952_413` |
| STOREW               | `         74_001` | `      6_790_092` |
| LOADW                | `         49_216` | `      3_970_069` |
| LOADW2               | `         38_007` | `      3_006_249` |
| SHINTW               | `         33_232` | `      3_323_200` |
| STOREW2              | `         21_346` | `      1_995_496` |
| FMUL                 | `         20_715` | `      2_552_622` |
| JAL                  | `         12_909` | `      1_019_832` |
| FSUB                 | `          9_467` | `      1_204_899` |
| HINT_INPUT           | `          4_769` | `        376_751` |
| CT_END               | `          3_921` | `        309_759` |
| CT_START             | `          3_921` | `        309_759` |
| BBE4MUL              | `          3_759` | `        698_187` |
| BEQ                  | `          3_429` | `        270_891` |
| COMP_POS2            | `          2_678` | `      1_981_720` |
| FE4ADD               | `          1_678` | `        311_438` |
| BBE4DIV              | `          1_239` | `        229_299` |
| FE4SUB               | `          1_238` | `        229_198` |
| PERM_POS2            | `            631` | `        466_940` |
| HINT_BITS            | `            104` | `          8_216` |
| FDIV                 | `              3` | `            360` |
| TERMINATE            | `              1` | `             79` |

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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/3ab9399da68817d146da71d6f8116f77cefd798a
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10709315147)
