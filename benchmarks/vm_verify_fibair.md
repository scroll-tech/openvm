| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| <span style="color: green">(-13.0 [-0.1%])</span> <div style='text-align: right'>20,044.0</div>  | <div style='text-align: right'>87,040,020</div>  | <span style="color: red">(+3,495 [+0.0%])</span> <div style='text-align: right'>29,802,222</div>  | <span style="color: green">(-13.0 [-0.5%])</span> <div style='text-align: right'>2,633.0</div>  | <div style='text-align: right'>36.0</div>  |

| chip_name | rows_used |
| --- | --- |
| ByteXor | <div style='text-align: right'>65,536</div>  |
| Core | <span style="color: red">(+52 [+0.0%])</span> <div style='text-align: right'>290,687</div>  |
| FieldArithmetic | <span style="color: green">(-7 [-0.0%])</span> <div style='text-align: right'>140,677</div>  |
| FieldExtension | <div style='text-align: right'>7,486</div>  |
| Memory | <div style='text-align: right'>97,480</div>  |
| Memory 2 | <span style="color: red">(+16 [+0.0%])</span> <div style='text-align: right'>40,623</div>  |
| Memory 3 | <span style="color: red">(+8 [+0.0%])</span> <div style='text-align: right'>20,313</div>  |
| Memory 4 | <div style='text-align: right'>3,851</div>  |
| Poseidon2 | <div style='text-align: right'>2,613</div>  |
| ProgramChip | <div style='text-align: right'>37,480</div>  |
| RangeChecker | <div style='text-align: right'>65,536</div>  |
| RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |

<details>
<summary>Click to expand</summary>

| dsl_ir | opcode | frequency |
| --- | --- | --- |
|  | JAL | <div style='text-align: right'>1</div>  |
|  | STOREW | <div style='text-align: right'>2</div>  |
| AddE | FE4ADD | <div style='text-align: right'>1,630</div>  |
| AddEFFI | LOADW | <div style='text-align: right'>128</div>  |
| AddEFFI | STOREW | <div style='text-align: right'>384</div>  |
| AddEI | ADD | <div style='text-align: right'>5,664</div>  |
| AddFI | ADD | <span style="color: green">(-7 [-0.2%])</span> <div style='text-align: right'>2,851</div>  |
| AddV | ADD | <div style='text-align: right'>1,815</div>  |
| AddVI | ADD | <div style='text-align: right'>35,364</div>  |
| Alloc | ADD | <div style='text-align: right'>13,011</div>  |
| Alloc | LOADW | <div style='text-align: right'>13,011</div>  |
| Alloc | MUL | <div style='text-align: right'>8,979</div>  |
| AssertEqE | BNE | <div style='text-align: right'>404</div>  |
| AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| AssertEqF | BNE | <div style='text-align: right'>4,901</div>  |
| AssertEqV | BNE | <div style='text-align: right'>3,540</div>  |
| AssertEqVI | BNE | <div style='text-align: right'>20</div>  |
| CycleTrackerEnd | CT_END | <div style='text-align: right'>3,807</div>  |
| CycleTrackerStart | CT_START | <div style='text-align: right'>3,807</div>  |
| DivE | BBE4DIV | <div style='text-align: right'>1,202</div>  |
| DivEIN | BBE4DIV | <div style='text-align: right'>1</div>  |
| DivEIN | STOREW | <div style='text-align: right'>4</div>  |
| DivFIN | DIV | <div style='text-align: right'>3</div>  |
| For | ADD | <div style='text-align: right'>34,831</div>  |
| For | BNE | <div style='text-align: right'>44,214</div>  |
| For | JAL | <div style='text-align: right'>9,383</div>  |
| For | LOADW | <div style='text-align: right'>700</div>  |
| For | STOREW | <div style='text-align: right'>8,683</div>  |
| Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| HintBitsF | HINT_BITS | <div style='text-align: right'>101</div>  |
| HintInputVec | HINT_INPUT | <div style='text-align: right'>4,032</div>  |
| IfEq | BNE | <div style='text-align: right'>722</div>  |
| IfEqI | BNE | <div style='text-align: right'>11,289</div>  |
| IfEqI | JAL | <span style="color: red">(+52 [+2.8%])</span> <div style='text-align: right'>1,933</div>  |
| IfNe | BEQ | <div style='text-align: right'>2,129</div>  |
| IfNe | JAL | <div style='text-align: right'>7</div>  |
| IfNeI | BEQ | <div style='text-align: right'>603</div>  |
| ImmE | STOREW | <div style='text-align: right'>2,008</div>  |
| ImmF | STOREW | <div style='text-align: right'>7,036</div>  |
| ImmV | STOREW | <div style='text-align: right'>12,751</div>  |
| LoadE | LOADW | <div style='text-align: right'>4,836</div>  |
| LoadE | LOADW2 | <div style='text-align: right'>14,000</div>  |
| LoadF | LOADW | <div style='text-align: right'>13,252</div>  |
| LoadF | LOADW2 | <div style='text-align: right'>3,096</div>  |
| LoadV | LOADW | <div style='text-align: right'>8,800</div>  |
| LoadV | LOADW2 | <div style='text-align: right'>19,451</div>  |
| MulE | BBE4MUL | <div style='text-align: right'>3,418</div>  |
| MulEF | MUL | <div style='text-align: right'>1,608</div>  |
| MulEI | BBE4MUL | <div style='text-align: right'>33</div>  |
| MulEI | STOREW | <div style='text-align: right'>132</div>  |
| MulF | MUL | <div style='text-align: right'>2,204</div>  |
| MulFI | MUL | <div style='text-align: right'>1</div>  |
| MulV | MUL | <div style='text-align: right'>3,131</div>  |
| MulVI | MUL | <div style='text-align: right'>2,604</div>  |
| Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>2,000</div>  |
| Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>613</div>  |
| StoreE | STOREW | <div style='text-align: right'>32,012</div>  |
| StoreE | STOREW2 | <div style='text-align: right'>4,816</div>  |
| StoreF | STOREW | <div style='text-align: right'>6,484</div>  |
| StoreF | STOREW2 | <div style='text-align: right'>3,962</div>  |
| StoreHintWord | ADD | <div style='text-align: right'>19,708</div>  |
| StoreHintWord | SHINTW | <div style='text-align: right'>26,871</div>  |
| StoreV | STOREW | <div style='text-align: right'>1,462</div>  |
| StoreV | STOREW2 | <div style='text-align: right'>11,093</div>  |
| SubE | FE4SUB | <div style='text-align: right'>1,202</div>  |
| SubEF | LOADW | <div style='text-align: right'>4,815</div>  |
| SubEF | SUB | <div style='text-align: right'>1,605</div>  |
| SubEI | ADD | <div style='text-align: right'>8</div>  |
| SubV | SUB | <div style='text-align: right'>3,101</div>  |
| SubVI | SUB | <div style='text-align: right'>3,789</div>  |
| SubVIN | SUB | <div style='text-align: right'>400</div>  |

