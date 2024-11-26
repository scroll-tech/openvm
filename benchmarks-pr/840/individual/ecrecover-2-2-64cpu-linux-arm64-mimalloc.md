| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>62,981,247</div>  | <div style='text-align: right'>1,153,229</div>  | <div style='text-align: right'>7,189.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| ecrecover_program | true | <div style='text-align: right'>10,260.0</div>  | <div style='text-align: right'>62,981,247</div>  | <div style='text-align: right'>1,153,229</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| ecrecover_program | ProgramChip | true | <div style='text-align: right'>17,109</div>  |
| ecrecover_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | true | <div style='text-align: right'>13,522</div>  |
| ecrecover_program | Merkle | true | <div style='text-align: right'>13,870</div>  |
| ecrecover_program | AccessAdapter<2> | true | <div style='text-align: right'>130</div>  |
| ecrecover_program | AccessAdapter<4> | true | <div style='text-align: right'>100</div>  |
| ecrecover_program | AccessAdapter<8> | true | <div style='text-align: right'>50,954</div>  |
| ecrecover_program | AccessAdapter<16> | true | <div style='text-align: right'>18,788</div>  |
| ecrecover_program | AccessAdapter<32> | true | <div style='text-align: right'>9,394</div>  |
| ecrecover_program | PhantomAir | true | <div style='text-align: right'>537</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>443,531</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>66,714</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>106,577</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>305,653</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>14,836</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>56,151</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>35,477</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>10,136</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>21,059</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>10,538</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>40,580</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>36,951</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>2</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>42</div>  |
| ecrecover_program | KeccakVmAir | true | <div style='text-align: right'>24</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>27,392</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <div style='text-align: right'>496</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <div style='text-align: right'>511</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true | <div style='text-align: right'>250</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true | <div style='text-align: right'>6</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true | <div style='text-align: right'>3,175</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| ecrecover_program | true |  | ADD | <div style='text-align: right'>328,002</div>  |
| ecrecover_program | true |  | AND | <div style='text-align: right'>67,891</div>  |
| ecrecover_program | true |  | AUIPC | <div style='text-align: right'>10,538</div>  |
| ecrecover_program | true |  | BEQ | <div style='text-align: right'>23,218</div>  |
| ecrecover_program | true |  | BGE | <div style='text-align: right'>1,808</div>  |
| ecrecover_program | true |  | BGEU | <div style='text-align: right'>1,548</div>  |
| ecrecover_program | true |  | BLT | <div style='text-align: right'>12</div>  |
| ecrecover_program | true |  | BLTU | <div style='text-align: right'>32,109</div>  |
| ecrecover_program | true |  | BNE | <div style='text-align: right'>32,933</div>  |
| ecrecover_program | true |  | DIVU | <div style='text-align: right'>2</div>  |
| ecrecover_program | true |  | EcAddNe | <div style='text-align: right'>496</div>  |
| ecrecover_program | true |  | EcDouble | <div style='text-align: right'>511</div>  |
| ecrecover_program | true |  | HINT_STOREW | <div style='text-align: right'>42</div>  |
| ecrecover_program | true |  | IS_EQ | <div style='text-align: right'>3,175</div>  |
| ecrecover_program | true |  | JAL | <div style='text-align: right'>4,265</div>  |
| ecrecover_program | true |  | JALR | <div style='text-align: right'>21,059</div>  |
| ecrecover_program | true |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| ecrecover_program | true |  | LOADB | <div style='text-align: right'>14,836</div>  |
| ecrecover_program | true |  | LOADBU | <div style='text-align: right'>2,470</div>  |
| ecrecover_program | true |  | LOADW | <div style='text-align: right'>138,280</div>  |
| ecrecover_program | true |  | LUI | <div style='text-align: right'>5,871</div>  |
| ecrecover_program | true |  | MUL | <div style='text-align: right'>40,580</div>  |
| ecrecover_program | true |  | MULHU | <div style='text-align: right'>36,951</div>  |
| ecrecover_program | true |  | ModularAddSub | <div style='text-align: right'>253</div>  |
| ecrecover_program | true |  | ModularMulDiv | <div style='text-align: right'>7</div>  |
| ecrecover_program | true |  | OR | <div style='text-align: right'>39,740</div>  |
| ecrecover_program | true |  | PHANTOM | <div style='text-align: right'>537</div>  |
| ecrecover_program | true |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program | true |  | SLL | <div style='text-align: right'>53,532</div>  |
| ecrecover_program | true |  | SLTU | <div style='text-align: right'>66,714</div>  |
| ecrecover_program | true |  | SRL | <div style='text-align: right'>53,045</div>  |
| ecrecover_program | true |  | STOREB | <div style='text-align: right'>23,124</div>  |
| ecrecover_program | true |  | STOREH | <div style='text-align: right'>1</div>  |
| ecrecover_program | true |  | STOREW | <div style='text-align: right'>141,778</div>  |
| ecrecover_program | true |  | SUB | <div style='text-align: right'>5,848</div>  |
| ecrecover_program | true |  | XOR | <div style='text-align: right'>2,050</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>11,808,072</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | ADD | <div style='text-align: right'>40</div>  |
| ecrecover_program | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>2,444,076</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>221,298</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>603,668</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>57,856</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>49,536</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>1,027,488</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>856,258</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>114</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcAddNe | <div style='text-align: right'>307,024</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcAddNe | <div style='text-align: right'>49,600</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcAddNe | <div style='text-align: right'>40,672</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcAddNe | <div style='text-align: right'>67,456</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcDouble | <div style='text-align: right'>343,392</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcDouble | <div style='text-align: right'>25,550</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcDouble | <div style='text-align: right'>20,951</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcDouble | <div style='text-align: right'>34,748</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>1,092</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>357</div>  |
| ecrecover_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>840</div>  |
| ecrecover_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>1,344</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | IS_EQ | <div style='text-align: right'>527,050</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | IS_EQ | <div style='text-align: right'>134,050</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | IS_EQ | <div style='text-align: right'>109,921</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | IS_EQ | <div style='text-align: right'>182,240</div>  |
| ecrecover_program | Boundary | true |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle | true |  | IS_EQ | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>76,770</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>589,652</div>  |
| ecrecover_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>715</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>429</div>  |
| ecrecover_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>519,260</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>98,800</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>221</div>  |
| ecrecover_program | Boundary | true |  | LOADBU | <div style='text-align: right'>520</div>  |
| ecrecover_program | Merkle | true |  | LOADBU | <div style='text-align: right'>512</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>5,531,200</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | LOADW | <div style='text-align: right'>126,600</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | LOADW | <div style='text-align: right'>103,812</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>198,526</div>  |
| ecrecover_program | Boundary | true |  | LOADW | <div style='text-align: right'>62,000</div>  |
| ecrecover_program | Merkle | true |  | LOADW | <div style='text-align: right'>86,592</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>105,678</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>1,257,980</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>1,441,089</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true |  | ModularAddSub | <div style='text-align: right'>50,347</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularAddSub | <div style='text-align: right'>25,300</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularAddSub | <div style='text-align: right'>20,746</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularAddSub | <div style='text-align: right'>34,289</div>  |
| ecrecover_program | Boundary | true |  | ModularAddSub | <div style='text-align: right'>680</div>  |
| ecrecover_program | Merkle | true |  | ModularAddSub | <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true |  | ModularMulDiv | <div style='text-align: right'>1,827</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularMulDiv | <div style='text-align: right'>350</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularMulDiv | <div style='text-align: right'>287</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularMulDiv | <div style='text-align: right'>476</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>1,430,640</div>  |
| ecrecover_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>3,222</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>2,837,196</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>2,468,418</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>2,811,385</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>924,960</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREB | <div style='text-align: right'>20,850</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREB | <div style='text-align: right'>34,153</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>30,532</div>  |
| ecrecover_program | Boundary | true |  | STOREB | <div style='text-align: right'>38,440</div>  |
| ecrecover_program | Merkle | true |  | STOREB | <div style='text-align: right'>104,000</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>5,671,120</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREW | <div style='text-align: right'>86,650</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREW | <div style='text-align: right'>53,997</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>203,235</div>  |
| ecrecover_program | Boundary | true |  | STOREW | <div style='text-align: right'>167,600</div>  |
| ecrecover_program | Merkle | true |  | STOREW | <div style='text-align: right'>244,864</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>210,528</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>73,800</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>14.0</div>  | <div style='text-align: right'>1,772.0</div>  | <div style='text-align: right'>1,290.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>196.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>62,981,247</div>  | <div style='text-align: right'>1,153,229</div>  | <div style='text-align: right'>7,189.0</div>  |

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
| ecrecover_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>456</div>  | <div style='text-align: right'>422</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>580</div>  | <div style='text-align: right'>540</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | <div style='text-align: right'>126</div>  | <div style='text-align: right'>94</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | <div style='text-align: right'>188</div>  | <div style='text-align: right'>156</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | <div style='text-align: right'>223</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>524,288</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>851,968</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>8,960</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>4,736</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,686,976</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <div style='text-align: right'>1,605,632</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <div style='text-align: right'>1,064,960</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>18,432</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>60,817,408</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>10,092,544</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>802,816</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>9,109,504</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>322</div>  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>3,968</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>64</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>32</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>20,545,536</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>751,104</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>512</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>899,072</div>  | <div style='text-align: right'>672</div>  | <div style='text-align: right'>1,084</div>  |  | <div style='text-align: right'>512</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>100,096</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>4,616</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>8</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>909,312</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <div style='text-align: right'>481.0</div>  | <div style='text-align: right'>4,936.0</div>  | <div style='text-align: right'>213,396,483</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/35c3de28a8c0fd6c23765b797e7f67031ea6ce81/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/35c3de28a8c0fd6c23765b797e7f67031ea6ce81

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12040367467)
