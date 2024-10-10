| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: red">(+4.0 [+0.6%])</span> <div style='text-align: right'>713.0</div>  | <div style='text-align: right'>1,325,889</div>  | <div style='text-align: right'>211,816</div>  | <span style="color: red">(+4.0 [+0.6%])</span> <div style='text-align: right'>715.0</div>  | <div style='text-align: right'>2.0</div>  |  |
| inner_verifier | <span style="color: green">(-932.0 [-1.1%])</span> <div style='text-align: right'>84,370.0</div>  | <div style='text-align: right'>716,308,500</div>  | <span style="color: red">(+48,437 [+0.0%])</span> <div style='text-align: right'>385,025,790</div>  | <span style="color: green">(-1,328.0 [-1.1%])</span> <div style='text-align: right'>116,987.0</div>  | <span style="color: green">(-396.0 [-1.2%])</span> <div style='text-align: right'>32,617.0</div>  | <span style="color: green">(-102.0 [-0.2%])</span> <div style='text-align: right'>46,041.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>28</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory | <div style='text-align: right'>65</div>  |
| bench_program_inner | Memory 2 | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory 3 | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory 4 | <div style='text-align: right'>5</div>  |
| bench_program_inner | Program | <div style='text-align: right'>37</div>  |
| bench_program_inner | RangeChecker | <div style='text-align: right'>65,536</div>  |
| inner_verifier | ByteXor | <div style='text-align: right'>65,536</div>  |
| inner_verifier | Core | <span style="color: red">(+696 [+0.0%])</span> <div style='text-align: right'>3,767,920</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+31 [+0.0%])</span> <div style='text-align: right'>1,538,175</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>843,097</div>  |
| inner_verifier | Memory | <div style='text-align: right'>584,699</div>  |
| inner_verifier | Memory 2 | <span style="color: red">(+88 [+0.0%])</span> <div style='text-align: right'>1,940,117</div>  |
| inner_verifier | Memory 3 | <span style="color: red">(+44 [+0.0%])</span> <div style='text-align: right'>970,124</div>  |
| inner_verifier | Memory 4 | <div style='text-align: right'>32,483</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>19,493</div>  |
| inner_verifier | Program | <div style='text-align: right'>199,131</div>  |
| inner_verifier | RangeChecker | <div style='text-align: right'>65,536</div>  |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | AddE | FE4ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddF | ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddVI | ADD | <div style='text-align: right'>6</div>  |
| bench_program_inner | Alloc | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | LOADW | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | MUL | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | BNE | <div style='text-align: right'>3</div>  |
| bench_program_inner | For | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner | For | STOREW | <div style='text-align: right'>1</div>  |
| bench_program_inner | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmE | STOREW | <div style='text-align: right'>8</div>  |
| bench_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>3</div>  |
| bench_program_inner | Keccak256 | KECCAK256 | <div style='text-align: right'>1</div>  |
| bench_program_inner | StoreV | STOREW2 | <div style='text-align: right'>2</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>223,742</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>123</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>369</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>168</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>66,532</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+31 [+0.3%])</span> <div style='text-align: right'>12,344</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>5,606</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>267,774</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>22,508</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>22,508</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>13,583</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>132</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>3,886</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,159</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>122</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <div style='text-align: right'>104,083</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <div style='text-align: right'>104,083</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>194,988</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>30</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>120</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>72</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>521,458</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>539,263</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>17,805</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>882</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>16,923</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>8,925</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>5,189</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>120,514</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+696 [+8.3%])</span> <div style='text-align: right'>9,071</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,385</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>20</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>886</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>12,368</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>13,357</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>21,163</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>41,212</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>799,980</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>10,939</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>298,869</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>10,978</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>61,601</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>408,006</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,584</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,432</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>2,558</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>10,232</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>22,005</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>12</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>7,734</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>184</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>6,657</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>12,836</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>10,908</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>10,984</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>11,212</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>101,582</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>188,221</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>197,828</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,333</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>23,056</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>13,773</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>1,167,840</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>389,280</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,288</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>240</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>13,893</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,239</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>336</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Audit | AddE | FE4ADD | <div style='text-align: right'>76</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>41</div>  |
| bench_program_inner | Audit | AddF | ADD | <div style='text-align: right'>19</div>  |
| bench_program_inner | FieldArithmeticAir | AddF | ADD | <div style='text-align: right'>31</div>  |
| bench_program_inner | Audit | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>186</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit | Alloc | LOADW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>124</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>62</div>  |
| bench_program_inner | FieldArithmeticAir | For | ADD | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>186</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>124</div>  |
| bench_program_inner | Audit | ImmE | STOREW | <div style='text-align: right'>152</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>496</div>  |
| bench_program_inner | Audit | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | Audit | ImmV | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>186</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Audit | Keccak256 | KECCAK256 | <div style='text-align: right'>722</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Audit | StoreV | STOREW2 | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>124</div>  |
| inner_verifier | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| inner_verifier | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>1,105,060</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>652,990</div>  |
| inner_verifier | Audit | AddE | FE4ADD | <div style='text-align: right'>2,077,764</div>  |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>9,173,422</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>869</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,027</div>  |
| inner_verifier | Audit | AddEFFI | LOADW | <div style='text-align: right'>380</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>8,118</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>869</div>  |
| inner_verifier | Audit | AddEFFI | STOREW | <div style='text-align: right'>1,140</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>24,354</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>338</div>  |
| inner_verifier | Audit | AddEFI | ADD | <div style='text-align: right'>2,052</div>  |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | <div style='text-align: right'>5,208</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+484 [+0.1%])</span> <div style='text-align: right'>370,744</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+286 [+0.1%])</span> <div style='text-align: right'>219,076</div>  |
| inner_verifier | Audit | AddEI | ADD | <div style='text-align: right'>1,132,096</div>  |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | <div style='text-align: right'>2,062,492</div>  |
| inner_verifier | Audit | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | <span style="color: red">(+961 [+0.3%])</span> <div style='text-align: right'>382,664</div>  |
| inner_verifier | Audit | AddV | ADD | <div style='text-align: right'>57</div>  |
| inner_verifier | FieldArithmeticAir | AddV | ADD | <div style='text-align: right'>173,786</div>  |
| inner_verifier | Audit | AddVI | ADD | <div style='text-align: right'>14,991</div>  |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>8,300,994</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>697,748</div>  |
| inner_verifier | Audit | Alloc | LOADW | <div style='text-align: right'>1,634</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>1,485,528</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>421,073</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>726</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>429</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <div style='text-align: right'>8,712</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <div style='text-align: right'>256,476</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <div style='text-align: right'>76,494</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <div style='text-align: right'>8,052</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>6,869,478</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>6,869,478</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>8,564,446</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>5,060,809</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | <div style='text-align: right'>7,994,508</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,210</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>715</div>  |
| inner_verifier | Audit | DivEIN | BBE4DIV | <div style='text-align: right'>304</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <div style='text-align: right'>1,230</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>429</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>117</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>7,920</div>  |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | <div style='text-align: right'>2,232</div>  |
| inner_verifier | FieldArithmeticAir | For | ADD | <div style='text-align: right'>16,165,198</div>  |
| inner_verifier | CoreAir | For | BNE | <div style='text-align: right'>35,591,358</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>418</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | For | JAL | <div style='text-align: right'>1,175,130</div>  |
| inner_verifier | Audit | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>58,212</div>  |
| inner_verifier | Audit | For | STOREW | <div style='text-align: right'>969</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,116,918</div>  |
| inner_verifier | CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,452</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>589,050</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <div style='text-align: right'>342,474</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <div style='text-align: right'>7,953,924</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: red">(+45,936 [+8.3%])</span> <div style='text-align: right'>598,686</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>421,410</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,320</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>58,476</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>3,234</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>1,911</div>  |
| inner_verifier | Audit | ImmE | STOREW | <div style='text-align: right'>214,624</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <div style='text-align: right'>816,288</div>  |
| inner_verifier | Audit | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>881,562</div>  |
| inner_verifier | Audit | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>1,396,758</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>61,688</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>36,452</div>  |
| inner_verifier | Audit | LoadE | LOADW | <div style='text-align: right'>503,120</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <div style='text-align: right'>2,719,992</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>22,704</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>13,416</div>  |
| inner_verifier | Audit | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <div style='text-align: right'>52,798,680</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>21,252</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>12,558</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,211</div>  |
| inner_verifier | Audit | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>721,974</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>583</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>351</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>374</div>  |
| inner_verifier | Audit | LoadF | LOADW2 | <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <div style='text-align: right'>19,725,354</div>  |
| inner_verifier | Audit | LoadV | LOADW | <div style='text-align: right'>13,680</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>724,548</div>  |
| inner_verifier | Audit | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>4,065,666</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+484 [+0.1%])</span> <div style='text-align: right'>493,240</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+286 [+0.1%])</span> <div style='text-align: right'>291,460</div>  |
| inner_verifier | Audit | MulE | BBE4MUL | <div style='text-align: right'>1,215,620</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | <div style='text-align: right'>16,728,246</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Audit | MulEF | MUL | <div style='text-align: right'>608</div>  |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | <div style='text-align: right'>49,104</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,694</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,001</div>  |
| inner_verifier | Audit | MulEFI | MUL | <div style='text-align: right'>24,244</div>  |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | <div style='text-align: right'>44,392</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>156,860</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>92,690</div>  |
| inner_verifier | Audit | MulEI | BBE4MUL | <div style='text-align: right'>154,660</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | <div style='text-align: right'>104,878</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>56,045</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>32,994</div>  |
| inner_verifier | Audit | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>675,312</div>  |
| inner_verifier | Audit | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulF | MUL | <div style='text-align: right'>682,155</div>  |
| inner_verifier | Audit | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | <div style='text-align: right'>372</div>  |
| inner_verifier | Audit | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | FieldArithmeticAir | MulV | MUL | <div style='text-align: right'>21,142</div>  |
| inner_verifier | Audit | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | <div style='text-align: right'>239,754</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,188</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>702</div>  |
| inner_verifier | Audit | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | FieldArithmeticAir | NegE | MUL | <div style='text-align: right'>5,704</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>276,276</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>163,254</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>106,743</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>2,782,626</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>598,477</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>354,068</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>233,274</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,365,448</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,436</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,394</div>  |
| inner_verifier | Audit | StoreE | STOREW | <div style='text-align: right'>207,252</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>719,928</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>45,276</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>26,754</div>  |
| inner_verifier | Audit | StoreE | STOREW2 | <div style='text-align: right'>26,752</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>724,944</div>  |
| inner_verifier | Audit | StoreF | STOREW | <div style='text-align: right'>213,028</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>739,992</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>522,071</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>308,919</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>203,609</div>  |
| inner_verifier | Audit | StoreF | STOREW2 | <div style='text-align: right'>55,404</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <div style='text-align: right'>6,704,412</div>  |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | <div style='text-align: right'>5,834,851</div>  |
| inner_verifier | Audit | StoreHintWord | SHINTW | <div style='text-align: right'>3,758,732</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>13,056,648</div>  |
| inner_verifier | Audit | StoreV | STOREW | <div style='text-align: right'>25,327</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>87,978</div>  |
| inner_verifier | Audit | StoreV | STOREW2 | <div style='text-align: right'>433,390</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>1,521,696</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>453,398</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>267,917</div>  |
| inner_verifier | Audit | SubE | FE4SUB | <div style='text-align: right'>958,132</div>  |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | <div style='text-align: right'>564,693</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>4,282,080</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <div style='text-align: right'>77,077,440</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>4,282,080</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>5,060,640</div>  |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | <div style='text-align: right'>12,067,680</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>299</div>  |
| inner_verifier | Audit | SubEFI | ADD | <div style='text-align: right'>22,800</div>  |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | <div style='text-align: right'>39,928</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>1,914</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,131</div>  |
| inner_verifier | Audit | SubEI | ADD | <div style='text-align: right'>608</div>  |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | <div style='text-align: right'>7,440</div>  |
| inner_verifier | Audit | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | FieldArithmeticAir | SubV | SUB | <div style='text-align: right'>430,683</div>  |
| inner_verifier | Audit | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | <div style='text-align: right'>38,409</div>  |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | <div style='text-align: right'>10,416</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | <div style='text-align: right'>576</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,392</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | FieldArithmeticAir | <div style='text-align: right'>1,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | <div style='text-align: right'>77</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | MemoryAuditAir | <div style='text-align: right'>4,480</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | ProgramAir<BabyBear> | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>360,710,144</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | FieldArithmeticAir | <div style='text-align: right'>98,566,144</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | FieldExtensionArithmeticAir | <div style='text-align: right'>59,768,832</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>373</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | MemoryAuditAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/a08603668fbfc3bccfcdbb13254db1065baac038
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11266932712)
