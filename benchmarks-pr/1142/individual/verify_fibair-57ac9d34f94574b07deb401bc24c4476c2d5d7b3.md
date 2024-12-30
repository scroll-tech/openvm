| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <span style="color: red">(+16,270 [+0.1%])</span> <div style='text-align: right'>14,607,000</div>  | <span style="color: green">(-222 [-0.1%])</span> <div style='text-align: right'>397,072</div>  | <span style="color: green">(-39.0 [-2.5%])</span> <div style='text-align: right'>1,512.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | interactions | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |
| ProgramAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |
| VmConnectorAir |  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>2</div>  |  |
| PersistentBoundaryAir<8> |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>2</div>  |  |
| MemoryMerkleAir<8> |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  |  | <div style='text-align: right'>2</div>  |  |
| AccessAdapterAir<2> |  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  |  |
| AccessAdapterAir<4> |  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  |  |
| AccessAdapterAir<8> |  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  |  |
| NativePoseidon2Air<BabyBearParameters>, 1> |  | <div style='text-align: right'>310</div>  | <div style='text-align: right'>31</div>  |  | <div style='text-align: right'>2</div>  |  |
| FriReducedOpeningAir |  | <div style='text-align: right'>76</div>  | <div style='text-align: right'>35</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> |  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  |  | <div style='text-align: right'>2</div>  |  |
| PhantomAir |  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>2</div>  |  |
| Poseidon2PeripheryAir<BabyBearParameters>, 1> |  | <div style='text-align: right'>286</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |  |
| VariableRangeCheckerAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |

| commit_exe_time_ms | fri.log_blowup | keygen_time_ms | main_trace_commit_time_ms | pcs_opening_time_ms | perm_trace_commit_time_ms | quotient_poly_commit_time_ms | quotient_poly_compute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| <span style="color: red">(+2.0 [+25.0%])</span> <div style='text-align: right'>10.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+1.0 [+0.5%])</span> <div style='text-align: right'>184.0</div>  | <div style='text-align: right'>5.0</div>  | <div style='text-align: right'>2.0</div>  | <div style='text-align: right'>0.0</div>  | <div style='text-align: right'>1.0</div>  | <div style='text-align: right'>0.0</div>  | <span style="color: green">(-2.0 [-18.2%])</span> <div style='text-align: right'>9.0</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>15.0</div>  |

