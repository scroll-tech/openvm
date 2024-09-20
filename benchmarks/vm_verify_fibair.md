## Benchmark for VM Verifier for Fibonacci Air
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 67_641_364 | 16200.00 | 1960.00 | 122.00 | 2040.00 | 12078.00 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| CoreAir              | 524_288    | 46_661_632  | 0     | [69] | [20] |
| ProgramAir           | 65_536     | 589_824     | 9     | [1] | [8] |
| FieldArithmeticAir   | 262_144    | 12_320_768  | 0     | [31] | [16] |
| FieldExtensionArithmeticAir | 8_192      | 884_736     | 0     | [68] | [40] |
| Poseidon2VmAir       | 4_096      | 2_465_792   | 0     | [502] | [100] |
| MemoryAuditAir       | 131_072    | 3_538_944   | 0     | [19] | [8] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |
| VmConnectorAir       | 2          | 20          | 1     | [2] | [8] |
<details>
<summary>

### Custom VM metrics

</summary>

| Name | Value |
|------|------:|
| Core                 | `        320_143` |
| FieldArithmetic      | `        164_539` |
| FieldExtension       | `          7_914` |
| Memory               | `        107_130` |
| Poseidon2            | `          3_309` |
| Program              | `         54_734` |
| RangeChecker         | `        131_072` |

#### Opcode metrics
| Name | Frequency | Trace Cells Contributed |
|------|------:|-----:|
| FADD                 | `        134_354` | `      4_231_056` |
| BNE                  | `         75_347` | `      5_198_943` |
| STOREW               | `         74_011` | `      5_961_056` |
| LOADW                | `         49_216` | `      3_470_099` |
| LOADW2               | `         38_007` | `      2_625_827` |
| SHINTW               | `         33_232` | `      2_924_416` |
| STOREW2              | `         21_346` | `      1_752_592` |
| FMUL                 | `         20_715` | `        702_623` |
| JAL                  | `         12_839` | `        885_910` |
| FSUB                 | `          9_467` | `        355_778` |
| HINT_INPUT           | `          4_769` | `        329_061` |
| CT_END               | `          3_921` | `        270_549` |
| CT_START             | `          3_921` | `        270_549` |
| BBE4MUL              | `          3_759` | `        258_120` |
| BEQ                  | `          3_429` | `        236_601` |
| COMP_POS2            | `          2_678` | `      1_344_356` |
| FE4ADD               | `          1_678` | `        115_016` |
| BBE4DIV              | `          1_239` | `         84_328` |
| FE4SUB               | `          1_238` | `         84_336` |
| PERM_POS2            | `            631` | `        316_762` |
| HINT_BITS            | `            104` | `          7_176` |
| FDIV                 | `              3` | `             93` |
| TERMINATE            | `              1` | `             69` |

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
| StoreV               | `         13_848` |
| ImmV                 | `         13_024` |
| StoreF               | `         10_962` |
| ImmF                 | `          7_243` |
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

Commit: https://github.com/axiom-crypto/afs-prototype/commit/4d1e1bc932b173b53aea599521d1c1c265aebc6b
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10965120249)
