| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>1,268.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>138,166,007</div>  | <div style='text-align: right'>3,349,709</div>  | <div style='text-align: right'>13,100.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>139,704,348</div>  | <div style='text-align: right'>3,522,052</div>  | <div style='text-align: right'>73,331.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_time_ms | fri.log_blowup | halo2_proof_time_ms | halo2_total_cells | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>44.0</div>  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>1</div>  |  |  |  |  | <div style='text-align: right'>1,268.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  |  | <div style='text-align: right'>138,166,007</div>  | <div style='text-align: right'>3,349,709</div>  | <div style='text-align: right'>13,100.0</div>  |
| root_verifier |  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>73,331.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>139,704,348</div>  | <div style='text-align: right'>3,522,052</div>  | <div style='text-align: right'>73,331.0</div>  |
| halo2_verifier |  |  | <div style='text-align: right'>401,329.0</div>  | <div style='text-align: right'>318,500,970.0</div>  |  |  |  |  |  |  |
| halo2_wrapper |  |  | <div style='text-align: right'>79,750.0</div>  |  |  |  |  |  |  |  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>26,624</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>512</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>992</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>105</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>308</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>14,848</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | 0 | <div style='text-align: right'>1,268.0</div>  | <div style='text-align: right'>8,262,221</div>  | <div style='text-align: right'>18.0</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>3,539.0</div>  | <div style='text-align: right'>138,166,007</div>  | <div style='text-align: right'>3,349,709</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>410,659</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>394,940</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>197,680</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>56,444</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>26,981</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>144,732</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>34,378</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>1,267,305</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>68,352</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>641,727</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,096,766</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>208,830</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>1,094,574</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>6,226</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>11,379</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>17,419</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>624,308</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>16,097</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>128</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>13,342</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>3,431</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>5,334</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>68,352</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>150,348</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>348,262</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>123,448</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>10,884</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>208,830</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>236,501</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>184,395</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>177,260</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>49,155</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>32,837,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>202,466</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>119,639</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,135</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>249,040</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>121,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>71,526</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>455,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>301,950</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>178,425</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>400,637</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>14,359,084</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>819</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>651,948</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>385,242</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>251,889</div>  |
| leaf_verifier | Boundary |  | 0 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>8,998,223</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>3,840</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>533,680</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>244,662</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>144,573</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>137,240</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>71,422</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>151,580</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>89,570</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,262,848</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>683,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>418</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>494</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>6,164,268</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>281,798</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>132,145</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>20,179</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>14,278,742</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>54,406</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>32,149</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>476</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>3,703,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>22,957</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>13,585</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>580,118</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>344,162</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>227,885</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>6,084,156</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>1,252,980</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>9,696,541</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>2,601,511</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>7,560,195</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>68,849</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>39,793</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>701,239</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>7,267,660</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>509,058</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>302,172</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>141,712</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>716,562</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>1,474,650</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>59,235</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>70,005</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>13,100.0</div>  | <div style='text-align: right'>399,935,960</div>  | <div style='text-align: right'>859.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| root_verifier | ProgramChip | <div style='text-align: right'>157,323</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>398,080</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>378,504</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>189,378</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>52,328</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>25,008</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>38,627</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,435,340</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>88,090</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>668,844</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | <div style='text-align: right'>1,085,655</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>175,148</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| root_verifier |  | ADD | <div style='text-align: right'>1,144,500</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,380</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,352</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>17,625</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>651,219</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>16,266</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,381</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,514</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>88,090</div>  |
| root_verifier |  | LOADW | <div style='text-align: right'>148,683</div>  |
| root_verifier |  | LOADW2 | <div style='text-align: right'>377,158</div>  |
| root_verifier |  | MUL | <div style='text-align: right'>202,402</div>  |
| root_verifier |  | PERM_POS2 | <div style='text-align: right'>8,742</div>  |
| root_verifier |  | PHANTOM | <div style='text-align: right'>175,148</div>  |
| root_verifier |  | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier |  | SHINTW | <div style='text-align: right'>222,071</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>169,030</div>  |
| root_verifier |  | STOREW2 | <div style='text-align: right'>168,713</div>  |
| root_verifier |  | SUB | <div style='text-align: right'>88,074</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>34,335,000</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>223,256</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>131,924</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>166,199</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>255,200</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,324</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,646</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>654,080</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>307,142</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>181,493</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>405,375</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>14,978,037</div>  |
| root_verifier | AccessAdapter<2> |  | BNE | <div style='text-align: right'>1,298</div>  |
| root_verifier | AccessAdapter<4> |  | BNE | <div style='text-align: right'>767</div>  |
| root_verifier | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>672,276</div>  |
| root_verifier | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>397,254</div>  |
| root_verifier | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>259,743</div>  |
| root_verifier | Boundary |  | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>9,092,694</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary |  | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>495,240</div>  |
| root_verifier | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>240,702</div>  |
| root_verifier | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>142,233</div>  |
| root_verifier | Boundary |  | FE4ADD | <div style='text-align: right'>107,316</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>140,560</div>  |
| root_verifier | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>127,666</div>  |
| root_verifier | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>75,439</div>  |
| root_verifier | Boundary |  | FE4SUB | <div style='text-align: right'>25,080</div>  |
| root_verifier | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>141,196</div>  |
| root_verifier | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>83,434</div>  |
| root_verifier | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>6,978,048</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>880,900</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,096,003</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>245,377</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>111,449</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>19,482</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,143</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,463,478</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>59,994</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>35,451</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>510</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,815</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,072,060</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>26,851</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>15,886</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>466,466</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>276,458</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,045</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,886,778</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,050,888</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,104,911</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,442,781</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>6,930,230</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>54,417</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>30,849</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>647,647</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>6,917,233</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>445,258</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>263,926</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>111,690</div>  |
| root_verifier | Boundary |  | STOREW2 | <div style='text-align: right'>759,759</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>2,642,220</div>  |
| root_verifier | AccessAdapter<2> |  | SUB | <div style='text-align: right'>58,619</div>  |
| root_verifier | AccessAdapter<4> |  | SUB | <div style='text-align: right'>69,277</div>  |
| root_verifier | Boundary |  | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| root_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<2> | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | AccessAdapterAir<4> | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | AccessAdapterAir<8> | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/29cdcb1b04eb3e3705755b881762c5d02f600d55/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/29cdcb1b04eb3e3705755b881762c5d02f600d55

Max Segment Length: 1000000

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12251592646)