| group | segment | execute_time_ms | generate_perm_trace_time_ms | main_trace_commit_time_ms | pcs_opening_time_ms | perm_trace_commit_time_ms | quotient_poly_commit_time_ms | quotient_poly_compute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 0 | <div style='text-align: right'>344.0</div>  | <div style='text-align: right'>31.0</div>  | <div style='text-align: right'>292.0</div>  | <div style='text-align: right'>489.0</div>  | <div style='text-align: right'>301.0</div>  | <div style='text-align: right'>188.0</div>  | <div style='text-align: right'>208.0</div>  | <span style="color: green">(-39.0 [-2.5%])</span> <div style='text-align: right'>1,512.0</div>  | <div style='text-align: right'>42,000,416</div>  | <span style="color: red">(+16,270 [+0.1%])</span> <div style='text-align: right'>14,607,000</div>  | <span style="color: green">(-222 [-0.1%])</span> <div style='text-align: right'>397,072</div>  | <span style="color: green">(-2.0 [-3.1%])</span> <div style='text-align: right'>62.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| verify_fibair | ProgramChip | 0 | <div style='text-align: right'>16,317</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | Boundary | 0 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | Merkle | 0 | <div style='text-align: right'>43,900</div>  |
| verify_fibair | AccessAdapter<2> | 0 | <span style="color: green">(-104 [-0.2%])</span> <div style='text-align: right'>58,108</div>  |
| verify_fibair | AccessAdapter<4> | 0 | <span style="color: green">(-52 [-0.1%])</span> <div style='text-align: right'>35,754</div>  |
| verify_fibair | AccessAdapter<8> | 0 | <div style='text-align: right'>23,300</div>  |
| verify_fibair | Arc<BabyBearParameters>, 1> | 0 | <span style="color: red">(+64 [+0.5%])</span> <div style='text-align: right'>13,455</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>336</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>2,186</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>68,144</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: green">(-111 [-2.1%])</span> <div style='text-align: right'>5,058</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>30,558</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>85,891</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>5,216</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair |  | JAL | 0 | <span style="color: green">(-5,168 [-100.0%])</span> <div style='text-align: right'>1</div>  |
| verify_fibair |  | STOREW | 0 | <span style="color: green">(-30,345 [-100.0%])</span> <div style='text-align: right'>2</div>  |
| verify_fibair | AddE | FE4ADD | 0 | <div style='text-align: right'>492</div>  |
| verify_fibair | AddEFFI | LOADW | 0 | <div style='text-align: right'>70</div>  |
| verify_fibair | AddEFFI | STOREW | 0 | <div style='text-align: right'>210</div>  |
| verify_fibair | AddEI | ADD | 0 | <div style='text-align: right'>2,584</div>  |
| verify_fibair | AddF | ADD | 0 | <div style='text-align: right'>1,333</div>  |
| verify_fibair | AddFI | ADD | 0 | <div style='text-align: right'>648</div>  |
| verify_fibair | AddV | ADD | 0 | <div style='text-align: right'>939</div>  |
| verify_fibair | AddVI | ADD | 0 | <div style='text-align: right'>15,976</div>  |
| verify_fibair | Alloc | ADD | 0 | <div style='text-align: right'>6,029</div>  |
| verify_fibair | Alloc | LOADW | 0 | <div style='text-align: right'>6,029</div>  |
| verify_fibair | Alloc | MUL | 0 | <div style='text-align: right'>4,066</div>  |
| verify_fibair | AssertEqE | BNE | 0 | <div style='text-align: right'>172</div>  |
| verify_fibair | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| verify_fibair | AssertEqF | BNE | 0 | <div style='text-align: right'>3,392</div>  |
| verify_fibair | AssertEqV | BNE | 0 | <div style='text-align: right'>177</div>  |
| verify_fibair | AssertEqVI | BNE | 0 | <div style='text-align: right'>20</div>  |
| verify_fibair | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>168</div>  |
| verify_fibair | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>168</div>  |
| verify_fibair | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>168</div>  |
| verify_fibair | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>336</div>  |
| verify_fibair | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| verify_fibair | CT-single-reduced-opening-eval | PHANTOM | 0 | <div style='text-align: right'>252</div>  |
| verify_fibair | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>168</div>  |
| verify_fibair | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>336</div>  |
| verify_fibair | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| verify_fibair | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| verify_fibair | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>84</div>  |
| verify_fibair | DivE | BBE4DIV | 0 | <div style='text-align: right'>296</div>  |
| verify_fibair | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1</div>  |
| verify_fibair | DivEIN | STOREW | 0 | <div style='text-align: right'>4</div>  |
| verify_fibair | DivFIN | DIV | 0 | <div style='text-align: right'>3</div>  |
| verify_fibair | For | ADD | 0 | <div style='text-align: right'>17,109</div>  |
| verify_fibair | For | BNE | 0 | <div style='text-align: right'>21,211</div>  |
| verify_fibair | For | JAL | 0 | <div style='text-align: right'>4,102</div>  |
| verify_fibair | For | LOADW | 0 | <div style='text-align: right'>294</div>  |
| verify_fibair | For | STOREW | 0 | <div style='text-align: right'>3,808</div>  |
| verify_fibair | FriReducedOpening | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>126</div>  |
| verify_fibair | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>43</div>  |
| verify_fibair | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>1,963</div>  |
| verify_fibair | IfEq | BNE | 0 | <div style='text-align: right'>321</div>  |
| verify_fibair | IfEqI | BNE | 0 | <div style='text-align: right'>3,843</div>  |
| verify_fibair | IfEqI | JAL | 0 | <div style='text-align: right'>953</div>  |
| verify_fibair | IfNe | BEQ | 0 | <div style='text-align: right'>1,163</div>  |
| verify_fibair | IfNe | JAL | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | IfNeI | BEQ | 0 | <div style='text-align: right'>255</div>  |
| verify_fibair | ImmE | STOREW | 0 | <div style='text-align: right'>848</div>  |
| verify_fibair | ImmF | STOREW | 0 | <div style='text-align: right'>4,335</div>  |
| verify_fibair | ImmV | STOREW | 0 | <div style='text-align: right'>4,130</div>  |
| verify_fibair | LoadE | LOADW | 0 | <div style='text-align: right'>2,052</div>  |
| verify_fibair | LoadE | LOADW2 | 0 | <div style='text-align: right'>4,536</div>  |
| verify_fibair | LoadF | LOADW | 0 | <div style='text-align: right'>6,929</div>  |
| verify_fibair | LoadF | LOADW2 | 0 | <div style='text-align: right'>972</div>  |
| verify_fibair | LoadHeapPtr | ADD | 0 | <div style='text-align: right'>1</div>  |
| verify_fibair | LoadV | LOADW | 0 | <div style='text-align: right'>2,671</div>  |
| verify_fibair | LoadV | LOADW2 | 0 | <div style='text-align: right'>9,061</div>  |
| verify_fibair | MulE | BBE4MUL | 0 | <div style='text-align: right'>858</div>  |
| verify_fibair | MulEF | MUL | 0 | <div style='text-align: right'>680</div>  |
| verify_fibair | MulEI | BBE4MUL | 0 | <div style='text-align: right'>33</div>  |
| verify_fibair | MulEI | STOREW | 0 | <div style='text-align: right'>132</div>  |
| verify_fibair | MulF | MUL | 0 | <div style='text-align: right'>2,429</div>  |
| verify_fibair | MulFI | MUL | 0 | <div style='text-align: right'>1,334</div>  |
| verify_fibair | MulVI | MUL | 0 | <div style='text-align: right'>1,348</div>  |
| verify_fibair | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>1,092</div>  |
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>265</div>  |
| verify_fibair | StoreE | STOREW | 0 | <div style='text-align: right'>13,456</div>  |
| verify_fibair | StoreE | STOREW2 | 0 | <div style='text-align: right'>2,032</div>  |
| verify_fibair | StoreF | STOREW | 0 | <div style='text-align: right'>2,772</div>  |
| verify_fibair | StoreF | STOREW2 | 0 | <div style='text-align: right'>1,679</div>  |
| verify_fibair | StoreHeapPtr | ADD | 0 | <div style='text-align: right'>1</div>  |
| verify_fibair | StoreHintWord | ADD | 0 | <div style='text-align: right'>10,355</div>  |
| verify_fibair | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>13,651</div>  |
| verify_fibair | StoreV | STOREW | 0 | <div style='text-align: right'>650</div>  |
| verify_fibair | StoreV | STOREW2 | 0 | <div style='text-align: right'>5,175</div>  |
| verify_fibair | SubE | FE4SUB | 0 | <div style='text-align: right'>506</div>  |
| verify_fibair | SubEF | LOADW | 0 | <div style='text-align: right'>393</div>  |
| verify_fibair | SubEF | SUB | 0 | <div style='text-align: right'>131</div>  |
| verify_fibair | SubEI | ADD | 0 | <div style='text-align: right'>8</div>  |
| verify_fibair | SubFI | SUB | 0 | <div style='text-align: right'>1,333</div>  |
| verify_fibair | SubV | SUB | 0 | <div style='text-align: right'>1,429</div>  |
| verify_fibair | SubVI | SUB | 0 | <div style='text-align: right'>239</div>  |
| verify_fibair | SubVIN | SUB | 0 | <div style='text-align: right'>168</div>  |
| verify_fibair | UnsafeCastVF | ADD | 0 | <div style='text-align: right'>1</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: green">(-51,680 [-100.0%])</span> <div style='text-align: right'>10</div>  |
| verify_fibair | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <span style="color: green">(-1,244,186 [-100.0%])</span> <div style='text-align: right'>41</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW | 0 | <span style="color: green">(-108,603 [-100.0%])</span> <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW | 0 | <span style="color: green">(-70,213 [-100.0%])</span> <div style='text-align: right'>13</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW | 0 | <span style="color: green">(-55,828 [-100.0%])</span> <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary |  | STOREW | 0 | <span style="color: green">(-131,360 [-100.0%])</span> <div style='text-align: right'>40</div>  |
| verify_fibair | Merkle |  | STOREW | 0 | <span style="color: green">(-557,440 [-99.8%])</span> <div style='text-align: right'>1,280</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | 0 | <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> | AddE | FE4ADD | 0 | <div style='text-align: right'>10,450</div>  |
| verify_fibair | AccessAdapter<4> | AddE | FE4ADD | 0 | <div style='text-align: right'>6,175</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | LOADW | 0 | <div style='text-align: right'>2,870</div>  |
| verify_fibair | AccessAdapter<2> | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| verify_fibair | AccessAdapter<4> | AddEFFI | LOADW | 0 | <div style='text-align: right'>364</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | STOREW | 0 | <div style='text-align: right'>8,610</div>  |
| verify_fibair | AccessAdapter<2> | AddEFFI | STOREW | 0 | <div style='text-align: right'>308</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 0 | <div style='text-align: right'>77,520</div>  |
| verify_fibair | AccessAdapter<2> | AddEI | ADD | 0 | <div style='text-align: right'>11,814</div>  |
| verify_fibair | AccessAdapter<4> | AddEI | ADD | 0 | <div style='text-align: right'>6,981</div>  |
| verify_fibair | AccessAdapter<8> | AddEI | ADD | 0 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | AddEI | ADD | 0 | <div style='text-align: right'>40</div>  |
| verify_fibair | Merkle | AddEI | ADD | 0 | <div style='text-align: right'>64</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | 0 | <div style='text-align: right'>39,990</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 0 | <div style='text-align: right'>19,440</div>  |
| verify_fibair | AccessAdapter<2> | AddFI | ADD | 0 | <div style='text-align: right'>154</div>  |
| verify_fibair | AccessAdapter<4> | AddFI | ADD | 0 | <div style='text-align: right'>26</div>  |
| verify_fibair | AccessAdapter<8> | AddFI | ADD | 0 | <div style='text-align: right'>34</div>  |
| verify_fibair | Boundary | AddFI | ADD | 0 | <div style='text-align: right'>80</div>  |
| verify_fibair | Merkle | AddFI | ADD | 0 | <div style='text-align: right'>64</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 0 | <div style='text-align: right'>28,170</div>  |
| verify_fibair | AccessAdapter<2> | AddV | ADD | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<4> | AddV | ADD | 0 | <div style='text-align: right'>13</div>  |
| verify_fibair | AccessAdapter<8> | AddV | ADD | 0 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | AddV | ADD | 0 | <div style='text-align: right'>40</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 0 | <div style='text-align: right'>479,280</div>  |
| verify_fibair | AccessAdapter<2> | AddVI | ADD | 0 | <div style='text-align: right'>506</div>  |
| verify_fibair | AccessAdapter<4> | AddVI | ADD | 0 | <div style='text-align: right'>520</div>  |
| verify_fibair | AccessAdapter<8> | AddVI | ADD | 0 | <div style='text-align: right'>663</div>  |
| verify_fibair | Boundary | AddVI | ADD | 0 | <div style='text-align: right'>1,560</div>  |
| verify_fibair | Merkle | AddVI | ADD | 0 | <div style='text-align: right'>2,624</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 0 | <div style='text-align: right'>180,870</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 0 | <div style='text-align: right'>247,189</div>  |
| verify_fibair | AccessAdapter<2> | Alloc | LOADW | 0 | <div style='text-align: right'>605</div>  |
| verify_fibair | AccessAdapter<4> | Alloc | LOADW | 0 | <div style='text-align: right'>403</div>  |
| verify_fibair | AccessAdapter<8> | Alloc | LOADW | 0 | <div style='text-align: right'>527</div>  |
| verify_fibair | Boundary | Alloc | LOADW | 0 | <div style='text-align: right'>1,240</div>  |
| verify_fibair | Merkle | Alloc | LOADW | 0 | <div style='text-align: right'>1,536</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 0 | <div style='text-align: right'>121,980</div>  |
| verify_fibair | AccessAdapter<2> | Alloc | MUL | 0 | <div style='text-align: right'>33</div>  |
| verify_fibair | AccessAdapter<4> | Alloc | MUL | 0 | <div style='text-align: right'>39</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | 0 | <div style='text-align: right'>3,956</div>  |
| verify_fibair | AccessAdapter<2> | AssertEqE | BNE | 0 | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> | AssertEqE | BNE | 0 | <div style='text-align: right'>559</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | 0 | <div style='text-align: right'>78,016</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | 0 | <div style='text-align: right'>4,071</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | 0 | <div style='text-align: right'>460</div>  |
| verify_fibair | PhantomAir | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | CT-single-reduced-opening-eval | PHANTOM | 0 | <div style='text-align: right'>1,512</div>  |
| verify_fibair | PhantomAir | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| verify_fibair | PhantomAir | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>1,008</div>  |
| verify_fibair | PhantomAir | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>2,016</div>  |
| verify_fibair | PhantomAir | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>3,024</div>  |
| verify_fibair | PhantomAir | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | 0 | <div style='text-align: right'>11,840</div>  |
| verify_fibair | AccessAdapter<2> | DivE | BBE4DIV | 0 | <div style='text-align: right'>2,882</div>  |
| verify_fibair | AccessAdapter<4> | DivE | BBE4DIV | 0 | <div style='text-align: right'>1,703</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>40</div>  |
| verify_fibair | AccessAdapter<2> | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>22</div>  |
| verify_fibair | AccessAdapter<4> | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | STOREW | 0 | <div style='text-align: right'>164</div>  |
| verify_fibair | AccessAdapter<2> | DivEIN | STOREW | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | 0 | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 0 | <div style='text-align: right'>513,270</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | 0 | <div style='text-align: right'>487,853</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | For | JAL | 0 | <div style='text-align: right'>41,020</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 0 | <div style='text-align: right'>12,054</div>  |
| verify_fibair | AccessAdapter<2> | For | LOADW | 0 | <div style='text-align: right'>231</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 0 | <div style='text-align: right'>156,128</div>  |
| verify_fibair | AccessAdapter<2> | For | STOREW | 0 | <div style='text-align: right'>253</div>  |
| verify_fibair | AccessAdapter<4> | For | STOREW | 0 | <div style='text-align: right'>260</div>  |
| verify_fibair | AccessAdapter<8> | For | STOREW | 0 | <div style='text-align: right'>340</div>  |
| verify_fibair | Boundary | For | STOREW | 0 | <div style='text-align: right'>800</div>  |
| verify_fibair | Merkle | For | STOREW | 0 | <div style='text-align: right'>1,024</div>  |
| verify_fibair | AccessAdapter<2> | FriReducedOpening | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> | FriReducedOpening | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>21,504</div>  |
| verify_fibair | PhantomAir | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>258</div>  |
| verify_fibair | PhantomAir | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>11,778</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | 0 | <div style='text-align: right'>7,383</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | 0 | <div style='text-align: right'>88,389</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | 0 | <div style='text-align: right'>9,530</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | 0 | <div style='text-align: right'>26,749</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 0 | <div style='text-align: right'>20</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | 0 | <div style='text-align: right'>5,865</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 0 | <div style='text-align: right'>34,768</div>  |
| verify_fibair | AccessAdapter<2> | ImmE | STOREW | 0 | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> | ImmE | STOREW | 0 | <div style='text-align: right'>559</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 0 | <div style='text-align: right'>177,735</div>  |
| verify_fibair | AccessAdapter<2> | ImmF | STOREW | 0 | <div style='text-align: right'>8,085</div>  |
| verify_fibair | AccessAdapter<4> | ImmF | STOREW | 0 | <div style='text-align: right'>8,762</div>  |
| verify_fibair | AccessAdapter<8> | ImmF | STOREW | 0 | <div style='text-align: right'>11,458</div>  |
| verify_fibair | Boundary | ImmF | STOREW | 0 | <div style='text-align: right'>26,960</div>  |
| verify_fibair | Merkle | ImmF | STOREW | 0 | <div style='text-align: right'>43,328</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 0 | <div style='text-align: right'>169,330</div>  |
| verify_fibair | AccessAdapter<2> | ImmV | STOREW | 0 | <div style='text-align: right'>847</div>  |
| verify_fibair | AccessAdapter<4> | ImmV | STOREW | 0 | <div style='text-align: right'>455</div>  |
| verify_fibair | AccessAdapter<8> | ImmV | STOREW | 0 | <div style='text-align: right'>595</div>  |
| verify_fibair | Boundary | ImmV | STOREW | 0 | <div style='text-align: right'>1,400</div>  |
| verify_fibair | Merkle | ImmV | STOREW | 0 | <div style='text-align: right'>2,624</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 0 | <div style='text-align: right'>84,132</div>  |
| verify_fibair | AccessAdapter<2> | LoadE | LOADW | 0 | <div style='text-align: right'>7,766</div>  |
| verify_fibair | AccessAdapter<4> | LoadE | LOADW | 0 | <div style='text-align: right'>4,589</div>  |
| verify_fibair | AccessAdapter<8> | LoadE | LOADW | 0 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | LoadE | LOADW | 0 | <div style='text-align: right'>40</div>  |
| verify_fibair | Merkle | LoadE | LOADW | 0 | <div style='text-align: right'>192</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW2 | 0 | <div style='text-align: right'>185,976</div>  |
| verify_fibair | AccessAdapter<2> | LoadE | LOADW2 | 0 | <div style='text-align: right'>12,056</div>  |
| verify_fibair | AccessAdapter<4> | LoadE | LOADW2 | 0 | <div style='text-align: right'>7,124</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 0 | <div style='text-align: right'>284,089</div>  |
| verify_fibair | AccessAdapter<2> | LoadF | LOADW | 0 | <div style='text-align: right'>18,546</div>  |
| verify_fibair | AccessAdapter<4> | LoadF | LOADW | 0 | <div style='text-align: right'>15,041</div>  |
| verify_fibair | AccessAdapter<8> | LoadF | LOADW | 0 | <div style='text-align: right'>15,368</div>  |
| verify_fibair | Boundary | LoadF | LOADW | 0 | <div style='text-align: right'>26,080</div>  |
| verify_fibair | Merkle | LoadF | LOADW | 0 | <div style='text-align: right'>41,920</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW2 | 0 | <div style='text-align: right'>39,852</div>  |
| verify_fibair | AccessAdapter<2> | LoadF | LOADW2 | 0 | <div style='text-align: right'>748</div>  |
| verify_fibair | AccessAdapter<4> | LoadF | LOADW2 | 0 | <div style='text-align: right'>559</div>  |
| verify_fibair | AccessAdapter<8> | LoadF | LOADW2 | 0 | <div style='text-align: right'>629</div>  |
| verify_fibair | Boundary | LoadF | LOADW2 | 0 | <div style='text-align: right'>1,000</div>  |
| verify_fibair | Merkle | LoadF | LOADW2 | 0 | <div style='text-align: right'>1,408</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 0 | <div style='text-align: right'>30</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 0 | <div style='text-align: right'>109,511</div>  |
| verify_fibair | AccessAdapter<2> | LoadV | LOADW | 0 | <div style='text-align: right'>154</div>  |
| verify_fibair | AccessAdapter<4> | LoadV | LOADW | 0 | <div style='text-align: right'>169</div>  |
| verify_fibair | AccessAdapter<8> | LoadV | LOADW | 0 | <div style='text-align: right'>221</div>  |
| verify_fibair | Boundary | LoadV | LOADW | 0 | <div style='text-align: right'>520</div>  |
| verify_fibair | Merkle | LoadV | LOADW | 0 | <div style='text-align: right'>768</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW2 | 0 | <div style='text-align: right'>371,501</div>  |
| verify_fibair | AccessAdapter<2> | LoadV | LOADW2 | 0 | <div style='text-align: right'>484</div>  |
| verify_fibair | AccessAdapter<4> | LoadV | LOADW2 | 0 | <div style='text-align: right'>286</div>  |
| verify_fibair | AccessAdapter<8> | LoadV | LOADW2 | 0 | <div style='text-align: right'>374</div>  |
| verify_fibair | Boundary | LoadV | LOADW2 | 0 | <div style='text-align: right'>880</div>  |
| verify_fibair | Merkle | LoadV | LOADW2 | 0 | <div style='text-align: right'>1,472</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | 0 | <div style='text-align: right'>34,320</div>  |
| verify_fibair | AccessAdapter<2> | MulE | BBE4MUL | 0 | <div style='text-align: right'>13,288</div>  |
| verify_fibair | AccessAdapter<4> | MulE | BBE4MUL | 0 | <div style='text-align: right'>7,852</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 0 | <div style='text-align: right'>20,400</div>  |
| verify_fibair | AccessAdapter<2> | MulEF | MUL | 0 | <div style='text-align: right'>3,740</div>  |
| verify_fibair | AccessAdapter<4> | MulEF | MUL | 0 | <div style='text-align: right'>2,210</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | 0 | <div style='text-align: right'>1,320</div>  |
| verify_fibair | AccessAdapter<2> | MulEI | BBE4MUL | 0 | <div style='text-align: right'>1,430</div>  |
| verify_fibair | AccessAdapter<4> | MulEI | BBE4MUL | 0 | <div style='text-align: right'>845</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 0 | <div style='text-align: right'>5,412</div>  |
| verify_fibair | AccessAdapter<2> | MulEI | STOREW | 0 | <div style='text-align: right'>693</div>  |
| verify_fibair | AccessAdapter<4> | MulEI | STOREW | 0 | <div style='text-align: right'>390</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 0 | <div style='text-align: right'>72,870</div>  |
| verify_fibair | AccessAdapter<2> | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 0 | <div style='text-align: right'>40,020</div>  |
| verify_fibair | AccessAdapter<2> | MulFI | MUL | 0 | <div style='text-align: right'>7,315</div>  |
| verify_fibair | AccessAdapter<4> | MulFI | MUL | 0 | <div style='text-align: right'>8,398</div>  |
| verify_fibair | AccessAdapter<8> | MulFI | MUL | 0 | <div style='text-align: right'>10,982</div>  |
| verify_fibair | Boundary | MulFI | MUL | 0 | <div style='text-align: right'>25,840</div>  |
| verify_fibair | Merkle | MulFI | MUL | 0 | <div style='text-align: right'>41,152</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 0 | <div style='text-align: right'>40,440</div>  |
| verify_fibair | AccessAdapter<2> | MulVI | MUL | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>18,564</div>  |
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>380,016</div>  |
| verify_fibair | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>8,806</div>  |
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>92,220</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 0 | <div style='text-align: right'>551,696</div>  |
| verify_fibair | AccessAdapter<2> | StoreE | STOREW | 0 | <div style='text-align: right'>77,704</div>  |
| verify_fibair | AccessAdapter<4> | StoreE | STOREW | 0 | <div style='text-align: right'>45,916</div>  |
| verify_fibair | AccessAdapter<8> | StoreE | STOREW | 0 | <div style='text-align: right'>31,467</div>  |
| verify_fibair | Boundary | StoreE | STOREW | 0 | <div style='text-align: right'>74,040</div>  |
| verify_fibair | Merkle | StoreE | STOREW | 0 | <div style='text-align: right'>225,152</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW2 | 0 | <div style='text-align: right'>83,312</div>  |
| verify_fibair | AccessAdapter<2> | StoreE | STOREW2 | 0 | <div style='text-align: right'>7,480</div>  |
| verify_fibair | AccessAdapter<4> | StoreE | STOREW2 | 0 | <div style='text-align: right'>4,420</div>  |
| verify_fibair | AccessAdapter<8> | StoreE | STOREW2 | 0 | <div style='text-align: right'>1,462</div>  |
| verify_fibair | Boundary | StoreE | STOREW2 | 0 | <div style='text-align: right'>3,440</div>  |
| verify_fibair | Merkle | StoreE | STOREW2 | 0 | <div style='text-align: right'>2,752</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 0 | <div style='text-align: right'>113,652</div>  |
| verify_fibair | AccessAdapter<2> | StoreF | STOREW | 0 | <div style='text-align: right'>15,246</div>  |
| verify_fibair | AccessAdapter<4> | StoreF | STOREW | 0 | <div style='text-align: right'>8,827</div>  |
| verify_fibair | AccessAdapter<8> | StoreF | STOREW | 0 | <div style='text-align: right'>5,780</div>  |
| verify_fibair | Boundary | StoreF | STOREW | 0 | <div style='text-align: right'>13,600</div>  |
| verify_fibair | Merkle | StoreF | STOREW | 0 | <div style='text-align: right'>236,480</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW2 | 0 | <div style='text-align: right'>68,839</div>  |
| verify_fibair | AccessAdapter<2> | StoreF | STOREW2 | 0 | <div style='text-align: right'>7,667</div>  |
| verify_fibair | AccessAdapter<4> | StoreF | STOREW2 | 0 | <div style='text-align: right'>4,498</div>  |
| verify_fibair | AccessAdapter<8> | StoreF | STOREW2 | 0 | <div style='text-align: right'>2,907</div>  |
| verify_fibair | Boundary | StoreF | STOREW2 | 0 | <div style='text-align: right'>6,800</div>  |
| verify_fibair | Merkle | StoreF | STOREW2 | 0 | <div style='text-align: right'>32,576</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | ADD | 0 | <div style='text-align: right'>30</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 0 | <div style='text-align: right'>310,650</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>559,691</div>  |
| verify_fibair | AccessAdapter<2> | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>89,463</div>  |
| verify_fibair | AccessAdapter<4> | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>69,849</div>  |
| verify_fibair | AccessAdapter<8> | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>69,683</div>  |
| verify_fibair | Boundary | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>163,960</div>  |
| verify_fibair | Merkle | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>582,720</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 0 | <div style='text-align: right'>26,650</div>  |
| verify_fibair | AccessAdapter<2> | StoreV | STOREW | 0 | <div style='text-align: right'>4,499</div>  |
| verify_fibair | AccessAdapter<4> | StoreV | STOREW | 0 | <div style='text-align: right'>5,031</div>  |
| verify_fibair | AccessAdapter<8> | StoreV | STOREW | 0 | <div style='text-align: right'>6,171</div>  |
| verify_fibair | Boundary | StoreV | STOREW | 0 | <div style='text-align: right'>14,520</div>  |
| verify_fibair | Merkle | StoreV | STOREW | 0 | <div style='text-align: right'>45,376</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW2 | 0 | <div style='text-align: right'>212,175</div>  |
| verify_fibair | AccessAdapter<2> | StoreV | STOREW2 | 0 | <div style='text-align: right'>23,089</div>  |
| verify_fibair | AccessAdapter<4> | StoreV | STOREW2 | 0 | <div style='text-align: right'>17,563</div>  |
| verify_fibair | AccessAdapter<8> | StoreV | STOREW2 | 0 | <div style='text-align: right'>17,323</div>  |
| verify_fibair | Boundary | StoreV | STOREW2 | 0 | <div style='text-align: right'>40,760</div>  |
| verify_fibair | Merkle | StoreV | STOREW2 | 0 | <div style='text-align: right'>54,016</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | 0 | <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> | SubE | FE4SUB | 0 | <div style='text-align: right'>18,546</div>  |
| verify_fibair | AccessAdapter<4> | SubE | FE4SUB | 0 | <div style='text-align: right'>10,959</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | LOADW | 0 | <div style='text-align: right'>16,113</div>  |
| verify_fibair | AccessAdapter<2> | SubEF | LOADW | 0 | <div style='text-align: right'>1,452</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | 0 | <div style='text-align: right'>3,930</div>  |
| verify_fibair | AccessAdapter<2> | SubEF | SUB | 0 | <div style='text-align: right'>1,452</div>  |
| verify_fibair | AccessAdapter<4> | SubEF | SUB | 0 | <div style='text-align: right'>1,716</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | 0 | <div style='text-align: right'>240</div>  |
| verify_fibair | AccessAdapter<2> | SubEI | ADD | 0 | <div style='text-align: right'>44</div>  |
| verify_fibair | AccessAdapter<4> | SubEI | ADD | 0 | <div style='text-align: right'>26</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | SUB | 0 | <div style='text-align: right'>39,990</div>  |
| verify_fibair | AccessAdapter<2> | SubFI | SUB | 0 | <div style='text-align: right'>14,630</div>  |
| verify_fibair | AccessAdapter<4> | SubFI | SUB | 0 | <div style='text-align: right'>16,796</div>  |
| verify_fibair | AccessAdapter<8> | SubFI | SUB | 0 | <div style='text-align: right'>21,964</div>  |
| verify_fibair | Boundary | SubFI | SUB | 0 | <div style='text-align: right'>51,680</div>  |
| verify_fibair | Merkle | SubFI | SUB | 0 | <div style='text-align: right'>82,688</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | 0 | <div style='text-align: right'>42,870</div>  |
| verify_fibair | AccessAdapter<2> | SubV | SUB | 0 | <div style='text-align: right'>22</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | 0 | <div style='text-align: right'>7,170</div>  |
| verify_fibair | AccessAdapter<2> | SubVI | SUB | 0 | <div style='text-align: right'>231</div>  |
| verify_fibair | AccessAdapter<4> | SubVI | SUB | 0 | <div style='text-align: right'>13</div>  |
| verify_fibair | AccessAdapter<8> | SubVI | SUB | 0 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary | SubVI | SUB | 0 | <div style='text-align: right'>40</div>  |
| verify_fibair | Merkle | SubVI | SUB | 0 | <div style='text-align: right'>64</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | 0 | <div style='text-align: right'>5,040</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | ADD | 0 | <div style='text-align: right'>30</div>  |

| group | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+16,270 [+0.1%])</span> <div style='text-align: right'>14,607,000</div>  | <span style="color: green">(-222 [-0.1%])</span> <div style='text-align: right'>397,072</div>  | <span style="color: green">(-39.0 [-2.5%])</span> <div style='text-align: right'>1,512.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>3,407,872</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <div style='text-align: right'>2,293,760</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,343,488</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | <div style='text-align: right'>851,968</div>  | <div style='text-align: right'>348</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>2,048</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>106,496</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>144</div>  |  | <div style='text-align: right'>512</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>311,296</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>8,650,752</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>2,326,528</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>11,141,120</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>300</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/57ac9d34f94574b07deb401bc24c4476c2d5d7b3/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/57ac9d34f94574b07deb401bc24c4476c2d5d7b3

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12550651853)
