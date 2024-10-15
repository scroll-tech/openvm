| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-65.0 [-0.4%])</span> <div style='text-align: right'>17,218.0</div>  | <span style="color: green">(-16 [-0.0%])</span> <div style='text-align: right'>20,200,817</div>  | <span style="color: green">(-13 [-0.0%])</span> <div style='text-align: right'>2,243,382</div>  | <span style="color: green">(-66.0 [-0.4%])</span> <div style='text-align: right'>17,286.0</div>  | <span style="color: green">(-1.0 [-1.4%])</span> <div style='text-align: right'>68.0</div>  |  |
| inner_verifier | <span style="color: red">(+44.0 [+0.0%])</span> <div style='text-align: right'>88,135.0</div>  | <span style="color: green">(-2,097,152 [-0.3%])</span> <div style='text-align: right'>735,445,012</div>  | <span style="color: green">(-1,645,113 [-0.4%])</span> <div style='text-align: right'>402,832,668</div>  | <span style="color: red">(+1,911.0 [+1.6%])</span> <div style='text-align: right'>118,759.0</div>  | <span style="color: red">(+1,867.0 [+6.5%])</span> <div style='text-align: right'>30,624.0</div>  | <span style="color: red">(+158.0 [+0.3%])</span> <div style='text-align: right'>47,389.0</div>  |

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
| inner_verifier | Core | <span style="color: red">(+509 [+0.0%])</span> <div style='text-align: right'>3,933,695</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-151 [-0.0%])</span> <div style='text-align: right'>1,662,153</div>  |
| inner_verifier | FieldExtension | <span style="color: green">(-172 [-0.0%])</span> <div style='text-align: right'>846,296</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-244 [-0.0%])</span> <div style='text-align: right'>1,964,001</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-122 [-0.0%])</span> <div style='text-align: right'>982,066</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>36,161</div>  |
| inner_verifier | Memory Boundary | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>627,640</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>22,901</div>  |
| inner_verifier | ProgramChip | <span style="color: red">(+64 [+0.0%])</span> <div style='text-align: right'>199,713</div>  |
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
| inner_verifier | AddE | FE4ADD | <span style="color: green">(-19 [-0.0%])</span> <div style='text-align: right'>224,519</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>143</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>429</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+16 [+9.5%])</span> <div style='text-align: right'>184</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+60 [+0.1%])</span> <div style='text-align: right'>67,936</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+24 [+0.2%])</span> <div style='text-align: right'>14,620</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>8,173</div>  |
| inner_verifier | AddVI | ADD | <span style="color: green">(-42 [-0.0%])</span> <div style='text-align: right'>293,951</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>30,855</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>30,855</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>18,311</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>136</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,894</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,278</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>159</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>105,322</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>105,322</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: green">(-42 [-0.0%])</span> <div style='text-align: right'>195,599</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>31</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>124</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>75</div>  |
| inner_verifier | For | ADD | <span style="color: green">(-92 [-0.0%])</span> <div style='text-align: right'>561,632</div>  |
| inner_verifier | For | BNE | <span style="color: green">(-92 [-0.0%])</span> <div style='text-align: right'>584,378</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>22,746</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,071</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>21,675</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>12,544</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>6,653</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>127,270</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+1,222 [+11.3%])</span> <div style='text-align: right'>12,013</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>9,555</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>25</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,116</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+4 [+0.0%])</span> <div style='text-align: right'>12,388</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,711</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: green">(-21 [-0.1%])</span> <div style='text-align: right'>22,995</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: green">(-8 [-0.0%])</span> <div style='text-align: right'>42,576</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: green">(-168 [-0.0%])</span> <div style='text-align: right'>805,360</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>13,948</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: green">(-63 [-0.0%])</span> <div style='text-align: right'>300,219</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>14,878</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>75,390</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: green">(-109 [-0.0%])</span> <div style='text-align: right'>409,457</div>  |
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
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>13,031</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>12,600</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,516</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,398</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>102,832</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>214,549</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>227,775</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,672</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>30,433</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>14,091</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: green">(-252 [-0.0%])</span> <div style='text-align: right'>1,170,870</div>  |
| inner_verifier | SubEF | SUB | <span style="color: green">(-84 [-0.0%])</span> <div style='text-align: right'>390,290</div>  |
| inner_verifier | SubEFI | ADD | <span style="color: green">(-4 [-0.3%])</span> <div style='text-align: right'>1,284</div>  |
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
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <div style='text-align: right'>76</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>41</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <div style='text-align: right'>19</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>124</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
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
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+88 [+0.0%])</span> <div style='text-align: right'>1,112,100</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+52 [+0.0%])</span> <div style='text-align: right'>657,150</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>2,077,764</div>  |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | <span style="color: green">(-779 [-0.0%])</span> <div style='text-align: right'>9,205,279</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>1,089</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,287</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>380</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>9,438</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>1,089</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,140</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>28,314</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>5,520</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: green">(-44 [-7.7%])</span> <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: green">(-26 [-7.7%])</span> <div style='text-align: right'>312</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,052</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>2,038,080</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+726 [+0.2%])</span> <div style='text-align: right'>377,388</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+429 [+0.2%])</span> <div style='text-align: right'>223,002</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>1,132,096</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>438,600</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>245,190</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>8,818,530</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>14,915</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>925,650</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>2,036,430</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>549,330</div>  |
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
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-1,386 [-0.0%])</span> <div style='text-align: right'>6,951,252</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-1,386 [-0.0%])</span> <div style='text-align: right'>6,951,252</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: green">(-1,848 [-0.0%])</span> <div style='text-align: right'>8,586,688</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: green">(-1,092 [-0.0%])</span> <div style='text-align: right'>5,073,952</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | <span style="color: green">(-1,722 [-0.0%])</span> <div style='text-align: right'>8,019,559</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,232</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>728</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>304</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <div style='text-align: right'>1,271</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>440</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>117</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>8,184</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,250</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>16,848,960</div>  |
| inner_verifier | CoreAir | For | BNE | <span style="color: green">(-6,072 [-0.0%])</span> <div style='text-align: right'>38,568,948</div>  |
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
| inner_verifier | CoreAir | IfEqI | BNE | <span style="color: green">(-1,386 [-0.0%])</span> <div style='text-align: right'>8,399,820</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: red">(+80,652 [+11.3%])</span> <div style='text-align: right'>792,858</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>630,630</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,650</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>73,656</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: green">(-110 [-3.3%])</span> <div style='text-align: right'>3,190</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: green">(-65 [-3.3%])</span> <div style='text-align: right'>1,885</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>214,624</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <span style="color: red">(+264 [+0.0%])</span> <div style='text-align: right'>817,608</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>1,036,926</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <span style="color: green">(-1,386 [-0.1%])</span> <div style='text-align: right'>1,517,670</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>66,836</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>39,494</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>503,120</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <span style="color: green">(-528 [-0.0%])</span> <div style='text-align: right'>2,810,016</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>29,634</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>17,511</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <span style="color: green">(-11,088 [-0.0%])</span> <div style='text-align: right'>53,153,760</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>26,796</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>15,834</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>10,353</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>920,568</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>693</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>416</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>459</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <span style="color: green">(-4,158 [-0.0%])</span> <div style='text-align: right'>19,814,454</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,794</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>981,948</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>4,975,740</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <div style='text-align: right'>500,588</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <div style='text-align: right'>295,802</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>1,215,620</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | <span style="color: green">(-4,469 [-0.0%])</span> <div style='text-align: right'>16,787,737</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>60,360</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>10,208</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>6,032</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>608</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>42,960</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: red">(+44 [+2.6%])</span> <div style='text-align: right'>1,738</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: red">(+26 [+2.6%])</span> <div style='text-align: right'>1,027</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>24,244</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+440 [+0.3%])</span> <div style='text-align: right'>159,236</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+260 [+0.3%])</span> <div style='text-align: right'>94,094</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>154,660</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | <div style='text-align: right'>106,559</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>56,892</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>33,462</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>686,136</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>788,790</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>390</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>332,430</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>5,640</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: green">(-44 [-3.6%])</span> <div style='text-align: right'>1,166</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: green">(-26 [-3.6%])</span> <div style='text-align: right'>689</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>414,876</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>245,154</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>160,293</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>4,125,660</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>615,505</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>364,130</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>239,904</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,446,958</div>  |
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
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>522,357</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>309,088</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>203,728</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>71,592</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <span style="color: green">(-1,386 [-0.0%])</span> <div style='text-align: right'>6,786,912</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>6,436,470</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: green">(-551 [-0.0%])</span> <div style='text-align: right'>4,327,725</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <span style="color: green">(-1,914 [-0.0%])</span> <div style='text-align: right'>15,033,150</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>31,768</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>110,352</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>575,548</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>2,008,578</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-22 [-0.0%])</span> <div style='text-align: right'>464,948</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-13 [-0.0%])</span> <div style='text-align: right'>274,742</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>958,132</div>  |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | <span style="color: green">(-82 [-0.0%])</span> <div style='text-align: right'>577,731</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: green">(-924 [-0.0%])</span> <div style='text-align: right'>4,293,190</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <span style="color: green">(-16,632 [-0.0%])</span> <div style='text-align: right'>77,277,420</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>11,708,700</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: green">(-924 [-0.0%])</span> <div style='text-align: right'>4,293,190</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: green">(-1,092 [-0.0%])</span> <div style='text-align: right'>5,073,770</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>38,520</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: green">(-66 [-13.0%])</span> <div style='text-align: right'>440</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: green">(-39 [-13.0%])</span> <div style='text-align: right'>260</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>22,800</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>7,440</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>1,980</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,170</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>608</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>495,570</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>41,520</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>13,230</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,392</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir, NewFieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | <div style='text-align: right'>77</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
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
| inner_verifier | VmAirWrapper<NativeAdapterAir, NewFieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | FieldExtensionArithmeticAir | <div style='text-align: right'>59,768,832</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6b543cb0a3bba565cd1dddf0904a54dcc3f97169
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11354735087)
