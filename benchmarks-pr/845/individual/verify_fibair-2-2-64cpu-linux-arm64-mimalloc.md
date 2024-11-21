| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,307,357</div>  | <div style='text-align: right'>199,267</div>  | <span style="color: green">(-9.0 [-0.6%])</span> <div style='text-align: right'>1,578.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |

| stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- |
| <span style="color: green">(-1.0 [-10.0%])</span> <div style='text-align: right'>9.0</div>  | <div style='text-align: right'>32</div>  |

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| verify_fibair | true | <span style="color: green">(-15.0 [-1.3%])</span> <div style='text-align: right'>1,101.0</div>  | <div style='text-align: right'>8,307,357</div>  | <div style='text-align: right'>199,267</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| verify_fibair | ProgramChip | true | <div style='text-align: right'>17,628</div>  |
| verify_fibair | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| verify_fibair | Boundary | true | <div style='text-align: right'>44,590</div>  |
| verify_fibair | AccessAdapter<2> | true | <div style='text-align: right'>22,004</div>  |
| verify_fibair | AccessAdapter<4> | true | <div style='text-align: right'>11,002</div>  |
| verify_fibair | AccessAdapter<8> | true | <div style='text-align: right'>3,220</div>  |
| verify_fibair | PhantomAir | true | <div style='text-align: right'>5,216</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | <div style='text-align: right'>85,882</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | <div style='text-align: right'>31,888</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | <div style='text-align: right'>5,170</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | <div style='text-align: right'>67,442</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | <div style='text-align: right'>2,186</div>  |
| verify_fibair | FriReducedOpeningAir | true | <div style='text-align: right'>336</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>1,357</div>  |
| verify_fibair | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair | true |  | JAL | <div style='text-align: right'>1</div>  |
| verify_fibair | true |  | STOREW | <div style='text-align: right'>2</div>  |
| verify_fibair | true | AddE | FE4ADD | <div style='text-align: right'>492</div>  |
| verify_fibair | true | AddEFFI | LOADW | <div style='text-align: right'>70</div>  |
| verify_fibair | true | AddEFFI | STOREW | <div style='text-align: right'>210</div>  |
| verify_fibair | true | AddEI | ADD | <div style='text-align: right'>2,584</div>  |
| verify_fibair | true | AddFI | ADD | <div style='text-align: right'>1,285</div>  |
| verify_fibair | true | AddV | ADD | <div style='text-align: right'>939</div>  |
| verify_fibair | true | AddVI | ADD | <div style='text-align: right'>17,307</div>  |
| verify_fibair | true | Alloc | ADD | <div style='text-align: right'>6,029</div>  |
| verify_fibair | true | Alloc | LOADW | <div style='text-align: right'>6,029</div>  |
| verify_fibair | true | Alloc | MUL | <div style='text-align: right'>4,066</div>  |
| verify_fibair | true | AssertEqE | BNE | <div style='text-align: right'>172</div>  |
| verify_fibair | true | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| verify_fibair | true | AssertEqF | BNE | <div style='text-align: right'>2,059</div>  |
| verify_fibair | true | AssertEqV | BNE | <div style='text-align: right'>1,510</div>  |
| verify_fibair | true | AssertEqVI | BNE | <div style='text-align: right'>20</div>  |
| verify_fibair | true | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-ReadingProofFromInput | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>336</div>  |
| verify_fibair | true | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>252</div>  |
| verify_fibair | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-verify-batch | PHANTOM | <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>336</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-verify-query | PHANTOM | <div style='text-align: right'>84</div>  |
| verify_fibair | true | DivE | BBE4DIV | <div style='text-align: right'>296</div>  |
| verify_fibair | true | DivEIN | BBE4DIV | <div style='text-align: right'>1</div>  |
| verify_fibair | true | DivEIN | STOREW | <div style='text-align: right'>4</div>  |
| verify_fibair | true | DivFIN | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair | true | For | ADD | <div style='text-align: right'>17,108</div>  |
| verify_fibair | true | For | BNE | <div style='text-align: right'>21,209</div>  |
| verify_fibair | true | For | JAL | <div style='text-align: right'>4,101</div>  |
| verify_fibair | true | For | LOADW | <div style='text-align: right'>294</div>  |
| verify_fibair | true | For | STOREW | <div style='text-align: right'>3,807</div>  |
| verify_fibair | true | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>126</div>  |
| verify_fibair | true | HintBitsF | PHANTOM | <div style='text-align: right'>43</div>  |
| verify_fibair | true | HintInputVec | PHANTOM | <div style='text-align: right'>1,963</div>  |
| verify_fibair | true | IfEq | BNE | <div style='text-align: right'>321</div>  |
| verify_fibair | true | IfEqI | BNE | <div style='text-align: right'>5,175</div>  |
| verify_fibair | true | IfEqI | JAL | <div style='text-align: right'>1,066</div>  |
| verify_fibair | true | IfNe | BEQ | <div style='text-align: right'>1,163</div>  |
| verify_fibair | true | IfNe | JAL | <div style='text-align: right'>2</div>  |
| verify_fibair | true | IfNeI | BEQ | <div style='text-align: right'>255</div>  |
| verify_fibair | true | ImmE | STOREW | <div style='text-align: right'>848</div>  |
| verify_fibair | true | ImmF | STOREW | <div style='text-align: right'>3,002</div>  |
| verify_fibair | true | ImmV | STOREW | <div style='text-align: right'>5,462</div>  |
| verify_fibair | true | LoadE | LOADW | <div style='text-align: right'>2,052</div>  |
| verify_fibair | true | LoadE | LOADW2 | <div style='text-align: right'>4,536</div>  |
| verify_fibair | true | LoadF | LOADW | <div style='text-align: right'>5,596</div>  |
| verify_fibair | true | LoadF | LOADW2 | <div style='text-align: right'>972</div>  |
| verify_fibair | true | LoadV | LOADW | <div style='text-align: right'>4,004</div>  |
| verify_fibair | true | LoadV | LOADW2 | <div style='text-align: right'>9,055</div>  |
| verify_fibair | true | MulE | BBE4MUL | <div style='text-align: right'>858</div>  |
| verify_fibair | true | MulEF | MUL | <div style='text-align: right'>680</div>  |
| verify_fibair | true | MulEI | BBE4MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | true | MulEI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | true | MulF | MUL | <div style='text-align: right'>1,096</div>  |
| verify_fibair | true | MulFI | MUL | <div style='text-align: right'>1</div>  |
| verify_fibair | true | MulV | MUL | <div style='text-align: right'>1,333</div>  |
| verify_fibair | true | MulVI | MUL | <div style='text-align: right'>1,348</div>  |
| verify_fibair | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>1,092</div>  |
| verify_fibair | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>265</div>  |
| verify_fibair | true | StoreE | STOREW | <div style='text-align: right'>13,456</div>  |
| verify_fibair | true | StoreE | STOREW2 | <div style='text-align: right'>2,032</div>  |
| verify_fibair | true | StoreF | STOREW | <div style='text-align: right'>2,772</div>  |
| verify_fibair | true | StoreF | STOREW2 | <div style='text-align: right'>1,678</div>  |
| verify_fibair | true | StoreHintWord | ADD | <div style='text-align: right'>10,355</div>  |
| verify_fibair | true | StoreHintWord | SHINTW | <div style='text-align: right'>13,651</div>  |
| verify_fibair | true | StoreV | STOREW | <div style='text-align: right'>650</div>  |
| verify_fibair | true | StoreV | STOREW2 | <div style='text-align: right'>5,175</div>  |
| verify_fibair | true | SubE | FE4SUB | <div style='text-align: right'>506</div>  |
| verify_fibair | true | SubEF | LOADW | <div style='text-align: right'>393</div>  |
| verify_fibair | true | SubEF | SUB | <div style='text-align: right'>131</div>  |
| verify_fibair | true | SubEI | ADD | <div style='text-align: right'>8</div>  |
| verify_fibair | true | SubV | SUB | <div style='text-align: right'>1,429</div>  |
| verify_fibair | true | SubVI | SUB | <div style='text-align: right'>1,572</div>  |
| verify_fibair | true | SubVIN | SUB | <div style='text-align: right'>168</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | <div style='text-align: right'>10</div>  |
| verify_fibair | Boundary | true |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | <div style='text-align: right'>82</div>  |
| verify_fibair | Boundary | true |  | STOREW | <div style='text-align: right'>22</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> | true | AddE | FE4ADD | <div style='text-align: right'>10,846</div>  |
| verify_fibair | AccessAdapter<4> | true | AddE | FE4ADD | <div style='text-align: right'>6,409</div>  |
| verify_fibair | Boundary | true | AddE | FE4ADD | <div style='text-align: right'>792</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | <div style='text-align: right'>2,870</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEFFI | LOADW | <div style='text-align: right'>156</div>  |
| verify_fibair | Boundary | true | AddEFFI | LOADW | <div style='text-align: right'>176</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | <div style='text-align: right'>8,610</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | Boundary | true | AddEFFI | STOREW | <div style='text-align: right'>528</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | <div style='text-align: right'>77,520</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEI | ADD | <div style='text-align: right'>11,902</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEI | ADD | <div style='text-align: right'>7,033</div>  |
| verify_fibair | Boundary | true | AddEI | ADD | <div style='text-align: right'>440</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | <div style='text-align: right'>38,550</div>  |
| verify_fibair | Boundary | true | AddFI | ADD | <div style='text-align: right'>231</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | <div style='text-align: right'>28,170</div>  |
| verify_fibair | Boundary | true | AddV | ADD | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | <div style='text-align: right'>519,210</div>  |
| verify_fibair | Boundary | true | AddVI | ADD | <div style='text-align: right'>15,675</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | <div style='text-align: right'>180,870</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | <div style='text-align: right'>247,189</div>  |
| verify_fibair | Boundary | true | Alloc | LOADW | <div style='text-align: right'>1,122</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | <div style='text-align: right'>121,980</div>  |
| verify_fibair | AccessAdapter<2> | true | Alloc | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<4> | true | Alloc | MUL | <div style='text-align: right'>39</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | <div style='text-align: right'>3,956</div>  |
| verify_fibair | AccessAdapter<2> | true | AssertEqE | BNE | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> | true | AssertEqE | BNE | <div style='text-align: right'>559</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | <div style='text-align: right'>47,357</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | <div style='text-align: right'>34,730</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | <div style='text-align: right'>460</div>  |
| verify_fibair | PhantomAir | true | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-ReadingProofFromInput | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>1,512</div>  |
| verify_fibair | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch | PHANTOM | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-verify-query | PHANTOM | <div style='text-align: right'>504</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | <div style='text-align: right'>11,840</div>  |
| verify_fibair | AccessAdapter<2> | true | DivE | BBE4DIV | <div style='text-align: right'>2,882</div>  |
| verify_fibair | AccessAdapter<4> | true | DivE | BBE4DIV | <div style='text-align: right'>1,703</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | <div style='text-align: right'>40</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | BBE4DIV | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | true | DivEIN | BBE4DIV | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | <div style='text-align: right'>164</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | STOREW | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | <div style='text-align: right'>513,240</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | <div style='text-align: right'>487,807</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | <div style='text-align: right'>41,010</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | <div style='text-align: right'>12,054</div>  |
| verify_fibair | Boundary | true | For | LOADW | <div style='text-align: right'>473</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | <div style='text-align: right'>156,087</div>  |
| verify_fibair | Boundary | true | For | STOREW | <div style='text-align: right'>462</div>  |
| verify_fibair | AccessAdapter<2> | true | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> | true | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriReducedOpeningAir | true | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>21,504</div>  |
| verify_fibair | PhantomAir | true | HintBitsF | PHANTOM | <div style='text-align: right'>258</div>  |
| verify_fibair | PhantomAir | true | HintInputVec | PHANTOM | <div style='text-align: right'>11,778</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | <div style='text-align: right'>7,383</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | <div style='text-align: right'>119,025</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | <div style='text-align: right'>10,660</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | <div style='text-align: right'>26,749</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | <div style='text-align: right'>20</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | <div style='text-align: right'>5,865</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | <div style='text-align: right'>34,768</div>  |
| verify_fibair | AccessAdapter<2> | true | ImmE | STOREW | <div style='text-align: right'>924</div>  |
| verify_fibair | AccessAdapter<4> | true | ImmE | STOREW | <div style='text-align: right'>546</div>  |
| verify_fibair | Boundary | true | ImmE | STOREW | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | <div style='text-align: right'>123,082</div>  |
| verify_fibair | Boundary | true | ImmF | STOREW | <div style='text-align: right'>1,496</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | <div style='text-align: right'>223,942</div>  |
| verify_fibair | Boundary | true | ImmV | STOREW | <div style='text-align: right'>16,258</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | <div style='text-align: right'>84,132</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW | <div style='text-align: right'>7,656</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW | <div style='text-align: right'>4,524</div>  |
| verify_fibair | Boundary | true | LoadE | LOADW | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | <div style='text-align: right'>185,976</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW2 | <div style='text-align: right'>12,056</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW2 | <div style='text-align: right'>7,124</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | <div style='text-align: right'>229,436</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW | <div style='text-align: right'>11,088</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW | <div style='text-align: right'>6,552</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW | <div style='text-align: right'>4,284</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW | <div style='text-align: right'>341</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | <div style='text-align: right'>39,852</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW2 | <div style='text-align: right'>396</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW2 | <div style='text-align: right'>234</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW2 | <div style='text-align: right'>204</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW2 | <div style='text-align: right'>583</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | <div style='text-align: right'>164,164</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW | <div style='text-align: right'>15,092</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | <div style='text-align: right'>371,255</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW2 | <div style='text-align: right'>968</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | <div style='text-align: right'>34,320</div>  |
| verify_fibair | AccessAdapter<2> | true | MulE | BBE4MUL | <div style='text-align: right'>13,882</div>  |
| verify_fibair | AccessAdapter<4> | true | MulE | BBE4MUL | <div style='text-align: right'>8,203</div>  |
| verify_fibair | Boundary | true | MulE | BBE4MUL | <div style='text-align: right'>572</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | <div style='text-align: right'>20,400</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEF | MUL | <div style='text-align: right'>3,718</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEF | MUL | <div style='text-align: right'>2,197</div>  |
| verify_fibair | Boundary | true | MulEF | MUL | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,320</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,892</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,118</div>  |
| verify_fibair | Boundary | true | MulEI | BBE4MUL | <div style='text-align: right'>924</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | <div style='text-align: right'>5,412</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | STOREW | <div style='text-align: right'>682</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | STOREW | <div style='text-align: right'>390</div>  |
| verify_fibair | Boundary | true | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | <div style='text-align: right'>32,880</div>  |
| verify_fibair | Boundary | true | MulF | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | <div style='text-align: right'>30</div>  |
| verify_fibair | Boundary | true | MulFI | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | <div style='text-align: right'>39,990</div>  |
| verify_fibair | Boundary | true | MulV | MUL | <div style='text-align: right'>14,641</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | <div style='text-align: right'>40,440</div>  |
| verify_fibair | Boundary | true | MulVI | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>18,564</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>610,428</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>8,806</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>148,135</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | <div style='text-align: right'>551,696</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW | <div style='text-align: right'>3,696</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW | <div style='text-align: right'>2,184</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW | <div style='text-align: right'>148,016</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | <div style='text-align: right'>83,312</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW2 | <div style='text-align: right'>3,696</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW2 | <div style='text-align: right'>2,184</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW2 | <div style='text-align: right'>7,568</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | <div style='text-align: right'>113,652</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW | <div style='text-align: right'>30,492</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | <div style='text-align: right'>68,798</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreF | STOREW2 | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreF | STOREW2 | <div style='text-align: right'>78</div>  |
| verify_fibair | AccessAdapter<8> | true | StoreF | STOREW2 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW2 | <div style='text-align: right'>15,070</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | <div style='text-align: right'>310,650</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | <div style='text-align: right'>559,691</div>  |
| verify_fibair | Boundary | true | StoreHintWord | SHINTW | <div style='text-align: right'>150,161</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | <div style='text-align: right'>26,650</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW | <div style='text-align: right'>7,150</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | <div style='text-align: right'>212,175</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW2 | <div style='text-align: right'>45,210</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubE | FE4SUB | <div style='text-align: right'>18,656</div>  |
| verify_fibair | AccessAdapter<4> | true | SubE | FE4SUB | <div style='text-align: right'>11,024</div>  |
| verify_fibair | Boundary | true | SubE | FE4SUB | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | <div style='text-align: right'>16,113</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | LOADW | <div style='text-align: right'>1,419</div>  |
| verify_fibair | Boundary | true | SubEF | LOADW | <div style='text-align: right'>99</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | <div style='text-align: right'>3,930</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | SUB | <div style='text-align: right'>1,419</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEF | SUB | <div style='text-align: right'>1,677</div>  |
| verify_fibair | Boundary | true | SubEF | SUB | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | <div style='text-align: right'>240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEI | ADD | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEI | ADD | <div style='text-align: right'>26</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | <div style='text-align: right'>42,870</div>  |
| verify_fibair | Boundary | true | SubV | SUB | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | <div style='text-align: right'>47,160</div>  |
| verify_fibair | Boundary | true | SubVI | SUB | <div style='text-align: right'>15,191</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | <div style='text-align: right'>5,040</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: green">(-1.0 [-5.9%])</span> <div style='text-align: right'>16.0</div>  | <span style="color: red">(+1.0 [+0.4%])</span> <div style='text-align: right'>262.0</div>  | <span style="color: red">(+1.0 [+0.5%])</span> <div style='text-align: right'>216.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>47.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8,307,357</div>  | <div style='text-align: right'>199,267</div>  | <span style="color: green">(-9.0 [-0.6%])</span> <div style='text-align: right'>1,578.0</div>  | <span style="color: green">(-1.0 [-4.5%])</span> <div style='text-align: right'>21.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VolatileBoundaryAir | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<2> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<4> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<8> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | 0 | <div style='text-align: right'>1,245,184</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <div style='text-align: right'>884,736</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <div style='text-align: right'>475,136</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>512</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>1,218,560</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| verify_fibair | 0 | <div style='text-align: right'>44.0</div>  | <span style="color: green">(-10.0 [-0.8%])</span> <div style='text-align: right'>1,272.0</div>  | <div style='text-align: right'>23,085,080</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b2237b57efa19b4ea70866def3507f6294f9f683/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/b2237b57efa19b4ea70866def3507f6294f9f683

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11948423267)
