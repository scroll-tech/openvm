| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: red">(+3.0 [+0.2%])</span> <div style='text-align: right'>1,372.0</div>  | <span style="color: green">(-417 [-0.0%])</span> <div style='text-align: right'>1,914,735</div>  | <span style="color: green">(-280 [-0.1%])</span> <div style='text-align: right'>276,045</div>  | <span style="color: red">(+4.0 [+0.3%])</span> <div style='text-align: right'>1,391.0</div>  | <span style="color: red">(+1.0 [+5.6%])</span> <div style='text-align: right'>19.0</div>  |  |
| inner_verifier | <span style="color: green">(-1,165.0 [-1.4%])</span> <div style='text-align: right'>82,636.0</div>  | <span style="color: green">(-51,642,368 [-7.3%])</span> <div style='text-align: right'>655,163,416</div>  | <span style="color: green">(-47,496,354 [-14.7%])</span> <div style='text-align: right'>275,546,541</div>  | <span style="color: green">(-4,386.0 [-4.5%])</span> <div style='text-align: right'>92,103.0</div>  | <span style="color: green">(-3,221.0 [-25.4%])</span> <div style='text-align: right'>9,467.0</div>  | <span style="color: red">(+184.0 [+0.4%])</span> <div style='text-align: right'>42,684.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | BranchEqual | <div style='text-align: right'>5</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | LoadStore | <div style='text-align: right'>20</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| inner_verifier | BranchEqual | <span style="color: red">(+5,942 [+0.8%])</span> <div style='text-align: right'>712,125</div>  |
| inner_verifier | Core | <span style="color: green">(-2,931,067 [-92.9%])</span> <div style='text-align: right'>225,510</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+10,655 [+0.7%])</span> <div style='text-align: right'>1,604,764</div>  |
| inner_verifier | FieldExtension | <span style="color: red">(+3,624 [+0.4%])</span> <div style='text-align: right'>863,348</div>  |
| inner_verifier | Jal | <span style="color: red">(+609 [+2.1%])</span> <div style='text-align: right'>30,215</div>  |
| inner_verifier | LoadStore | <div style='text-align: right'>2,951,984</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+8,330 [+0.4%])</span> <div style='text-align: right'>1,988,543</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+4,144 [+0.4%])</span> <div style='text-align: right'>994,315</div>  |
| inner_verifier | Memory AccessAdapter<8> | <span style="color: red">(+128 [+0.4%])</span> <div style='text-align: right'>33,772</div>  |
| inner_verifier | Memory Boundary | <span style="color: red">(+1,523 [+0.3%])</span> <div style='text-align: right'>599,443</div>  |
| inner_verifier | Poseidon2 | <span style="color: red">(+64 [+0.3%])</span> <div style='text-align: right'>20,503</div>  |
| inner_verifier | ProgramChip | <span style="color: red">(+798 [+0.4%])</span> <div style='text-align: right'>203,243</div>  |

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
| inner_verifier | AddE | FE4ADD | <span style="color: red">(+927 [+0.4%])</span> <div style='text-align: right'>228,950</div>  |
| inner_verifier | AddEFFI | LOADW | <span style="color: red">(+2 [+1.5%])</span> <div style='text-align: right'>134</div>  |
| inner_verifier | AddEFFI | STOREW | <span style="color: red">(+6 [+1.5%])</span> <div style='text-align: right'>402</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+8 [+3.6%])</span> <div style='text-align: right'>232</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+128 [+0.2%])</span> <div style='text-align: right'>67,612</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+432 [+3.2%])</span> <div style='text-align: right'>14,032</div>  |
| inner_verifier | AddV | ADD | <span style="color: red">(+91 [+1.5%])</span> <div style='text-align: right'>6,330</div>  |
| inner_verifier | AddVI | ADD | <span style="color: red">(+2,044 [+0.7%])</span> <div style='text-align: right'>280,638</div>  |
| inner_verifier | Alloc | ADD | <span style="color: red">(+221 [+0.9%])</span> <div style='text-align: right'>24,731</div>  |
| inner_verifier | Alloc | LOADW | <span style="color: red">(+221 [+0.9%])</span> <div style='text-align: right'>24,731</div>  |
| inner_verifier | Alloc | MUL | <span style="color: red">(+118 [+0.8%])</span> <div style='text-align: right'>14,927</div>  |
| inner_verifier | AssertEqE | BNE | <span style="color: red">(+4 [+2.9%])</span> <div style='text-align: right'>144</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <span style="color: red">(+11 [+0.9%])</span> <div style='text-align: right'>1,215</div>  |
| inner_verifier | AssertEqVI | BNE | <span style="color: red">(+11 [+6.4%])</span> <div style='text-align: right'>182</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: red">(+735 [+0.7%])</span> <div style='text-align: right'>107,842</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: red">(+735 [+0.7%])</span> <div style='text-align: right'>107,842</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: red">(+884 [+0.4%])</span> <div style='text-align: right'>199,845</div>  |
| inner_verifier | DivEIN | BBE4DIV | <span style="color: red">(+3 [+8.3%])</span> <div style='text-align: right'>39</div>  |
| inner_verifier | DivEIN | STOREW | <span style="color: red">(+12 [+8.3%])</span> <div style='text-align: right'>156</div>  |
| inner_verifier | DivFIN | DIV | <span style="color: red">(+7 [+8.1%])</span> <div style='text-align: right'>93</div>  |
| inner_verifier | For | ADD | <span style="color: red">(+3,703 [+0.7%])</span> <div style='text-align: right'>544,346</div>  |
| inner_verifier | For | BNE | <span style="color: red">(+4,202 [+0.7%])</span> <div style='text-align: right'>564,596</div>  |
| inner_verifier | For | JAL | <span style="color: red">(+499 [+2.5%])</span> <div style='text-align: right'>20,250</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,029</div>  |
| inner_verifier | For | STOREW | <span style="color: red">(+499 [+2.7%])</span> <div style='text-align: right'>19,221</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <span style="color: red">(+103 [+1.1%])</span> <div style='text-align: right'>9,804</div>  |
| inner_verifier | IfEq | BNE | <span style="color: red">(+755 [+10.0%])</span> <div style='text-align: right'>8,342</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: red">(+954 [+0.8%])</span> <div style='text-align: right'>125,611</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+109 [+1.1%])</span> <div style='text-align: right'>9,943</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <span style="color: red">(+1 [+5.0%])</span> <div style='text-align: right'>21</div>  |
| inner_verifier | IfNeI | BEQ | <span style="color: red">(+5 [+0.5%])</span> <div style='text-align: right'>1,084</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+16 [+0.1%])</span> <div style='text-align: right'>12,524</div>  |
| inner_verifier | ImmF | STOREW | <span style="color: red">(+170 [+1.1%])</span> <div style='text-align: right'>16,043</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: red">(+92 [+0.4%])</span> <div style='text-align: right'>22,395</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: red">(+176 [+0.4%])</span> <div style='text-align: right'>42,408</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: red">(+4,708 [+0.6%])</span> <div style='text-align: right'>823,260</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: red">(+1,851 [+0.6%])</span> <div style='text-align: right'>307,710</div>  |
| inner_verifier | LoadV | LOADW | <span style="color: red">(+124 [+1.0%])</span> <div style='text-align: right'>11,967</div>  |
| inner_verifier | LoadV | LOADW2 | <span style="color: red">(+3,304 [+4.7%])</span> <div style='text-align: right'>74,233</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: red">(+1,773 [+0.4%])</span> <div style='text-align: right'>417,900</div>  |
| inner_verifier | MulEF | MUL | <span style="color: red">(+24 [+1.4%])</span> <div style='text-align: right'>1,740</div>  |
| inner_verifier | MulEFI | MUL | <span style="color: red">(+12 [+0.8%])</span> <div style='text-align: right'>1,436</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: red">(+35 [+1.3%])</span> <div style='text-align: right'>2,755</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: red">(+140 [+1.3%])</span> <div style='text-align: right'>11,020</div>  |
| inner_verifier | MulF | MUL | <span style="color: red">(+766 [+3.1%])</span> <div style='text-align: right'>25,143</div>  |
| inner_verifier | MulFI | MUL | <span style="color: red">(+1 [+7.1%])</span> <div style='text-align: right'>15</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <span style="color: red">(+91 [+1.1%])</span> <div style='text-align: right'>8,532</div>  |
| inner_verifier | NegE | MUL | <span style="color: red">(+4 [+2.0%])</span> <div style='text-align: right'>208</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,287</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+64 [+0.5%])</span> <div style='text-align: right'>13,216</div>  |
| inner_verifier | StoreE | STOREW | <span style="color: red">(+12 [+0.1%])</span> <div style='text-align: right'>11,280</div>  |
| inner_verifier | StoreE | STOREW2 | <span style="color: red">(+672 [+5.4%])</span> <div style='text-align: right'>13,172</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,388</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: red">(+549 [+0.5%])</span> <div style='text-align: right'>104,507</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: red">(+709 [+0.4%])</span> <div style='text-align: right'>196,032</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: red">(+812 [+0.4%])</span> <div style='text-align: right'>206,518</div>  |
| inner_verifier | StoreV | STOREW | <span style="color: red">(+15 [+1.1%])</span> <div style='text-align: right'>1,439</div>  |
| inner_verifier | StoreV | STOREW2 | <span style="color: red">(+665 [+2.6%])</span> <div style='text-align: right'>26,128</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>13,859</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: red">(+5,298 [+0.4%])</span> <div style='text-align: right'>1,196,844</div>  |
| inner_verifier | SubEF | SUB | <span style="color: red">(+1,766 [+0.4%])</span> <div style='text-align: right'>398,948</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,256</div>  |
| inner_verifier | SubEI | ADD | <span style="color: red">(+24 [+8.3%])</span> <div style='text-align: right'>312</div>  |
| inner_verifier | SubV | SUB | <span style="color: red">(+505 [+3.3%])</span> <div style='text-align: right'>15,891</div>  |
| inner_verifier | SubVI | SUB | <span style="color: red">(+1 [+0.1%])</span> <div style='text-align: right'>1,271</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>40</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <div style='text-align: right'>44</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>69</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>41</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>11</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>46</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>328</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <div style='text-align: right'>88</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>123</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <div style='text-align: right'>418</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <div style='text-align: right'>22</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <span style="color: red">(+37,080 [+0.4%])</span> <div style='text-align: right'>9,158,000</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+2,376 [+0.2%])</span> <div style='text-align: right'>1,112,694</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+1,404 [+0.2%])</span> <div style='text-align: right'>657,501</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <span style="color: green">(-2,596 [-0.2%])</span> <div style='text-align: right'>1,194,556</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>5,494</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: red">(+11 [+1.2%])</span> <div style='text-align: right'>957</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: red">(+13 [+1.2%])</span> <div style='text-align: right'>1,131</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>16,482</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: red">(+11 [+1.2%])</span> <div style='text-align: right'>957</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>792</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <span style="color: red">(+240 [+3.6%])</span> <div style='text-align: right'>6,960</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: red">(+66 [+7.5%])</span> <div style='text-align: right'>946</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: red">(+39 [+7.5%])</span> <div style='text-align: right'>559</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <span style="color: red">(+3,840 [+0.2%])</span> <div style='text-align: right'>2,028,360</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+1,320 [+0.3%])</span> <div style='text-align: right'>380,160</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+780 [+0.3%])</span> <div style='text-align: right'>224,640</div>  |
| inner_verifier | Boundary | AddEI | ADD | <span style="color: green">(-968 [-0.1%])</span> <div style='text-align: right'>654,192</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+12,960 [+3.2%])</span> <div style='text-align: right'>420,960</div>  |
| inner_verifier | Boundary | AddFI | ADD | <span style="color: green">(-11 [-4.2%])</span> <div style='text-align: right'>253</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <span style="color: red">(+2,730 [+1.5%])</span> <div style='text-align: right'>189,900</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <span style="color: red">(+61,320 [+0.7%])</span> <div style='text-align: right'>8,419,140</div>  |
| inner_verifier | Boundary | AddVI | ADD | <span style="color: red">(+11 [+0.1%])</span> <div style='text-align: right'>8,679</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <span style="color: red">(+6,630 [+0.9%])</span> <div style='text-align: right'>741,930</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>1,013,971</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <span style="color: red">(+3,540 [+0.8%])</span> <div style='text-align: right'>447,810</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <span style="color: red">(+92 [+2.9%])</span> <div style='text-align: right'>3,312</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <span style="color: red">(+22 [+2.9%])</span> <div style='text-align: right'>792</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <span style="color: red">(+13 [+2.9%])</span> <div style='text-align: right'>468</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <span style="color: red">(+253 [+0.9%])</span> <div style='text-align: right'>27,945</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <span style="color: red">(+253 [+6.4%])</span> <div style='text-align: right'>4,186</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-3,841,887 [-65.2%])</span> <div style='text-align: right'>2,048,998</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-3,841,887 [-65.2%])</span> <div style='text-align: right'>2,048,998</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <span style="color: red">(+35,360 [+0.4%])</span> <div style='text-align: right'>7,993,800</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: red">(+38,874 [+0.4%])</span> <div style='text-align: right'>8,777,164</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: red">(+22,971 [+0.4%])</span> <div style='text-align: right'>5,186,506</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <span style="color: red">(+120 [+8.3%])</span> <div style='text-align: right'>1,560</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+66 [+4.3%])</span> <div style='text-align: right'>1,606</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+39 [+4.3%])</span> <div style='text-align: right'>949</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>396</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>6,396</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <span style="color: red">(+44 [+8.5%])</span> <div style='text-align: right'>561</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <span style="color: red">(+13 [+9.1%])</span> <div style='text-align: right'>156</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <span style="color: red">(+210 [+8.1%])</span> <div style='text-align: right'>2,790</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <span style="color: red">(+111,090 [+0.7%])</span> <div style='text-align: right'>16,330,380</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <span style="color: red">(+96,646 [+0.7%])</span> <div style='text-align: right'>12,985,708</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <span style="color: red">(+4,990 [+2.5%])</span> <div style='text-align: right'>202,500</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <span style="color: red">(+22 [+4.7%])</span> <div style='text-align: right'>495</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <span style="color: red">(+26 [+4.7%])</span> <div style='text-align: right'>585</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>42,189</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>231</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>788,061</div>  |
| inner_verifier | Boundary | For | STOREW | <span style="color: red">(+22 [+4.1%])</span> <div style='text-align: right'>561</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <span style="color: green">(-792 [-65.5%])</span> <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <span style="color: green">(-347,279 [-65.1%])</span> <div style='text-align: right'>186,276</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <span style="color: red">(+17,365 [+10.0%])</span> <div style='text-align: right'>191,866</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <span style="color: red">(+21,942 [+0.8%])</span> <div style='text-align: right'>2,889,053</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: red">(+1,090 [+1.1%])</span> <div style='text-align: right'>99,430</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>158,539</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <span style="color: red">(+10 [+5.0%])</span> <div style='text-align: right'>210</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <span style="color: red">(+115 [+0.5%])</span> <div style='text-align: right'>24,932</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>513,484</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: green">(-22 [-0.5%])</span> <div style='text-align: right'>3,982</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: green">(-13 [-0.5%])</span> <div style='text-align: right'>2,353</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <span style="color: green">(-132 [-0.1%])</span> <div style='text-align: right'>124,168</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>657,763</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>1,353</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>918,195</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>1,738,728</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: green">(-2,596 [-4.2%])</span> <div style='text-align: right'>59,246</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: green">(-1,534 [-4.2%])</span> <div style='text-align: right'>35,009</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <span style="color: red">(+9,504 [+3.2%])</span> <div style='text-align: right'>309,760</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>33,753,660</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>470,393</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <span style="color: green">(-11 [-3.8%])</span> <div style='text-align: right'>275</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>12,616,110</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <span style="color: red">(+22 [+7.4%])</span> <div style='text-align: right'>319</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>490,647</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>7,975</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>3,043,553</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <span style="color: red">(+70,920 [+0.4%])</span> <div style='text-align: right'>16,716,000</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+792 [+0.2%])</span> <div style='text-align: right'>498,872</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+468 [+0.2%])</span> <div style='text-align: right'>294,788</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <span style="color: green">(-4,576 [-0.7%])</span> <div style='text-align: right'>697,972</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <span style="color: red">(+720 [+1.4%])</span> <div style='text-align: right'>52,200</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: red">(+44 [+0.5%])</span> <div style='text-align: right'>8,492</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: red">(+26 [+0.5%])</span> <div style='text-align: right'>5,018</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <span style="color: red">(+360 [+0.8%])</span> <div style='text-align: right'>43,080</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: red">(+198 [+10.5%])</span> <div style='text-align: right'>2,090</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: red">(+117 [+10.5%])</span> <div style='text-align: right'>1,235</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <span style="color: green">(-176 [-1.3%])</span> <div style='text-align: right'>13,640</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <span style="color: red">(+1,400 [+1.3%])</span> <div style='text-align: right'>110,200</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+924 [+0.6%])</span> <div style='text-align: right'>163,790</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+546 [+0.6%])</span> <div style='text-align: right'>96,785</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <span style="color: green">(-44 [-0.1%])</span> <div style='text-align: right'>87,252</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>451,820</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: red">(+770 [+1.3%])</span> <div style='text-align: right'>60,368</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: red">(+455 [+1.3%])</span> <div style='text-align: right'>35,542</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <span style="color: red">(+22,980 [+3.1%])</span> <div style='text-align: right'>754,290</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <span style="color: red">(+30 [+7.1%])</span> <div style='text-align: right'>450</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <span style="color: red">(+2,730 [+1.1%])</span> <div style='text-align: right'>255,960</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <span style="color: red">(+120 [+2.0%])</span> <div style='text-align: right'>6,240</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: green">(-22 [-1.6%])</span> <div style='text-align: right'>1,320</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: green">(-13 [-1.6%])</span> <div style='text-align: right'>780</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>1,408</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,045,966</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+3,025 [+0.5%])</span> <div style='text-align: right'>621,951</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+1,651 [+0.5%])</span> <div style='text-align: right'>367,796</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+1,088 [+0.5%])</span> <div style='text-align: right'>242,267</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+26,752 [+0.5%])</span> <div style='text-align: right'>5,524,288</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>462,480</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <span style="color: red">(+132 [+0.1%])</span> <div style='text-align: right'>124,080</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>540,052</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <span style="color: red">(+3,696 [+7.0%])</span> <div style='text-align: right'>56,364</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <span style="color: red">(+2,184 [+7.0%])</span> <div style='text-align: right'>33,306</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>16,456</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>548,908</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>147,268</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>4,284,787</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: red">(+3,025 [+0.6%])</span> <div style='text-align: right'>532,587</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: red">(+1,651 [+0.5%])</span> <div style='text-align: right'>314,990</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: red">(+1,088 [+0.5%])</span> <div style='text-align: right'>207,587</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <span style="color: red">(+176 [+0.5%])</span> <div style='text-align: right'>34,452</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <span style="color: red">(+21,270 [+0.4%])</span> <div style='text-align: right'>5,880,960</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>8,467,238</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: red">(+8,932 [+0.4%])</span> <div style='text-align: right'>2,271,698</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>58,999</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <span style="color: red">(+165 [+1.1%])</span> <div style='text-align: right'>15,829</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>1,071,248</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <span style="color: red">(+7,315 [+2.6%])</span> <div style='text-align: right'>284,933</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <span style="color: red">(+80 [+0.0%])</span> <div style='text-align: right'>554,360</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-242 [-0.1%])</span> <div style='text-align: right'>455,356</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-143 [-0.1%])</span> <div style='text-align: right'>269,074</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <span style="color: green">(-440 [-0.1%])</span> <div style='text-align: right'>554,180</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>49,070,604</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: red">(+19,426 [+0.4%])</span> <div style='text-align: right'>4,388,428</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <span style="color: red">(+52,980 [+0.4%])</span> <div style='text-align: right'>11,968,440</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: red">(+19,426 [+0.4%])</span> <div style='text-align: right'>4,388,428</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: red">(+22,958 [+0.4%])</span> <div style='text-align: right'>5,186,324</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>37,680</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: red">(+264 [+54.5%])</span> <div style='text-align: right'>748</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: red">(+156 [+54.5%])</span> <div style='text-align: right'>442</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <span style="color: green">(-572 [-4.5%])</span> <div style='text-align: right'>12,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <span style="color: red">(+720 [+8.3%])</span> <div style='text-align: right'>9,360</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: red">(+110 [+4.9%])</span> <div style='text-align: right'>2,354</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: red">(+65 [+4.9%])</span> <div style='text-align: right'>1,391</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <span style="color: red">(+15,150 [+3.3%])</span> <div style='text-align: right'>476,730</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <span style="color: red">(+30 [+0.1%])</span> <div style='text-align: right'>38,130</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>2,944</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | CoreAir | <span style="color: green">(-3,137 [-99.0%])</span> <div style='text-align: right'>31</div>  | <span style="color: green">(-67 [-80.7%])</span> <div style='text-align: right'>16</div>  | <span style="color: green">(-16 [-84.2%])</span> <div style='text-align: right'>3</div>  | <span style="color: green">(-36 [-65.5%])</span> <div style='text-align: right'>19</div>  | <span style="color: green">(-32 [-72.7%])</span> <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <span style="color: green">(-31 [-96.9%])</span> <div style='text-align: right'>1</div>  |
| bench_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>2,720</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>568</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | CoreAir | <span style="color: green">(-307,494,912 [-97.8%])</span> <div style='text-align: right'>7,077,888</div>  | <span style="color: green">(-62 [-80.5%])</span> <div style='text-align: right'>15</div>  | <span style="color: green">(-16 [-84.2%])</span> <div style='text-align: right'>3</div>  | <span style="color: green">(-36 [-65.5%])</span> <div style='text-align: right'>19</div>  | <span style="color: green">(-12 [-60.0%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: green">(-4 [-50.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-3,932,160 [-93.8%])</span> <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>255,852,544</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11493051621)
