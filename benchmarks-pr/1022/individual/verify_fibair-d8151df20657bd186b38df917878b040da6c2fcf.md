| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <span style="color: red">(+3,340 [+0.0%])</span> <div style='text-align: right'>48,128,287</div>  | <span style="color: red">(+360 [+0.1%])</span> <div style='text-align: right'>397,294</div>  | <span style="color: red">(+41.0 [+1.3%])</span> <div style='text-align: right'>3,119.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | interactions | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |
| ProgramAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |
| VmConnectorAir |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>4</div>  |  |
| PersistentBoundaryAir<8> |  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>4</div>  |  |
| MemoryMerkleAir<8> |  | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  |  | <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<2> |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<4> |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<8> |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>4</div>  |  |
| Poseidon2VmAir<BabyBearParameters> |  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  |  | <div style='text-align: right'>4</div>  |  |
| FriReducedOpeningAir |  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  | <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  | <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  | <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  | <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  | <div style='text-align: right'>4</div>  |  |
| PhantomAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>4</div>  |  |
| VariableRangeCheckerAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <div style='text-align: right'>8.0</div>  | <span style="color: red">(+133.0 [+8.1%])</span> <div style='text-align: right'>1,765.0</div>  | <span style="color: red">(+8.0 [+1.2%])</span> <div style='text-align: right'>650.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+4.0 [+1.1%])</span> <div style='text-align: right'>353.0</div>  | <span style="color: red">(+2.0 [+22.2%])</span> <div style='text-align: right'>11.0</div>  | <div style='text-align: right'>32</div>  | <span style="color: red">(+1.0 [+6.7%])</span> <div style='text-align: right'>16.0</div>  |

