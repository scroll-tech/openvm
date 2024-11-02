| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>17,562,987</div>  | <div style='text-align: right'>441,414</div>  | <span style="color: green">(-37.0 [-0.6%])</span> <div style='text-align: right'>5,917.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |

| stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- |
| <div style='text-align: right'>4.0</div>  | <div style='text-align: right'>32</div>  |

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| verify_fibair | true | <span style="color: red">(+73.0 [+1.1%])</span> <div style='text-align: right'>6,459.0</div>  | <div style='text-align: right'>17,562,987</div>  | <div style='text-align: right'>441,414</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| verify_fibair | BranchEqual | true | <div style='text-align: right'>67,826</div>  |
| verify_fibair | FieldArithmetic | true | <div style='text-align: right'>140,649</div>  |
| verify_fibair | FieldExtension | true | <div style='text-align: right'>7,486</div>  |
| verify_fibair | Jal | true | <div style='text-align: right'>11,304</div>  |
| verify_fibair | LoadStore | true | <div style='text-align: right'>199,789</div>  |
| verify_fibair | Memory AccessAdapter<2> | true | <div style='text-align: right'>40,607</div>  |
| verify_fibair | Memory AccessAdapter<4> | true | <div style='text-align: right'>20,305</div>  |
| verify_fibair | Memory AccessAdapter<8> | true | <div style='text-align: right'>3,851</div>  |
| verify_fibair | Memory Boundary | true | <div style='text-align: right'>97,480</div>  |
| verify_fibair | Phantom | true | <div style='text-align: right'>11,747</div>  |
| verify_fibair | Poseidon2 | true | <div style='text-align: right'>2,613</div>  |
| verify_fibair | ProgramChip | true | <div style='text-align: right'>37,505</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair | true |  | JAL | <div style='text-align: right'>1</div>  |
| verify_fibair | true |  | STOREW | <div style='text-align: right'>2</div>  |
| verify_fibair | true | AddE | FE4ADD | <div style='text-align: right'>1,630</div>  |
| verify_fibair | true | AddEFFI | LOADW | <div style='text-align: right'>128</div>  |
| verify_fibair | true | AddEFFI | STOREW | <div style='text-align: right'>384</div>  |
| verify_fibair | true | AddEI | ADD | <div style='text-align: right'>5,664</div>  |
| verify_fibair | true | AddFI | ADD | <div style='text-align: right'>2,823</div>  |
| verify_fibair | true | AddV | ADD | <div style='text-align: right'>1,815</div>  |
| verify_fibair | true | AddVI | ADD | <div style='text-align: right'>35,364</div>  |
| verify_fibair | true | Alloc | ADD | <div style='text-align: right'>13,011</div>  |
| verify_fibair | true | Alloc | LOADW | <div style='text-align: right'>13,011</div>  |
| verify_fibair | true | Alloc | MUL | <div style='text-align: right'>8,979</div>  |
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
| verify_fibair | true | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>600</div>  |
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
| verify_fibair | true | DivE | BBE4DIV | <div style='text-align: right'>1,202</div>  |
| verify_fibair | true | DivEIN | BBE4DIV | <div style='text-align: right'>1</div>  |
| verify_fibair | true | DivEIN | STOREW | <div style='text-align: right'>4</div>  |
| verify_fibair | true | DivFIN | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair | true | For | ADD | <div style='text-align: right'>34,831</div>  |
| verify_fibair | true | For | BNE | <div style='text-align: right'>44,214</div>  |
| verify_fibair | true | For | JAL | <div style='text-align: right'>9,383</div>  |
| verify_fibair | true | For | LOADW | <div style='text-align: right'>700</div>  |
| verify_fibair | true | For | STOREW | <div style='text-align: right'>8,683</div>  |
| verify_fibair | true | HintBitsF | PHANTOM | <div style='text-align: right'>101</div>  |
| verify_fibair | true | HintInputVec | PHANTOM | <div style='text-align: right'>4,032</div>  |
| verify_fibair | true | IfEq | BNE | <div style='text-align: right'>722</div>  |
| verify_fibair | true | IfEqI | BNE | <div style='text-align: right'>11,289</div>  |
| verify_fibair | true | IfEqI | JAL | <div style='text-align: right'>1,913</div>  |
| verify_fibair | true | IfNe | BEQ | <div style='text-align: right'>2,129</div>  |
| verify_fibair | true | IfNe | JAL | <div style='text-align: right'>7</div>  |
| verify_fibair | true | IfNeI | BEQ | <div style='text-align: right'>603</div>  |
| verify_fibair | true | ImmE | STOREW | <div style='text-align: right'>2,008</div>  |
| verify_fibair | true | ImmF | STOREW | <div style='text-align: right'>7,036</div>  |
| verify_fibair | true | ImmV | STOREW | <div style='text-align: right'>12,751</div>  |
| verify_fibair | true | LoadE | LOADW | <div style='text-align: right'>4,836</div>  |
| verify_fibair | true | LoadE | LOADW2 | <div style='text-align: right'>14,000</div>  |
| verify_fibair | true | LoadF | LOADW | <div style='text-align: right'>13,252</div>  |
| verify_fibair | true | LoadF | LOADW2 | <div style='text-align: right'>3,096</div>  |
| verify_fibair | true | LoadV | LOADW | <div style='text-align: right'>8,800</div>  |
| verify_fibair | true | LoadV | LOADW2 | <div style='text-align: right'>19,451</div>  |
| verify_fibair | true | MulE | BBE4MUL | <div style='text-align: right'>3,418</div>  |
| verify_fibair | true | MulEF | MUL | <div style='text-align: right'>1,608</div>  |
| verify_fibair | true | MulEI | BBE4MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | true | MulEI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | true | MulF | MUL | <div style='text-align: right'>2,204</div>  |
| verify_fibair | true | MulFI | MUL | <div style='text-align: right'>1</div>  |
| verify_fibair | true | MulV | MUL | <div style='text-align: right'>3,131</div>  |
| verify_fibair | true | MulVI | MUL | <div style='text-align: right'>2,604</div>  |
| verify_fibair | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>2,000</div>  |
| verify_fibair | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>613</div>  |
| verify_fibair | true | StoreE | STOREW | <div style='text-align: right'>32,012</div>  |
| verify_fibair | true | StoreE | STOREW2 | <div style='text-align: right'>4,816</div>  |
| verify_fibair | true | StoreF | STOREW | <div style='text-align: right'>6,484</div>  |
| verify_fibair | true | StoreF | STOREW2 | <div style='text-align: right'>3,962</div>  |
| verify_fibair | true | StoreHintWord | ADD | <div style='text-align: right'>19,708</div>  |
| verify_fibair | true | StoreHintWord | SHINTW | <div style='text-align: right'>26,871</div>  |
| verify_fibair | true | StoreV | STOREW | <div style='text-align: right'>1,462</div>  |
| verify_fibair | true | StoreV | STOREW2 | <div style='text-align: right'>11,093</div>  |
| verify_fibair | true | SubE | FE4SUB | <div style='text-align: right'>1,202</div>  |
| verify_fibair | true | SubEF | LOADW | <div style='text-align: right'>4,815</div>  |
| verify_fibair | true | SubEF | SUB | <div style='text-align: right'>1,605</div>  |
| verify_fibair | true | SubEI | ADD | <div style='text-align: right'>8</div>  |
| verify_fibair | true | SubV | SUB | <div style='text-align: right'>3,101</div>  |
| verify_fibair | true | SubVI | SUB | <div style='text-align: right'>3,789</div>  |
| verify_fibair | true | SubVIN | SUB | <div style='text-align: right'>400</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | <div style='text-align: right'>10</div>  |
| verify_fibair | Boundary | true |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | <div style='text-align: right'>82</div>  |
| verify_fibair | Boundary | true |  | STOREW | <div style='text-align: right'>22</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | <div style='text-align: right'>65,200</div>  |
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
| verify_fibair | AccessAdapter<2> | true | AddEI | ADD | <div style='text-align: right'>24,398</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEI | ADD | <div style='text-align: right'>14,417</div>  |
| verify_fibair | Boundary | true | AddEI | ADD | <div style='text-align: right'>440</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | <div style='text-align: right'>84,690</div>  |
| verify_fibair | Boundary | true | AddFI | ADD | <div style='text-align: right'>242</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | <div style='text-align: right'>54,450</div>  |
| verify_fibair | Boundary | true | AddV | ADD | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | <div style='text-align: right'>1,060,920</div>  |
| verify_fibair | Boundary | true | AddVI | ADD | <div style='text-align: right'>35,486</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | <div style='text-align: right'>390,330</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | <div style='text-align: right'>533,451</div>  |
| verify_fibair | Boundary | true | Alloc | LOADW | <div style='text-align: right'>1,815</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | <div style='text-align: right'>269,370</div>  |
| verify_fibair | AccessAdapter<2> | true | Alloc | MUL | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | true | Alloc | MUL | <div style='text-align: right'>26</div>  |
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
| verify_fibair | PhantomAir | true | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>3,600</div>  |
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
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | <div style='text-align: right'>48,080</div>  |
| verify_fibair | AccessAdapter<2> | true | DivE | BBE4DIV | <div style='text-align: right'>35,310</div>  |
| verify_fibair | AccessAdapter<4> | true | DivE | BBE4DIV | <div style='text-align: right'>20,865</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | <div style='text-align: right'>40</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | BBE4DIV | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | true | DivEIN | BBE4DIV | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | <div style='text-align: right'>164</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | STOREW | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | <div style='text-align: right'>1,044,930</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | <div style='text-align: right'>1,016,922</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | <div style='text-align: right'>93,830</div>  |
| verify_fibair | AccessAdapter<2> | true | For | JAL | <div style='text-align: right'>55</div>  |
| verify_fibair | AccessAdapter<4> | true | For | JAL | <div style='text-align: right'>65</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | <div style='text-align: right'>28,700</div>  |
| verify_fibair | Boundary | true | For | LOADW | <div style='text-align: right'>1,100</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | <div style='text-align: right'>356,003</div>  |
| verify_fibair | Boundary | true | For | STOREW | <div style='text-align: right'>572</div>  |
| verify_fibair | PhantomAir | true | HintBitsF | PHANTOM | <div style='text-align: right'>606</div>  |
| verify_fibair | PhantomAir | true | HintInputVec | PHANTOM | <div style='text-align: right'>24,192</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | <div style='text-align: right'>16,606</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | <div style='text-align: right'>259,647</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | <div style='text-align: right'>19,130</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | <div style='text-align: right'>48,967</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | <div style='text-align: right'>70</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | <div style='text-align: right'>13,869</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | <div style='text-align: right'>82,328</div>  |
| verify_fibair | AccessAdapter<2> | true | ImmE | STOREW | <div style='text-align: right'>2,200</div>  |
| verify_fibair | AccessAdapter<4> | true | ImmE | STOREW | <div style='text-align: right'>1,300</div>  |
| verify_fibair | Boundary | true | ImmE | STOREW | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | <div style='text-align: right'>288,476</div>  |
| verify_fibair | Boundary | true | ImmF | STOREW | <div style='text-align: right'>2,156</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | <div style='text-align: right'>522,791</div>  |
| verify_fibair | Boundary | true | ImmV | STOREW | <div style='text-align: right'>37,334</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | <div style='text-align: right'>198,276</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW | <div style='text-align: right'>17,688</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW | <div style='text-align: right'>10,452</div>  |
| verify_fibair | Boundary | true | LoadE | LOADW | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | <div style='text-align: right'>574,000</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW2 | <div style='text-align: right'>28,666</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW2 | <div style='text-align: right'>16,939</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | <div style='text-align: right'>543,332</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW | <div style='text-align: right'>26,400</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW | <div style='text-align: right'>15,600</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW | <div style='text-align: right'>10,200</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW | <div style='text-align: right'>330</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | <div style='text-align: right'>126,936</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW2 | <div style='text-align: right'>715</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW2 | <div style='text-align: right'>429</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW2 | <div style='text-align: right'>323</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW2 | <div style='text-align: right'>1,210</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | <div style='text-align: right'>360,800</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW | <div style='text-align: right'>34,881</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | <div style='text-align: right'>797,491</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | <div style='text-align: right'>136,720</div>  |
| verify_fibair | AccessAdapter<2> | true | MulE | BBE4MUL | <div style='text-align: right'>33,352</div>  |
| verify_fibair | AccessAdapter<4> | true | MulE | BBE4MUL | <div style='text-align: right'>19,708</div>  |
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
| verify_fibair | AccessAdapter<2> | true | MulEI | STOREW | <div style='text-align: right'>638</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | STOREW | <div style='text-align: right'>338</div>  |
| verify_fibair | Boundary | true | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | <div style='text-align: right'>66,120</div>  |
| verify_fibair | Boundary | true | MulF | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | <div style='text-align: right'>30</div>  |
| verify_fibair | Boundary | true | MulFI | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | <div style='text-align: right'>93,930</div>  |
| verify_fibair | Boundary | true | MulV | MUL | <div style='text-align: right'>34,408</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | <div style='text-align: right'>78,120</div>  |
| verify_fibair | Boundary | true | MulVI | MUL | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>88,000</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>52,000</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>34,000</div>  |
| verify_fibair | Poseidon2VmAir<BabyBear> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>836,000</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>53,801</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>31,798</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>20,842</div>  |
| verify_fibair | Poseidon2VmAir<BabyBear> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>256,234</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | <div style='text-align: right'>1,312,492</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW | <div style='text-align: right'>8,800</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW | <div style='text-align: right'>5,200</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW | <div style='text-align: right'>352,132</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | <div style='text-align: right'>197,456</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW2 | <div style='text-align: right'>8,800</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW2 | <div style='text-align: right'>5,200</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW2 | <div style='text-align: right'>17,776</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | <div style='text-align: right'>265,844</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW | <div style='text-align: right'>71,324</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | <div style='text-align: right'>162,442</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreF | STOREW2 | <div style='text-align: right'>231</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreF | STOREW2 | <div style='text-align: right'>143</div>  |
| verify_fibair | AccessAdapter<8> | true | StoreF | STOREW2 | <div style='text-align: right'>102</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW2 | <div style='text-align: right'>35,838</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | <div style='text-align: right'>591,240</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | <div style='text-align: right'>1,101,711</div>  |
| verify_fibair | Boundary | true | StoreHintWord | SHINTW | <div style='text-align: right'>295,581</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | <div style='text-align: right'>59,942</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW | <div style='text-align: right'>16,082</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | <div style='text-align: right'>454,813</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW2 | <div style='text-align: right'>93,258</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | <div style='text-align: right'>48,080</div>  |
| verify_fibair | AccessAdapter<2> | true | SubE | FE4SUB | <div style='text-align: right'>44,176</div>  |
| verify_fibair | AccessAdapter<4> | true | SubE | FE4SUB | <div style='text-align: right'>26,104</div>  |
| verify_fibair | Boundary | true | SubE | FE4SUB | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | <div style='text-align: right'>197,415</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | LOADW | <div style='text-align: right'>17,633</div>  |
| verify_fibair | Boundary | true | SubEF | LOADW | <div style='text-align: right'>99</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | <div style='text-align: right'>48,150</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | SUB | <div style='text-align: right'>17,633</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEF | SUB | <div style='text-align: right'>20,839</div>  |
| verify_fibair | Boundary | true | SubEF | SUB | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | <div style='text-align: right'>240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEI | ADD | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEI | ADD | <div style='text-align: right'>26</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | <div style='text-align: right'>93,030</div>  |
| verify_fibair | Boundary | true | SubV | SUB | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | <div style='text-align: right'>113,670</div>  |
| verify_fibair | Boundary | true | SubVI | SUB | <div style='text-align: right'>35,519</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | <div style='text-align: right'>12,000</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VolatileBoundaryAir | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<2> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<4> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<8> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<16> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<32> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<64> | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>8</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>8</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>8</div>  |
| verify_fibair | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: green">(-2.0 [-4.1%])</span> <div style='text-align: right'>47.0</div>  | <span style="color: green">(-1.0 [-0.2%])</span> <div style='text-align: right'>601.0</div>  | <span style="color: green">(-1.0 [-0.2%])</span> <div style='text-align: right'>508.0</div>  | <div style='text-align: right'>63.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>17,562,987</div>  | <div style='text-align: right'>441,414</div>  | <span style="color: green">(-37.0 [-0.6%])</span> <div style='text-align: right'>5,917.0</div>  | <span style="color: red">(+1.0 [+2.3%])</span> <div style='text-align: right'>45.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | 0 | <div style='text-align: right'>2,490,368</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <div style='text-align: right'>1,507,328</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <div style='text-align: right'>819,200</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <div style='text-align: right'>237,568</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>229,376</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>15,990,784</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>262,144</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>5,636,096</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>360,448</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>12,058,624</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>458,752</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | Poseidon2VmAir<BabyBear> | 0 | <div style='text-align: right'>1,826,816</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: green">(-36.0 [-0.7%])</span> <div style='text-align: right'>5,316.0</div>  | <div style='text-align: right'>43,974,680</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5025033d31155cd45102b5435bf8d924b3a39e3d/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/5025033d31155cd45102b5435bf8d924b3a39e3d
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11638085562)
