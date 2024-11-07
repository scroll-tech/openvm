| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: green">(-2 [-66.7%])</span> <div style='text-align: right'>1</div>  | <span style="color: red">(+370,970 [+2.2%])</span> <div style='text-align: right'>17,368,249</div>  | <span style="color: red">(+86 [+0.0%])</span> <div style='text-align: right'>427,483</div>  | <span style="color: green">(-2,413.0 [-52.6%])</span> <div style='text-align: right'>2,175.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |

| stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- |
| <div style='text-align: right'>10.0</div>  | <div style='text-align: right'>32</div>  |

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| verify_fibair | true | <span style="color: red">(+29.0 [+0.4%])</span> <div style='text-align: right'>6,644.0</div>  | <span style="color: red">(+370,970 [+2.2%])</span> <div style='text-align: right'>17,368,249</div>  | <span style="color: red">(+86 [+0.0%])</span> <div style='text-align: right'>427,483</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | <div style='text-align: right'>66,572</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | <span style="color: red">(+62 [+0.6%])</span> <div style='text-align: right'>10,956</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | <span style="color: red">(+24 [+0.0%])</span> <div style='text-align: right'>138,575</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | <div style='text-align: right'>191,828</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | <div style='text-align: right'>4,886</div>  |
| verify_fibair | FriMatOpeningAir | true | <div style='text-align: right'>800</div>  |
| verify_fibair | Memory AccessAdapter<2> | true | <span style="color: red">(+12 [+0.0%])</span> <div style='text-align: right'>35,429</div>  |
| verify_fibair | Memory AccessAdapter<4> | true | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>17,715</div>  |
| verify_fibair | Memory AccessAdapter<8> | true | <div style='text-align: right'>3,841</div>  |
| verify_fibair | Memory Boundary | true | <div style='text-align: right'>97,438</div>  |
| verify_fibair | PhantomAir | true | <div style='text-align: right'>11,746</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>2,620</div>  |
| verify_fibair | ProgramChip | true | <div style='text-align: right'>35,430</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair | true |  | JAL | <div style='text-align: right'>1</div>  |
| verify_fibair | true |  | STOREW | <div style='text-align: right'>2</div>  |
| verify_fibair | true | AddE | FE4ADD | <div style='text-align: right'>1,130</div>  |
| verify_fibair | true | AddEFFI | LOADW | <div style='text-align: right'>128</div>  |
| verify_fibair | true | AddEFFI | STOREW | <div style='text-align: right'>384</div>  |
| verify_fibair | true | AddEI | ADD | <div style='text-align: right'>5,664</div>  |
| verify_fibair | true | AddFI | ADD | <span style="color: red">(+24 [+0.9%])</span> <div style='text-align: right'>2,802</div>  |
| verify_fibair | true | AddV | ADD | <div style='text-align: right'>1,815</div>  |
| verify_fibair | true | AddVI | ADD | <div style='text-align: right'>35,697</div>  |
| verify_fibair | true | Alloc | ADD | <div style='text-align: right'>13,007</div>  |
| verify_fibair | true | Alloc | LOADW | <div style='text-align: right'>13,007</div>  |
| verify_fibair | true | Alloc | MUL | <div style='text-align: right'>8,976</div>  |
| verify_fibair | true | AssertEqE | BNE | <div style='text-align: right'>404</div>  |
| verify_fibair | true | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| verify_fibair | true | AssertEqF | BNE | <div style='text-align: right'>4,901</div>  |
| verify_fibair | true | AssertEqV | BNE | <div style='text-align: right'>3,540</div>  |
| verify_fibair | true | AssertEqVI | BNE | <div style='text-align: right'>20</div>  |
| verify_fibair | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>400</div>  |
| verify_fibair | true | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>400</div>  |
| verify_fibair | true | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>400</div>  |
| verify_fibair | true | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>800</div>  |
| verify_fibair | true | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>1,200</div>  |
| verify_fibair | true | CT-single-mat-reduced-opening | PHANTOM | <div style='text-align: right'>600</div>  |
| verify_fibair | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-verify-batch | PHANTOM | <div style='text-align: right'>400</div>  |
| verify_fibair | true | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>800</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>1,200</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>1,200</div>  |
| verify_fibair | true | CT-verify-query | PHANTOM | <div style='text-align: right'>200</div>  |
| verify_fibair | true | DivE | BBE4DIV | <div style='text-align: right'>702</div>  |
| verify_fibair | true | DivEIN | BBE4DIV | <div style='text-align: right'>1</div>  |
| verify_fibair | true | DivEIN | STOREW | <div style='text-align: right'>4</div>  |
| verify_fibair | true | DivFIN | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair | true | For | ADD | <div style='text-align: right'>33,882</div>  |
| verify_fibair | true | For | BNE | <div style='text-align: right'>42,951</div>  |
| verify_fibair | true | For | JAL | <div style='text-align: right'>9,069</div>  |
| verify_fibair | true | For | LOADW | <div style='text-align: right'>700</div>  |
| verify_fibair | true | For | STOREW | <div style='text-align: right'>8,369</div>  |
| verify_fibair | true | FriMatOpening | FRI_FOLD | <div style='text-align: right'>300</div>  |
| verify_fibair | true | HintBitsF | PHANTOM | <div style='text-align: right'>101</div>  |
| verify_fibair | true | HintInputVec | PHANTOM | <div style='text-align: right'>4,031</div>  |
| verify_fibair | true | IfEq | BNE | <div style='text-align: right'>727</div>  |
| verify_fibair | true | IfEqI | BNE | <div style='text-align: right'>11,293</div>  |
| verify_fibair | true | IfEqI | JAL | <span style="color: red">(+62 [+3.4%])</span> <div style='text-align: right'>1,884</div>  |
| verify_fibair | true | IfNe | BEQ | <div style='text-align: right'>2,129</div>  |
| verify_fibair | true | IfNe | JAL | <div style='text-align: right'>2</div>  |
| verify_fibair | true | IfNeI | BEQ | <div style='text-align: right'>603</div>  |
| verify_fibair | true | ImmE | STOREW | <div style='text-align: right'>2,008</div>  |
| verify_fibair | true | ImmF | STOREW | <div style='text-align: right'>7,004</div>  |
| verify_fibair | true | ImmV | STOREW | <div style='text-align: right'>12,768</div>  |
| verify_fibair | true | LoadE | LOADW | <div style='text-align: right'>4,836</div>  |
| verify_fibair | true | LoadE | LOADW2 | <div style='text-align: right'>10,800</div>  |
| verify_fibair | true | LoadF | LOADW | <div style='text-align: right'>13,252</div>  |
| verify_fibair | true | LoadF | LOADW2 | <div style='text-align: right'>2,248</div>  |
| verify_fibair | true | LoadV | LOADW | <div style='text-align: right'>8,798</div>  |
| verify_fibair | true | LoadV | LOADW2 | <div style='text-align: right'>19,851</div>  |
| verify_fibair | true | MulE | BBE4MUL | <div style='text-align: right'>1,818</div>  |
| verify_fibair | true | MulEF | MUL | <div style='text-align: right'>1,608</div>  |
| verify_fibair | true | MulEI | BBE4MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | true | MulEI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | true | MulF | MUL | <div style='text-align: right'>2,204</div>  |
| verify_fibair | true | MulFI | MUL | <div style='text-align: right'>1</div>  |
| verify_fibair | true | MulV | MUL | <div style='text-align: right'>3,131</div>  |
| verify_fibair | true | MulVI | MUL | <div style='text-align: right'>2,604</div>  |
| verify_fibair | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>2,000</div>  |
| verify_fibair | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>620</div>  |
| verify_fibair | true | StoreE | STOREW | <div style='text-align: right'>32,016</div>  |
| verify_fibair | true | StoreE | STOREW2 | <div style='text-align: right'>4,816</div>  |
| verify_fibair | true | StoreF | STOREW | <div style='text-align: right'>6,484</div>  |
| verify_fibair | true | StoreF | STOREW2 | <div style='text-align: right'>3,882</div>  |
| verify_fibair | true | StoreHintWord | ADD | <div style='text-align: right'>19,707</div>  |
| verify_fibair | true | StoreHintWord | SHINTW | <div style='text-align: right'>26,869</div>  |
| verify_fibair | true | StoreV | STOREW | <div style='text-align: right'>1,462</div>  |
| verify_fibair | true | StoreV | STOREW2 | <div style='text-align: right'>11,093</div>  |
| verify_fibair | true | SubE | FE4SUB | <div style='text-align: right'>1,202</div>  |
| verify_fibair | true | SubEF | LOADW | <div style='text-align: right'>915</div>  |
| verify_fibair | true | SubEF | SUB | <div style='text-align: right'>305</div>  |
| verify_fibair | true | SubEI | ADD | <div style='text-align: right'>8</div>  |
| verify_fibair | true | SubV | SUB | <div style='text-align: right'>3,101</div>  |
| verify_fibair | true | SubVI | SUB | <div style='text-align: right'>3,660</div>  |
| verify_fibair | true | SubVIN | SUB | <div style='text-align: right'>400</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | <div style='text-align: right'>10</div>  |
| verify_fibair | Boundary | true |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | <div style='text-align: right'>82</div>  |
| verify_fibair | Boundary | true |  | STOREW | <div style='text-align: right'>22</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | <div style='text-align: right'>45,200</div>  |
| verify_fibair | AccessAdapter<2> | true | AddE | FE4ADD | <div style='text-align: right'>24,882</div>  |
| verify_fibair | AccessAdapter<4> | true | AddE | FE4ADD | <div style='text-align: right'>14,703</div>  |
| verify_fibair | Boundary | true | AddE | FE4ADD | <div style='text-align: right'>792</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | <div style='text-align: right'>5,248</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEFFI | LOADW | <div style='text-align: right'>156</div>  |
| verify_fibair | Boundary | true | AddEFFI | LOADW | <div style='text-align: right'>176</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | <div style='text-align: right'>15,744</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | Boundary | true | AddEFFI | STOREW | <div style='text-align: right'>528</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | <div style='text-align: right'>169,920</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEI | ADD | <span style="color: red">(+66 [+0.3%])</span> <div style='text-align: right'>24,464</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEI | ADD | <span style="color: red">(+39 [+0.3%])</span> <div style='text-align: right'>14,456</div>  |
| verify_fibair | Boundary | true | AddEI | ADD | <div style='text-align: right'>440</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | <span style="color: red">(+720 [+0.9%])</span> <div style='text-align: right'>84,060</div>  |
| verify_fibair | Boundary | true | AddFI | ADD | <div style='text-align: right'>231</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | <div style='text-align: right'>54,450</div>  |
| verify_fibair | Boundary | true | AddV | ADD | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | <div style='text-align: right'>1,070,910</div>  |
| verify_fibair | Boundary | true | AddVI | ADD | <div style='text-align: right'>35,453</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | <div style='text-align: right'>390,210</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | <div style='text-align: right'>533,287</div>  |
| verify_fibair | Boundary | true | Alloc | LOADW | <div style='text-align: right'>1,760</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | <div style='text-align: right'>269,280</div>  |
| verify_fibair | AccessAdapter<2> | true | Alloc | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<4> | true | Alloc | MUL | <div style='text-align: right'>39</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | <div style='text-align: right'>9,292</div>  |
| verify_fibair | AccessAdapter<2> | true | AssertEqE | BNE | <div style='text-align: right'>2,222</div>  |
| verify_fibair | AccessAdapter<4> | true | AssertEqE | BNE | <div style='text-align: right'>1,313</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | <div style='text-align: right'>112,723</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | <div style='text-align: right'>81,420</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | <div style='text-align: right'>460</div>  |
| verify_fibair | PhantomAir | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>2,400</div>  |
| verify_fibair | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>2,400</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>2,400</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>4,800</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>7,200</div>  |
| verify_fibair | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | <div style='text-align: right'>3,600</div>  |
| verify_fibair | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch | PHANTOM | <div style='text-align: right'>2,400</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>4,800</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>7,200</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>7,200</div>  |
| verify_fibair | PhantomAir | true | CT-verify-query | PHANTOM | <div style='text-align: right'>1,200</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | <div style='text-align: right'>28,080</div>  |
| verify_fibair | AccessAdapter<2> | true | DivE | BBE4DIV | <div style='text-align: right'>6,710</div>  |
| verify_fibair | AccessAdapter<4> | true | DivE | BBE4DIV | <div style='text-align: right'>3,965</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | <div style='text-align: right'>40</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | BBE4DIV | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | true | DivEIN | BBE4DIV | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | <div style='text-align: right'>164</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | STOREW | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | <div style='text-align: right'>1,016,460</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | <div style='text-align: right'>987,873</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | <div style='text-align: right'>90,690</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | <div style='text-align: right'>28,700</div>  |
| verify_fibair | Boundary | true | For | LOADW | <div style='text-align: right'>1,100</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | <div style='text-align: right'>343,129</div>  |
| verify_fibair | Boundary | true | For | STOREW | <div style='text-align: right'>462</div>  |
| verify_fibair | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | <div style='text-align: right'>4,576</div>  |
| verify_fibair | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | <div style='text-align: right'>2,704</div>  |
| verify_fibair | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | <div style='text-align: right'>51,200</div>  |
| verify_fibair | PhantomAir | true | HintBitsF | PHANTOM | <div style='text-align: right'>606</div>  |
| verify_fibair | PhantomAir | true | HintInputVec | PHANTOM | <div style='text-align: right'>24,186</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | <div style='text-align: right'>16,721</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | <div style='text-align: right'>259,739</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | <span style="color: red">(+620 [+3.4%])</span> <div style='text-align: right'>18,840</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | <div style='text-align: right'>48,967</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | <div style='text-align: right'>20</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | <div style='text-align: right'>13,869</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | <div style='text-align: right'>82,328</div>  |
| verify_fibair | AccessAdapter<2> | true | ImmE | STOREW | <div style='text-align: right'>2,200</div>  |
| verify_fibair | AccessAdapter<4> | true | ImmE | STOREW | <div style='text-align: right'>1,300</div>  |
| verify_fibair | Boundary | true | ImmE | STOREW | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | <div style='text-align: right'>287,164</div>  |
| verify_fibair | Boundary | true | ImmF | STOREW | <div style='text-align: right'>2,134</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | <div style='text-align: right'>523,488</div>  |
| verify_fibair | Boundary | true | ImmV | STOREW | <div style='text-align: right'>37,312</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | <div style='text-align: right'>198,276</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW | <div style='text-align: right'>17,864</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW | <div style='text-align: right'>10,556</div>  |
| verify_fibair | Boundary | true | LoadE | LOADW | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | <div style='text-align: right'>442,800</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW2 | <div style='text-align: right'>28,644</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW2 | <div style='text-align: right'>16,926</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | <div style='text-align: right'>543,332</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW | <div style='text-align: right'>26,400</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW | <div style='text-align: right'>15,600</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW | <div style='text-align: right'>10,200</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW | <div style='text-align: right'>341</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | <div style='text-align: right'>92,168</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW2 | <div style='text-align: right'>715</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW2 | <div style='text-align: right'>429</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW2 | <div style='text-align: right'>323</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW2 | <div style='text-align: right'>1,221</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | <div style='text-align: right'>360,718</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW | <div style='text-align: right'>34,870</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | <div style='text-align: right'>813,891</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW2 | <div style='text-align: right'>957</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | <div style='text-align: right'>72,720</div>  |
| verify_fibair | AccessAdapter<2> | true | MulE | BBE4MUL | <span style="color: red">(+66 [+0.2%])</span> <div style='text-align: right'>28,996</div>  |
| verify_fibair | AccessAdapter<4> | true | MulE | BBE4MUL | <span style="color: red">(+39 [+0.2%])</span> <div style='text-align: right'>17,134</div>  |
| verify_fibair | Boundary | true | MulE | BBE4MUL | <div style='text-align: right'>572</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | <div style='text-align: right'>48,240</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEF | MUL | <div style='text-align: right'>8,822</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEF | MUL | <div style='text-align: right'>5,213</div>  |
| verify_fibair | Boundary | true | MulEF | MUL | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,320</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,892</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,118</div>  |
| verify_fibair | Boundary | true | MulEI | BBE4MUL | <div style='text-align: right'>924</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | <div style='text-align: right'>5,412</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | STOREW | <div style='text-align: right'>682</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | STOREW | <div style='text-align: right'>390</div>  |
| verify_fibair | Boundary | true | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | <div style='text-align: right'>66,120</div>  |
| verify_fibair | Boundary | true | MulF | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | <div style='text-align: right'>30</div>  |
| verify_fibair | Boundary | true | MulFI | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | <div style='text-align: right'>93,930</div>  |
| verify_fibair | Boundary | true | MulV | MUL | <div style='text-align: right'>34,419</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | <div style='text-align: right'>78,120</div>  |
| verify_fibair | Boundary | true | MulVI | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>88,000</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>52,000</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>34,000</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+282,000 [+33.7%])</span> <div style='text-align: right'>1,118,000</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>53,702</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>31,733</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>20,757</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+87,420 [+33.7%])</span> <div style='text-align: right'>346,580</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | <div style='text-align: right'>1,312,656</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW | <div style='text-align: right'>8,800</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW | <div style='text-align: right'>5,200</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW | <div style='text-align: right'>352,176</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | <div style='text-align: right'>197,456</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW2 | <div style='text-align: right'>8,800</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW2 | <div style='text-align: right'>5,200</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW2 | <div style='text-align: right'>17,776</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | <div style='text-align: right'>265,844</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW | <div style='text-align: right'>71,324</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | <div style='text-align: right'>159,162</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreF | STOREW2 | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreF | STOREW2 | <div style='text-align: right'>78</div>  |
| verify_fibair | AccessAdapter<8> | true | StoreF | STOREW2 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW2 | <div style='text-align: right'>35,486</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | <div style='text-align: right'>591,210</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | <div style='text-align: right'>1,101,629</div>  |
| verify_fibair | Boundary | true | StoreHintWord | SHINTW | <div style='text-align: right'>295,559</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | <div style='text-align: right'>59,942</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW | <div style='text-align: right'>16,082</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | <div style='text-align: right'>454,813</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW2 | <div style='text-align: right'>93,258</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | <div style='text-align: right'>48,080</div>  |
| verify_fibair | AccessAdapter<2> | true | SubE | FE4SUB | <div style='text-align: right'>44,176</div>  |
| verify_fibair | AccessAdapter<4> | true | SubE | FE4SUB | <div style='text-align: right'>26,104</div>  |
| verify_fibair | Boundary | true | SubE | FE4SUB | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | <div style='text-align: right'>37,515</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | LOADW | <div style='text-align: right'>3,333</div>  |
| verify_fibair | Boundary | true | SubEF | LOADW | <div style='text-align: right'>99</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | <div style='text-align: right'>9,150</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | SUB | <div style='text-align: right'>3,333</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEF | SUB | <div style='text-align: right'>3,939</div>  |
| verify_fibair | Boundary | true | SubEF | SUB | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | <div style='text-align: right'>240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEI | ADD | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEI | ADD | <div style='text-align: right'>26</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | <div style='text-align: right'>93,030</div>  |
| verify_fibair | Boundary | true | SubV | SUB | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | <div style='text-align: right'>109,800</div>  |
| verify_fibair | Boundary | true | SubVI | SUB | <div style='text-align: right'>35,607</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | <div style='text-align: right'>12,000</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: green">(-31.0 [-68.9%])</span> <div style='text-align: right'>14.0</div>  | <span style="color: green">(-13.0 [-1.9%])</span> <div style='text-align: right'>686.0</div>  | <span style="color: green">(-8.0 [-1.3%])</span> <div style='text-align: right'>593.0</div>  | <span style="color: green">(-2 [-66.7%])</span> <div style='text-align: right'>1</div>  | <span style="color: green">(-33.0 [-49.3%])</span> <div style='text-align: right'>34.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+370,970 [+2.2%])</span> <div style='text-align: right'>17,368,249</div>  | <span style="color: red">(+86 [+0.0%])</span> <div style='text-align: right'>427,483</div>  | <span style="color: green">(-2,413.0 [-52.6%])</span> <div style='text-align: right'>2,175.0</div>  | <span style="color: red">(+1.0 [+2.3%])</span> <div style='text-align: right'>45.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <span style="color: red">(+1 [+12.5%])</span> <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | <span style="color: red">(+1 [+6.2%])</span> <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<2> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<4> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<8> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<16> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<32> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AccessAdapterAir<64> | <span style="color: red">(+3 [+27.3%])</span> <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | PhantomAir | <span style="color: red">(+1 [+25.0%])</span> <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <span style="color: red">(+6 [+20.0%])</span> <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <span style="color: green">(-6 [-75.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <span style="color: red">(+7 [+33.3%])</span> <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <span style="color: red">(+2 [+33.3%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <span style="color: green">(-6 [-75.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <span style="color: red">(+5 [+22.7%])</span> <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <span style="color: green">(-6 [-75.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <span style="color: red">(+5 [+22.7%])</span> <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <span style="color: green">(-6 [-75.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | FriMatOpeningAir | <span style="color: red">(+23 [+43.4%])</span> <div style='text-align: right'>76</div>  | <div style='text-align: right'>35</div>  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <span style="color: red">(+151 [+40.4%])</span> <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <span style="color: green">(-6 [-75.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | VmConnectorAir | 0 | <span style="color: red">(+8 [+33.3%])</span> <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <span style="color: red">(+4 [+50.0%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | 0 | <span style="color: red">(+524,288 [+21.1%])</span> <div style='text-align: right'>3,014,656</div>  | <div style='text-align: right'>11</div>  | <span style="color: red">(+4 [+50.0%])</span> <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <span style="color: red">(+786,432 [+52.2%])</span> <div style='text-align: right'>2,293,760</div>  | <div style='text-align: right'>11</div>  | <span style="color: red">(+12 [+100.0%])</span> <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <span style="color: red">(+393,216 [+48.0%])</span> <div style='text-align: right'>1,212,416</div>  | <div style='text-align: right'>13</div>  | <span style="color: red">(+12 [+100.0%])</span> <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <span style="color: red">(+98,304 [+41.4%])</span> <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <span style="color: red">(+12 [+100.0%])</span> <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | PhantomAir | 0 | <span style="color: red">(+65,536 [+28.6%])</span> <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>6</div>  | <span style="color: red">(+4 [+50.0%])</span> <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <span style="color: red">(+6,291,456 [+39.3%])</span> <div style='text-align: right'>22,282,240</div>  | <div style='text-align: right'>41</div>  | <span style="color: red">(+24 [+120.0%])</span> <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>262,144</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <span style="color: red">(+3,670,016 [+65.1%])</span> <div style='text-align: right'>9,306,112</div>  | <div style='text-align: right'>23</div>  | <span style="color: red">(+28 [+140.0%])</span> <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <span style="color: red">(+131,072 [+36.4%])</span> <div style='text-align: right'>491,520</div>  | <div style='text-align: right'>10</div>  | <span style="color: red">(+8 [+66.7%])</span> <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <span style="color: red">(+5,242,880 [+43.5%])</span> <div style='text-align: right'>17,301,504</div>  | <div style='text-align: right'>30</div>  | <span style="color: red">(+20 [+125.0%])</span> <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>262,144</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <span style="color: red">(+163,840 [+35.7%])</span> <div style='text-align: right'>622,592</div>  | <div style='text-align: right'>40</div>  | <span style="color: red">(+20 [+125.0%])</span> <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | FriMatOpeningAir | 0 | <span style="color: red">(+94,208 [+79.3%])</span> <div style='text-align: right'>212,992</div>  | <div style='text-align: right'>64</div>  | <span style="color: red">(+92 [+176.9%])</span> <div style='text-align: right'>144</div>  |  | <div style='text-align: right'>1,024</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: red">(+741,376 [+40.6%])</span> <div style='text-align: right'>2,568,192</div>  | <span style="color: red">(+141 [+33.7%])</span> <div style='text-align: right'>559</div>  | <span style="color: red">(+40 [+142.9%])</span> <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: green">(-2,400.0 [-61.7%])</span> <div style='text-align: right'>1,489.0</div>  | <span style="color: red">(+18,202,632 [+41.3%])</span> <div style='text-align: right'>62,296,096</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ea85f8341a90498aa4927523d1d056b44435eede/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/ea85f8341a90498aa4927523d1d056b44435eede
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11729958295)
