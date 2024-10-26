| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,389.0</div>  | <span style="color: green">(-28 [-0.0%])</span> <div style='text-align: right'>1,914,704</div>  | <div style='text-align: right'>276,045</div>  | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,408.0</div>  | <div style='text-align: right'>19.0</div>  |  |
| inner_verifier | <span style="color: green">(-1,683.0 [-2.0%])</span> <div style='text-align: right'>81,022.0</div>  | <span style="color: green">(-2,621,440 [-0.4%])</span> <div style='text-align: right'>651,755,544</div>  | <span style="color: green">(-4,388,908 [-1.6%])</span> <div style='text-align: right'>270,322,468</div>  | <span style="color: green">(-2,089.0 [-2.3%])</span> <div style='text-align: right'>90,411.0</div>  | <span style="color: green">(-406.0 [-4.1%])</span> <div style='text-align: right'>9,389.0</div>  | <span style="color: red">(+131.0 [+0.3%])</span> <div style='text-align: right'>42,788.0</div>  |

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
| inner_verifier | BranchEqual | <span style="color: green">(-7,185 [-1.0%])</span> <div style='text-align: right'>704,605</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-13,447 [-0.8%])</span> <div style='text-align: right'>1,590,551</div>  |
| inner_verifier | FieldExtension | <span style="color: green">(-5,590 [-0.6%])</span> <div style='text-align: right'>857,223</div>  |
| inner_verifier | Jal | <span style="color: green">(-557 [-1.9%])</span> <div style='text-align: right'>29,091</div>  |
| inner_verifier | LoadStore | <span style="color: green">(-25,345 [-0.9%])</span> <div style='text-align: right'>2,925,017</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-12,920 [-0.7%])</span> <div style='text-align: right'>1,974,371</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-6,460 [-0.7%])</span> <div style='text-align: right'>987,229</div>  |
| inner_verifier | Memory AccessAdapter<8> | <span style="color: green">(-170 [-0.5%])</span> <div style='text-align: right'>33,560</div>  |
| inner_verifier | Memory Boundary | <span style="color: green">(-1,842 [-0.3%])</span> <div style='text-align: right'>597,523</div>  |
| inner_verifier | Phantom | <div style='text-align: right'>223,349</div>  |
| inner_verifier | Poseidon2 | <span style="color: green">(-85 [-0.4%])</span> <div style='text-align: right'>20,397</div>  |
| inner_verifier | ProgramChip | <span style="color: green">(-1,096 [-0.5%])</span> <div style='text-align: right'>202,045</div>  |

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
| inner_verifier | AddE | FE4ADD | <span style="color: green">(-1,421 [-0.6%])</span> <div style='text-align: right'>227,386</div>  |
| inner_verifier | AddEFFI | LOADW | <span style="color: green">(-2 [-1.5%])</span> <div style='text-align: right'>131</div>  |
| inner_verifier | AddEFFI | STOREW | <span style="color: green">(-6 [-1.5%])</span> <div style='text-align: right'>393</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: green">(-16 [-6.7%])</span> <div style='text-align: right'>224</div>  |
| inner_verifier | AddEI | ADD | <span style="color: green">(-160 [-0.2%])</span> <div style='text-align: right'>67,404</div>  |
| inner_verifier | AddFI | ADD | <span style="color: green">(-423 [-3.0%])</span> <div style='text-align: right'>13,620</div>  |
| inner_verifier | AddV | ADD | <span style="color: green">(-91 [-1.4%])</span> <div style='text-align: right'>6,239</div>  |
| inner_verifier | AddVI | ADD | <span style="color: green">(-2,506 [-0.9%])</span> <div style='text-align: right'>278,008</div>  |
| inner_verifier | Alloc | ADD | <span style="color: green">(-221 [-0.9%])</span> <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | LOADW | <span style="color: green">(-221 [-0.9%])</span> <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | MUL | <span style="color: green">(-118 [-0.8%])</span> <div style='text-align: right'>14,809</div>  |
| inner_verifier | AssertEqE | BNE | <span style="color: green">(-4 [-2.8%])</span> <div style='text-align: right'>140</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <span style="color: green">(-11 [-0.9%])</span> <div style='text-align: right'>1,204</div>  |
| inner_verifier | AssertEqVI | BNE | <span style="color: green">(-11 [-6.0%])</span> <div style='text-align: right'>171</div>  |
| inner_verifier | CycleTrackerEnd | PHANTOM | <div style='text-align: right'>106,813</div>  |
| inner_verifier | CycleTrackerStart | PHANTOM | <div style='text-align: right'>106,813</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: green">(-1,346 [-0.7%])</span> <div style='text-align: right'>198,373</div>  |
| inner_verifier | DivEIN | BBE4DIV | <span style="color: green">(-3 [-7.7%])</span> <div style='text-align: right'>36</div>  |
| inner_verifier | DivEIN | STOREW | <span style="color: green">(-12 [-7.7%])</span> <div style='text-align: right'>144</div>  |
| inner_verifier | DivFIN | DIV | <span style="color: green">(-7 [-7.5%])</span> <div style='text-align: right'>86</div>  |
| inner_verifier | For | ADD | <span style="color: green">(-4,715 [-0.9%])</span> <div style='text-align: right'>539,358</div>  |
| inner_verifier | For | BNE | <span style="color: green">(-5,214 [-0.9%])</span> <div style='text-align: right'>559,109</div>  |
| inner_verifier | For | JAL | <span style="color: green">(-499 [-2.5%])</span> <div style='text-align: right'>19,751</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,029</div>  |
| inner_verifier | For | STOREW | <span style="color: green">(-499 [-2.6%])</span> <div style='text-align: right'>18,722</div>  |
| inner_verifier | HintBitsF | PHANTOM | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | PHANTOM | <div style='text-align: right'>9,701</div>  |
| inner_verifier | IfEq | BNE | <span style="color: green">(-755 [-9.1%])</span> <div style='text-align: right'>7,587</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: green">(-1,185 [-0.9%])</span> <div style='text-align: right'>124,364</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: green">(-57 [-0.6%])</span> <div style='text-align: right'>9,319</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <span style="color: green">(-1 [-4.8%])</span> <div style='text-align: right'>20</div>  |
| inner_verifier | IfNeI | BEQ | <span style="color: green">(-5 [-0.5%])</span> <div style='text-align: right'>1,079</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: green">(-24 [-0.2%])</span> <div style='text-align: right'>12,492</div>  |
| inner_verifier | ImmF | STOREW | <span style="color: green">(-170 [-1.1%])</span> <div style='text-align: right'>15,873</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: green">(-134 [-0.6%])</span> <div style='text-align: right'>22,262</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: green">(-264 [-0.6%])</span> <div style='text-align: right'>42,120</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: green">(-6,556 [-0.8%])</span> <div style='text-align: right'>816,200</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,474</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: green">(-2,544 [-0.8%])</span> <div style='text-align: right'>304,979</div>  |
| inner_verifier | LoadV | LOADW | <span style="color: green">(-124 [-1.0%])</span> <div style='text-align: right'>11,843</div>  |
| inner_verifier | LoadV | LOADW2 | <span style="color: green">(-3,304 [-4.5%])</span> <div style='text-align: right'>70,929</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: green">(-2,778 [-0.7%])</span> <div style='text-align: right'>414,863</div>  |
| inner_verifier | MulEF | MUL | <span style="color: green">(-24 [-1.4%])</span> <div style='text-align: right'>1,716</div>  |
| inner_verifier | MulEFI | MUL | <span style="color: green">(-24 [-1.7%])</span> <div style='text-align: right'>1,408</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: green">(-35 [-1.3%])</span> <div style='text-align: right'>2,720</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: green">(-140 [-1.3%])</span> <div style='text-align: right'>10,880</div>  |
| inner_verifier | MulF | MUL | <span style="color: green">(-766 [-3.0%])</span> <div style='text-align: right'>24,377</div>  |
| inner_verifier | MulFI | MUL | <span style="color: green">(-1 [-6.7%])</span> <div style='text-align: right'>14</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <span style="color: green">(-91 [-1.1%])</span> <div style='text-align: right'>8,441</div>  |
| inner_verifier | NegE | MUL | <span style="color: green">(-4 [-1.9%])</span> <div style='text-align: right'>204</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,287</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-85 [-0.6%])</span> <div style='text-align: right'>13,110</div>  |
| inner_verifier | StoreE | STOREW | <span style="color: green">(-12 [-0.1%])</span> <div style='text-align: right'>11,268</div>  |
| inner_verifier | StoreE | STOREW2 | <span style="color: green">(-672 [-5.1%])</span> <div style='text-align: right'>12,500</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,388</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: green">(-780 [-0.7%])</span> <div style='text-align: right'>103,666</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: green">(-1,028 [-0.5%])</span> <div style='text-align: right'>194,918</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: green">(-1,131 [-0.5%])</span> <div style='text-align: right'>205,301</div>  |
| inner_verifier | StoreV | STOREW | <span style="color: green">(-15 [-1.0%])</span> <div style='text-align: right'>1,424</div>  |
| inner_verifier | StoreV | STOREW2 | <span style="color: green">(-665 [-2.5%])</span> <div style='text-align: right'>25,463</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: green">(-7 [-0.1%])</span> <div style='text-align: right'>13,845</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: green">(-8,070 [-0.7%])</span> <div style='text-align: right'>1,188,024</div>  |
| inner_verifier | SubEF | SUB | <span style="color: green">(-2,690 [-0.7%])</span> <div style='text-align: right'>396,008</div>  |
| inner_verifier | SubEFI | ADD | <span style="color: green">(-32 [-2.5%])</span> <div style='text-align: right'>1,224</div>  |
| inner_verifier | SubEI | ADD | <span style="color: green">(-24 [-7.7%])</span> <div style='text-align: right'>288</div>  |
| inner_verifier | SubV | SUB | <span style="color: green">(-505 [-3.2%])</span> <div style='text-align: right'>15,386</div>  |
| inner_verifier | SubVI | SUB | <span style="color: green">(-1 [-0.1%])</span> <div style='text-align: right'>1,270</div>  |
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
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <span style="color: green">(-56,840 [-0.6%])</span> <div style='text-align: right'>9,095,440</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: green">(-2,750 [-0.2%])</span> <div style='text-align: right'>1,109,724</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: green">(-1,625 [-0.2%])</span> <div style='text-align: right'>655,746</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>1,194,556</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <span style="color: green">(-82 [-1.5%])</span> <div style='text-align: right'>5,371</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: green">(-11 [-1.2%])</span> <div style='text-align: right'>924</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: green">(-13 [-1.2%])</span> <div style='text-align: right'>1,092</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <span style="color: green">(-246 [-1.5%])</span> <div style='text-align: right'>16,113</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: green">(-11 [-1.2%])</span> <div style='text-align: right'>924</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>792</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <span style="color: green">(-480 [-6.7%])</span> <div style='text-align: right'>6,720</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: green">(-110 [-11.4%])</span> <div style='text-align: right'>858</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: green">(-65 [-11.4%])</span> <div style='text-align: right'>507</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <span style="color: green">(-4,800 [-0.2%])</span> <div style='text-align: right'>2,022,120</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: green">(-1,408 [-0.4%])</span> <div style='text-align: right'>378,202</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: green">(-832 [-0.4%])</span> <div style='text-align: right'>223,483</div>  |
| inner_verifier | Boundary | AddEI | ADD | <span style="color: red">(+44 [+0.0%])</span> <div style='text-align: right'>654,236</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: green">(-12,690 [-3.0%])</span> <div style='text-align: right'>408,600</div>  |
| inner_verifier | Boundary | AddFI | ADD | <span style="color: red">(+11 [+4.3%])</span> <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <span style="color: green">(-2,730 [-1.4%])</span> <div style='text-align: right'>187,170</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <span style="color: green">(-75,180 [-0.9%])</span> <div style='text-align: right'>8,340,240</div>  |
| inner_verifier | Boundary | AddVI | ADD | <span style="color: green">(-11 [-0.1%])</span> <div style='text-align: right'>8,668</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <span style="color: green">(-6,630 [-0.9%])</span> <div style='text-align: right'>735,300</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <span style="color: green">(-9,061 [-0.9%])</span> <div style='text-align: right'>1,004,910</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <span style="color: green">(-3,540 [-0.8%])</span> <div style='text-align: right'>444,270</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <span style="color: green">(-92 [-2.8%])</span> <div style='text-align: right'>3,220</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <span style="color: green">(-22 [-2.8%])</span> <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <span style="color: green">(-13 [-2.8%])</span> <div style='text-align: right'>455</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <span style="color: green">(-253 [-0.9%])</span> <div style='text-align: right'>27,692</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <span style="color: green">(-253 [-6.0%])</span> <div style='text-align: right'>3,933</div>  |
| inner_verifier | PhantomAir | CycleTrackerEnd | PHANTOM | <div style='text-align: right'>640,878</div>  |
| inner_verifier | PhantomAir | CycleTrackerStart | PHANTOM | <div style='text-align: right'>640,878</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <span style="color: green">(-53,840 [-0.7%])</span> <div style='text-align: right'>7,934,920</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: green">(-59,202 [-0.7%])</span> <div style='text-align: right'>8,712,418</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: green">(-34,983 [-0.7%])</span> <div style='text-align: right'>5,148,247</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <span style="color: green">(-120 [-7.7%])</span> <div style='text-align: right'>1,440</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: green">(-110 [-6.8%])</span> <div style='text-align: right'>1,496</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: green">(-65 [-6.8%])</span> <div style='text-align: right'>884</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>396</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <span style="color: green">(-492 [-7.7%])</span> <div style='text-align: right'>5,904</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <span style="color: green">(-44 [-7.8%])</span> <div style='text-align: right'>517</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <span style="color: green">(-13 [-8.3%])</span> <div style='text-align: right'>143</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <span style="color: green">(-210 [-7.5%])</span> <div style='text-align: right'>2,580</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <span style="color: green">(-141,450 [-0.9%])</span> <div style='text-align: right'>16,180,740</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <span style="color: green">(-119,922 [-0.9%])</span> <div style='text-align: right'>12,859,507</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <span style="color: green">(-4,990 [-2.5%])</span> <div style='text-align: right'>197,510</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <span style="color: green">(-22 [-4.4%])</span> <div style='text-align: right'>473</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <span style="color: green">(-26 [-4.4%])</span> <div style='text-align: right'>559</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>42,189</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>231</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <span style="color: green">(-20,459 [-2.6%])</span> <div style='text-align: right'>767,602</div>  |
| inner_verifier | Boundary | For | STOREW | <span style="color: green">(-22 [-3.9%])</span> <div style='text-align: right'>539</div>  |
| inner_verifier | PhantomAir | HintBitsF | PHANTOM | <div style='text-align: right'>132</div>  |
| inner_verifier | PhantomAir | HintInputVec | PHANTOM | <div style='text-align: right'>58,206</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <span style="color: green">(-17,365 [-9.1%])</span> <div style='text-align: right'>174,501</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <span style="color: green">(-27,255 [-0.9%])</span> <div style='text-align: right'>2,860,372</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: green">(-570 [-0.6%])</span> <div style='text-align: right'>93,190</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>158,539</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <span style="color: green">(-10 [-4.8%])</span> <div style='text-align: right'>200</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <span style="color: green">(-115 [-0.5%])</span> <div style='text-align: right'>24,817</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <span style="color: green">(-984 [-0.2%])</span> <div style='text-align: right'>512,172</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: green">(-44 [-1.1%])</span> <div style='text-align: right'>3,806</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: green">(-26 [-1.1%])</span> <div style='text-align: right'>2,249</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <span style="color: green">(-44 [-0.0%])</span> <div style='text-align: right'>124,124</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <span style="color: green">(-6,970 [-1.1%])</span> <div style='text-align: right'>650,793</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>1,353</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <span style="color: green">(-5,494 [-0.6%])</span> <div style='text-align: right'>912,742</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <span style="color: green">(-10,824 [-0.6%])</span> <div style='text-align: right'>1,726,920</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: green">(-880 [-1.5%])</span> <div style='text-align: right'>58,190</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: green">(-520 [-1.5%])</span> <div style='text-align: right'>34,385</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>309,760</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <span style="color: green">(-268,796 [-0.8%])</span> <div style='text-align: right'>33,464,200</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>470,434</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <span style="color: red">(+11 [+4.0%])</span> <div style='text-align: right'>286</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <span style="color: green">(-104,304 [-0.8%])</span> <div style='text-align: right'>12,504,139</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <span style="color: green">(-22 [-6.9%])</span> <div style='text-align: right'>297</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <span style="color: green">(-5,084 [-1.0%])</span> <div style='text-align: right'>485,563</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>7,975</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <span style="color: green">(-135,464 [-4.5%])</span> <div style='text-align: right'>2,908,089</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <span style="color: green">(-111,120 [-0.7%])</span> <div style='text-align: right'>16,594,520</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: green">(-3,190 [-0.6%])</span> <div style='text-align: right'>495,132</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: green">(-1,885 [-0.6%])</span> <div style='text-align: right'>292,578</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>698,060</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <span style="color: green">(-720 [-1.4%])</span> <div style='text-align: right'>51,480</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: green">(-88 [-1.0%])</span> <div style='text-align: right'>8,426</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: green">(-52 [-1.0%])</span> <div style='text-align: right'>4,979</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <span style="color: green">(-720 [-1.7%])</span> <div style='text-align: right'>42,240</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: green">(-242 [-12.0%])</span> <div style='text-align: right'>1,782</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: green">(-143 [-12.0%])</span> <div style='text-align: right'>1,053</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>13,640</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <span style="color: green">(-1,400 [-1.3%])</span> <div style='text-align: right'>108,800</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: green">(-1,540 [-0.9%])</span> <div style='text-align: right'>162,382</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: green">(-910 [-0.9%])</span> <div style='text-align: right'>95,953</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>87,252</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <span style="color: green">(-5,740 [-1.3%])</span> <div style='text-align: right'>446,080</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: green">(-770 [-1.3%])</span> <div style='text-align: right'>59,598</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: green">(-455 [-1.3%])</span> <div style='text-align: right'>35,087</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <span style="color: green">(-22,980 [-3.0%])</span> <div style='text-align: right'>731,310</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <span style="color: green">(-30 [-6.7%])</span> <div style='text-align: right'>420</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <span style="color: green">(-2,730 [-1.1%])</span> <div style='text-align: right'>253,230</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <span style="color: green">(-120 [-1.9%])</span> <div style='text-align: right'>6,120</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: green">(-44 [-3.3%])</span> <div style='text-align: right'>1,298</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: green">(-26 [-3.3%])</span> <div style='text-align: right'>767</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>1,408</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,045,966</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-4,180 [-0.7%])</span> <div style='text-align: right'>617,309</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-2,470 [-0.7%])</span> <div style='text-align: right'>365,053</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-1,445 [-0.6%])</span> <div style='text-align: right'>240,465</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-35,530 [-0.6%])</span> <div style='text-align: right'>5,479,980</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <span style="color: green">(-492 [-0.1%])</span> <div style='text-align: right'>461,988</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <span style="color: green">(-132 [-0.1%])</span> <div style='text-align: right'>123,948</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <span style="color: green">(-27,552 [-5.1%])</span> <div style='text-align: right'>512,500</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <span style="color: green">(-3,696 [-6.6%])</span> <div style='text-align: right'>52,668</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <span style="color: green">(-2,184 [-6.6%])</span> <div style='text-align: right'>31,122</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>16,456</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>548,908</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>147,268</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <span style="color: green">(-31,980 [-0.7%])</span> <div style='text-align: right'>4,250,306</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: green">(-4,180 [-0.8%])</span> <div style='text-align: right'>527,945</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: green">(-2,470 [-0.8%])</span> <div style='text-align: right'>312,247</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: green">(-1,445 [-0.7%])</span> <div style='text-align: right'>205,785</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <span style="color: green">(-176 [-0.5%])</span> <div style='text-align: right'>34,276</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <span style="color: green">(-30,840 [-0.5%])</span> <div style='text-align: right'>5,847,540</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <span style="color: green">(-46,371 [-0.5%])</span> <div style='text-align: right'>8,417,341</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: green">(-12,441 [-0.5%])</span> <div style='text-align: right'>2,258,311</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <span style="color: green">(-615 [-1.0%])</span> <div style='text-align: right'>58,384</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <span style="color: green">(-165 [-1.0%])</span> <div style='text-align: right'>15,664</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <span style="color: green">(-27,265 [-2.5%])</span> <div style='text-align: right'>1,043,983</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <span style="color: green">(-7,315 [-2.6%])</span> <div style='text-align: right'>277,618</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <span style="color: green">(-280 [-0.1%])</span> <div style='text-align: right'>553,800</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-88 [-0.0%])</span> <div style='text-align: right'>455,048</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-52 [-0.0%])</span> <div style='text-align: right'>268,892</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>554,180</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <span style="color: green">(-330,870 [-0.7%])</span> <div style='text-align: right'>48,708,984</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: green">(-29,590 [-0.7%])</span> <div style='text-align: right'>4,356,077</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <span style="color: green">(-80,700 [-0.7%])</span> <div style='text-align: right'>11,880,240</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: green">(-29,590 [-0.7%])</span> <div style='text-align: right'>4,356,077</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: green">(-34,970 [-0.7%])</span> <div style='text-align: right'>5,148,091</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <span style="color: green">(-960 [-2.5%])</span> <div style='text-align: right'>36,720</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: green">(-176 [-23.5%])</span> <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: green">(-104 [-23.5%])</span> <div style='text-align: right'>338</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>12,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <span style="color: green">(-720 [-7.7%])</span> <div style='text-align: right'>8,640</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: green">(-132 [-5.7%])</span> <div style='text-align: right'>2,200</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: green">(-78 [-5.7%])</span> <div style='text-align: right'>1,300</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <span style="color: green">(-15,150 [-3.2%])</span> <div style='text-align: right'>461,580</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <span style="color: green">(-30 [-0.1%])</span> <div style='text-align: right'>38,100</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>2,944</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>2,720</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>568</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>255,852,544</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/660cd532d8f859642ccada0e57eec006150568e6/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/660cd532d8f859642ccada0e57eec006150568e6
AWS Instance Type: [64cpu-linux-arm64](https://instances.vantage.sh/aws/ec2/64cpu-linux-arm64)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11534733566)