| segment | total_cycles | trace_gen_time_ms |
| --- | --- | --- |
| 0 | <span style="color: red">(+180 [+0.1%])</span> <div style='text-align: right'>198,647</div>  | <span style="color: red">(+124.0 [+12.5%])</span> <div style='text-align: right'>1,113.0</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: red">(+41.0 [+1.3%])</span> <div style='text-align: right'>3,119.0</div>  | <div style='text-align: right'>107,769,880</div>  | <span style="color: red">(+3,340 [+0.0%])</span> <div style='text-align: right'>48,128,287</div>  | <span style="color: red">(+360 [+0.1%])</span> <div style='text-align: right'>397,294</div>  | <span style="color: red">(+122.0 [+12.4%])</span> <div style='text-align: right'>1,108.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| verify_fibair | ProgramChip | 0 | <div style='text-align: right'>16,317</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| verify_fibair | Boundary | 0 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | Merkle | 0 | <div style='text-align: right'>43,900</div>  |
| verify_fibair | AccessAdapter<2> | 0 | <span style="color: red">(+88 [+0.2%])</span> <div style='text-align: right'>58,212</div>  |
| verify_fibair | AccessAdapter<4> | 0 | <span style="color: red">(+44 [+0.1%])</span> <div style='text-align: right'>35,806</div>  |
| verify_fibair | AccessAdapter<8> | 0 | <div style='text-align: right'>23,300</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>66,670</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>336</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>2,186</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>68,144</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: red">(+180 [+3.6%])</span> <div style='text-align: right'>5,169</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>30,558</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>85,891</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>5,216</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair |  | ADD | 0 | <div style='text-align: right'>54,984</div>  |
| verify_fibair |  | BBE4DIV | 0 | <div style='text-align: right'>297</div>  |
| verify_fibair |  | BBE4MUL | 0 | <div style='text-align: right'>891</div>  |
| verify_fibair |  | BEQ | 0 | <div style='text-align: right'>1,418</div>  |
| verify_fibair |  | BNE | 0 | <div style='text-align: right'>29,140</div>  |
| verify_fibair |  | COMP_POS2 | 0 | <div style='text-align: right'>1,092</div>  |
| verify_fibair |  | DIV | 0 | <div style='text-align: right'>3</div>  |
| verify_fibair |  | FE4ADD | 0 | <div style='text-align: right'>492</div>  |
| verify_fibair |  | FE4SUB | 0 | <div style='text-align: right'>506</div>  |
| verify_fibair |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>126</div>  |
| verify_fibair |  | JAL | 0 | <span style="color: red">(+180 [+3.6%])</span> <div style='text-align: right'>5,169</div>  |
| verify_fibair |  | LOADW | 0 | <div style='text-align: right'>18,438</div>  |
| verify_fibair |  | LOADW2 | 0 | <div style='text-align: right'>14,569</div>  |
| verify_fibair |  | MUL | 0 | <div style='text-align: right'>9,857</div>  |
| verify_fibair |  | PERM_POS2 | 0 | <div style='text-align: right'>265</div>  |
| verify_fibair |  | PHANTOM | 0 | <div style='text-align: right'>5,216</div>  |
| verify_fibair |  | SHINTW | 0 | <div style='text-align: right'>13,651</div>  |
| verify_fibair |  | STOREW | 0 | <div style='text-align: right'>30,347</div>  |
| verify_fibair |  | STOREW2 | 0 | <div style='text-align: right'>8,886</div>  |
| verify_fibair |  | SUB | 0 | <div style='text-align: right'>3,300</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <div style='text-align: right'>1,649,520</div>  |
| verify_fibair | AccessAdapter<2> |  | ADD | 0 | <span style="color: red">(+484 [+3.8%])</span> <div style='text-align: right'>13,101</div>  |
| verify_fibair | AccessAdapter<4> |  | ADD | 0 | <span style="color: red">(+286 [+3.8%])</span> <div style='text-align: right'>7,904</div>  |
| verify_fibair | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>731</div>  |
| verify_fibair | Boundary |  | ADD | 0 | <div style='text-align: right'>1,720</div>  |
| verify_fibair | Merkle |  | ADD | 0 | <div style='text-align: right'>2,752</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>11,880</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>2,904</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>1,716</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>35,640</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4MUL | 0 | <span style="color: red">(+484 [+3.3%])</span> <div style='text-align: right'>15,290</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4MUL | 0 | <span style="color: red">(+286 [+3.3%])</span> <div style='text-align: right'>9,035</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>32,614</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>670,220</div>  |
| verify_fibair | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>559</div>  |
| verify_fibair | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>18,564</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>610,428</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>10,450</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>6,175</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>18,546</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>10,959</div>  |
| verify_fibair | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>21,504</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: red">(+1,800 [+3.6%])</span> <div style='text-align: right'>51,690</div>  |
| verify_fibair | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>755,958</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>29,062</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>20,566</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>16,133</div>  |
| verify_fibair | Boundary |  | LOADW | 0 | <div style='text-align: right'>27,880</div>  |
| verify_fibair | Merkle |  | LOADW | 0 | <div style='text-align: right'>44,416</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>597,329</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>13,288</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>7,969</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>1,003</div>  |
| verify_fibair | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,880</div>  |
| verify_fibair | Merkle |  | LOADW2 | 0 | <div style='text-align: right'>2,880</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>295,710</div>  |
| verify_fibair | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>11,110</div>  |
| verify_fibair | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>10,647</div>  |
| verify_fibair | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>10,982</div>  |
| verify_fibair | Boundary |  | MUL | 0 | <div style='text-align: right'>25,840</div>  |
| verify_fibair | Merkle |  | MUL | 0 | <div style='text-align: right'>41,152</div>  |
| verify_fibair | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>8,806</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>148,135</div>  |
| verify_fibair | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>31,296</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>559,691</div>  |
| verify_fibair | AccessAdapter<2> |  | SHINTW | 0 | <div style='text-align: right'>89,463</div>  |
| verify_fibair | AccessAdapter<4> |  | SHINTW | 0 | <div style='text-align: right'>69,849</div>  |
| verify_fibair | AccessAdapter<8> |  | SHINTW | 0 | <div style='text-align: right'>69,683</div>  |
| verify_fibair | Boundary |  | SHINTW | 0 | <div style='text-align: right'>163,960</div>  |
| verify_fibair | Merkle |  | SHINTW | 0 | <div style='text-align: right'>582,720</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>1,244,227</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW | 0 | <div style='text-align: right'>108,614</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW | 0 | <div style='text-align: right'>70,226</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>55,845</div>  |
| verify_fibair | Boundary |  | STOREW | 0 | <div style='text-align: right'>131,400</div>  |
| verify_fibair | Merkle |  | STOREW | 0 | <div style='text-align: right'>558,720</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>364,326</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>38,236</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>26,481</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>21,692</div>  |
| verify_fibair | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>51,000</div>  |
| verify_fibair | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>89,344</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>99,000</div>  |
| verify_fibair | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>16,335</div>  |
| verify_fibair | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>18,525</div>  |
| verify_fibair | AccessAdapter<8> |  | SUB | 0 | <div style='text-align: right'>21,981</div>  |
| verify_fibair | Boundary |  | SUB | 0 | <div style='text-align: right'>51,720</div>  |
| verify_fibair | Merkle |  | SUB | 0 | <div style='text-align: right'>82,752</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: red">(+18.0 [+2.9%])</span> <div style='text-align: right'>642.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+3,340 [+0.0%])</span> <div style='text-align: right'>48,128,287</div>  | <span style="color: red">(+360 [+0.1%])</span> <div style='text-align: right'>397,294</div>  | <span style="color: red">(+41.0 [+1.3%])</span> <div style='text-align: right'>3,119.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>917,504</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <div style='text-align: right'>1,769,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,081,344</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>77,987,840</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>512</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d8151df20657bd186b38df917878b040da6c2fcf/verify_fibair-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/d8151df20657bd186b38df917878b040da6c2fcf

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12322504632)
