| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <span style="color: green">(-9,195,877 [-52.9%])</span> <div style='text-align: right'>8,172,372</div>  | <span style="color: green">(-228,724 [-53.5%])</span> <div style='text-align: right'>198,759</div>  | <span style="color: green">(-602.0 [-27.7%])</span> <div style='text-align: right'>1,573.0</div>  |


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
| verify_fibair | true | <span style="color: green">(-3,594.0 [-54.1%])</span> <div style='text-align: right'>3,050.0</div>  | <span style="color: green">(-9,195,877 [-52.9%])</span> <div style='text-align: right'>8,172,372</div>  | <span style="color: green">(-228,724 [-53.5%])</span> <div style='text-align: right'>198,759</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | <span style="color: green">(-34,684 [-52.1%])</span> <div style='text-align: right'>31,888</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | <span style="color: green">(-5,786 [-52.8%])</span> <div style='text-align: right'>5,170</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | <span style="color: green">(-71,637 [-51.7%])</span> <div style='text-align: right'>66,938</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | <span style="color: green">(-105,946 [-55.2%])</span> <div style='text-align: right'>85,882</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | <span style="color: green">(-2,700 [-55.3%])</span> <div style='text-align: right'>2,186</div>  |
| verify_fibair | FriMatOpeningAir | true | <span style="color: green">(-464 [-58.0%])</span> <div style='text-align: right'>336</div>  |
| verify_fibair | Memory AccessAdapter<2> | true | <span style="color: green">(-18,965 [-53.5%])</span> <div style='text-align: right'>16,464</div>  |
| verify_fibair | Memory AccessAdapter<4> | true | <span style="color: green">(-9,483 [-53.5%])</span> <div style='text-align: right'>8,232</div>  |
| verify_fibair | Memory AccessAdapter<8> | true | <span style="color: green">(-1,966 [-51.2%])</span> <div style='text-align: right'>1,875</div>  |
| verify_fibair | Memory Boundary | true | <span style="color: green">(-52,850 [-54.2%])</span> <div style='text-align: right'>44,588</div>  |
| verify_fibair | PhantomAir | true | <span style="color: green">(-6,534 [-55.6%])</span> <div style='text-align: right'>5,212</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | <span style="color: green">(-1,263 [-48.2%])</span> <div style='text-align: right'>1,357</div>  |
| verify_fibair | ProgramChip | true | <span style="color: green">(-17,806 [-50.3%])</span> <div style='text-align: right'>17,624</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair | true |  | JAL | <div style='text-align: right'>1</div>  |
| verify_fibair | true |  | STOREW | <div style='text-align: right'>2</div>  |
| verify_fibair | true | AddE | FE4ADD | <span style="color: green">(-638 [-56.5%])</span> <div style='text-align: right'>492</div>  |
| verify_fibair | true | AddEFFI | LOADW | <span style="color: green">(-58 [-45.3%])</span> <div style='text-align: right'>70</div>  |
| verify_fibair | true | AddEFFI | STOREW | <span style="color: green">(-174 [-45.3%])</span> <div style='text-align: right'>210</div>  |
| verify_fibair | true | AddEI | ADD | <span style="color: green">(-3,080 [-54.4%])</span> <div style='text-align: right'>2,584</div>  |
| verify_fibair | true | AddFI | ADD | <span style="color: green">(-1,517 [-54.1%])</span> <div style='text-align: right'>1,285</div>  |
| verify_fibair | true | AddV | ADD | <span style="color: green">(-876 [-48.3%])</span> <div style='text-align: right'>939</div>  |
| verify_fibair | true | AddVI | ADD | <span style="color: green">(-18,894 [-52.9%])</span> <div style='text-align: right'>16,803</div>  |
| verify_fibair | true | Alloc | ADD | <span style="color: green">(-6,978 [-53.6%])</span> <div style='text-align: right'>6,029</div>  |
| verify_fibair | true | Alloc | LOADW | <span style="color: green">(-6,978 [-53.6%])</span> <div style='text-align: right'>6,029</div>  |
| verify_fibair | true | Alloc | MUL | <span style="color: green">(-4,910 [-54.7%])</span> <div style='text-align: right'>4,066</div>  |
| verify_fibair | true | AssertEqE | BNE | <span style="color: green">(-232 [-57.4%])</span> <div style='text-align: right'>172</div>  |
| verify_fibair | true | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| verify_fibair | true | AssertEqF | BNE | <span style="color: green">(-2,842 [-58.0%])</span> <div style='text-align: right'>2,059</div>  |
| verify_fibair | true | AssertEqV | BNE | <span style="color: green">(-2,030 [-57.3%])</span> <div style='text-align: right'>1,510</div>  |
| verify_fibair | true | AssertEqVI | BNE | <div style='text-align: right'>20</div>  |
| verify_fibair | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-compute-reduced-opening | PHANTOM | <span style="color: green">(-232 [-58.0%])</span> <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-exp-reverse-bits-len | PHANTOM | <span style="color: green">(-232 [-58.0%])</span> <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-poseidon2-hash | PHANTOM | <span style="color: green">(-232 [-58.0%])</span> <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-poseidon2-hash-ext | PHANTOM | <span style="color: green">(-464 [-58.0%])</span> <div style='text-align: right'>336</div>  |
| verify_fibair | true | CT-poseidon2-hash-setup | PHANTOM | <span style="color: green">(-696 [-58.0%])</span> <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-single-mat-reduced-opening | PHANTOM | <span style="color: green">(-348 [-58.0%])</span> <div style='text-align: right'>252</div>  |
| verify_fibair | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| verify_fibair | true | CT-verify-batch | PHANTOM | <span style="color: green">(-232 [-58.0%])</span> <div style='text-align: right'>168</div>  |
| verify_fibair | true | CT-verify-batch-ext | PHANTOM | <span style="color: green">(-464 [-58.0%])</span> <div style='text-align: right'>336</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast | PHANTOM | <span style="color: green">(-696 [-58.0%])</span> <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <span style="color: green">(-696 [-58.0%])</span> <div style='text-align: right'>504</div>  |
| verify_fibair | true | CT-verify-query | PHANTOM | <span style="color: green">(-116 [-58.0%])</span> <div style='text-align: right'>84</div>  |
| verify_fibair | true | DivE | BBE4DIV | <span style="color: green">(-406 [-57.8%])</span> <div style='text-align: right'>296</div>  |
| verify_fibair | true | DivEIN | BBE4DIV | <div style='text-align: right'>1</div>  |
| verify_fibair | true | DivEIN | STOREW | <div style='text-align: right'>4</div>  |
| verify_fibair | true | DivFIN | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair | true | For | ADD | <span style="color: green">(-16,774 [-49.5%])</span> <div style='text-align: right'>17,108</div>  |
| verify_fibair | true | For | BNE | <span style="color: green">(-21,742 [-50.6%])</span> <div style='text-align: right'>21,209</div>  |
| verify_fibair | true | For | JAL | <span style="color: green">(-4,968 [-54.8%])</span> <div style='text-align: right'>4,101</div>  |
| verify_fibair | true | For | LOADW | <span style="color: green">(-406 [-58.0%])</span> <div style='text-align: right'>294</div>  |
| verify_fibair | true | For | STOREW | <span style="color: green">(-4,562 [-54.5%])</span> <div style='text-align: right'>3,807</div>  |
| verify_fibair | true | FriMatOpening | FRI_FOLD | <span style="color: green">(-174 [-58.0%])</span> <div style='text-align: right'>126</div>  |
| verify_fibair | true | HintBitsF | PHANTOM | <span style="color: green">(-58 [-57.4%])</span> <div style='text-align: right'>43</div>  |
| verify_fibair | true | HintInputVec | PHANTOM | <span style="color: green">(-2,068 [-51.3%])</span> <div style='text-align: right'>1,963</div>  |
| verify_fibair | true | IfEq | BNE | <span style="color: green">(-406 [-55.8%])</span> <div style='text-align: right'>321</div>  |
| verify_fibair | true | IfEqI | BNE | <span style="color: green">(-6,118 [-54.2%])</span> <div style='text-align: right'>5,175</div>  |
| verify_fibair | true | IfEqI | JAL | <span style="color: green">(-818 [-43.4%])</span> <div style='text-align: right'>1,066</div>  |
| verify_fibair | true | IfNe | BEQ | <span style="color: green">(-966 [-45.4%])</span> <div style='text-align: right'>1,163</div>  |
| verify_fibair | true | IfNe | JAL | <div style='text-align: right'>2</div>  |
| verify_fibair | true | IfNeI | BEQ | <span style="color: green">(-348 [-57.7%])</span> <div style='text-align: right'>255</div>  |
| verify_fibair | true | ImmE | STOREW | <span style="color: green">(-1,160 [-57.8%])</span> <div style='text-align: right'>848</div>  |
| verify_fibair | true | ImmF | STOREW | <span style="color: green">(-4,002 [-57.1%])</span> <div style='text-align: right'>3,002</div>  |
| verify_fibair | true | ImmV | STOREW | <span style="color: green">(-7,306 [-57.2%])</span> <div style='text-align: right'>5,462</div>  |
| verify_fibair | true | LoadE | LOADW | <span style="color: green">(-2,784 [-57.6%])</span> <div style='text-align: right'>2,052</div>  |
| verify_fibair | true | LoadE | LOADW2 | <span style="color: green">(-6,264 [-58.0%])</span> <div style='text-align: right'>4,536</div>  |
| verify_fibair | true | LoadF | LOADW | <span style="color: green">(-7,656 [-57.8%])</span> <div style='text-align: right'>5,596</div>  |
| verify_fibair | true | LoadF | LOADW2 | <span style="color: green">(-1,276 [-56.8%])</span> <div style='text-align: right'>972</div>  |
| verify_fibair | true | LoadV | LOADW | <span style="color: green">(-4,794 [-54.5%])</span> <div style='text-align: right'>4,004</div>  |
| verify_fibair | true | LoadV | LOADW2 | <span style="color: green">(-10,796 [-54.4%])</span> <div style='text-align: right'>9,055</div>  |
| verify_fibair | true | MulE | BBE4MUL | <span style="color: green">(-960 [-52.8%])</span> <div style='text-align: right'>858</div>  |
| verify_fibair | true | MulEF | MUL | <span style="color: green">(-928 [-57.7%])</span> <div style='text-align: right'>680</div>  |
| verify_fibair | true | MulEI | BBE4MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | true | MulEI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | true | MulF | MUL | <span style="color: green">(-1,108 [-50.3%])</span> <div style='text-align: right'>1,096</div>  |
| verify_fibair | true | MulFI | MUL | <div style='text-align: right'>1</div>  |
| verify_fibair | true | MulV | MUL | <span style="color: green">(-1,798 [-57.4%])</span> <div style='text-align: right'>1,333</div>  |
| verify_fibair | true | MulVI | MUL | <span style="color: green">(-1,256 [-48.2%])</span> <div style='text-align: right'>1,348</div>  |
| verify_fibair | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: green">(-908 [-45.4%])</span> <div style='text-align: right'>1,092</div>  |
| verify_fibair | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-355 [-57.3%])</span> <div style='text-align: right'>265</div>  |
| verify_fibair | true | StoreE | STOREW | <span style="color: green">(-18,560 [-58.0%])</span> <div style='text-align: right'>13,456</div>  |
| verify_fibair | true | StoreE | STOREW2 | <span style="color: green">(-2,784 [-57.8%])</span> <div style='text-align: right'>2,032</div>  |
| verify_fibair | true | StoreF | STOREW | <span style="color: green">(-3,712 [-57.2%])</span> <div style='text-align: right'>2,772</div>  |
| verify_fibair | true | StoreF | STOREW2 | <span style="color: green">(-2,204 [-56.8%])</span> <div style='text-align: right'>1,678</div>  |
| verify_fibair | true | StoreHintWord | ADD | <span style="color: green">(-9,352 [-47.5%])</span> <div style='text-align: right'>10,355</div>  |
| verify_fibair | true | StoreHintWord | SHINTW | <span style="color: green">(-13,218 [-49.2%])</span> <div style='text-align: right'>13,651</div>  |
| verify_fibair | true | StoreV | STOREW | <span style="color: green">(-812 [-55.5%])</span> <div style='text-align: right'>650</div>  |
| verify_fibair | true | StoreV | STOREW2 | <span style="color: green">(-5,918 [-53.3%])</span> <div style='text-align: right'>5,175</div>  |
| verify_fibair | true | SubE | FE4SUB | <span style="color: green">(-696 [-57.9%])</span> <div style='text-align: right'>506</div>  |
| verify_fibair | true | SubEF | LOADW | <span style="color: green">(-522 [-57.0%])</span> <div style='text-align: right'>393</div>  |
| verify_fibair | true | SubEF | SUB | <span style="color: green">(-174 [-57.0%])</span> <div style='text-align: right'>131</div>  |
| verify_fibair | true | SubEI | ADD | <div style='text-align: right'>8</div>  |
| verify_fibair | true | SubV | SUB | <span style="color: green">(-1,672 [-53.9%])</span> <div style='text-align: right'>1,429</div>  |
| verify_fibair | true | SubVI | SUB | <span style="color: green">(-2,088 [-57.0%])</span> <div style='text-align: right'>1,572</div>  |
| verify_fibair | true | SubVIN | SUB | <span style="color: green">(-232 [-58.0%])</span> <div style='text-align: right'>168</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | <div style='text-align: right'>10</div>  |
| verify_fibair | Boundary | true |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | <div style='text-align: right'>82</div>  |
| verify_fibair | Boundary | true |  | STOREW | <div style='text-align: right'>22</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | <span style="color: green">(-25,520 [-56.5%])</span> <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> | true | AddE | FE4ADD | <span style="color: green">(-14,036 [-56.4%])</span> <div style='text-align: right'>10,846</div>  |
| verify_fibair | AccessAdapter<4> | true | AddE | FE4ADD | <span style="color: green">(-8,294 [-56.4%])</span> <div style='text-align: right'>6,409</div>  |
| verify_fibair | Boundary | true | AddE | FE4ADD | <div style='text-align: right'>792</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | <span style="color: green">(-2,378 [-45.3%])</span> <div style='text-align: right'>2,870</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEFFI | LOADW | <div style='text-align: right'>156</div>  |
| verify_fibair | Boundary | true | AddEFFI | LOADW | <div style='text-align: right'>176</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | <span style="color: green">(-7,134 [-45.3%])</span> <div style='text-align: right'>8,610</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEFFI | STOREW | <div style='text-align: right'>132</div>  |
| verify_fibair | Boundary | true | AddEFFI | STOREW | <div style='text-align: right'>528</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | <span style="color: green">(-92,400 [-54.4%])</span> <div style='text-align: right'>77,520</div>  |
| verify_fibair | AccessAdapter<2> | true | AddEI | ADD | <span style="color: green">(-12,562 [-51.3%])</span> <div style='text-align: right'>11,902</div>  |
| verify_fibair | AccessAdapter<4> | true | AddEI | ADD | <span style="color: green">(-7,423 [-51.3%])</span> <div style='text-align: right'>7,033</div>  |
| verify_fibair | Boundary | true | AddEI | ADD | <div style='text-align: right'>440</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | <span style="color: green">(-45,510 [-54.1%])</span> <div style='text-align: right'>38,550</div>  |
| verify_fibair | Boundary | true | AddFI | ADD | <div style='text-align: right'>231</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | <span style="color: green">(-26,280 [-48.3%])</span> <div style='text-align: right'>28,170</div>  |
| verify_fibair | Boundary | true | AddV | ADD | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | <span style="color: green">(-566,820 [-52.9%])</span> <div style='text-align: right'>504,090</div>  |
| verify_fibair | Boundary | true | AddVI | ADD | <span style="color: green">(-19,778 [-55.8%])</span> <div style='text-align: right'>15,675</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | <span style="color: green">(-209,340 [-53.6%])</span> <div style='text-align: right'>180,870</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | <span style="color: green">(-286,098 [-53.6%])</span> <div style='text-align: right'>247,189</div>  |
| verify_fibair | Boundary | true | Alloc | LOADW | <span style="color: green">(-638 [-36.2%])</span> <div style='text-align: right'>1,122</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | <span style="color: green">(-147,300 [-54.7%])</span> <div style='text-align: right'>121,980</div>  |
| verify_fibair | AccessAdapter<2> | true | Alloc | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<4> | true | Alloc | MUL | <div style='text-align: right'>39</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | <span style="color: green">(-5,336 [-57.4%])</span> <div style='text-align: right'>3,956</div>  |
| verify_fibair | AccessAdapter<2> | true | AssertEqE | BNE | <span style="color: green">(-1,276 [-57.4%])</span> <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> | true | AssertEqE | BNE | <span style="color: green">(-754 [-57.4%])</span> <div style='text-align: right'>559</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | <span style="color: green">(-65,366 [-58.0%])</span> <div style='text-align: right'>47,357</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | <span style="color: green">(-46,690 [-57.3%])</span> <div style='text-align: right'>34,730</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | <div style='text-align: right'>460</div>  |
| verify_fibair | PhantomAir | true | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | <span style="color: green">(-1,392 [-58.0%])</span> <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | <span style="color: green">(-1,392 [-58.0%])</span> <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash | PHANTOM | <span style="color: green">(-1,392 [-58.0%])</span> <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | <span style="color: green">(-2,784 [-58.0%])</span> <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | <span style="color: green">(-4,176 [-58.0%])</span> <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | <span style="color: green">(-2,088 [-58.0%])</span> <div style='text-align: right'>1,512</div>  |
| verify_fibair | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch | PHANTOM | <span style="color: green">(-1,392 [-58.0%])</span> <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-ext | PHANTOM | <span style="color: green">(-2,784 [-58.0%])</span> <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | <span style="color: green">(-4,176 [-58.0%])</span> <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | <span style="color: green">(-4,176 [-58.0%])</span> <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | true | CT-verify-query | PHANTOM | <span style="color: green">(-696 [-58.0%])</span> <div style='text-align: right'>504</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | <span style="color: green">(-16,240 [-57.8%])</span> <div style='text-align: right'>11,840</div>  |
| verify_fibair | AccessAdapter<2> | true | DivE | BBE4DIV | <span style="color: green">(-3,828 [-57.0%])</span> <div style='text-align: right'>2,882</div>  |
| verify_fibair | AccessAdapter<4> | true | DivE | BBE4DIV | <span style="color: green">(-2,262 [-57.0%])</span> <div style='text-align: right'>1,703</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | <div style='text-align: right'>40</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | BBE4DIV | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | true | DivEIN | BBE4DIV | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | <div style='text-align: right'>164</div>  |
| verify_fibair | AccessAdapter<2> | true | DivEIN | STOREW | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | <span style="color: green">(-503,220 [-49.5%])</span> <div style='text-align: right'>513,240</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | <span style="color: green">(-500,066 [-50.6%])</span> <div style='text-align: right'>487,807</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | <span style="color: green">(-49,680 [-54.8%])</span> <div style='text-align: right'>41,010</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | <span style="color: green">(-16,646 [-58.0%])</span> <div style='text-align: right'>12,054</div>  |
| verify_fibair | Boundary | true | For | LOADW | <span style="color: green">(-638 [-58.0%])</span> <div style='text-align: right'>462</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | <span style="color: green">(-187,042 [-54.5%])</span> <div style='text-align: right'>156,087</div>  |
| verify_fibair | Boundary | true | For | STOREW | <div style='text-align: right'>462</div>  |
| verify_fibair | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | <span style="color: green">(-2,552 [-55.8%])</span> <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | <span style="color: green">(-1,508 [-55.8%])</span> <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | <span style="color: green">(-29,696 [-58.0%])</span> <div style='text-align: right'>21,504</div>  |
| verify_fibair | PhantomAir | true | HintBitsF | PHANTOM | <span style="color: green">(-348 [-57.4%])</span> <div style='text-align: right'>258</div>  |
| verify_fibair | PhantomAir | true | HintInputVec | PHANTOM | <span style="color: green">(-12,408 [-51.3%])</span> <div style='text-align: right'>11,778</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | <span style="color: green">(-9,338 [-55.8%])</span> <div style='text-align: right'>7,383</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | <span style="color: green">(-140,714 [-54.2%])</span> <div style='text-align: right'>119,025</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | <span style="color: green">(-8,180 [-43.4%])</span> <div style='text-align: right'>10,660</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | <span style="color: green">(-22,218 [-45.4%])</span> <div style='text-align: right'>26,749</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | <div style='text-align: right'>20</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | <span style="color: green">(-8,004 [-57.7%])</span> <div style='text-align: right'>5,865</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | <span style="color: green">(-47,560 [-57.8%])</span> <div style='text-align: right'>34,768</div>  |
| verify_fibair | AccessAdapter<2> | true | ImmE | STOREW | <span style="color: green">(-1,276 [-58.0%])</span> <div style='text-align: right'>924</div>  |
| verify_fibair | AccessAdapter<4> | true | ImmE | STOREW | <span style="color: green">(-754 [-58.0%])</span> <div style='text-align: right'>546</div>  |
| verify_fibair | Boundary | true | ImmE | STOREW | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | <span style="color: green">(-164,082 [-57.1%])</span> <div style='text-align: right'>123,082</div>  |
| verify_fibair | Boundary | true | ImmF | STOREW | <span style="color: green">(-638 [-29.9%])</span> <div style='text-align: right'>1,496</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | <span style="color: green">(-299,546 [-57.2%])</span> <div style='text-align: right'>223,942</div>  |
| verify_fibair | Boundary | true | ImmV | STOREW | <span style="color: green">(-21,054 [-56.4%])</span> <div style='text-align: right'>16,258</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | <span style="color: green">(-114,144 [-57.6%])</span> <div style='text-align: right'>84,132</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW | <span style="color: green">(-10,208 [-57.1%])</span> <div style='text-align: right'>7,656</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW | <span style="color: green">(-6,032 [-57.1%])</span> <div style='text-align: right'>4,524</div>  |
| verify_fibair | Boundary | true | LoadE | LOADW | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | <span style="color: green">(-256,824 [-58.0%])</span> <div style='text-align: right'>185,976</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadE | LOADW2 | <span style="color: green">(-16,588 [-57.9%])</span> <div style='text-align: right'>12,056</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadE | LOADW2 | <span style="color: green">(-9,802 [-57.9%])</span> <div style='text-align: right'>7,124</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | <span style="color: green">(-313,896 [-57.8%])</span> <div style='text-align: right'>229,436</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW | <span style="color: green">(-15,312 [-58.0%])</span> <div style='text-align: right'>11,088</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW | <span style="color: green">(-9,048 [-58.0%])</span> <div style='text-align: right'>6,552</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW | <span style="color: green">(-5,916 [-58.0%])</span> <div style='text-align: right'>4,284</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW | <div style='text-align: right'>341</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | <span style="color: green">(-52,316 [-56.8%])</span> <div style='text-align: right'>39,852</div>  |
| verify_fibair | AccessAdapter<2> | true | LoadF | LOADW2 | <span style="color: green">(-319 [-44.6%])</span> <div style='text-align: right'>396</div>  |
| verify_fibair | AccessAdapter<4> | true | LoadF | LOADW2 | <span style="color: green">(-195 [-45.5%])</span> <div style='text-align: right'>234</div>  |
| verify_fibair | AccessAdapter<8> | true | LoadF | LOADW2 | <span style="color: green">(-119 [-36.8%])</span> <div style='text-align: right'>204</div>  |
| verify_fibair | Boundary | true | LoadF | LOADW2 | <span style="color: green">(-638 [-52.3%])</span> <div style='text-align: right'>583</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | <span style="color: green">(-196,554 [-54.5%])</span> <div style='text-align: right'>164,164</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW | <span style="color: green">(-19,778 [-56.7%])</span> <div style='text-align: right'>15,092</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | <span style="color: green">(-442,636 [-54.4%])</span> <div style='text-align: right'>371,255</div>  |
| verify_fibair | Boundary | true | LoadV | LOADW2 | <div style='text-align: right'>957</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | <span style="color: green">(-38,400 [-52.8%])</span> <div style='text-align: right'>34,320</div>  |
| verify_fibair | AccessAdapter<2> | true | MulE | BBE4MUL | <span style="color: green">(-15,114 [-52.1%])</span> <div style='text-align: right'>13,882</div>  |
| verify_fibair | AccessAdapter<4> | true | MulE | BBE4MUL | <span style="color: green">(-8,931 [-52.1%])</span> <div style='text-align: right'>8,203</div>  |
| verify_fibair | Boundary | true | MulE | BBE4MUL | <div style='text-align: right'>572</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | <span style="color: green">(-27,840 [-57.7%])</span> <div style='text-align: right'>20,400</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEF | MUL | <span style="color: green">(-5,104 [-57.9%])</span> <div style='text-align: right'>3,718</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEF | MUL | <span style="color: green">(-3,016 [-57.9%])</span> <div style='text-align: right'>2,197</div>  |
| verify_fibair | Boundary | true | MulEF | MUL | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,320</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,892</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | BBE4MUL | <div style='text-align: right'>1,118</div>  |
| verify_fibair | Boundary | true | MulEI | BBE4MUL | <div style='text-align: right'>924</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | <div style='text-align: right'>5,412</div>  |
| verify_fibair | AccessAdapter<2> | true | MulEI | STOREW | <div style='text-align: right'>682</div>  |
| verify_fibair | AccessAdapter<4> | true | MulEI | STOREW | <div style='text-align: right'>390</div>  |
| verify_fibair | Boundary | true | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | <span style="color: green">(-33,240 [-50.3%])</span> <div style='text-align: right'>32,880</div>  |
| verify_fibair | Boundary | true | MulF | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | <div style='text-align: right'>30</div>  |
| verify_fibair | Boundary | true | MulFI | MUL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | <span style="color: green">(-53,940 [-57.4%])</span> <div style='text-align: right'>39,990</div>  |
| verify_fibair | Boundary | true | MulV | MUL | <span style="color: green">(-19,778 [-57.5%])</span> <div style='text-align: right'>14,641</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | <span style="color: green">(-37,680 [-48.2%])</span> <div style='text-align: right'>40,440</div>  |
| verify_fibair | Boundary | true | MulVI | MUL | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: green">(-39,952 [-45.4%])</span> <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: green">(-23,608 [-45.4%])</span> <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: green">(-15,436 [-45.4%])</span> <div style='text-align: right'>18,564</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: green">(-507,572 [-45.4%])</span> <div style='text-align: right'>610,428</div>  |
| verify_fibair | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-30,932 [-57.6%])</span> <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-18,278 [-57.6%])</span> <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-11,951 [-57.6%])</span> <div style='text-align: right'>8,806</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-198,445 [-57.3%])</span> <div style='text-align: right'>148,135</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | <span style="color: green">(-760,960 [-58.0%])</span> <div style='text-align: right'>551,696</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW | <span style="color: green">(-5,104 [-58.0%])</span> <div style='text-align: right'>3,696</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW | <span style="color: green">(-3,016 [-58.0%])</span> <div style='text-align: right'>2,184</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW | <span style="color: green">(-204,160 [-58.0%])</span> <div style='text-align: right'>148,016</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | <span style="color: green">(-114,144 [-57.8%])</span> <div style='text-align: right'>83,312</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreE | STOREW2 | <span style="color: green">(-5,104 [-58.0%])</span> <div style='text-align: right'>3,696</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreE | STOREW2 | <span style="color: green">(-3,016 [-58.0%])</span> <div style='text-align: right'>2,184</div>  |
| verify_fibair | Boundary | true | StoreE | STOREW2 | <span style="color: green">(-10,208 [-57.4%])</span> <div style='text-align: right'>7,568</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | <span style="color: green">(-152,192 [-57.2%])</span> <div style='text-align: right'>113,652</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW | <span style="color: green">(-40,832 [-57.2%])</span> <div style='text-align: right'>30,492</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | <span style="color: green">(-90,364 [-56.8%])</span> <div style='text-align: right'>68,798</div>  |
| verify_fibair | AccessAdapter<2> | true | StoreF | STOREW2 | <div style='text-align: right'>132</div>  |
| verify_fibair | AccessAdapter<4> | true | StoreF | STOREW2 | <div style='text-align: right'>78</div>  |
| verify_fibair | AccessAdapter<8> | true | StoreF | STOREW2 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | true | StoreF | STOREW2 | <span style="color: green">(-20,416 [-57.5%])</span> <div style='text-align: right'>15,070</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | <span style="color: green">(-280,560 [-47.5%])</span> <div style='text-align: right'>310,650</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | <span style="color: green">(-541,938 [-49.2%])</span> <div style='text-align: right'>559,691</div>  |
| verify_fibair | Boundary | true | StoreHintWord | SHINTW | <span style="color: green">(-145,398 [-49.2%])</span> <div style='text-align: right'>150,161</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | <span style="color: green">(-33,292 [-55.5%])</span> <div style='text-align: right'>26,650</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW | <span style="color: green">(-8,932 [-55.5%])</span> <div style='text-align: right'>7,150</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | <span style="color: green">(-242,638 [-53.3%])</span> <div style='text-align: right'>212,175</div>  |
| verify_fibair | Boundary | true | StoreV | STOREW2 | <span style="color: green">(-48,048 [-51.5%])</span> <div style='text-align: right'>45,210</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | <span style="color: green">(-27,840 [-57.9%])</span> <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubE | FE4SUB | <span style="color: green">(-25,520 [-57.8%])</span> <div style='text-align: right'>18,656</div>  |
| verify_fibair | AccessAdapter<4> | true | SubE | FE4SUB | <span style="color: green">(-15,080 [-57.8%])</span> <div style='text-align: right'>11,024</div>  |
| verify_fibair | Boundary | true | SubE | FE4SUB | <div style='text-align: right'>220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | <span style="color: green">(-21,402 [-57.0%])</span> <div style='text-align: right'>16,113</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | LOADW | <span style="color: green">(-1,914 [-57.4%])</span> <div style='text-align: right'>1,419</div>  |
| verify_fibair | Boundary | true | SubEF | LOADW | <div style='text-align: right'>99</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | <span style="color: green">(-5,220 [-57.0%])</span> <div style='text-align: right'>3,930</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEF | SUB | <span style="color: green">(-1,914 [-57.4%])</span> <div style='text-align: right'>1,419</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEF | SUB | <span style="color: green">(-2,262 [-57.4%])</span> <div style='text-align: right'>1,677</div>  |
| verify_fibair | Boundary | true | SubEF | SUB | <div style='text-align: right'>33</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | <div style='text-align: right'>240</div>  |
| verify_fibair | AccessAdapter<2> | true | SubEI | ADD | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<4> | true | SubEI | ADD | <div style='text-align: right'>26</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | <span style="color: green">(-50,160 [-53.9%])</span> <div style='text-align: right'>42,870</div>  |
| verify_fibair | Boundary | true | SubV | SUB | <div style='text-align: right'>44</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | <span style="color: green">(-62,640 [-57.0%])</span> <div style='text-align: right'>47,160</div>  |
| verify_fibair | Boundary | true | SubVI | SUB | <span style="color: green">(-20,416 [-57.3%])</span> <div style='text-align: right'>15,191</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | <span style="color: green">(-6,960 [-58.0%])</span> <div style='text-align: right'>5,040</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: red">(+2.0 [+14.3%])</span> <div style='text-align: right'>16.0</div>  | <span style="color: green">(-373.0 [-54.4%])</span> <div style='text-align: right'>313.0</div>  | <span style="color: green">(-327.0 [-55.1%])</span> <div style='text-align: right'>266.0</div>  | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <span style="color: red">(+13.0 [+38.2%])</span> <div style='text-align: right'>47.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-9,195,877 [-52.9%])</span> <div style='text-align: right'>8,172,372</div>  | <span style="color: green">(-228,724 [-53.5%])</span> <div style='text-align: right'>198,759</div>  | <span style="color: green">(-602.0 [-27.7%])</span> <div style='text-align: right'>1,573.0</div>  | <span style="color: green">(-23.0 [-51.1%])</span> <div style='text-align: right'>22.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <span style="color: green">(-1 [-11.1%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VolatileBoundaryAir | <span style="color: green">(-1 [-5.9%])</span> <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<2> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<4> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<8> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<16> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<32> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<64> | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | PhantomAir | <span style="color: green">(-1 [-20.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <span style="color: green">(-5 [-13.9%])</span> <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <span style="color: green">(-5 [-17.9%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <span style="color: green">(-2 [-25.0%])</span> <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | FriMatOpeningAir | <span style="color: green">(-17 [-22.4%])</span> <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <span style="color: green">(-8 [-1.5%])</span> <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <span style="color: green">(-589,824 [-50.0%])</span> <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmConnectorAir | 0 | <span style="color: green">(-8 [-25.0%])</span> <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | 0 | <span style="color: green">(-1,769,472 [-58.7%])</span> <div style='text-align: right'>1,245,184</div>  | <div style='text-align: right'>11</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: green">(-65,536 [-50.0%])</span> <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <span style="color: green">(-1,409,024 [-61.4%])</span> <div style='text-align: right'>884,736</div>  | <div style='text-align: right'>11</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <span style="color: green">(-737,280 [-60.8%])</span> <div style='text-align: right'>475,136</div>  | <div style='text-align: right'>13</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: green">(-16,384 [-50.0%])</span> <div style='text-align: right'>16,384</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <span style="color: green">(-200,704 [-59.8%])</span> <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>17</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: green">(-4,096 [-50.0%])</span> <div style='text-align: right'>4,096</div>  |
| verify_fibair | PhantomAir | 0 | <span style="color: green">(-180,224 [-61.1%])</span> <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: green">(-8,192 [-50.0%])</span> <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <span style="color: green">(-13,762,560 [-61.8%])</span> <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <span style="color: green">(-20 [-45.5%])</span> <div style='text-align: right'>24</div>  |  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <span style="color: green">(-7,634,944 [-82.0%])</span> <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <span style="color: green">(-20 [-41.7%])</span> <div style='text-align: right'>28</div>  |  | <span style="color: green">(-98,304 [-75.0%])</span> <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <span style="color: green">(-311,296 [-63.3%])</span> <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <span style="color: green">(-8 [-40.0%])</span> <div style='text-align: right'>12</div>  |  | <span style="color: green">(-8,192 [-50.0%])</span> <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <span style="color: green">(-10,747,904 [-62.1%])</span> <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <span style="color: green">(-16 [-44.4%])</span> <div style='text-align: right'>20</div>  |  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <span style="color: green">(-376,832 [-60.5%])</span> <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <span style="color: green">(-16 [-44.4%])</span> <div style='text-align: right'>20</div>  |  | <span style="color: green">(-4,096 [-50.0%])</span> <div style='text-align: right'>4,096</div>  |
| verify_fibair | FriMatOpeningAir | 0 | <span style="color: green">(-141,312 [-66.3%])</span> <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <span style="color: green">(-68 [-47.2%])</span> <div style='text-align: right'>76</div>  |  | <span style="color: green">(-512 [-50.0%])</span> <div style='text-align: right'>512</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-1,349,632 [-52.6%])</span> <div style='text-align: right'>1,218,560</div>  | <div style='text-align: right'>559</div>  | <span style="color: green">(-32 [-47.1%])</span> <div style='text-align: right'>36</div>  |  | <span style="color: green">(-2,048 [-50.0%])</span> <div style='text-align: right'>2,048</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: green">(-229.0 [-15.4%])</span> <div style='text-align: right'>1,260.0</div>  | <span style="color: green">(-39,211,016 [-62.9%])</span> <div style='text-align: right'>23,085,080</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/2f1818cbda5e2b77846508bcafb51062f63b4413
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11730338268)
