| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>12,292,198</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>38,989.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>988,755,517</div>  | <div style='text-align: right'>24,145,123</div>  | <div style='text-align: right'>91,048.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>572,610,155</div>  | <div style='text-align: right'>14,505,962</div>  | <div style='text-align: right'>54,922.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,846,849</div>  | <div style='text-align: right'>7,270,261</div>  | <div style='text-align: right'>27,489.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,156,893</div>  | <div style='text-align: right'>3,636,894</div>  | <div style='text-align: right'>73,959.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_time_ms | fri.log_blowup | halo2_keygen_time_ms | halo2_proof_time_ms | halo2_total_cells | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>7,720.0</div>  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>12,292,198</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>38,989.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>988,755,517</div>  | <div style='text-align: right'>24,145,123</div>  | <div style='text-align: right'>91,048.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>572,610,155</div>  | <div style='text-align: right'>14,505,962</div>  | <div style='text-align: right'>54,922.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>286,846,849</div>  | <div style='text-align: right'>7,270,261</div>  | <div style='text-align: right'>27,489.0</div>  |
| root_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  | <div style='text-align: right'>73,959.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>144,156,893</div>  | <div style='text-align: right'>3,636,894</div>  | <div style='text-align: right'>73,959.0</div>  |
| static_verifier |  |  | <div style='text-align: right'>577,384.0</div>  | <div style='text-align: right'>393,652.0</div>  | <div style='text-align: right'>368,013,674.0</div>  |  |  |  |  |  |  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_continuation_program | ProgramChip | <div style='text-align: right'>6,547</div>  |
| fibonacci_continuation_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | Boundary | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | Merkle | <div style='text-align: right'>252</div>  |
| fibonacci_continuation_program | AccessAdapter<8> | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>111,118</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>222,227</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>51</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>333,341</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>1,000,039</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>306</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |
| root_verifier | ProgramChip | <div style='text-align: right'>154,102</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>412,026</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>388,104</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>194,178</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,598</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,101</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,042</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,489,888</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>90,784</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,237</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,757</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>176,745</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_continuation_program |  | ADD | <div style='text-align: right'>1,000,022</div>  |
| fibonacci_continuation_program |  | AND | <div style='text-align: right'>5</div>  |
| fibonacci_continuation_program |  | AUIPC | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program |  | BEQ | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | BGEU | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | BLT | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | BLTU | <div style='text-align: right'>7</div>  |
| fibonacci_continuation_program |  | BNE | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | HINT_STOREW | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | JAL | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | JALR | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program |  | LOADBU | <div style='text-align: right'>6</div>  |
| fibonacci_continuation_program |  | LOADW | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program |  | LUI | <div style='text-align: right'>10</div>  |
| fibonacci_continuation_program |  | OR | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | PHANTOM | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLL | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLTU | <div style='text-align: right'>333,341</div>  |
| fibonacci_continuation_program |  | SRL | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREB | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREW | <div style='text-align: right'>26</div>  |
| fibonacci_continuation_program |  | SUB | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | XOR | <div style='text-align: right'>4</div>  |
| root_verifier |  | ADD | <div style='text-align: right'>1,187,325</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,422</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,812</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>18,805</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>674,432</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>17,400</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,168</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,640</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>90,784</div>  |
| root_verifier |  | LOADW | <div style='text-align: right'>154,524</div>  |
| root_verifier |  | LOADW2 | <div style='text-align: right'>384,056</div>  |
| root_verifier |  | MUL | <div style='text-align: right'>211,055</div>  |
| root_verifier |  | PERM_POS2 | <div style='text-align: right'>8,701</div>  |
| root_verifier |  | PHANTOM | <div style='text-align: right'>176,745</div>  |
| root_verifier |  | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier |  | SHINTW | <div style='text-align: right'>232,680</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>172,704</div>  |
| root_verifier |  | STOREW2 | <div style='text-align: right'>171,793</div>  |
| root_verifier |  | SUB | <div style='text-align: right'>91,144</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>36,000,792</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | ADD | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | ADD | <div style='text-align: right'>3,712</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>231</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>2,888,964</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>96</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>224</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>2,888,964</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>2,000,052</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>448</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>720</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | LOADW | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | LOADW | <div style='text-align: right'>2,304</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>159</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>12,333,617</div>  |
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
| fibonacci_continuation_program | Merkle |  | STOREW | <div style='text-align: right'>2,048</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BEQ | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | BEQ | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | BEQ | <div style='text-align: right'>192</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BNE | <div style='text-align: right'>17</div>  |
| fibonacci_continuation_program | Boundary |  | BNE | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | Merkle |  | BNE | <div style='text-align: right'>64</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>35,619,750</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>215,864</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>127,556</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>163,559</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>256,880</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,192</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,568</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>672,480</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>310,156</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>183,274</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>155,144</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>432,515</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>15,511,936</div>  |
| root_verifier | AccessAdapter<2> |  | BNE | <div style='text-align: right'>1,298</div>  |
| root_verifier | AccessAdapter<4> |  | BNE | <div style='text-align: right'>767</div>  |
| root_verifier | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>722,172</div>  |
| root_verifier | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>426,738</div>  |
| root_verifier | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>279,021</div>  |
| root_verifier | Boundary |  | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>9,726,600</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary |  | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>486,720</div>  |
| root_verifier | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>232,650</div>  |
| root_verifier | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>137,475</div>  |
| root_verifier | Boundary |  | FE4ADD | <div style='text-align: right'>103,576</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>145,600</div>  |
| root_verifier | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>132,000</div>  |
| root_verifier | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>78,000</div>  |
| root_verifier | Boundary |  | FE4SUB | <div style='text-align: right'>24,904</div>  |
| root_verifier | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>141,196</div>  |
| root_verifier | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>83,434</div>  |
| root_verifier | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>6,978,048</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>907,840</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,335,484</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>249,711</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>114,010</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,209</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,746,296</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>62,788</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>37,102</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,804</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,331,650</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>28,171</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>16,666</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>470,206</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>278,668</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,062</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,863,859</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,060,470</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,539,880</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,559,480</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>7,080,864</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>55,693</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>31,603</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>661,045</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>7,043,513</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>445,280</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>263,939</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>110,262</div>  |
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
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 6 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 6 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 6 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 6 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | ProgramAir | 7 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 7 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 7 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 7 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 7 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 7 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 7 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 7 | <div style='text-align: right'>2,031,616</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>32,768</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 7 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 7 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 7 | <div style='text-align: right'>10,092,544</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 7 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 7 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | PhantomAir | 7 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 7 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 7 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | 0 | <div style='text-align: right'>277.0</div>  | <div style='text-align: right'>5,078.0</div>  | <div style='text-align: right'>197,760,980</div>  |
| fibonacci_continuation_program | 1 | <div style='text-align: right'>252.0</div>  | <div style='text-align: right'>4,789.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 2 | <div style='text-align: right'>283.0</div>  | <div style='text-align: right'>4,958.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 3 | <div style='text-align: right'>295.0</div>  | <div style='text-align: right'>5,020.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 4 | <div style='text-align: right'>280.0</div>  | <div style='text-align: right'>5,022.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 5 | <div style='text-align: right'>273.0</div>  | <div style='text-align: right'>4,979.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 6 | <div style='text-align: right'>265.0</div>  | <div style='text-align: right'>5,020.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 7 | <div style='text-align: right'>81.0</div>  | <div style='text-align: right'>2,117.0</div>  | <div style='text-align: right'>55,440,402</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>6,891.0</div>  | <div style='text-align: right'>263,192,562</div>  | <div style='text-align: right'>6,439,931</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,466.0</div>  | <div style='text-align: right'>240,444,569</div>  | <div style='text-align: right'>5,867,007</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,519.0</div>  | <div style='text-align: right'>240,413,189</div>  | <div style='text-align: right'>5,864,226</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>6,608.0</div>  | <div style='text-align: right'>244,705,197</div>  | <div style='text-align: right'>5,973,959</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>106,825</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>760,143</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>722,224</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>361,532</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,730</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>53,011</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>61,900</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,506,470</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>137,890</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,233,230</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,069,781</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,457</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>106,825</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>724,605</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>664,780</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>332,726</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,844</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>50,068</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,230</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,302,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>129,803</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,115,630</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,889,721</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>106,825</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>724,605</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>664,576</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>332,624</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,844</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>50,068</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,230</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,302,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>127,022</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,115,630</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,889,721</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>106,825</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>724,494</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>676,100</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>338,470</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>104,910</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>50,037</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>220,248</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>56,257</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>2,329,842</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>129,521</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>1,136,960</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,928,827</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>334,331</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,127,384</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>10,994</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>21,204</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>36,789</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,196,441</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>33,978</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>22,848</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>137,890</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>293,853</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>637,154</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>271,590</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>19,033</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>368,457</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>462,760</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>354,604</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>321,410</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>107,282</div>  |
| leaf_verifier |  | 1 | ADD | <div style='text-align: right'>1,952,362</div>  |
| leaf_verifier |  | 1 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 1 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 1 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 1 | BNE | <div style='text-align: right'>1,078,997</div>  |
| leaf_verifier |  | 1 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 1 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier |  | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>129,803</div>  |
| leaf_verifier |  | 1 | LOADW | <div style='text-align: right'>281,355</div>  |
| leaf_verifier |  | 1 | LOADW2 | <div style='text-align: right'>552,812</div>  |
| leaf_verifier |  | 1 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 1 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier |  | 1 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 1 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>336,688</div>  |
| leaf_verifier |  | 1 | STOREW2 | <div style='text-align: right'>283,532</div>  |
| leaf_verifier |  | 1 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 2 | ADD | <div style='text-align: right'>1,952,362</div>  |
| leaf_verifier |  | 2 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 2 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 2 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 2 | BNE | <div style='text-align: right'>1,078,997</div>  |
| leaf_verifier |  | 2 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 2 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier |  | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>127,022</div>  |
| leaf_verifier |  | 2 | LOADW | <div style='text-align: right'>281,355</div>  |
| leaf_verifier |  | 2 | LOADW2 | <div style='text-align: right'>552,812</div>  |
| leaf_verifier |  | 2 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 2 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier |  | 2 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 2 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>336,688</div>  |
| leaf_verifier |  | 2 | STOREW2 | <div style='text-align: right'>283,532</div>  |
| leaf_verifier |  | 2 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 3 | ADD | <div style='text-align: right'>1,978,460</div>  |
| leaf_verifier |  | 3 | BBE4DIV | <div style='text-align: right'>9,924</div>  |
| leaf_verifier |  | 3 | BBE4MUL | <div style='text-align: right'>19,300</div>  |
| leaf_verifier |  | 3 | BEQ | <div style='text-align: right'>35,589</div>  |
| leaf_verifier |  | 3 | BNE | <div style='text-align: right'>1,101,371</div>  |
| leaf_verifier |  | 3 | COMP_POS2 | <div style='text-align: right'>32,897</div>  |
| leaf_verifier |  | 3 | DIV | <div style='text-align: right'>186</div>  |
| leaf_verifier |  | 3 | FE4ADD | <div style='text-align: right'>20,492</div>  |
| leaf_verifier |  | 3 | FE4SUB | <div style='text-align: right'>6,541</div>  |
| leaf_verifier |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>8,148</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>129,521</div>  |
| leaf_verifier |  | 3 | LOADW | <div style='text-align: right'>282,716</div>  |
| leaf_verifier |  | 3 | LOADW2 | <div style='text-align: right'>575,712</div>  |
| leaf_verifier |  | 3 | MUL | <div style='text-align: right'>253,216</div>  |
| leaf_verifier |  | 3 | PERM_POS2 | <div style='text-align: right'>17,140</div>  |
| leaf_verifier |  | 3 | PHANTOM | <div style='text-align: right'>334,331</div>  |
| leaf_verifier |  | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 3 | SHINTW | <div style='text-align: right'>436,683</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>339,290</div>  |
| leaf_verifier |  | 3 | STOREW2 | <div style='text-align: right'>294,426</div>  |
| leaf_verifier |  | 3 | SUB | <div style='text-align: right'>97,980</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>63,821,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>362,912</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>214,448</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>141,284</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>439,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>207,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>122,577</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>848,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>481,426</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>284,479</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>141,240</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>846,147</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>27,518,143</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>2,640</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>1,560</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>18,993,702</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>913,920</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>382,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>225,758</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>111,804</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>234,212</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>138,398</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,224</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>1,378,900</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>693</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>819</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>12,047,973</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>512,457</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>244,114</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,670</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>26,123,314</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,243</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>8,147,700</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>46,068</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>27,261</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,736</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,029,688</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>611,182</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>408,561</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>10,639,447</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>2,210,742</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>18,973,160</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,090,360</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>14,538,764</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>116,468</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>66,989</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>1,368,631</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>13,177,810</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>846,868</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>503,152</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>237,609</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>1,410,398</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>3,218,460</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>101,684</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>120,172</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | ADD | <div style='text-align: right'>58,570,860</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | ADD | <div style='text-align: right'>314,248</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | ADD | <div style='text-align: right'>185,692</div>  |
| leaf_verifier | Boundary |  | 1 | ADD | <div style='text-align: right'>133,320</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4MUL | <div style='text-align: right'>421,080</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4MUL | <div style='text-align: right'>248,820</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BNE | <div style='text-align: right'>24,816,931</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4ADD | <div style='text-align: right'>777,920</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4ADD | <div style='text-align: right'>337,502</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4ADD | <div style='text-align: right'>199,433</div>  |
| leaf_verifier | Boundary |  | 1 | FE4ADD | <div style='text-align: right'>117,832</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4SUB | <div style='text-align: right'>229,548</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4SUB | <div style='text-align: right'>135,642</div>  |
| leaf_verifier | Boundary |  | 1 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>1,298,030</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | LOADW | <div style='text-align: right'>11,535,555</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW | <div style='text-align: right'>456,412</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW | <div style='text-align: right'>220,844</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW | <div style='text-align: right'>21,384</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | LOADW2 | <div style='text-align: right'>22,665,292</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW2 | <div style='text-align: right'>1,243</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | MUL | <div style='text-align: right'>43,032</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | MUL | <div style='text-align: right'>25,467</div>  |
| leaf_verifier | Boundary |  | 1 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | PhantomAir |  | 1 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 1 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>13,804,208</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW | <div style='text-align: right'>97,295</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW | <div style='text-align: right'>55,796</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>1,342,847</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW2 | <div style='text-align: right'>11,624,812</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW2 | <div style='text-align: right'>685,960</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW2 | <div style='text-align: right'>407,524</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | SUB | <div style='text-align: right'>84,920</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | SUB | <div style='text-align: right'>100,360</div>  |
| leaf_verifier | Boundary |  | 1 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | ADD | <div style='text-align: right'>58,570,860</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | ADD | <div style='text-align: right'>313,126</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | ADD | <div style='text-align: right'>185,029</div>  |
| leaf_verifier | Boundary |  | 2 | ADD | <div style='text-align: right'>133,320</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4MUL | <div style='text-align: right'>419,958</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4MUL | <div style='text-align: right'>248,157</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BNE | <div style='text-align: right'>24,816,931</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4ADD | <div style='text-align: right'>777,920</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4ADD | <div style='text-align: right'>337,502</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4ADD | <div style='text-align: right'>199,433</div>  |
| leaf_verifier | Boundary |  | 2 | FE4ADD | <div style='text-align: right'>117,832</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4SUB | <div style='text-align: right'>229,548</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4SUB | <div style='text-align: right'>135,642</div>  |
| leaf_verifier | Boundary |  | 2 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>1,270,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | LOADW | <div style='text-align: right'>11,535,555</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW | <div style='text-align: right'>456,412</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW | <div style='text-align: right'>220,844</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW | <div style='text-align: right'>21,384</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | LOADW2 | <div style='text-align: right'>22,665,292</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW2 | <div style='text-align: right'>1,243</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | MUL | <div style='text-align: right'>43,032</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | MUL | <div style='text-align: right'>25,467</div>  |
| leaf_verifier | Boundary |  | 2 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | PhantomAir |  | 2 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 2 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 2 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>13,804,208</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW | <div style='text-align: right'>97,295</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW | <div style='text-align: right'>55,796</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>1,342,847</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW2 | <div style='text-align: right'>11,624,812</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW2 | <div style='text-align: right'>685,960</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW2 | <div style='text-align: right'>407,524</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | SUB | <div style='text-align: right'>84,920</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | SUB | <div style='text-align: right'>100,360</div>  |
| leaf_verifier | Boundary |  | 2 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | ADD | <div style='text-align: right'>59,353,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | ADD | <div style='text-align: right'>330,242</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | ADD | <div style='text-align: right'>195,143</div>  |
| leaf_verifier | Boundary |  | 3 | ADD | <div style='text-align: right'>133,408</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4DIV | <div style='text-align: right'>396,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4DIV | <div style='text-align: right'>184,536</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4DIV | <div style='text-align: right'>109,044</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4MUL | <div style='text-align: right'>772,000</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4MUL | <div style='text-align: right'>441,386</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4MUL | <div style='text-align: right'>260,819</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BEQ | <div style='text-align: right'>818,547</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BNE | <div style='text-align: right'>25,331,533</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BNE | <div style='text-align: right'>2,552</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BNE | <div style='text-align: right'>1,508</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | COMP_POS2 | <div style='text-align: right'>1,341,252</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | COMP_POS2 | <div style='text-align: right'>792,558</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | COMP_POS2 | <div style='text-align: right'>518,211</div>  |
| leaf_verifier | Boundary |  | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | COMP_POS2 | <div style='text-align: right'>18,389,423</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | DIV | <div style='text-align: right'>5,580</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4ADD | <div style='text-align: right'>819,680</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4ADD | <div style='text-align: right'>352,000</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4ADD | <div style='text-align: right'>208,000</div>  |
| leaf_verifier | Boundary |  | 3 | FE4ADD | <div style='text-align: right'>117,832</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4SUB | <div style='text-align: right'>261,640</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4SUB | <div style='text-align: right'>225,808</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4SUB | <div style='text-align: right'>133,432</div>  |
| leaf_verifier | Boundary |  | 3 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>229,944</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>135,876</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,095,872</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>1,295,210</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | JAL | <div style='text-align: right'>605</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | JAL | <div style='text-align: right'>715</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | LOADW | <div style='text-align: right'>11,591,356</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW | <div style='text-align: right'>474,925</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW | <div style='text-align: right'>228,475</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW | <div style='text-align: right'>40,171</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW | <div style='text-align: right'>21,384</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | LOADW2 | <div style='text-align: right'>23,604,192</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW2 | <div style='text-align: right'>111,628</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW2 | <div style='text-align: right'>65,962</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW2 | <div style='text-align: right'>969</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW2 | <div style='text-align: right'>1,243</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | MUL | <div style='text-align: right'>7,596,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | MUL | <div style='text-align: right'>43,329</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | MUL | <div style='text-align: right'>25,649</div>  |
| leaf_verifier | Boundary |  | 3 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | PERM_POS2 | <div style='text-align: right'>943,624</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | PERM_POS2 | <div style='text-align: right'>560,326</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | PERM_POS2 | <div style='text-align: right'>373,524</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | PERM_POS2 | <div style='text-align: right'>9,581,260</div>  |
| leaf_verifier | PhantomAir |  | 3 | PHANTOM | <div style='text-align: right'>2,005,986</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 3 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | SHINTW | <div style='text-align: right'>17,904,003</div>  |
| leaf_verifier | Boundary |  | 3 | SHINTW | <div style='text-align: right'>4,803,513</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>13,910,890</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW | <div style='text-align: right'>107,910</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW | <div style='text-align: right'>62,049</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>1,331,143</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW2 | <div style='text-align: right'>12,071,466</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW2 | <div style='text-align: right'>746,042</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW2 | <div style='text-align: right'>443,573</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW2 | <div style='text-align: right'>208,301</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW2 | <div style='text-align: right'>1,340,680</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | SUB | <div style='text-align: right'>2,939,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | SUB | <div style='text-align: right'>90,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | SUB | <div style='text-align: right'>106,964</div>  |
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
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
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
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
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
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,499.0</div>  | <div style='text-align: right'>21,556.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,448.0</div>  | <div style='text-align: right'>21,323.0</div>  | <div style='text-align: right'>615,320,024</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,436.0</div>  | <div style='text-align: right'>21,238.0</div>  | <div style='text-align: right'>615,320,024</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>1,450.0</div>  | <div style='text-align: right'>21,098.0</div>  | <div style='text-align: right'>615,320,024</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,599.0</div>  | <div style='text-align: right'>286,313,756</div>  | <div style='text-align: right'>7,253,720</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>7,743.0</div>  | <div style='text-align: right'>286,296,399</div>  | <div style='text-align: right'>7,252,242</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>7,907.0</div>  | <div style='text-align: right'>286,846,849</div>  | <div style='text-align: right'>7,270,261</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>153,829</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>771,372</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>752,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>376,634</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,972</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,132</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>77,861</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,968,647</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>177,818</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,384,910</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,315</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>153,829</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>771,372</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>752,660</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>376,666</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>109,056</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>52,174</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,836</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,967,358</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>177,681</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,384,883</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,228,273</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>153,829</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>772,856</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>753,868</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>377,186</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,064</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,178</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,084</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,979,554</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>179,676</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,423</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,225</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | ADD | <div style='text-align: right'>2,368,501</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>33,433</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BNE | <div style='text-align: right'>1,347,385</div>  |
| internal_verifier_height_0 |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4ADD | <div style='text-align: right'>24,304</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>177,818</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW | <div style='text-align: right'>308,421</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW2 | <div style='text-align: right'>767,828</div>  |
| internal_verifier_height_0 |  | 0 | 0 | MUL | <div style='text-align: right'>418,726</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,440</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>344,160</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW2 | <div style='text-align: right'>343,386</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SUB | <div style='text-align: right'>180,692</div>  |
| internal_verifier_height_0 |  | 0 | 1 | ADD | <div style='text-align: right'>2,367,968</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>33,408</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BNE | <div style='text-align: right'>1,347,358</div>  |
| internal_verifier_height_0 |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4ADD | <div style='text-align: right'>24,304</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>177,681</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW | <div style='text-align: right'>308,421</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW2 | <div style='text-align: right'>767,828</div>  |
| internal_verifier_height_0 |  | 0 | 1 | MUL | <div style='text-align: right'>418,222</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,482</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>344,118</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW2 | <div style='text-align: right'>343,386</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SUB | <div style='text-align: right'>180,440</div>  |
| internal_verifier_height_1 |  | 1 | 2 | ADD | <div style='text-align: right'>2,374,452</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>33,624</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BEQ | <div style='text-align: right'>37,609</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BNE | <div style='text-align: right'>1,348,814</div>  |
| internal_verifier_height_1 |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,776</div>  |
| internal_verifier_height_1 |  | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4ADD | <div style='text-align: right'>24,336</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>179,676</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW | <div style='text-align: right'>308,769</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW2 | <div style='text-align: right'>768,112</div>  |
| internal_verifier_height_1 |  | 1 | 2 | MUL | <div style='text-align: right'>422,086</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,402</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PHANTOM | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SHINTW | <div style='text-align: right'>465,308</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>344,450</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW2 | <div style='text-align: right'>343,586</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SUB | <div style='text-align: right'>182,288</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | ADD | <div style='text-align: right'>71,055,030</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | ADD | <div style='text-align: right'>456,302</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | ADD | <div style='text-align: right'>269,633</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | ADD | <div style='text-align: right'>162,811</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>252,010</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>148,915</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,337,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>550,594</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>325,351</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>155,936</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BNE | <div style='text-align: right'>30,989,855</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>972,160</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>425,326</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>251,329</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4ADD | <div style='text-align: right'>104,500</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>251,900</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>148,850</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>1,778,180</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW | <div style='text-align: right'>12,645,261</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW | <div style='text-align: right'>500,005</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW | <div style='text-align: right'>228,709</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW | <div style='text-align: right'>22,209</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>31,480,948</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,749</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | MUL | <div style='text-align: right'>12,561,780</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | MUL | <div style='text-align: right'>57,376</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | MUL | <div style='text-align: right'>33,943</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>939,312</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>557,232</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,770</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,748,960</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 0 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 0 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>14,110,560</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW | <div style='text-align: right'>116,116</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW | <div style='text-align: right'>65,650</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>1,280,609</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>14,078,826</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>889,460</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>527,774</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>221,170</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | SUB | <div style='text-align: right'>5,420,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | ADD | <div style='text-align: right'>71,039,040</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | ADD | <div style='text-align: right'>456,654</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | ADD | <div style='text-align: right'>269,841</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | ADD | <div style='text-align: right'>162,811</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>252,010</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>148,915</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,336,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>550,946</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>325,559</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>155,936</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BNE | <div style='text-align: right'>30,989,234</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>972,160</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>425,326</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>251,329</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4ADD | <div style='text-align: right'>104,500</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>251,900</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>148,850</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>1,776,810</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW | <div style='text-align: right'>12,645,261</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW | <div style='text-align: right'>500,005</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW | <div style='text-align: right'>228,709</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW | <div style='text-align: right'>22,209</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>31,480,948</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,749</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | MUL | <div style='text-align: right'>12,546,660</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | MUL | <div style='text-align: right'>57,376</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | MUL | <div style='text-align: right'>33,943</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>939,312</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>557,232</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>371,484</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,772,438</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 1 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 1 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>14,108,838</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW | <div style='text-align: right'>116,116</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW | <div style='text-align: right'>65,650</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>1,280,609</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>14,078,826</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>889,460</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>527,774</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>221,884</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | SUB | <div style='text-align: right'>5,413,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | ADD | <div style='text-align: right'>71,233,560</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | ADD | <div style='text-align: right'>457,402</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | ADD | <div style='text-align: right'>270,283</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | ADD | <div style='text-align: right'>163,647</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>251,658</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>148,707</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,344,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>552,992</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>326,768</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>155,144</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BEQ | <div style='text-align: right'>865,007</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BNE | <div style='text-align: right'>31,022,722</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,441,440</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>851,760</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>556,920</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,439,784</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>973,440</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>425,524</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>251,446</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4ADD | <div style='text-align: right'>103,576</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>251,856</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>148,824</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4SUB | <div style='text-align: right'>24,904</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>1,796,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | JAL | <div style='text-align: right'>682</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | JAL | <div style='text-align: right'>806</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW | <div style='text-align: right'>12,659,529</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW | <div style='text-align: right'>500,577</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW | <div style='text-align: right'>229,151</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW | <div style='text-align: right'>22,561</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>31,492,592</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,749</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | MUL | <div style='text-align: right'>12,662,580</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | MUL | <div style='text-align: right'>57,530</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | MUL | <div style='text-align: right'>34,034</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>940,412</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>370,124</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,727,718</div>  |
| internal_verifier_height_1 | PhantomAir |  | 1 | 2 | PHANTOM | <div style='text-align: right'>2,120,910</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | 2 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | SHINTW | <div style='text-align: right'>19,077,628</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SHINTW | <div style='text-align: right'>5,118,388</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>14,122,450</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW | <div style='text-align: right'>116,490</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW | <div style='text-align: right'>65,767</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>1,281,577</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>14,087,026</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>890,560</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>220,524</div>  |
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
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
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
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
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
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | height | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,681.0</div>  | <div style='text-align: right'>25,894.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,696.0</div>  | <div style='text-align: right'>25,651.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,714.0</div>  | <div style='text-align: right'>25,775.0</div>  | <div style='text-align: right'>760,809,944</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
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

