| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-1,894 [-0.0%])</span> <div style='text-align: right'>306,765,769</div>  | <div style='text-align: right'>5,787,691</div>  | <span style="color: green">(-327.0 [-0.8%])</span> <div style='text-align: right'>38,174.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| ecrecover_program | true | <span style="color: red">(+3,787.0 [+4.6%])</span> <div style='text-align: right'>86,063.0</div>  | <span style="color: green">(-1,894 [-0.0%])</span> <div style='text-align: right'>306,765,769</div>  | <div style='text-align: right'>5,787,691</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| ecrecover_program | ProgramChip | true | <div style='text-align: right'>17,443</div>  |
| ecrecover_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | true | <div style='text-align: right'>63,474</div>  |
| ecrecover_program | Merkle | true | <div style='text-align: right'>63,814</div>  |
| ecrecover_program | AccessAdapter<2> | true | <span style="color: green">(-132 [-16.9%])</span> <div style='text-align: right'>650</div>  |
| ecrecover_program | AccessAdapter<4> | true | <span style="color: green">(-34 [-8.5%])</span> <div style='text-align: right'>364</div>  |
| ecrecover_program | AccessAdapter<8> | true | <div style='text-align: right'>253,578</div>  |
| ecrecover_program | AccessAdapter<16> | true | <div style='text-align: right'>95,116</div>  |
| ecrecover_program | AccessAdapter<32> | true | <div style='text-align: right'>47,558</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <div style='text-align: right'>2,555</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <div style='text-align: right'>2,550</div>  |
| ecrecover_program | KeccakVmAir | true | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true | <div style='text-align: right'>16,045</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true | <div style='text-align: right'>26</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true | <div style='text-align: right'>1,281</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>174</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>10</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>184,755</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>202,885</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>52,831</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>105,597</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>50,866</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>178,112</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>282,142</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>74,192</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>1,537,036</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>536,117</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>333,732</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>2,224,089</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | true | <div style='text-align: right'>2,673</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>127,288</div>  |
| ecrecover_program | VariableRangeCheckerAir | true | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| ecrecover_program | true |  | ADD | <div style='text-align: right'>1,644,021</div>  |
| ecrecover_program | true |  | AND | <div style='text-align: right'>340,105</div>  |
| ecrecover_program | true |  | AUIPC | <div style='text-align: right'>52,831</div>  |
| ecrecover_program | true |  | BEQ | <div style='text-align: right'>116,626</div>  |
| ecrecover_program | true |  | BGE | <div style='text-align: right'>9,005</div>  |
| ecrecover_program | true |  | BGEU | <div style='text-align: right'>7,807</div>  |
| ecrecover_program | true |  | BLT | <div style='text-align: right'>65</div>  |
| ecrecover_program | true |  | BLTU | <div style='text-align: right'>161,235</div>  |
| ecrecover_program | true |  | BNE | <div style='text-align: right'>165,516</div>  |
| ecrecover_program | true |  | DIVU | <div style='text-align: right'>10</div>  |
| ecrecover_program | true |  | EcAddNe | <div style='text-align: right'>2,550</div>  |
| ecrecover_program | true |  | EcDouble | <div style='text-align: right'>2,555</div>  |
| ecrecover_program | true |  | HINT_STOREW | <div style='text-align: right'>174</div>  |
| ecrecover_program | true |  | IS_EQ | <div style='text-align: right'>16,049</div>  |
| ecrecover_program | true |  | JAL | <div style='text-align: right'>21,533</div>  |
| ecrecover_program | true |  | JALR | <div style='text-align: right'>105,597</div>  |
| ecrecover_program | true |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program | true |  | LOADB | <div style='text-align: right'>74,192</div>  |
| ecrecover_program | true |  | LOADBU | <div style='text-align: right'>13,380</div>  |
| ecrecover_program | true |  | LOADW | <div style='text-align: right'>695,156</div>  |
| ecrecover_program | true |  | LUI | <div style='text-align: right'>29,333</div>  |
| ecrecover_program | true |  | MUL | <div style='text-align: right'>202,885</div>  |
| ecrecover_program | true |  | MULHU | <div style='text-align: right'>184,755</div>  |
| ecrecover_program | true |  | ModularAddSub | <div style='text-align: right'>1,292</div>  |
| ecrecover_program | true |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program | true |  | OR | <div style='text-align: right'>200,706</div>  |
| ecrecover_program | true |  | PHANTOM | <div style='text-align: right'>2,673</div>  |
| ecrecover_program | true |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program | true |  | SLL | <div style='text-align: right'>269,672</div>  |
| ecrecover_program | true |  | SLTU | <div style='text-align: right'>333,732</div>  |
| ecrecover_program | true |  | SRL | <div style='text-align: right'>266,445</div>  |
| ecrecover_program | true |  | STOREB | <div style='text-align: right'>115,692</div>  |
| ecrecover_program | true |  | STOREH | <div style='text-align: right'>5</div>  |
| ecrecover_program | true |  | STOREW | <div style='text-align: right'>712,803</div>  |
| ecrecover_program | true |  | SUB | <div style='text-align: right'>28,988</div>  |
| ecrecover_program | true |  | XOR | <div style='text-align: right'>10,269</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>59,184,756</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | ADD | <div style='text-align: right'>40</div>  |
| ecrecover_program | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>12,243,780</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>1,109,451</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>3,032,276</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>288,160</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>249,824</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>2,080</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>5,159,520</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>4,303,416</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>570</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcAddNe | <div style='text-align: right'>1,578,450</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcAddNe | <div style='text-align: right'>255,000</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcAddNe | <div style='text-align: right'>209,100</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcAddNe | <div style='text-align: right'>346,800</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcDouble | <div style='text-align: right'>1,387,365</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcDouble | <div style='text-align: right'>127,750</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcDouble | <div style='text-align: right'>104,755</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcDouble | <div style='text-align: right'>173,740</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>4,524</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>1,513</div>  |
| ecrecover_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>6,016</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | IS_EQ | <div style='text-align: right'>2,664,134</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | IS_EQ | <div style='text-align: right'>675,250</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | IS_EQ | <div style='text-align: right'>553,705</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | IS_EQ | <div style='text-align: right'>918,272</div>  |
| ecrecover_program | Boundary | true |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle | true |  | IS_EQ | <div style='text-align: right'>640</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>387,594</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>2,956,716</div>  |
| ecrecover_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>3,575</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>2,145</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | KECCAK256 | <div style='text-align: right'>68</div>  |
| ecrecover_program | Boundary | true |  | KECCAK256 | <div style='text-align: right'>160</div>  |
| ecrecover_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | Merkle | true |  | KECCAK256 | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>2,596,720</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>535,200</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>765</div>  |
| ecrecover_program | Boundary | true |  | LOADBU | <div style='text-align: right'>1,800</div>  |
| ecrecover_program | Merkle | true |  | LOADBU | <div style='text-align: right'>2,496</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>27,806,240</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | LOADW | <div style='text-align: right'>643,350</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | LOADW | <div style='text-align: right'>527,547</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>997,798</div>  |
| ecrecover_program | Boundary | true |  | LOADW | <div style='text-align: right'>289,040</div>  |
| ecrecover_program | Merkle | true |  | LOADW | <div style='text-align: right'>384,576</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>527,994</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>6,289,435</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>7,205,445</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true |  | ModularAddSub | <div style='text-align: right'>257,108</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularAddSub | <div style='text-align: right'>129,200</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularAddSub | <div style='text-align: right'>105,944</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | ModularAddSub | <span style="color: green">(-221 [-50.0%])</span> <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularAddSub | <div style='text-align: right'>175,746</div>  |
| ecrecover_program | Boundary | true |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle | true |  | ModularAddSub | <div style='text-align: right'>2,752</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularMulDiv | <div style='text-align: right'>1,750</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularMulDiv | <div style='text-align: right'>1,435</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularMulDiv | <div style='text-align: right'>2,380</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>7,225,416</div>  |
| ecrecover_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>16,038</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>14,292,616</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>12,348,084</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>14,121,585</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>4,627,680</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREB | <div style='text-align: right'>103,300</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREB | <div style='text-align: right'>169,207</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>151,232</div>  |
| ecrecover_program | Boundary | true |  | STOREB | <div style='text-align: right'>190,360</div>  |
| ecrecover_program | Merkle | true |  | STOREB | <div style='text-align: right'>512,128</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>200</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>28,512,120</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREW | <div style='text-align: right'>441,550</div>  |
| ecrecover_program | AccessAdapter<2> | true |  | STOREW | <div style='text-align: right'>2,860</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREW | <div style='text-align: right'>277,570</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | STOREW | <div style='text-align: right'>1,716</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>1,003,646</div>  |
| ecrecover_program | Boundary | true |  | STOREW | <div style='text-align: right'>783,480</div>  |
| ecrecover_program | Merkle | true |  | STOREW | <div style='text-align: right'>1,129,664</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>1,043,568</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>369,684</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>13.0</div>  | <span style="color: green">(-121.0 [-1.4%])</span> <div style='text-align: right'>8,354.0</div>  | <span style="color: green">(-48.0 [-0.8%])</span> <div style='text-align: right'>6,328.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-6.0 [-2.7%])</span> <div style='text-align: right'>215.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-1,894 [-0.0%])</span> <div style='text-align: right'>306,765,769</div>  | <div style='text-align: right'>5,787,691</div>  | <span style="color: green">(-327.0 [-0.8%])</span> <div style='text-align: right'>38,174.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>449</div>  | <div style='text-align: right'>411</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>456</div>  | <div style='text-align: right'>422</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | <div style='text-align: right'>223</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | <div style='text-align: right'>188</div>  | <div style='text-align: right'>156</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | <div style='text-align: right'>126</div>  | <div style='text-align: right'>94</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>3,407,872</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>35,840</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>18,944</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>512</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>10,747,904</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <div style='text-align: right'>6,422,528</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <div style='text-align: right'>4,259,840</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>5,615,616</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>6,008,832</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>3,637,248</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>18,464</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>32</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>800,768</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>2,576</div>  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  | <div style='text-align: right'>16</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>36,438,016</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>29,097,984</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>4,063,232</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>14,548,992</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>110,100,480</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>486,539,264</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>82,182,144</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: green">(-73.0 [-3.5%])</span> <div style='text-align: right'>2,024.0</div>  | <span style="color: green">(-133.0 [-0.5%])</span> <div style='text-align: right'>27,796.0</div>  | <div style='text-align: right'>1,163,751,921</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/133a7e06ce2b35d927e97b42d61e81d831ed6323/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/133a7e06ce2b35d927e97b42d61e81d831ed6323

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12112494248)