</details>

<details>
<summary>Click to expand</summary>

| air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- |
| Audit |  | JAL | <div style='text-align: right'>19</div>  |
| CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>24,882</div>  |
| AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>14,703</div>  |
| Audit | AddE | FE4ADD | <div style='text-align: right'>1,368</div>  |
| FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>66,830</div>  |
| AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>156</div>  |
| Audit | AddEFFI | LOADW | <div style='text-align: right'>304</div>  |
| CoreAir | AddEFFI | LOADW | <div style='text-align: right'>8,448</div>  |
| AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>132</div>  |
| Audit | AddEFFI | STOREW | <div style='text-align: right'>912</div>  |
| CoreAir | AddEFFI | STOREW | <div style='text-align: right'>25,344</div>  |
| AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+88 [+0.4%])</span> <div style='text-align: right'>24,486</div>  |
| AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+52 [+0.4%])</span> <div style='text-align: right'>14,469</div>  |
| Audit | AddEI | ADD | <div style='text-align: right'>760</div>  |
| FieldArithmeticAir | AddEI | ADD | <div style='text-align: right'>175,584</div>  |
| Audit | AddFI | ADD | <div style='text-align: right'>418</div>  |
| FieldArithmeticAir | AddFI | ADD | <span style="color: green">(-217 [-0.2%])</span> <div style='text-align: right'>88,381</div>  |
| Audit | AddV | ADD | <div style='text-align: right'>57</div>  |
| FieldArithmeticAir | AddV | ADD | <div style='text-align: right'>56,265</div>  |
| Audit | AddVI | ADD | <div style='text-align: right'>61,294</div>  |
| FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>1,096,284</div>  |
| FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>403,341</div>  |
| Audit | Alloc | LOADW | <div style='text-align: right'>3,135</div>  |
| CoreAir | Alloc | LOADW | <div style='text-align: right'>858,726</div>  |
| AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>278,349</div>  |
| AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>2,222</div>  |
| AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>1,313</div>  |
| CoreAir | AssertEqE | BNE | <div style='text-align: right'>26,664</div>  |
| CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| CoreAir | AssertEqF | BNE | <div style='text-align: right'>323,466</div>  |
| CoreAir | AssertEqV | BNE | <div style='text-align: right'>233,640</div>  |
| CoreAir | AssertEqVI | BNE | <div style='text-align: right'>1,320</div>  |
| CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>251,262</div>  |
| CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>251,262</div>  |
| AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>35,310</div>  |
| AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>20,865</div>  |
| FieldExtensionArithmeticAir | DivE | BBE4DIV | <div style='text-align: right'>49,282</div>  |
| AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>22</div>  |
| AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>13</div>  |
| FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <div style='text-align: right'>41</div>  |
| AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>11</div>  |
| CoreAir | DivEIN | STOREW | <div style='text-align: right'>264</div>  |
| FieldArithmeticAir | DivFIN | DIV | <div style='text-align: right'>93</div>  |
| FieldArithmeticAir | For | ADD | <div style='text-align: right'>1,079,761</div>  |
| CoreAir | For | BNE | <div style='text-align: right'>2,918,124</div>  |
| AccessAdapter<2> | For | JAL | <div style='text-align: right'>55</div>  |
| AccessAdapter<4> | For | JAL | <div style='text-align: right'>65</div>  |
| CoreAir | For | JAL | <div style='text-align: right'>619,278</div>  |
| Audit | For | LOADW | <div style='text-align: right'>1,900</div>  |
| CoreAir | For | LOADW | <div style='text-align: right'>46,200</div>  |
| Audit | For | STOREW | <div style='text-align: right'>988</div>  |
| CoreAir | For | STOREW | <div style='text-align: right'>573,078</div>  |
| CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>6,666</div>  |
| CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>266,112</div>  |
| CoreAir | IfEq | BNE | <div style='text-align: right'>47,652</div>  |
| CoreAir | IfEqI | BNE | <div style='text-align: right'>745,074</div>  |
| CoreAir | IfEqI | JAL | <span style="color: red">(+3,432 [+2.8%])</span> <div style='text-align: right'>127,578</div>  |
| CoreAir | IfNe | BEQ | <div style='text-align: right'>140,514</div>  |
| CoreAir | IfNe | JAL | <div style='text-align: right'>462</div>  |
| CoreAir | IfNeI | BEQ | <div style='text-align: right'>39,798</div>  |
| AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>2,200</div>  |
| AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>1,300</div>  |
| Audit | ImmE | STOREW | <div style='text-align: right'>76</div>  |
| CoreAir | ImmE | STOREW | <div style='text-align: right'>132,528</div>  |
| Audit | ImmF | STOREW | <div style='text-align: right'>3,724</div>  |
| CoreAir | ImmF | STOREW | <div style='text-align: right'>464,376</div>  |
| Audit | ImmV | STOREW | <div style='text-align: right'>64,486</div>  |
| CoreAir | ImmV | STOREW | <div style='text-align: right'>841,566</div>  |
| AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>17,688</div>  |
| AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>10,452</div>  |
| Audit | LoadE | LOADW | <div style='text-align: right'>380</div>  |
| CoreAir | LoadE | LOADW | <div style='text-align: right'>319,176</div>  |
| AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>28,666</div>  |
| AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>16,939</div>  |
| CoreAir | LoadE | LOADW2 | <div style='text-align: right'>924,000</div>  |
| AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>26,400</div>  |
| AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>15,600</div>  |
| AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>10,200</div>  |
| Audit | LoadF | LOADW | <div style='text-align: right'>570</div>  |
| CoreAir | LoadF | LOADW | <div style='text-align: right'>874,632</div>  |
| AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>715</div>  |
| AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>429</div>  |
| AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>323</div>  |
| Audit | LoadF | LOADW2 | <div style='text-align: right'>2,090</div>  |
| CoreAir | LoadF | LOADW2 | <div style='text-align: right'>204,336</div>  |
| Audit | LoadV | LOADW | <div style='text-align: right'>60,249</div>  |
| CoreAir | LoadV | LOADW | <div style='text-align: right'>580,800</div>  |
| Audit | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| CoreAir | LoadV | LOADW2 | <div style='text-align: right'>1,283,766</div>  |
| AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+88 [+0.3%])</span> <div style='text-align: right'>33,440</div>  |
| AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+52 [+0.3%])</span> <div style='text-align: right'>19,760</div>  |
| Audit | MulE | BBE4MUL | <div style='text-align: right'>988</div>  |
| FieldExtensionArithmeticAir | MulE | BBE4MUL | <div style='text-align: right'>140,138</div>  |
| AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,822</div>  |
| AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>5,213</div>  |
| Audit | MulEF | MUL | <div style='text-align: right'>76</div>  |
| FieldArithmeticAir | MulEF | MUL | <div style='text-align: right'>49,848</div>  |
| AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>1,892</div>  |
| AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>1,118</div>  |
| Audit | MulEI | BBE4MUL | <div style='text-align: right'>1,596</div>  |
| FieldExtensionArithmeticAir | MulEI | BBE4MUL | <div style='text-align: right'>1,353</div>  |
| AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>638</div>  |
| AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>338</div>  |
| Audit | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| CoreAir | MulEI | STOREW | <div style='text-align: right'>8,712</div>  |
| Audit | MulF | MUL | <div style='text-align: right'>19</div>  |
| FieldArithmeticAir | MulF | MUL | <div style='text-align: right'>68,324</div>  |
| Audit | MulFI | MUL | <div style='text-align: right'>19</div>  |
| FieldArithmeticAir | MulFI | MUL | <div style='text-align: right'>31</div>  |
| Audit | MulV | MUL | <div style='text-align: right'>59,432</div>  |
| FieldArithmeticAir | MulV | MUL | <div style='text-align: right'>97,061</div>  |
| Audit | MulVI | MUL | <div style='text-align: right'>76</div>  |
| FieldArithmeticAir | MulVI | MUL | <div style='text-align: right'>80,724</div>  |
| AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>88,000</div>  |
| AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>52,000</div>  |
| AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>34,000</div>  |
| Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>836,000</div>  |
| AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>53,801</div>  |
| AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>31,798</div>  |
| AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>20,842</div>  |
| Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>256,234</div>  |
| AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>8,800</div>  |
| AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>5,200</div>  |
| Audit | StoreE | STOREW | <div style='text-align: right'>608,228</div>  |
| CoreAir | StoreE | STOREW | <div style='text-align: right'>2,112,792</div>  |
| AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>8,800</div>  |
| AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>5,200</div>  |
| Audit | StoreE | STOREW2 | <div style='text-align: right'>30,704</div>  |
| CoreAir | StoreE | STOREW2 | <div style='text-align: right'>317,856</div>  |
| Audit | StoreF | STOREW | <div style='text-align: right'>123,196</div>  |
| CoreAir | StoreF | STOREW | <div style='text-align: right'>427,944</div>  |
| AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>231</div>  |
| AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>143</div>  |
| AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>102</div>  |
| Audit | StoreF | STOREW2 | <div style='text-align: right'>61,902</div>  |
| CoreAir | StoreF | STOREW2 | <div style='text-align: right'>261,492</div>  |
| FieldArithmeticAir | StoreHintWord | ADD | <div style='text-align: right'>610,948</div>  |
| Audit | StoreHintWord | SHINTW | <div style='text-align: right'>510,549</div>  |
| CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>1,773,486</div>  |
| Audit | StoreV | STOREW | <div style='text-align: right'>27,778</div>  |
| CoreAir | StoreV | STOREW | <div style='text-align: right'>96,492</div>  |
| Audit | StoreV | STOREW2 | <div style='text-align: right'>161,082</div>  |
| CoreAir | StoreV | STOREW2 | <div style='text-align: right'>732,138</div>  |
| AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>44,176</div>  |
| AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>26,104</div>  |
| Audit | SubE | FE4SUB | <div style='text-align: right'>380</div>  |
| FieldExtensionArithmeticAir | SubE | FE4SUB | <div style='text-align: right'>49,282</div>  |
| AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>17,633</div>  |
| Audit | SubEF | LOADW | <div style='text-align: right'>171</div>  |
| CoreAir | SubEF | LOADW | <div style='text-align: right'>317,790</div>  |
| AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>17,633</div>  |
| AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>20,839</div>  |
| Audit | SubEF | SUB | <div style='text-align: right'>57</div>  |
| FieldArithmeticAir | SubEF | SUB | <div style='text-align: right'>49,755</div>  |
| AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>44</div>  |
| AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>26</div>  |
| FieldArithmeticAir | SubEI | ADD | <div style='text-align: right'>248</div>  |
| Audit | SubV | SUB | <div style='text-align: right'>76</div>  |
| FieldArithmeticAir | SubV | SUB | <div style='text-align: right'>96,131</div>  |
| Audit | SubVI | SUB | <div style='text-align: right'>61,351</div>  |
| FieldArithmeticAir | SubVI | SUB | <div style='text-align: right'>117,459</div>  |
| FieldArithmeticAir | SubVIN | SUB | <div style='text-align: right'>12,400</div>  |

</details>

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ProgramAir<BabyBear> | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| CoreAir | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| FieldArithmeticAir | <div style='text-align: right'>12,320,768</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>262,144</div>  |
| FieldExtensionArithmeticAir | <div style='text-align: right'>466,944</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>8,192</div>  |
| Poseidon2VmAir<BabyBear> | <div style='text-align: right'>1,826,816</div>  | <div style='text-align: right'>373</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,096</div>  |
| XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| MemoryAuditAir | <div style='text-align: right'>3,538,944</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>131,072</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>1,507,328</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>819,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>237,568</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8,192</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/199ddf35b9228b2b9a3e348c724c52ad8d8419dc/verify_fibair.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/199ddf35b9228b2b9a3e348c724c52ad8d8419dc
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11270029083)