| group | cell_tracker_span | fixed_cells | lookup_advice_cells | simple_advice_cells |
| --- | --- | --- | --- | --- |
| static_verifier | VerifierProgram | <div style='text-align: right'>263,691</div>  | <div style='text-align: right'>440,178</div>  | <div style='text-align: right'>1,654,112</div>  |
| static_verifier | VerifierProgram;PoseidonCell | <div style='text-align: right'>5,800</div>  |  | <div style='text-align: right'>40,240</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds | <div style='text-align: right'>94,741</div>  | <div style='text-align: right'>1,320</div>  | <div style='text-align: right'>661,792</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds;PoseidonCell | <div style='text-align: right'>11,600</div>  |  | <div style='text-align: right'>80,480</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges | <div style='text-align: right'>161,671</div>  | <div style='text-align: right'>3,028</div>  | <div style='text-align: right'>1,138,770</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges;PoseidonCell | <div style='text-align: right'>18,850</div>  |  | <div style='text-align: right'>130,780</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold | <div style='text-align: right'>10,752</div>  |  | <div style='text-align: right'>21,504</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch | <div style='text-align: right'>28,224</div>  |  | <div style='text-align: right'>244,608</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>13,345,332</div>  | <div style='text-align: right'>710,640</div>  | <div style='text-align: right'>85,621,536</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;PoseidonCell | <div style='text-align: right'>4,169,760</div>  |  | <div style='text-align: right'>28,857,024</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening | <div style='text-align: right'>55,902</div>  | <div style='text-align: right'>80,640</div>  | <div style='text-align: right'>319,536</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;exp-reverse-bits-len | <div style='text-align: right'>2,750,496</div>  | <div style='text-align: right'>3,891,048</div>  | <div style='text-align: right'>16,255,428</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;single-reduced-opening-eval | <div style='text-align: right'>69,125,826</div>  | <div style='text-align: right'>83,318,760</div>  | <div style='text-align: right'>402,344,544</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges | <div style='text-align: right'>5,544</div>  | <div style='text-align: right'>8,064</div>  | <div style='text-align: right'>30,240</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query | <div style='text-align: right'>2,128,182</div>  | <div style='text-align: right'>2,890,440</div>  | <div style='text-align: right'>12,284,160</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext | <div style='text-align: right'>63,504</div>  |  | <div style='text-align: right'>550,368</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>1,947,078</div>  | <div style='text-align: right'>240,912</div>  | <div style='text-align: right'>12,612,600</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;PoseidonCell | <div style='text-align: right'>7,726,320</div>  |  | <div style='text-align: right'>53,470,368</div>  |
| static_verifier | VerifierProgram;stage-e-verify-constraints | <div style='text-align: right'>3,998,544</div>  | <div style='text-align: right'>5,220,844</div>  | <div style='text-align: right'>22,903,380</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4134771938b7b60bee1c3502ceb81961003a2599/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/4134771938b7b60bee1c3502ceb81961003a2599

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12112616548)
