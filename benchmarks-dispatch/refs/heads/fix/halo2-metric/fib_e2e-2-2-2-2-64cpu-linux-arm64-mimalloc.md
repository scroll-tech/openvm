| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>12,292,198</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>38,739.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>988,921,566</div>  | <div style='text-align: right'>24,154,564</div>  | <div style='text-align: right'>91,945.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>572,768,092</div>  | <div style='text-align: right'>14,517,592</div>  | <div style='text-align: right'>54,296.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,927,939</div>  | <div style='text-align: right'>7,276,375</div>  | <div style='text-align: right'>27,342.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,133,298</div>  | <div style='text-align: right'>3,633,933</div>  | <div style='text-align: right'>73,924.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_time_ms | fri.log_blowup | halo2_keygen_time_ms | halo2_proof_time_ms | halo2_total_cells | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>7,720.0</div>  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>12,292,198</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>38,739.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>988,921,566</div>  | <div style='text-align: right'>24,154,564</div>  | <div style='text-align: right'>91,945.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>572,768,092</div>  | <div style='text-align: right'>14,517,592</div>  | <div style='text-align: right'>54,296.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  |  | <div style='text-align: right'>286,927,939</div>  | <div style='text-align: right'>7,276,375</div>  | <div style='text-align: right'>27,342.0</div>  |
| root_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  | <div style='text-align: right'>73,924.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>144,133,298</div>  | <div style='text-align: right'>3,633,933</div>  | <div style='text-align: right'>73,924.0</div>  |
| static_verifier |  |  | <div style='text-align: right'>521,199.0</div>  | <div style='text-align: right'>399,824.0</div>  | <div style='text-align: right'>333,363,332.0</div>  |  |  |  |  |  |  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_continuation_program | ProgramChip | <div style='text-align: right'>6,547</div>  |
| fibonacci_continuation_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | Boundary | <div style='text-align: right'>52</div>  |
| fibonacci_continuation_program | Merkle | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapter<8> | <div style='text-align: right'>52</div>  |
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
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>308</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |
| root_verifier | ProgramChip | <div style='text-align: right'>154,163</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>412,045</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>387,980</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>194,116</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,602</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,103</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,042</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,489,968</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>87,554</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,273</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,908</div>  |
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
| root_verifier |  | ADD | <div style='text-align: right'>1,187,405</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,422</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,812</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>18,805</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>674,468</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>17,400</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,168</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,640</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>87,554</div>  |
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
| fibonacci_continuation_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>17</div>  |
| fibonacci_continuation_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>192</div>  |
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
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>35,622,150</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>215,094</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>127,101</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>163,559</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>256,880</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,192</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,568</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>672,480</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>309,386</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>182,819</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>155,144</div>  |
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
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>875,540</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,336,222</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>249,711</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>114,010</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,407</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,750,232</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>62,788</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>37,102</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,815</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,331,650</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>28,171</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>16,666</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>470,294</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>278,720</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,096</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,864,977</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,060,470</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,539,880</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,559,480</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>7,081,725</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>55,693</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>31,603</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>661,045</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>7,044,169</div>  |
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
| fibonacci_continuation_program | 0 | <div style='text-align: right'>262.0</div>  | <div style='text-align: right'>5,146.0</div>  | <div style='text-align: right'>197,760,980</div>  |
| fibonacci_continuation_program | 1 | <div style='text-align: right'>274.0</div>  | <div style='text-align: right'>4,584.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 2 | <div style='text-align: right'>270.0</div>  | <div style='text-align: right'>4,970.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 3 | <div style='text-align: right'>287.0</div>  | <div style='text-align: right'>5,049.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 4 | <div style='text-align: right'>271.0</div>  | <div style='text-align: right'>5,031.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 5 | <div style='text-align: right'>275.0</div>  | <div style='text-align: right'>4,889.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 6 | <div style='text-align: right'>283.0</div>  | <div style='text-align: right'>4,981.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| fibonacci_continuation_program | 7 | <div style='text-align: right'>86.0</div>  | <div style='text-align: right'>2,081.0</div>  | <div style='text-align: right'>55,440,402</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>6,896.0</div>  | <div style='text-align: right'>263,224,511</div>  | <div style='text-align: right'>6,441,269</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,488.0</div>  | <div style='text-align: right'>240,481,419</div>  | <div style='text-align: right'>5,869,070</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,483.0</div>  | <div style='text-align: right'>240,478,739</div>  | <div style='text-align: right'>5,868,676</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>6,646.0</div>  | <div style='text-align: right'>244,736,897</div>  | <div style='text-align: right'>5,975,549</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>106,866</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>760,162</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>722,322</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>361,582</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,738</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>53,015</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>61,900</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,506,640</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>138,659</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,233,307</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,070,099</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,457</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>106,866</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>724,624</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>664,872</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>332,774</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,230</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,302,828</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>131,388</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>106,866</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>724,624</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>664,944</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>332,810</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,230</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,302,828</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>130,994</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>106,866</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>724,513</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>676,126</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>338,484</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>104,918</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>50,041</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>220,248</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>56,257</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>2,329,992</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>130,603</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>1,137,028</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,929,113</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>334,331</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,127,554</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>10,994</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>21,204</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>36,789</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,196,518</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>33,978</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>22,848</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>138,659</div>  |
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
| leaf_verifier |  | 1 | ADD | <div style='text-align: right'>1,952,502</div>  |
| leaf_verifier |  | 1 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 1 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 1 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 1 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 1 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 1 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier |  | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>131,388</div>  |
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
| leaf_verifier |  | 2 | ADD | <div style='text-align: right'>1,952,502</div>  |
| leaf_verifier |  | 2 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 2 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 2 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 2 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 2 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 2 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier |  | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>130,994</div>  |
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
| leaf_verifier |  | 3 | ADD | <div style='text-align: right'>1,978,610</div>  |
| leaf_verifier |  | 3 | BBE4DIV | <div style='text-align: right'>9,924</div>  |
| leaf_verifier |  | 3 | BBE4MUL | <div style='text-align: right'>19,300</div>  |
| leaf_verifier |  | 3 | BEQ | <div style='text-align: right'>35,589</div>  |
| leaf_verifier |  | 3 | BNE | <div style='text-align: right'>1,101,439</div>  |
| leaf_verifier |  | 3 | COMP_POS2 | <div style='text-align: right'>32,897</div>  |
| leaf_verifier |  | 3 | DIV | <div style='text-align: right'>186</div>  |
| leaf_verifier |  | 3 | FE4ADD | <div style='text-align: right'>20,492</div>  |
| leaf_verifier |  | 3 | FE4SUB | <div style='text-align: right'>6,541</div>  |
| leaf_verifier |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>8,148</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>130,603</div>  |
| leaf_verifier |  | 3 | LOADW | <div style='text-align: right'>282,752</div>  |
| leaf_verifier |  | 3 | LOADW2 | <div style='text-align: right'>575,892</div>  |
| leaf_verifier |  | 3 | MUL | <div style='text-align: right'>253,216</div>  |
| leaf_verifier |  | 3 | PERM_POS2 | <div style='text-align: right'>17,144</div>  |
| leaf_verifier |  | 3 | PHANTOM | <div style='text-align: right'>334,331</div>  |
| leaf_verifier |  | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 3 | SHINTW | <div style='text-align: right'>436,683</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>339,330</div>  |
| leaf_verifier |  | 3 | STOREW2 | <div style='text-align: right'>294,456</div>  |
| leaf_verifier |  | 3 | SUB | <div style='text-align: right'>97,980</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>63,826,620</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>363,264</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>214,656</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>141,295</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>439,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>207,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>122,577</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>848,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>481,778</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>284,687</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>141,240</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>846,147</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>27,519,914</div>  |
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
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>1,386,590</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>693</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>819</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>12,049,449</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>512,457</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>244,114</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>26,131,678</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>8,147,700</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>46,068</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>27,261</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,736</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,029,875</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>611,299</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>408,629</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>10,641,683</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>2,210,742</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>18,973,160</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,090,360</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>14,540,568</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>116,468</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>66,989</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>1,368,675</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>13,179,204</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>847,055</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>503,269</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>237,677</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>1,410,398</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>3,218,460</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>101,684</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>120,172</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | ADD | <div style='text-align: right'>58,575,060</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | ADD | <div style='text-align: right'>314,600</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | ADD | <div style='text-align: right'>185,900</div>  |
| leaf_verifier | Boundary |  | 1 | ADD | <div style='text-align: right'>133,331</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4MUL | <div style='text-align: right'>421,432</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4MUL | <div style='text-align: right'>249,028</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BNE | <div style='text-align: right'>24,818,403</div>  |
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
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>1,313,880</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW | <div style='text-align: right'>456,412</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW | <div style='text-align: right'>220,844</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | MUL | <div style='text-align: right'>43,032</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | MUL | <div style='text-align: right'>25,467</div>  |
| leaf_verifier | Boundary |  | 1 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 1 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 1 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW | <div style='text-align: right'>97,295</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW | <div style='text-align: right'>55,796</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>1,342,891</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | SUB | <div style='text-align: right'>84,920</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | SUB | <div style='text-align: right'>100,360</div>  |
| leaf_verifier | Boundary |  | 1 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | ADD | <div style='text-align: right'>58,575,060</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | ADD | <div style='text-align: right'>314,996</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | ADD | <div style='text-align: right'>186,134</div>  |
| leaf_verifier | Boundary |  | 2 | ADD | <div style='text-align: right'>133,331</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4MUL | <div style='text-align: right'>421,828</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4MUL | <div style='text-align: right'>249,262</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BNE | <div style='text-align: right'>24,818,403</div>  |
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
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>1,309,940</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW | <div style='text-align: right'>456,412</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW | <div style='text-align: right'>220,844</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | MUL | <div style='text-align: right'>43,032</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | MUL | <div style='text-align: right'>25,467</div>  |
| leaf_verifier | Boundary |  | 2 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 2 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 2 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 2 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW | <div style='text-align: right'>97,295</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW | <div style='text-align: right'>55,796</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>1,342,891</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | SUB | <div style='text-align: right'>84,920</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | SUB | <div style='text-align: right'>100,360</div>  |
| leaf_verifier | Boundary |  | 2 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | ADD | <div style='text-align: right'>59,358,300</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | ADD | <div style='text-align: right'>330,220</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | ADD | <div style='text-align: right'>195,130</div>  |
| leaf_verifier | Boundary |  | 3 | ADD | <div style='text-align: right'>133,419</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4DIV | <div style='text-align: right'>396,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4DIV | <div style='text-align: right'>184,536</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4DIV | <div style='text-align: right'>109,044</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4MUL | <div style='text-align: right'>772,000</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4MUL | <div style='text-align: right'>441,364</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4MUL | <div style='text-align: right'>260,806</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4MUL | <div style='text-align: right'>146,784</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BEQ | <div style='text-align: right'>818,547</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BNE | <div style='text-align: right'>25,333,097</div>  |
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
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>1,306,030</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | JAL | <div style='text-align: right'>605</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | JAL | <div style='text-align: right'>715</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | LOADW | <div style='text-align: right'>11,592,832</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW | <div style='text-align: right'>474,925</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW | <div style='text-align: right'>228,475</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW | <div style='text-align: right'>40,171</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | LOADW2 | <div style='text-align: right'>23,611,572</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW2 | <div style='text-align: right'>111,628</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW2 | <div style='text-align: right'>65,962</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW2 | <div style='text-align: right'>969</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | MUL | <div style='text-align: right'>7,596,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | MUL | <div style='text-align: right'>43,329</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | MUL | <div style='text-align: right'>25,649</div>  |
| leaf_verifier | Boundary |  | 3 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | PERM_POS2 | <div style='text-align: right'>943,789</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | PERM_POS2 | <div style='text-align: right'>560,430</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | PERM_POS2 | <div style='text-align: right'>373,592</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | PERM_POS2 | <div style='text-align: right'>9,583,496</div>  |
| leaf_verifier | PhantomAir |  | 3 | PHANTOM | <div style='text-align: right'>2,005,986</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 3 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | SHINTW | <div style='text-align: right'>17,904,003</div>  |
| leaf_verifier | Boundary |  | 3 | SHINTW | <div style='text-align: right'>4,803,513</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>13,912,530</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW | <div style='text-align: right'>107,910</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW | <div style='text-align: right'>62,049</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>1,331,187</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW2 | <div style='text-align: right'>12,072,696</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW2 | <div style='text-align: right'>746,207</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW2 | <div style='text-align: right'>443,677</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW2 | <div style='text-align: right'>208,369</div>  |
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
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
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
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,533.0</div>  | <div style='text-align: right'>21,571.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,471.0</div>  | <div style='text-align: right'>21,693.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,487.0</div>  | <div style='text-align: right'>21,405.0</div>  | <div style='text-align: right'>615,320,024</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>1,477.0</div>  | <div style='text-align: right'>21,308.0</div>  | <div style='text-align: right'>615,320,024</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,734.0</div>  | <div style='text-align: right'>286,394,563</div>  | <div style='text-align: right'>7,259,429</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>7,820.0</div>  | <div style='text-align: right'>286,373,529</div>  | <div style='text-align: right'>7,258,163</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>7,926.0</div>  | <div style='text-align: right'>286,927,939</div>  | <div style='text-align: right'>7,276,375</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>153,890</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>771,391</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>752,820</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>376,746</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,896</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,094</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>77,886</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,970,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>181,648</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,385,009</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,659</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>153,890</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>771,391</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>752,764</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>376,718</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>109,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>52,178</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,836</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,967,518</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>183,064</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,384,955</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,228,575</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>153,890</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>772,875</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>754,088</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>377,296</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,182</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,084</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,979,714</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>185,252</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,495</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,527</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | ADD | <div style='text-align: right'>2,369,194</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>33,458</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BNE | <div style='text-align: right'>1,347,484</div>  |
| internal_verifier_height_0 |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4ADD | <div style='text-align: right'>24,304</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>181,648</div>  |
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
| internal_verifier_height_0 |  | 0 | 1 | ADD | <div style='text-align: right'>2,368,128</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>33,408</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BNE | <div style='text-align: right'>1,347,430</div>  |
| internal_verifier_height_0 |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4ADD | <div style='text-align: right'>24,304</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>183,064</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW | <div style='text-align: right'>308,457</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW2 | <div style='text-align: right'>768,020</div>  |
| internal_verifier_height_0 |  | 0 | 1 | MUL | <div style='text-align: right'>418,222</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,486</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>344,160</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW2 | <div style='text-align: right'>343,418</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SUB | <div style='text-align: right'>180,440</div>  |
| internal_verifier_height_1 |  | 1 | 2 | ADD | <div style='text-align: right'>2,374,612</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>33,624</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BEQ | <div style='text-align: right'>37,609</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BNE | <div style='text-align: right'>1,348,886</div>  |
| internal_verifier_height_1 |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,776</div>  |
| internal_verifier_height_1 |  | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4ADD | <div style='text-align: right'>24,336</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>185,252</div>  |
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
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | ADD | <div style='text-align: right'>71,075,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | ADD | <div style='text-align: right'>457,358</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | ADD | <div style='text-align: right'>270,257</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | ADD | <div style='text-align: right'>162,811</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>252,010</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>148,915</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,338,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>551,650</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>325,975</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>155,936</div>  |
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
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>1,816,480</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW | <div style='text-align: right'>500,005</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW | <div style='text-align: right'>228,709</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | MUL | <div style='text-align: right'>12,576,900</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | MUL | <div style='text-align: right'>57,376</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | MUL | <div style='text-align: right'>33,943</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,124</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,727,718</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 0 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 0 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>14,114,004</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW | <div style='text-align: right'>116,116</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW | <div style='text-align: right'>65,650</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>1,280,609</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>889,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>220,524</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | SUB | <div style='text-align: right'>5,428,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | ADD | <div style='text-align: right'>71,043,840</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | ADD | <div style='text-align: right'>457,050</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | ADD | <div style='text-align: right'>270,075</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | ADD | <div style='text-align: right'>162,811</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>252,010</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>148,915</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,336,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>551,342</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>325,793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>155,936</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BNE | <div style='text-align: right'>30,990,890</div>  |
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
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>1,830,640</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW | <div style='text-align: right'>500,005</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW | <div style='text-align: right'>228,709</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | MUL | <div style='text-align: right'>12,546,660</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | MUL | <div style='text-align: right'>57,376</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | MUL | <div style='text-align: right'>33,943</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>371,552</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,774,674</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 1 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 1 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>14,110,560</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW | <div style='text-align: right'>116,116</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW | <div style='text-align: right'>65,650</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>1,280,609</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>889,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>221,952</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | SUB | <div style='text-align: right'>5,413,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | ADD | <div style='text-align: right'>71,238,360</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | ADD | <div style='text-align: right'>458,436</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | ADD | <div style='text-align: right'>270,894</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | ADD | <div style='text-align: right'>163,647</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>251,658</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>148,707</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,344,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>554,026</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>327,379</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>155,144</div>  |
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
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>1,852,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | JAL | <div style='text-align: right'>682</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | JAL | <div style='text-align: right'>806</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW | <div style='text-align: right'>12,661,005</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW | <div style='text-align: right'>500,577</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW | <div style='text-align: right'>229,151</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW | <div style='text-align: right'>22,759</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>31,500,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | MUL | <div style='text-align: right'>12,662,580</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | MUL | <div style='text-align: right'>57,530</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | MUL | <div style='text-align: right'>34,034</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>940,588</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>557,440</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>370,192</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,729,954</div>  |
| internal_verifier_height_1 | PhantomAir |  | 1 | 2 | PHANTOM | <div style='text-align: right'>2,120,910</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | 2 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | SHINTW | <div style='text-align: right'>19,077,628</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SHINTW | <div style='text-align: right'>5,118,388</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>14,124,172</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW | <div style='text-align: right'>116,490</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW | <div style='text-align: right'>65,767</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>1,281,577</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>14,088,338</div>  |
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
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,702.0</div>  | <div style='text-align: right'>25,717.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,706.0</div>  | <div style='text-align: right'>25,171.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,646.0</div>  | <div style='text-align: right'>25,696.0</div>  | <div style='text-align: right'>760,809,944</div>  |

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
| static_verifier | VerifierProgram | <div style='text-align: right'>256,155</div>  | <div style='text-align: right'>220,089</div>  | <div style='text-align: right'>802,768</div>  |
| static_verifier | VerifierProgram;PoseidonCell | <div style='text-align: right'>5,800</div>  |  | <div style='text-align: right'>20,120</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds | <div style='text-align: right'>94,034</div>  | <div style='text-align: right'>456</div>  | <div style='text-align: right'>333,615</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds;PoseidonCell | <div style='text-align: right'>13,775</div>  |  | <div style='text-align: right'>47,785</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges | <div style='text-align: right'>150,897</div>  | <div style='text-align: right'>1,499</div>  | <div style='text-align: right'>534,902</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges;PoseidonCell | <div style='text-align: right'>18,850</div>  |  | <div style='text-align: right'>65,390</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold | <div style='text-align: right'>10,752</div>  |  | <div style='text-align: right'>10,752</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch | <div style='text-align: right'>28,224</div>  |  | <div style='text-align: right'>122,304</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>3,633,252</div>  | <div style='text-align: right'>177,660</div>  | <div style='text-align: right'>11,957,148</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;PoseidonCell | <div style='text-align: right'>4,169,760</div>  |  | <div style='text-align: right'>14,428,512</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening | <div style='text-align: right'>55,902</div>  | <div style='text-align: right'>40,320</div>  | <div style='text-align: right'>159,768</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;exp-reverse-bits-len | <div style='text-align: right'>2,750,496</div>  | <div style='text-align: right'>1,945,524</div>  | <div style='text-align: right'>8,127,714</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;single-reduced-opening-eval | <div style='text-align: right'>69,125,826</div>  | <div style='text-align: right'>41,659,380</div>  | <div style='text-align: right'>201,172,272</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges | <div style='text-align: right'>5,544</div>  | <div style='text-align: right'>4,032</div>  | <div style='text-align: right'>15,120</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query | <div style='text-align: right'>2,128,182</div>  | <div style='text-align: right'>1,445,220</div>  | <div style='text-align: right'>6,142,080</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext | <div style='text-align: right'>63,504</div>  |  | <div style='text-align: right'>275,184</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>832,230</div>  | <div style='text-align: right'>120,456</div>  | <div style='text-align: right'>2,735,964</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;PoseidonCell | <div style='text-align: right'>7,726,320</div>  |  | <div style='text-align: right'>26,735,184</div>  |
| static_verifier | VerifierProgram;stage-e-verify-constraints | <div style='text-align: right'>3,998,544</div>  | <div style='text-align: right'>2,610,422</div>  | <div style='text-align: right'>11,451,690</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/dd7f9b0fc828c0b6bc3842cdb3f0f8e5ae0f2d48

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12150610310)
