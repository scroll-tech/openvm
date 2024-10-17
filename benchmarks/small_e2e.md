| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-25.0 [-0.1%])</span> <div style='text-align: right'>17,166.0</div>  | <div style='text-align: right'>20,200,816</div>  | <div style='text-align: right'>2,243,381</div>  | <span style="color: green">(-25.0 [-0.1%])</span> <div style='text-align: right'>17,235.0</div>  | <div style='text-align: right'>69.0</div>  |  |
| inner_verifier | <span style="color: green">(-2,196.0 [-2.4%])</span> <div style='text-align: right'>88,405.0</div>  | <div style='text-align: right'>734,396,436</div>  | <span style="color: red">(+62,076 [+0.0%])</span> <div style='text-align: right'>401,908,670</div>  | <span style="color: green">(-2,678.0 [-2.2%])</span> <div style='text-align: right'>117,225.0</div>  | <span style="color: green">(-482.0 [-1.6%])</span> <div style='text-align: right'>28,820.0</div>  | <span style="color: green">(-315.0 [-0.7%])</span> <div style='text-align: right'>45,564.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>28</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| bench_program_inner | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | ByteXor | <div style='text-align: right'>65,536</div>  |
| inner_verifier | Core | <span style="color: red">(+916 [+0.0%])</span> <div style='text-align: right'>3,932,999</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+12 [+0.0%])</span> <div style='text-align: right'>1,661,972</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>846,158</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+72 [+0.0%])</span> <div style='text-align: right'>1,963,401</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+36 [+0.0%])</span> <div style='text-align: right'>981,745</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>36,119</div>  |
| inner_verifier | Memory Boundary | <div style='text-align: right'>627,611</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>22,880</div>  |
| inner_verifier | ProgramChip | <div style='text-align: right'>199,811</div>  |
| inner_verifier | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |

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
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>224,508</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>143</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>429</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>184</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>68,012</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+12 [+0.1%])</span> <div style='text-align: right'>14,614</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>8,173</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>293,909</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>30,855</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>30,855</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>18,311</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>136</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,894</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,278</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>159</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <div style='text-align: right'>105,301</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <div style='text-align: right'>105,301</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>195,557</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>31</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>124</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>75</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>561,540</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>584,286</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>22,746</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,071</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>21,675</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>12,544</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>6,653</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>127,249</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+916 [+8.3%])</span> <div style='text-align: right'>12,009</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>9,555</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>25</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,116</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>12,392</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,711</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>22,995</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>42,568</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>805,192</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>13,948</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>300,156</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>14,878</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>75,390</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>409,372</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>2,012</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,432</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>2,599</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>10,396</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>26,293</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>13</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>11,081</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>188</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>9,870</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>13,010</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>12,600</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,516</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,398</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>102,811</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>214,520</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>227,746</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,672</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>30,433</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>14,091</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>1,170,618</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>390,206</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,280</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>248</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>16,519</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,384</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>441</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>40</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <div style='text-align: right'>76</div>  |
| bench_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <div style='text-align: right'>19</div>  |
| bench_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>124</div>  |
| bench_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>186</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>124</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <div style='text-align: right'>152</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>496</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>186</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <div style='text-align: right'>722</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>124</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>8,980,320</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>1,111,726</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>656,929</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>2,077,764</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>1,111</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,313</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>380</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>9,438</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>1,111</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,140</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>28,314</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>5,520</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>638</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>377</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,052</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>2,040,360</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+396 [+0.1%])</span> <div style='text-align: right'>377,938</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+234 [+0.1%])</span> <div style='text-align: right'>223,327</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>1,132,096</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+360 [+0.1%])</span> <div style='text-align: right'>438,420</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>245,190</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>8,817,270</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>14,915</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>925,650</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>2,036,430</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>549,330</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>748</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>442</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <div style='text-align: right'>8,976</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <div style='text-align: right'>323,004</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <div style='text-align: right'>84,348</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <div style='text-align: right'>10,494</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>6,949,866</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>6,949,866</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>7,822,280</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>8,584,840</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>5,072,860</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,240</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,210</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>715</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>304</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>440</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>117</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>8,184</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,250</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>16,846,200</div>  |
| inner_verifier | CoreAir | For | BNE | <div style='text-align: right'>38,562,876</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>484</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>572</div>  |
| inner_verifier | CoreAir | For | JAL | <div style='text-align: right'>1,501,236</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>70,686</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>988</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,430,550</div>  |
| inner_verifier | CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,452</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>827,904</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <div style='text-align: right'>439,098</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <div style='text-align: right'>8,398,434</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: red">(+60,456 [+8.3%])</span> <div style='text-align: right'>792,594</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>630,630</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,650</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>73,656</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>3,410</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>2,015</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>214,624</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <div style='text-align: right'>817,872</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>1,036,926</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>1,517,670</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>65,428</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>38,662</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>503,120</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <div style='text-align: right'>2,809,488</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>29,634</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>17,511</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <div style='text-align: right'>53,142,672</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>26,796</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>15,834</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>10,353</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>920,568</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>693</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>416</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>459</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <div style='text-align: right'>19,810,296</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,794</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>981,948</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>4,975,740</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>16,374,880</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+396 [+0.1%])</span> <div style='text-align: right'>499,796</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+234 [+0.1%])</span> <div style='text-align: right'>295,334</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>1,215,620</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>60,360</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>10,252</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>6,058</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>608</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>42,960</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,958</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,157</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>24,244</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>103,960</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>158,488</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>93,652</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>154,660</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>56,892</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>33,462</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>686,136</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>788,790</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>390</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>332,430</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>5,640</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,100</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>650</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>414,876</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>245,154</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>160,293</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>4,125,660</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>615,274</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>363,857</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>239,547</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,438,180</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>9,746</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>5,759</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>239,400</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>831,600</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>48,972</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>28,938</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>35,112</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>826,056</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>254,562</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>884,268</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>522,126</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>308,815</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>203,371</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>71,592</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <div style='text-align: right'>6,785,526</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>6,435,600</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>4,327,174</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>15,031,236</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>31,768</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>110,352</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>575,548</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>2,008,578</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>563,640</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>464,574</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>274,521</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>958,132</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>4,292,266</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <div style='text-align: right'>77,260,788</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>11,706,180</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>4,292,266</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>5,072,678</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>38,400</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>312</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>22,800</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>7,440</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,046</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,209</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>608</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>495,570</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>41,520</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>13,230</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,392</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>4,480</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>360,710,144</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67eacebd87410ab0de5765fcdcb0402cbf30ee6a/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/67eacebd87410ab0de5765fcdcb0402cbf30ee6a
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11388207240)
