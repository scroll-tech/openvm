| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-3.0 [-0.2%])</span> <div style='text-align: right'>1,379.0</div>  | <div style='text-align: right'>1,916,688</div>  | <div style='text-align: right'>145,728</div>  | <span style="color: green">(-3.0 [-0.2%])</span> <div style='text-align: right'>1,401.0</div>  | <div style='text-align: right'>22.0</div>  |  |
| inner_verifier | <span style="color: green">(-728.0 [-0.9%])</span> <div style='text-align: right'>84,728.0</div>  | <div style='text-align: right'>715,194,392</div>  | <span style="color: red">(+18,830 [+0.0%])</span> <div style='text-align: right'>328,202,057</div>  | <span style="color: green">(-767.0 [-0.8%])</span> <div style='text-align: right'>98,007.0</div>  | <span style="color: green">(-39.0 [-0.3%])</span> <div style='text-align: right'>13,279.0</div>  | <span style="color: green">(-351.0 [-0.8%])</span> <div style='text-align: right'>45,951.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | BranchEqual | <div style='text-align: right'>5</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>20</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| inner_verifier | BranchEqual | <div style='text-align: right'>707,539</div>  |
| inner_verifier | Core | <div style='text-align: right'>3,163,707</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+46 [+0.0%])</span> <div style='text-align: right'>1,597,225</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>861,824</div>  |
| inner_verifier | Jal | <span style="color: red">(+1,598 [+5.5%])</span> <div style='text-align: right'>30,429</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+84 [+0.0%])</span> <div style='text-align: right'>1,985,023</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+42 [+0.0%])</span> <div style='text-align: right'>992,576</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>33,686</div>  |
| inner_verifier | Memory Boundary | <div style='text-align: right'>598,268</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>20,460</div>  |
| inner_verifier | ProgramChip | <div style='text-align: right'>202,821</div>  |

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
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>228,553</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>396</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>220</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>67,576</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+46 [+0.3%])</span> <div style='text-align: right'>13,648</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>6,239</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>279,098</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>14,809</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>140</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,204</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>171</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <div style='text-align: right'>107,359</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <div style='text-align: right'>107,359</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>199,465</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>36</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>144</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>86</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>541,747</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>561,498</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>19,751</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,029</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>18,722</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>9,701</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>7,587</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>124,909</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+1,598 [+17.6%])</span> <div style='text-align: right'>10,657</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>20</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,079</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>12,568</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,873</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>22,345</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>42,328</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>820,568</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>306,615</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>11,843</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>70,929</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>417,165</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,716</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,440</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>2,728</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>10,912</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>24,377</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>14</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>8,441</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>204</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,287</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>13,173</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>11,268</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,500</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,388</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>104,210</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>195,671</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>206,054</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,424</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>25,463</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>13,877</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>1,194,570</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>398,190</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,256</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>288</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>15,386</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,270</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>110</div>  |
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
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>110</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>69</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>55</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>46</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <div style='text-align: right'>152</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>440</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>110</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>165</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <div style='text-align: right'>722</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>110</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>110</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>9,142,120</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>1,111,968</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>657,072</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>2,070,848</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>968</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,144</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>7,260</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>968</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,254</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>21,780</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>6,600</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>792</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>468</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,128</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>2,027,280</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+462 [+0.1%])</span> <div style='text-align: right'>379,390</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+273 [+0.1%])</span> <div style='text-align: right'>224,185</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>1,130,424</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+1,380 [+0.3%])</span> <div style='text-align: right'>409,440</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>456</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>187,170</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>8,372,940</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>14,915</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>735,300</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>1,348,050</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>444,270</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>3,220</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>455</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>27,692</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>3,933</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>5,904,745</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>5,904,745</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>7,978,600</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>8,760,466</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>5,176,639</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,440</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,474</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>871</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>456</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>517</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>143</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>7,920</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,580</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>16,252,410</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>12,914,454</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>197,510</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>473</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>559</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>56,595</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>969</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,029,710</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,210</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>533,555</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>174,501</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>2,872,907</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: red">(+15,980 [+17.6%])</span> <div style='text-align: right'>106,570</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>158,539</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>200</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>24,817</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>4,092</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>2,418</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>214,700</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <div style='text-align: right'>691,240</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>873,015</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,067</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>1,228,975</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>62,216</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>36,764</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>517,104</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <div style='text-align: right'>2,328,040</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <div style='text-align: right'>45,131,240</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>631,015</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>513</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <div style='text-align: right'>16,863,825</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,737</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>651,365</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>3,901,095</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>16,686,600</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+462 [+0.1%])</span> <div style='text-align: right'>499,466</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+273 [+0.1%])</span> <div style='text-align: right'>295,139</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>1,211,820</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>51,480</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,492</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>5,018</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>43,200</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>2,002</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,183</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>23,864</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>109,120</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>163,878</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>96,837</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>153,292</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>59,774</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>35,191</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>600,160</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>731,310</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>420</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>253,230</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>6,120</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,276</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>754</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,045,966</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>620,312</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>366,964</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>241,536</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,506,314</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>214,092</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>619,740</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>52,668</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>31,122</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>28,424</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>687,500</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>254,372</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>736,340</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>530,948</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>314,158</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>206,856</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>59,204</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <div style='text-align: right'>5,731,550</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>5,870,130</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>3,915,026</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>11,332,970</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>27,056</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>78,320</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>479,522</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>1,400,465</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>555,080</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>456,016</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>269,464</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>957,904</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>4,380,090</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <div style='text-align: right'>65,701,350</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>11,945,700</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>4,380,090</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>5,176,470</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>37,680</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>299</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>22,116</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>8,640</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,376</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,404</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>461,580</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>38,100</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,168</div>  | <div style='text-align: right'>83</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>55</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>568</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
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
| inner_verifier | CoreAir | <div style='text-align: right'>314,572,800</div>  | <div style='text-align: right'>77</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>55</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/48c321bdf2d7d5fd8b5594755ce4d68d306bc523/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/48c321bdf2d7d5fd8b5594755ce4d68d306bc523
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11486085336)
