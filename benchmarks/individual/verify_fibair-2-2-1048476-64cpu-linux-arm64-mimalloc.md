| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <span style="color: red">(+3,532 [+0.0%])</span> <div style='text-align: right'>48,127,147</div>  | <span style="color: red">(+174 [+0.1%])</span> <div style='text-align: right'>198,582</div>  | <span style="color: green">(-38.0 [-1.3%])</span> <div style='text-align: right'>2,947.0</div>  |


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
| <div style='text-align: right'>8.0</div>  | <span style="color: green">(-2.0 [-0.1%])</span> <div style='text-align: right'>1,701.0</div>  | <span style="color: green">(-2.0 [-0.3%])</span> <div style='text-align: right'>608.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-5.0 [-1.4%])</span> <div style='text-align: right'>345.0</div>  | <span style="color: red">(+1.0 [+8.3%])</span> <div style='text-align: right'>13.0</div>  | <div style='text-align: right'>32</div>  | <span style="color: red">(+1.0 [+7.1%])</span> <div style='text-align: right'>15.0</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 | <span style="color: green">(-38.0 [-1.3%])</span> <div style='text-align: right'>2,947.0</div>  | <div style='text-align: right'>107,769,880</div>  | <span style="color: red">(+3,532 [+0.0%])</span> <div style='text-align: right'>48,127,147</div>  | <span style="color: red">(+174 [+0.1%])</span> <div style='text-align: right'>198,582</div>  | <span style="color: green">(-2.0 [-0.2%])</span> <div style='text-align: right'>1,083.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>16,317</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <div style='text-align: right'>22,770</div>  |
| app_proof | Merkle | 0 | <div style='text-align: right'>43,900</div>  |
| app_proof | AccessAdapter<2> | 0 | <span style="color: red">(+100 [+0.2%])</span> <div style='text-align: right'>58,184</div>  |
| app_proof | AccessAdapter<4> | 0 | <span style="color: red">(+50 [+0.1%])</span> <div style='text-align: right'>35,792</div>  |
| app_proof | AccessAdapter<8> | 0 | <div style='text-align: right'>23,300</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>66,670</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>336</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>2,186</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>68,144</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: red">(+172 [+3.5%])</span> <div style='text-align: right'>5,104</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>30,558</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>85,891</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>5,216</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>54,984</div>  |
| app_proof |  | BBE4DIV | 0 | <div style='text-align: right'>297</div>  |
| app_proof |  | BBE4MUL | 0 | <div style='text-align: right'>891</div>  |
| app_proof |  | BEQ | 0 | <div style='text-align: right'>1,418</div>  |
| app_proof |  | BNE | 0 | <div style='text-align: right'>29,140</div>  |
| app_proof |  | COMP_POS2 | 0 | <div style='text-align: right'>1,092</div>  |
| app_proof |  | DIV | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | FE4ADD | 0 | <div style='text-align: right'>492</div>  |
| app_proof |  | FE4SUB | 0 | <div style='text-align: right'>506</div>  |
| app_proof |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>126</div>  |
| app_proof |  | JAL | 0 | <span style="color: red">(+172 [+3.5%])</span> <div style='text-align: right'>5,104</div>  |
| app_proof |  | LOADW | 0 | <div style='text-align: right'>18,438</div>  |
| app_proof |  | LOADW2 | 0 | <div style='text-align: right'>14,569</div>  |
| app_proof |  | MUL | 0 | <div style='text-align: right'>9,857</div>  |
| app_proof |  | PERM_POS2 | 0 | <div style='text-align: right'>265</div>  |
| app_proof |  | PHANTOM | 0 | <div style='text-align: right'>5,216</div>  |
| app_proof |  | SHINTW | 0 | <div style='text-align: right'>13,651</div>  |
| app_proof |  | STOREW | 0 | <div style='text-align: right'>30,347</div>  |
| app_proof |  | STOREW2 | 0 | <div style='text-align: right'>8,886</div>  |
| app_proof |  | SUB | 0 | <div style='text-align: right'>3,300</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <span style="color: red">(+60 [+0.0%])</span> <div style='text-align: right'>1,649,520</div>  |
| app_proof | AccessAdapter<2> |  | ADD | 0 | <span style="color: red">(+462 [+3.7%])</span> <div style='text-align: right'>12,947</div>  |
| app_proof | AccessAdapter<4> |  | ADD | 0 | <span style="color: red">(+273 [+3.6%])</span> <div style='text-align: right'>7,813</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <span style="color: green">(-68 [-8.5%])</span> <div style='text-align: right'>731</div>  |
| app_proof | Boundary |  | ADD | 0 | <span style="color: green">(-160 [-8.5%])</span> <div style='text-align: right'>1,720</div>  |
| app_proof | Merkle |  | ADD | 0 | <span style="color: green">(-256 [-8.5%])</span> <div style='text-align: right'>2,752</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>11,880</div>  |
| app_proof | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>2,904</div>  |
| app_proof | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>1,716</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>35,640</div>  |
| app_proof | AccessAdapter<2> |  | BBE4MUL | 0 | <span style="color: red">(+550 [+3.8%])</span> <div style='text-align: right'>15,136</div>  |
| app_proof | AccessAdapter<4> |  | BBE4MUL | 0 | <span style="color: red">(+325 [+3.8%])</span> <div style='text-align: right'>8,944</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>32,614</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>670,220</div>  |
| app_proof | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>946</div>  |
| app_proof | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>559</div>  |
| app_proof | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>48,048</div>  |
| app_proof | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>28,392</div>  |
| app_proof | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>18,564</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>610,428</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>90</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>19,680</div>  |
| app_proof | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>10,450</div>  |
| app_proof | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>6,175</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>20,240</div>  |
| app_proof | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>18,546</div>  |
| app_proof | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>10,959</div>  |
| app_proof | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>2,024</div>  |
| app_proof | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>1,196</div>  |
| app_proof | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>21,504</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: red">(+1,720 [+3.5%])</span> <div style='text-align: right'>51,040</div>  |
| app_proof | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| app_proof | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>13</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>755,958</div>  |
| app_proof | AccessAdapter<2> |  | LOADW | 0 | <span style="color: red">(+110 [+0.4%])</span> <div style='text-align: right'>29,062</div>  |
| app_proof | AccessAdapter<4> |  | LOADW | 0 | <span style="color: red">(+91 [+0.4%])</span> <div style='text-align: right'>20,566</div>  |
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <span style="color: red">(+119 [+0.7%])</span> <div style='text-align: right'>16,133</div>  |
| app_proof | Boundary |  | LOADW | 0 | <span style="color: red">(+280 [+1.0%])</span> <div style='text-align: right'>27,880</div>  |
| app_proof | Merkle |  | LOADW | 0 | <span style="color: green">(-64 [-0.1%])</span> <div style='text-align: right'>44,416</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>597,329</div>  |
| app_proof | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>13,288</div>  |
| app_proof | AccessAdapter<4> |  | LOADW2 | 0 | <span style="color: green">(-13 [-0.2%])</span> <div style='text-align: right'>7,969</div>  |
| app_proof | AccessAdapter<8> |  | LOADW2 | 0 | <span style="color: green">(-17 [-1.7%])</span> <div style='text-align: right'>1,003</div>  |
| app_proof | Boundary |  | LOADW2 | 0 | <span style="color: green">(-40 [-2.1%])</span> <div style='text-align: right'>1,880</div>  |
| app_proof | Merkle |  | LOADW2 | 0 | <span style="color: red">(+64 [+2.3%])</span> <div style='text-align: right'>2,880</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>295,710</div>  |
| app_proof | AccessAdapter<2> |  | MUL | 0 | <span style="color: red">(+11 [+0.1%])</span> <div style='text-align: right'>11,110</div>  |
| app_proof | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>10,647</div>  |
| app_proof | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>10,982</div>  |
| app_proof | Boundary |  | MUL | 0 | <div style='text-align: right'>25,840</div>  |
| app_proof | Merkle |  | MUL | 0 | <div style='text-align: right'>41,152</div>  |
| app_proof | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>22,770</div>  |
| app_proof | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>13,455</div>  |
| app_proof | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>8,806</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>148,135</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>31,296</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>559,691</div>  |
| app_proof | AccessAdapter<2> |  | SHINTW | 0 | <div style='text-align: right'>89,463</div>  |
| app_proof | AccessAdapter<4> |  | SHINTW | 0 | <div style='text-align: right'>69,849</div>  |
| app_proof | AccessAdapter<8> |  | SHINTW | 0 | <div style='text-align: right'>69,683</div>  |
| app_proof | Boundary |  | SHINTW | 0 | <div style='text-align: right'>163,960</div>  |
| app_proof | Merkle |  | SHINTW | 0 | <div style='text-align: right'>582,720</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>1,244,227</div>  |
| app_proof | AccessAdapter<2> |  | STOREW | 0 | <span style="color: red">(+55 [+0.1%])</span> <div style='text-align: right'>108,614</div>  |
| app_proof | AccessAdapter<4> |  | STOREW | 0 | <span style="color: red">(+78 [+0.1%])</span> <div style='text-align: right'>70,226</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <span style="color: red">(+102 [+0.2%])</span> <div style='text-align: right'>55,845</div>  |
| app_proof | Boundary |  | STOREW | 0 | <span style="color: red">(+240 [+0.2%])</span> <div style='text-align: right'>131,400</div>  |
| app_proof | Merkle |  | STOREW | 0 | <span style="color: red">(+896 [+0.2%])</span> <div style='text-align: right'>558,720</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>364,326</div>  |
| app_proof | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>38,236</div>  |
| app_proof | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>26,481</div>  |
| app_proof | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>21,692</div>  |
| app_proof | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>51,000</div>  |
| app_proof | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>89,344</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>99,000</div>  |
| app_proof | AccessAdapter<2> |  | SUB | 0 | <span style="color: green">(-88 [-0.5%])</span> <div style='text-align: right'>16,335</div>  |
| app_proof | AccessAdapter<4> |  | SUB | 0 | <span style="color: green">(-104 [-0.6%])</span> <div style='text-align: right'>18,525</div>  |
| app_proof | AccessAdapter<8> |  | SUB | 0 | <span style="color: green">(-136 [-0.6%])</span> <div style='text-align: right'>21,981</div>  |
| app_proof | Boundary |  | SUB | 0 | <span style="color: green">(-320 [-0.6%])</span> <div style='text-align: right'>51,720</div>  |
| app_proof | Merkle |  | SUB | 0 | <span style="color: green">(-640 [-0.8%])</span> <div style='text-align: right'>82,752</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: red">(+6.0 [+1.0%])</span> <div style='text-align: right'>595.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+3,532 [+0.0%])</span> <div style='text-align: right'>48,127,147</div>  | <span style="color: red">(+174 [+0.1%])</span> <div style='text-align: right'>198,582</div>  | <span style="color: green">(-38.0 [-1.3%])</span> <div style='text-align: right'>2,947.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>917,504</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | AccessAdapterAir<2> | 0 | <div style='text-align: right'>1,769,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,081,344</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>77,987,840</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>512</div>  |
| app_proof | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,092.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7c863e90b469cdc54044af5bb83dfe15523ca920/verify_fibair-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/7c863e90b469cdc54044af5bb83dfe15523ca920

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12261907304)
