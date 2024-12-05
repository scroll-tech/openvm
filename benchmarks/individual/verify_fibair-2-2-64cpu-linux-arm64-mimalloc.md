| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <span style="color: red">(+1,990 [+0.0%])</span> <div style='text-align: right'>48,128,225</div>  | <span style="color: red">(+80 [+0.0%])</span> <div style='text-align: right'>198,645</div>  | <span style="color: green">(-48.0 [-0.8%])</span> <div style='text-align: right'>5,657.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |

| stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- |
| <div style='text-align: right'>13.0</div>  | <div style='text-align: right'>32</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <span style="color: green">(-2.0 [-20.0%])</span> <div style='text-align: right'>8.0</div>  | <span style="color: green">(-15.0 [-0.9%])</span> <div style='text-align: right'>1,699.0</div>  | <span style="color: red">(+2.0 [+0.3%])</span> <div style='text-align: right'>582.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+2.0 [+3.2%])</span> <div style='text-align: right'>64.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+1,990 [+0.0%])</span> <div style='text-align: right'>48,128,225</div>  | <span style="color: red">(+80 [+0.0%])</span> <div style='text-align: right'>198,645</div>  | <span style="color: green">(-48.0 [-0.8%])</span> <div style='text-align: right'>5,657.0</div>  | <div style='text-align: right'>14.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | PersistentBoundaryAir<8> | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | MemoryMerkleAir<8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<2> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<4> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<8> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | ProgramChip | <div style='text-align: right'>16,315</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>2</div>  |
| verify_fibair | Boundary | <div style='text-align: right'>22,770</div>  |
| verify_fibair | Merkle | <div style='text-align: right'>43,900</div>  |
| verify_fibair | AccessAdapter<2> | <span style="color: red">(+68 [+0.1%])</span> <div style='text-align: right'>58,212</div>  |
| verify_fibair | AccessAdapter<4> | <span style="color: red">(+34 [+0.1%])</span> <div style='text-align: right'>35,806</div>  |
| verify_fibair | AccessAdapter<8> | <div style='text-align: right'>23,300</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>66,670</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>336</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>2,186</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>68,142</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | <span style="color: red">(+80 [+1.6%])</span> <div style='text-align: right'>5,169</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>30,558</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>85,891</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>5,216</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | ADD | <div style='text-align: right'>54,982</div>  |
| verify_fibair |  | BBE4DIV | <div style='text-align: right'>297</div>  |
| verify_fibair |  | BBE4MUL | <div style='text-align: right'>891</div>  |
| verify_fibair |  | BEQ | <div style='text-align: right'>1,418</div>  |
| verify_fibair |  | BNE | <div style='text-align: right'>29,140</div>  |
| verify_fibair |  | COMP_POS2 | <div style='text-align: right'>1,092</div>  |
| verify_fibair |  | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair |  | FE4ADD | <div style='text-align: right'>492</div>  |
| verify_fibair |  | FE4SUB | <div style='text-align: right'>506</div>  |
| verify_fibair |  | FRI_REDUCED_OPENING | <div style='text-align: right'>126</div>  |
| verify_fibair |  | JAL | <span style="color: red">(+80 [+1.6%])</span> <div style='text-align: right'>5,169</div>  |
| verify_fibair |  | LOADW | <div style='text-align: right'>18,438</div>  |
| verify_fibair |  | LOADW2 | <div style='text-align: right'>14,569</div>  |
| verify_fibair |  | MUL | <div style='text-align: right'>9,857</div>  |
| verify_fibair |  | PERM_POS2 | <div style='text-align: right'>265</div>  |
| verify_fibair |  | PHANTOM | <div style='text-align: right'>5,216</div>  |
| verify_fibair |  | SHINTW | <div style='text-align: right'>13,651</div>  |
| verify_fibair |  | STOREW | <div style='text-align: right'>30,347</div>  |
| verify_fibair |  | STOREW2 | <div style='text-align: right'>8,886</div>  |
| verify_fibair |  | SUB | <div style='text-align: right'>3,300</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>1,649,460</div>  |
| verify_fibair | AccessAdapter<2> |  | ADD | <span style="color: red">(+374 [+2.9%])</span> <div style='text-align: right'>13,189</div>  |
| verify_fibair | AccessAdapter<4> |  | ADD | <span style="color: red">(+221 [+2.9%])</span> <div style='text-align: right'>7,956</div>  |
| verify_fibair | AccessAdapter<8> |  | ADD | <div style='text-align: right'>799</div>  |
| verify_fibair | Boundary |  | ADD | <div style='text-align: right'>1,880</div>  |
| verify_fibair | Merkle |  | ADD | <div style='text-align: right'>3,008</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>11,880</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>2,904</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>1,716</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>35,640</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4MUL | <span style="color: red">(+374 [+2.5%])</span> <div style='text-align: right'>15,290</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4MUL | <span style="color: red">(+221 [+2.5%])</span> <div style='text-align: right'>9,035</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>32,614</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>670,220</div>  |
| verify_fibair | AccessAdapter<2> |  | BNE | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> |  | BNE | <div style='text-align: right'>559</div>  |
| verify_fibair | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>18,564</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>610,428</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>10,450</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>6,175</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>18,546</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>10,959</div>  |
| verify_fibair | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>21,504</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <span style="color: red">(+800 [+1.6%])</span> <div style='text-align: right'>51,690</div>  |
| verify_fibair | AccessAdapter<2> |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | AccessAdapter<4> |  | JAL | <div style='text-align: right'>13</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>755,958</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>28,952</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>20,475</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>16,014</div>  |
| verify_fibair | Boundary |  | LOADW | <div style='text-align: right'>27,600</div>  |
| verify_fibair | Merkle |  | LOADW | <div style='text-align: right'>44,480</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>597,329</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>13,288</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>7,982</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>1,020</div>  |
| verify_fibair | Boundary |  | LOADW2 | <div style='text-align: right'>1,920</div>  |
| verify_fibair | Merkle |  | LOADW2 | <div style='text-align: right'>2,816</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>295,710</div>  |
| verify_fibair | AccessAdapter<2> |  | MUL | <div style='text-align: right'>11,099</div>  |
| verify_fibair | AccessAdapter<4> |  | MUL | <div style='text-align: right'>10,647</div>  |
| verify_fibair | AccessAdapter<8> |  | MUL | <div style='text-align: right'>10,982</div>  |
| verify_fibair | Boundary |  | MUL | <div style='text-align: right'>25,840</div>  |
| verify_fibair | Merkle |  | MUL | <div style='text-align: right'>41,152</div>  |
| verify_fibair | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>8,806</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>148,135</div>  |
| verify_fibair | PhantomAir |  | PHANTOM | <div style='text-align: right'>31,296</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>559,691</div>  |
| verify_fibair | AccessAdapter<2> |  | SHINTW | <div style='text-align: right'>89,463</div>  |
| verify_fibair | AccessAdapter<4> |  | SHINTW | <div style='text-align: right'>69,849</div>  |
| verify_fibair | AccessAdapter<8> |  | SHINTW | <div style='text-align: right'>69,683</div>  |
| verify_fibair | Boundary |  | SHINTW | <div style='text-align: right'>163,960</div>  |
| verify_fibair | Merkle |  | SHINTW | <div style='text-align: right'>582,720</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>1,244,227</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>108,559</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>70,148</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>55,743</div>  |
| verify_fibair | Boundary |  | STOREW | <div style='text-align: right'>131,160</div>  |
| verify_fibair | Merkle |  | STOREW | <div style='text-align: right'>557,824</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>364,326</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>38,236</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>26,481</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>21,692</div>  |
| verify_fibair | Boundary |  | STOREW2 | <div style='text-align: right'>51,000</div>  |
| verify_fibair | Merkle |  | STOREW2 | <div style='text-align: right'>89,344</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>99,000</div>  |
| verify_fibair | AccessAdapter<2> |  | SUB | <div style='text-align: right'>16,423</div>  |
| verify_fibair | AccessAdapter<4> |  | SUB | <div style='text-align: right'>18,629</div>  |
| verify_fibair | AccessAdapter<8> |  | SUB | <div style='text-align: right'>22,117</div>  |
| verify_fibair | Boundary |  | SUB | <div style='text-align: right'>52,040</div>  |
| verify_fibair | Merkle |  | SUB | <div style='text-align: right'>83,392</div>  |

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
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,078.0</div>  | <span style="color: green">(-32.0 [-1.1%])</span> <div style='text-align: right'>2,880.0</div>  | <div style='text-align: right'>107,769,880</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/ecfafcbd887b24f981f7d85281806f8fea391547/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/ecfafcbd887b24f981f7d85281806f8fea391547

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12172688592)
