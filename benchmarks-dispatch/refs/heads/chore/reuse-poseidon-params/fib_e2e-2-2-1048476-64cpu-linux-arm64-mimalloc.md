| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>52,082,233</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>36,556.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>873,434,844</div>  | <div style='text-align: right'>21,316,709</div>  | <div style='text-align: right'>80,284.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>568,622,774</div>  | <div style='text-align: right'>14,408,108</div>  | <div style='text-align: right'>54,840.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>287,095,837</div>  | <div style='text-align: right'>7,278,493</div>  | <div style='text-align: right'>27,280.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,237,926</div>  | <div style='text-align: right'>3,636,448</div>  | <div style='text-align: right'>73,754.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_time_ms | fri.log_blowup | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>7,679.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>7</div>  |  |  | <div style='text-align: right'>52,082,233</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>36,556.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>873,434,844</div>  | <div style='text-align: right'>21,316,709</div>  | <div style='text-align: right'>80,284.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>568,622,774</div>  | <div style='text-align: right'>14,408,108</div>  | <div style='text-align: right'>54,840.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>287,095,837</div>  | <div style='text-align: right'>7,278,493</div>  | <div style='text-align: right'>27,280.0</div>  |
| root_verifier |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>73,754.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>144,237,926</div>  | <div style='text-align: right'>3,636,448</div>  | <div style='text-align: right'>73,754.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_continuation_program | ProgramChip | <div style='text-align: right'>5,874</div>  |
| fibonacci_continuation_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | Boundary | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | Merkle | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapter<8> | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>116,505</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>233,001</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>51</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>349,501</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>1,048,519</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>310</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |
| root_verifier | ProgramChip | <div style='text-align: right'>157,323</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>412,533</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>390,048</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>195,150</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,602</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,103</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,358</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,491,232</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>88,489</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,273</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,908</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>176,745</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_continuation_program |  | ADD | <div style='text-align: right'>1,048,502</div>  |
| fibonacci_continuation_program |  | AND | <div style='text-align: right'>5</div>  |
| fibonacci_continuation_program |  | AUIPC | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program |  | BEQ | <div style='text-align: right'>116,501</div>  |
| fibonacci_continuation_program |  | BGEU | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | BLT | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | BLTU | <div style='text-align: right'>7</div>  |
| fibonacci_continuation_program |  | BNE | <div style='text-align: right'>116,501</div>  |
| fibonacci_continuation_program |  | HINT_STOREW | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | JAL | <div style='text-align: right'>116,501</div>  |
| fibonacci_continuation_program |  | JALR | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program |  | LOADBU | <div style='text-align: right'>6</div>  |
| fibonacci_continuation_program |  | LOADW | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program |  | LUI | <div style='text-align: right'>10</div>  |
| fibonacci_continuation_program |  | OR | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | PHANTOM | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLL | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLTU | <div style='text-align: right'>349,501</div>  |
| fibonacci_continuation_program |  | SRL | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREB | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREW | <div style='text-align: right'>26</div>  |
| fibonacci_continuation_program |  | SUB | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | XOR | <div style='text-align: right'>4</div>  |
| root_verifier |  | ADD | <div style='text-align: right'>1,188,669</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,422</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,812</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>18,805</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>674,468</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>17,400</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,484</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,640</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>88,489</div>  |
| root_verifier |  | LOADW | <div style='text-align: right'>154,542</div>  |
| root_verifier |  | LOADW2 | <div style='text-align: right'>384,152</div>  |
| root_verifier |  | MUL | <div style='text-align: right'>211,055</div>  |
| root_verifier |  | PERM_POS2 | <div style='text-align: right'>8,703</div>  |
| root_verifier |  | PHANTOM | <div style='text-align: right'>176,745</div>  |
| root_verifier |  | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier |  | SHINTW | <div style='text-align: right'>232,680</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>172,725</div>  |
| root_verifier |  | STOREW2 | <div style='text-align: right'>171,809</div>  |
| root_verifier |  | SUB | <div style='text-align: right'>91,144</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>37,746,072</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | ADD | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | ADD | <div style='text-align: right'>3,712</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>231</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>3,029,026</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>96</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>224</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>3,029,026</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>2,097,018</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>448</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>720</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | LOADW | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | LOADW | <div style='text-align: right'>2,432</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>159</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>12,931,537</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | SLTU | <div style='text-align: right'>51</div>  |
| fibonacci_continuation_program | Boundary |  | SLTU | <div style='text-align: right'>120</div>  |
| fibonacci_continuation_program | Merkle |  | SLTU | <div style='text-align: right'>3,648</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>53</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>17</div>  |
| fibonacci_continuation_program | Boundary |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>1,040</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>255</div>  |
| fibonacci_continuation_program | Boundary |  | STOREW | <div style='text-align: right'>600</div>  |
| fibonacci_continuation_program | Merkle |  | STOREW | <div style='text-align: right'>1,984</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BEQ | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | BEQ | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | BEQ | <div style='text-align: right'>3,520</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BNE | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | BNE | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | BNE | <div style='text-align: right'>3,520</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>35,660,070</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>225,566</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>133,289</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>167,255</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>256,880</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,170</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,555</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>672,480</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>310,860</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>183,690</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>432,515</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>15,512,764</div>  |
| root_verifier | AccessAdapter<2> |  | BNE | <div style='text-align: right'>1,298</div>  |
| root_verifier | AccessAdapter<4> |  | BNE | <div style='text-align: right'>767</div>  |
| root_verifier | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>722,172</div>  |
| root_verifier | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>426,738</div>  |
| root_verifier | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>279,021</div>  |
| root_verifier | Boundary |  | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>9,726,600</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary |  | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>499,360</div>  |
| root_verifier | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>242,418</div>  |
| root_verifier | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>143,247</div>  |
| root_verifier | Boundary |  | FE4ADD | <div style='text-align: right'>106,172</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>145,600</div>  |
| root_verifier | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>132,154</div>  |
| root_verifier | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>78,091</div>  |
| root_verifier | Boundary |  | FE4SUB | <div style='text-align: right'>24,860</div>  |
| root_verifier | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>141,196</div>  |
| root_verifier | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>83,434</div>  |
| root_verifier | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>6,978,048</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>884,890</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,336,222</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>249,557</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>113,958</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,407</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,750,232</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>62,788</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>37,102</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,815</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,331,650</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>27,929</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>16,523</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>470,294</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>278,720</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,096</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,864,977</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,060,470</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,539,880</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,559,480</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>7,081,725</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>55,693</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>31,564</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>661,001</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>7,044,169</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>445,368</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>263,991</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>110,296</div>  |
| root_verifier | Boundary |  | STOREW2 | <div style='text-align: right'>789,393</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>2,734,320</div>  |
| root_verifier | AccessAdapter<2> |  | SUB | <div style='text-align: right'>58,619</div>  |
| root_verifier | AccessAdapter<4> |  | SUB | <div style='text-align: right'>69,277</div>  |
| root_verifier | Boundary |  | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>420</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 1 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 1 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 1 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 1 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 2 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 2 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 2 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 2 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 2 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 2 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 2 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 2 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 2 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 3 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 3 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 3 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 3 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 3 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 3 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 3 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 3 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 3 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 4 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 4 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 4 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 4 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 4 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 4 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 4 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 4 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 4 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 4 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 5 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 5 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 5 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 5 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 5 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 5 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 5 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 5 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 5 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 5 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 6 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 6 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 6 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 6 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 6 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 6 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 6 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 6 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | 0 | <div style='text-align: right'>279.0</div>  | <div style='text-align: right'>5,113.0</div>  | <div style='text-align: right'>197,760,980</div>  |
| fibonacci_continuation_program | 1 | <div style='text-align: right'>276.0</div>  | <div style='text-align: right'>4,845.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 2 | <div style='text-align: right'>287.0</div>  | <div style='text-align: right'>4,963.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 3 | <div style='text-align: right'>301.0</div>  | <div style='text-align: right'>4,953.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 4 | <div style='text-align: right'>295.0</div>  | <div style='text-align: right'>4,888.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 5 | <div style='text-align: right'>280.0</div>  | <div style='text-align: right'>4,875.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 6 | <div style='text-align: right'>266.0</div>  | <div style='text-align: right'>4,935.0</div>  | <div style='text-align: right'>197,587,986</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>7,004.0</div>  | <div style='text-align: right'>263,349,319</div>  | <div style='text-align: right'>6,442,829</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,476.0</div>  | <div style='text-align: right'>240,574,207</div>  | <div style='text-align: right'>5,870,365</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,505.0</div>  | <div style='text-align: right'>240,578,317</div>  | <div style='text-align: right'>5,870,748</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>3,728.0</div>  | <div style='text-align: right'>128,933,001</div>  | <div style='text-align: right'>3,132,767</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>760,650</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>725,234</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>363,038</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,738</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>53,015</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>62,362</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,508,488</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>137,909</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,233,307</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,070,099</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,457</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>725,112</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>667,036</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>333,856</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,544</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,304,084</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>131,113</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>725,112</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>667,052</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>333,864</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,544</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,304,084</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>131,496</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>399,934</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>366,244</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>183,332</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>54,678</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>26,056</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>117,852</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>29,813</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>1,221,522</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>66,552</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>597,318</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,009,931</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>177,213</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,129,402</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>10,994</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>21,204</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>36,789</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,196,518</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>33,978</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>23,310</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>137,909</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>293,889</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>637,358</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>271,590</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>19,037</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>368,457</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>462,760</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>354,648</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>321,444</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>107,282</div>  |
| leaf_verifier |  | 1 | ADD | <div style='text-align: right'>1,953,758</div>  |
| leaf_verifier |  | 1 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 1 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 1 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 1 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 1 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 1 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>131,113</div>  |
| leaf_verifier |  | 1 | LOADW | <div style='text-align: right'>281,391</div>  |
| leaf_verifier |  | 1 | LOADW2 | <div style='text-align: right'>552,980</div>  |
| leaf_verifier |  | 1 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 1 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 1 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 1 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>336,726</div>  |
| leaf_verifier |  | 1 | STOREW2 | <div style='text-align: right'>283,560</div>  |
| leaf_verifier |  | 1 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 2 | ADD | <div style='text-align: right'>1,953,758</div>  |
| leaf_verifier |  | 2 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 2 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 2 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 2 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 2 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 2 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>131,496</div>  |
| leaf_verifier |  | 2 | LOADW | <div style='text-align: right'>281,391</div>  |
| leaf_verifier |  | 2 | LOADW2 | <div style='text-align: right'>552,980</div>  |
| leaf_verifier |  | 2 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 2 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 2 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 2 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>336,726</div>  |
| leaf_verifier |  | 2 | STOREW2 | <div style='text-align: right'>283,560</div>  |
| leaf_verifier |  | 2 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 3 | ADD | <div style='text-align: right'>1,038,366</div>  |
| leaf_verifier |  | 3 | BBE4DIV | <div style='text-align: right'>5,240</div>  |
| leaf_verifier |  | 3 | BBE4MUL | <div style='text-align: right'>10,144</div>  |
| leaf_verifier |  | 3 | BEQ | <div style='text-align: right'>18,327</div>  |
| leaf_verifier |  | 3 | BNE | <div style='text-align: right'>578,991</div>  |
| leaf_verifier |  | 3 | COMP_POS2 | <div style='text-align: right'>16,979</div>  |
| leaf_verifier |  | 3 | DIV | <div style='text-align: right'>100</div>  |
| leaf_verifier |  | 3 | FE4ADD | <div style='text-align: right'>11,059</div>  |
| leaf_verifier |  | 3 | FE4SUB | <div style='text-align: right'>3,370</div>  |
| leaf_verifier |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>4,326</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>66,552</div>  |
| leaf_verifier |  | 3 | LOADW | <div style='text-align: right'>147,493</div>  |
| leaf_verifier |  | 3 | LOADW2 | <div style='text-align: right'>303,986</div>  |
| leaf_verifier |  | 3 | MUL | <div style='text-align: right'>131,855</div>  |
| leaf_verifier |  | 3 | PERM_POS2 | <div style='text-align: right'>9,077</div>  |
| leaf_verifier |  | 3 | PHANTOM | <div style='text-align: right'>177,213</div>  |
| leaf_verifier |  | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 3 | SHINTW | <div style='text-align: right'>228,476</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>174,472</div>  |
| leaf_verifier |  | 3 | STOREW2 | <div style='text-align: right'>155,504</div>  |
| leaf_verifier |  | 3 | SUB | <div style='text-align: right'>51,201</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>63,882,060</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>378,026</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>223,379</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,047</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>439,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>207,460</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>122,590</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>848,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>484,704</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>286,416</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>846,147</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>27,519,914</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>2,640</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>1,560</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>18,993,702</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>932,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>395,296</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>233,584</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>234,036</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>138,294</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>1,379,090</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>693</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>819</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>12,049,449</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>512,105</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>243,919</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>26,131,678</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>8,147,700</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>45,826</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>27,118</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,029,875</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>611,299</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>408,629</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>10,641,683</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>2,210,742</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>18,973,160</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,090,360</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>14,540,568</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>116,600</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>67,041</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>1,368,543</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>13,179,204</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>847,055</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>503,269</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>237,677</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>1,410,398</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>3,218,460</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>101,706</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>120,198</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | ADD | <div style='text-align: right'>58,612,740</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | ADD | <div style='text-align: right'>325,050</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | ADD | <div style='text-align: right'>192,075</div>  |
| leaf_verifier | Boundary |  | 1 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4MUL | <div style='text-align: right'>423,786</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4MUL | <div style='text-align: right'>250,419</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BNE | <div style='text-align: right'>24,818,403</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 1 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 1 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>1,311,130</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 1 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 1 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 1 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW | <div style='text-align: right'>97,559</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW | <div style='text-align: right'>55,926</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>1,342,759</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 1 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | ADD | <div style='text-align: right'>58,612,740</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | ADD | <div style='text-align: right'>325,138</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | ADD | <div style='text-align: right'>192,127</div>  |
| leaf_verifier | Boundary |  | 2 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4MUL | <div style='text-align: right'>423,874</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4MUL | <div style='text-align: right'>250,471</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BNE | <div style='text-align: right'>24,818,403</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 2 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 2 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>1,314,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 2 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 2 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 2 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 2 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW | <div style='text-align: right'>97,559</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW | <div style='text-align: right'>55,926</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>1,342,759</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 2 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | ADD | <div style='text-align: right'>31,150,980</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | ADD | <div style='text-align: right'>169,466</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | ADD | <div style='text-align: right'>100,139</div>  |
| leaf_verifier | Boundary |  | 3 | ADD | <div style='text-align: right'>137,555</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4DIV | <div style='text-align: right'>209,600</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4DIV | <div style='text-align: right'>98,098</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4DIV | <div style='text-align: right'>57,967</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4MUL | <div style='text-align: right'>405,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4MUL | <div style='text-align: right'>264,374</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4MUL | <div style='text-align: right'>156,221</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4MUL | <div style='text-align: right'>145,904</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BEQ | <div style='text-align: right'>421,521</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BNE | <div style='text-align: right'>13,316,793</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BNE | <div style='text-align: right'>1,298</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BNE | <div style='text-align: right'>767</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | COMP_POS2 | <div style='text-align: right'>696,300</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | COMP_POS2 | <div style='text-align: right'>411,450</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | COMP_POS2 | <div style='text-align: right'>269,025</div>  |
| leaf_verifier | Boundary |  | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | COMP_POS2 | <div style='text-align: right'>9,491,261</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | DIV | <div style='text-align: right'>3,000</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4ADD | <div style='text-align: right'>442,360</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4ADD | <div style='text-align: right'>215,798</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4ADD | <div style='text-align: right'>127,517</div>  |
| leaf_verifier | Boundary |  | 3 | FE4ADD | <div style='text-align: right'>119,768</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4SUB | <div style='text-align: right'>134,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4SUB | <div style='text-align: right'>122,012</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4SUB | <div style='text-align: right'>72,098</div>  |
| leaf_verifier | Boundary |  | 3 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>122,716</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>72,514</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,542,528</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>665,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | JAL | <div style='text-align: right'>330</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | JAL | <div style='text-align: right'>390</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW | <div style='text-align: right'>6,047,213</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW | <div style='text-align: right'>249,436</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW | <div style='text-align: right'>119,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW | <div style='text-align: right'>20,893</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW | <div style='text-align: right'>21,329</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW2 | <div style='text-align: right'>12,463,426</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW2 | <div style='text-align: right'>57,200</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW2 | <div style='text-align: right'>33,800</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW2 | <div style='text-align: right'>493</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | MUL | <div style='text-align: right'>3,955,650</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | MUL | <div style='text-align: right'>22,165</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | MUL | <div style='text-align: right'>13,117</div>  |
| leaf_verifier | Boundary |  | 3 | MUL | <div style='text-align: right'>32,208</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | PERM_POS2 | <div style='text-align: right'>495,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | PERM_POS2 | <div style='text-align: right'>293,891</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | PERM_POS2 | <div style='text-align: right'>195,738</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | PERM_POS2 | <div style='text-align: right'>5,074,043</div>  |
| leaf_verifier | PhantomAir |  | 3 | PHANTOM | <div style='text-align: right'>1,063,278</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 3 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | SHINTW | <div style='text-align: right'>9,367,516</div>  |
| leaf_verifier | Boundary |  | 3 | SHINTW | <div style='text-align: right'>2,513,236</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>7,153,352</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW | <div style='text-align: right'>57,893</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW | <div style='text-align: right'>33,384</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>688,721</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW2 | <div style='text-align: right'>6,375,664</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW2 | <div style='text-align: right'>401,786</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW2 | <div style='text-align: right'>238,784</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW2 | <div style='text-align: right'>112,404</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW2 | <div style='text-align: right'>697,752</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | SUB | <div style='text-align: right'>1,536,030</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | SUB | <div style='text-align: right'>48,059</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | SUB | <div style='text-align: right'>56,797</div>  |
| leaf_verifier | Boundary |  | 3 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>1,966,080</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>68,157,440</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,554.0</div>  | <div style='text-align: right'>21,318.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,475.0</div>  | <div style='text-align: right'>21,473.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,442.0</div>  | <div style='text-align: right'>21,545.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>737.0</div>  | <div style='text-align: right'>10,740.0</div>  | <div style='text-align: right'>311,462,360</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,572.0</div>  | <div style='text-align: right'>286,574,151</div>  | <div style='text-align: right'>7,263,122</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>7,712.0</div>  | <div style='text-align: right'>282,048,623</div>  | <div style='text-align: right'>7,144,986</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>7,930.0</div>  | <div style='text-align: right'>287,095,837</div>  | <div style='text-align: right'>7,278,493</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>771,879</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>756,512</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>378,592</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,896</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,094</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>78,518</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,972,624</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>182,181</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,385,009</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,659</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>757,668</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>745,268</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>372,970</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>106,542</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>50,959</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,811</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,917,674</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>175,352</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,361,242</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,199,466</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>351,846</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>773,363</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>758,012</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>379,258</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,182</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,716</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,982,242</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>184,210</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,495</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,527</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | ADD | <div style='text-align: right'>2,371,722</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>33,458</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BNE | <div style='text-align: right'>1,347,484</div>  |
| internal_verifier_height_0 |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4ADD | <div style='text-align: right'>24,936</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>182,181</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW | <div style='text-align: right'>308,457</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW2 | <div style='text-align: right'>768,020</div>  |
| internal_verifier_height_0 |  | 0 | 0 | MUL | <div style='text-align: right'>419,230</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,402</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>344,244</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW2 | <div style='text-align: right'>343,418</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SUB | <div style='text-align: right'>180,944</div>  |
| internal_verifier_height_0 |  | 0 | 1 | ADD | <div style='text-align: right'>2,328,663</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,802</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>33,006</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BEQ | <div style='text-align: right'>36,387</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BNE | <div style='text-align: right'>1,324,855</div>  |
| internal_verifier_height_0 |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>33,600</div>  |
| internal_verifier_height_0 |  | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4ADD | <div style='text-align: right'>24,849</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4SUB | <div style='text-align: right'>7,154</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>175,352</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW | <div style='text-align: right'>302,772</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW2 | <div style='text-align: right'>761,168</div>  |
| internal_verifier_height_0 |  | 0 | 1 | MUL | <div style='text-align: right'>410,493</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,359</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PHANTOM | <div style='text-align: right'>351,846</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SHINTW | <div style='text-align: right'>454,305</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>340,799</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW2 | <div style='text-align: right'>340,422</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SUB | <div style='text-align: right'>177,790</div>  |
| internal_verifier_height_1 |  | 1 | 2 | ADD | <div style='text-align: right'>2,377,140</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>33,624</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BEQ | <div style='text-align: right'>37,609</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BNE | <div style='text-align: right'>1,348,886</div>  |
| internal_verifier_height_1 |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,776</div>  |
| internal_verifier_height_1 |  | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4ADD | <div style='text-align: right'>24,968</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>184,210</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW | <div style='text-align: right'>308,805</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW2 | <div style='text-align: right'>768,304</div>  |
| internal_verifier_height_1 |  | 1 | 2 | MUL | <div style='text-align: right'>422,086</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,406</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PHANTOM | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SHINTW | <div style='text-align: right'>465,308</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>344,492</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW2 | <div style='text-align: right'>343,618</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SUB | <div style='text-align: right'>182,288</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | ADD | <div style='text-align: right'>71,151,660</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | ADD | <div style='text-align: right'>478,258</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | ADD | <div style='text-align: right'>282,607</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | ADD | <div style='text-align: right'>166,287</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,338,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>553,586</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>327,119</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BNE | <div style='text-align: right'>30,992,132</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>997,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>443,212</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>261,898</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>252,450</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>149,175</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>1,821,810</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW | <div style='text-align: right'>499,191</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW | <div style='text-align: right'>228,254</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | MUL | <div style='text-align: right'>12,576,900</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | MUL | <div style='text-align: right'>56,826</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | MUL | <div style='text-align: right'>33,618</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,124</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,727,718</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 0 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 0 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>14,114,004</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW | <div style='text-align: right'>115,588</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW | <div style='text-align: right'>65,312</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>1,280,653</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>889,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>220,524</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | SUB | <div style='text-align: right'>5,428,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | ADD | <div style='text-align: right'>69,859,890</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | ADD | <div style='text-align: right'>474,848</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | ADD | <div style='text-align: right'>280,592</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | ADD | <div style='text-align: right'>166,287</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>512,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,320,240</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>550,044</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>325,026</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BEQ | <div style='text-align: right'>836,901</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BNE | <div style='text-align: right'>30,471,665</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,389,696</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>821,184</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>536,928</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>18,782,400</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>993,960</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>441,320</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>260,780</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>286,160</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>247,830</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>146,445</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>1,753,520</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW | <div style='text-align: right'>12,413,652</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW | <div style='text-align: right'>495,451</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW | <div style='text-align: right'>226,018</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW | <div style='text-align: right'>39,270</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>31,207,888</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>122,804</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>72,566</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,037</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | MUL | <div style='text-align: right'>12,314,790</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | MUL | <div style='text-align: right'>55,902</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | MUL | <div style='text-align: right'>33,072</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>935,748</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>555,126</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>368,679</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,703,681</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 1 | PHANTOM | <div style='text-align: right'>2,111,076</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 1 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | SHINTW | <div style='text-align: right'>18,626,505</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SHINTW | <div style='text-align: right'>4,997,355</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>13,972,759</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW | <div style='text-align: right'>114,532</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW | <div style='text-align: right'>64,714</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>1,268,179</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>13,957,302</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>889,614</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>527,865</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>220,524</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW2 | <div style='text-align: right'>1,539,912</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | SUB | <div style='text-align: right'>5,333,700</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | ADD | <div style='text-align: right'>71,314,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | ADD | <div style='text-align: right'>479,556</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | ADD | <div style='text-align: right'>283,374</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | ADD | <div style='text-align: right'>167,343</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>251,658</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>148,707</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,344,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>557,370</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>329,355</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BEQ | <div style='text-align: right'>865,007</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BNE | <div style='text-align: right'>31,024,378</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,441,440</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>851,760</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>556,920</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,439,784</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>998,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>443,322</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>261,963</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4ADD | <div style='text-align: right'>106,172</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>252,296</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>149,084</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4SUB | <div style='text-align: right'>24,860</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>1,842,100</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | JAL | <div style='text-align: right'>682</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | JAL | <div style='text-align: right'>806</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW | <div style='text-align: right'>12,661,005</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW | <div style='text-align: right'>500,214</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW | <div style='text-align: right'>228,995</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW | <div style='text-align: right'>22,759</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>31,500,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | MUL | <div style='text-align: right'>12,662,580</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | MUL | <div style='text-align: right'>57,090</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | MUL | <div style='text-align: right'>33,774</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>940,588</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>557,440</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>370,192</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,729,954</div>  |
| internal_verifier_height_1 | PhantomAir |  | 1 | 2 | PHANTOM | <div style='text-align: right'>2,120,910</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | 2 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | SHINTW | <div style='text-align: right'>19,077,628</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SHINTW | <div style='text-align: right'>5,118,388</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>14,124,172</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW | <div style='text-align: right'>116,457</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW | <div style='text-align: right'>65,689</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>1,281,533</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>14,088,338</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>890,736</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>527,982</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>220,592</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW2 | <div style='text-align: right'>1,578,786</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | SUB | <div style='text-align: right'>5,468,640</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | height | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramAir | 0 | 0 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramAir | 0 | 1 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramAir | 1 | 2 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_1 | VolatileBoundaryAir | 1 | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<2> | 1 | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<4> | 1 | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | AccessAdapterAir<8> | 1 | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | height | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,734.0</div>  | <div style='text-align: right'>25,699.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,699.0</div>  | <div style='text-align: right'>25,708.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,686.0</div>  | <div style='text-align: right'>25,594.0</div>  | <div style='text-align: right'>760,809,944</div>  |

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

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc38e23792fe3d355d83ab2d76f8395459111007/fib_e2e-2-2-1048476-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/bc38e23792fe3d355d83ab2d76f8395459111007

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12246411064)
