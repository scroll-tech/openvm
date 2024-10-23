| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,368.0</div>  | <div style='text-align: right'>1,916,220</div>  | <div style='text-align: right'>146,113</div>  | <span style="color: green">(-2.0 [-0.1%])</span> <div style='text-align: right'>1,389.0</div>  | <span style="color: green">(-1.0 [-4.5%])</span> <div style='text-align: right'>21.0</div>  |  |
| inner_verifier | <span style="color: red">(+585.0 [+0.7%])</span> <div style='text-align: right'>82,720.0</div>  | <div style='text-align: right'>690,356,248</div>  | <span style="color: green">(-17,720 [-0.0%])</span> <div style='text-align: right'>362,566,204</div>  | <span style="color: red">(+187.0 [+0.2%])</span> <div style='text-align: right'>96,835.0</div>  | <span style="color: green">(-398.0 [-2.7%])</span> <div style='text-align: right'>14,115.0</div>  | <span style="color: red">(+19.0 [+0.0%])</span> <div style='text-align: right'>45,543.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>27</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| inner_verifier | Core | <span style="color: green">(-291 [-0.0%])</span> <div style='text-align: right'>3,797,397</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>1,557,760</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>843,682</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-8 [-0.0%])</span> <div style='text-align: right'>1,944,023</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>972,034</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>33,096</div>  |
| inner_verifier | Memory Boundary | <div style='text-align: right'>592,417</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>20,103</div>  |
| inner_verifier | ProgramChip | <div style='text-align: right'>199,371</div>  |

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
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmE | STOREW | <div style='text-align: right'>8</div>  |
| bench_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>3</div>  |
| bench_program_inner | Keccak256 | KECCAK256 | <div style='text-align: right'>1</div>  |
| bench_program_inner | StoreV | STOREW2 | <div style='text-align: right'>2</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>223,919</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>128</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>384</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>188</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>66,940</div>  |
| inner_verifier | AddFI | ADD | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>12,416</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>6,049</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>271,881</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>23,942</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>23,942</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>14,447</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>132</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,182</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>149</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <div style='text-align: right'>104,398</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <div style='text-align: right'>104,398</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>195,093</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>30</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>120</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>72</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>528,357</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>546,986</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>18,629</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>966</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>17,663</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>9,495</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>6,158</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>121,363</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: green">(-291 [-3.1%])</span> <div style='text-align: right'>8,967</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>21</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,006</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>12,388</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>14,525</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>21,682</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>41,480</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>800,736</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>299,007</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>11,595</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>64,069</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>408,232</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,668</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,424</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>2,570</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>10,280</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>22,173</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>12</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>8,259</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>184</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,224</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>12,879</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>11,244</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>11,156</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>12,380</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>101,810</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>192,455</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>202,632</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,394</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>24,133</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>13,838</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>1,168,350</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>389,450</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,256</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>240</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>14,040</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,268</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>120</div>  |
| bench_program_inner | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>40</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <div style='text-align: right'>76</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <div style='text-align: right'>19</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>120</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>180</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>60</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>120</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <div style='text-align: right'>152</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>480</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>120</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>180</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <div style='text-align: right'>722</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>120</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>60</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>120</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>8,956,760</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>1,105,742</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>653,393</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>2,075,560</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>913</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,079</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>7,680</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>913</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,254</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>23,040</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>5,640</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>338</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,128</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>2,008,200</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: green">(-44 [-0.0%])</span> <div style='text-align: right'>372,878</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: green">(-26 [-0.0%])</span> <div style='text-align: right'>220,337</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>1,132,096</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: green">(-120 [-0.0%])</span> <div style='text-align: right'>372,480</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>181,470</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>8,156,430</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>14,877</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>718,260</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>1,436,520</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>433,410</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>726</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>429</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <div style='text-align: right'>7,920</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>240</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <div style='text-align: right'>243,240</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <div style='text-align: right'>70,920</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <div style='text-align: right'>8,940</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>6,263,880</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>6,263,880</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>7,803,720</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>8,568,142</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>5,062,993</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,200</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,232</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>728</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>456</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>429</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>117</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>7,200</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,160</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>15,850,710</div>  |
| inner_verifier | CoreAir | For | BNE | <div style='text-align: right'>32,819,160</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>429</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>507</div>  |
| inner_verifier | CoreAir | For | JAL | <div style='text-align: right'>1,117,740</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>57,960</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>912</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,059,780</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,320</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>569,700</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <div style='text-align: right'>369,480</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <div style='text-align: right'>7,281,780</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: green">(-17,460 [-3.1%])</span> <div style='text-align: right'>538,020</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>413,580</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,260</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>60,360</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>3,366</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>1,989</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>214,852</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <div style='text-align: right'>743,280</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>871,500</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,067</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>1,300,920</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>61,116</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>36,114</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>505,172</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <div style='text-align: right'>2,488,800</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <div style='text-align: right'>48,044,160</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>688,380</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <div style='text-align: right'>17,940,420</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,737</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>695,700</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>3,844,140</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>16,329,280</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: green">(-44 [-0.0%])</span> <div style='text-align: right'>492,998</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: green">(-26 [-0.0%])</span> <div style='text-align: right'>291,317</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>1,215,924</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>50,040</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,272</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>4,888</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>42,720</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,892</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,118</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>24,092</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>102,800</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>156,728</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>92,612</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>153,596</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>56,298</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>33,137</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>616,800</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>665,190</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>360</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>247,770</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>5,520</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,122</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>663</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,019,632</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>605,022</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>357,656</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>235,450</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,383,422</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>213,636</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>674,640</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>45,276</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>26,754</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>28,424</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>669,360</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>235,220</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>742,800</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>521,202</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>308,126</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>202,912</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>58,596</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <div style='text-align: right'>6,108,600</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>5,773,650</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>3,850,008</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>12,157,920</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>26,486</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>83,640</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>454,252</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>1,447,980</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>553,520</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>455,444</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>269,126</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>958,436</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>4,283,950</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <div style='text-align: right'>70,101,000</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>11,683,500</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>4,283,950</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>5,062,850</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>37,680</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>462</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>273</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>22,344</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>7,200</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>1,914</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,131</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>421,200</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>38,040</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,328</div>  | <div style='text-align: right'>104</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>60</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>4,480</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>335,544,320</div>  | <div style='text-align: right'>98</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>60</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/3b9ee416f89e23890a1ca47d00a8d8c1ce71c565
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11471094631)
