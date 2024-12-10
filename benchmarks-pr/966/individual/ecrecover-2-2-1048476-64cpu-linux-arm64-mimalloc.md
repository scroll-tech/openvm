| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>10,567,808</div>  | <div style='text-align: right'>106,444</div>  | <span style="color: green">(-1,870.0 [-80.7%])</span> <div style='text-align: right'>448.0</div>  |
| app_proof | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>2,012.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <span style="color: green">(-2.0 [-28.6%])</span> <div style='text-align: right'>5.0</div>  | <span style="color: green">(-2.0 [-0.6%])</span> <div style='text-align: right'>350.0</div>  | <span style="color: red">(+733.0 [+293.2%])</span> <div style='text-align: right'>983.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+76,262.0 [+29107.6%])</span> <div style='text-align: right'>76,524.0</div>  |  | <div style='text-align: right'>10,567,808</div>  | <div style='text-align: right'>106,444</div>  | <span style="color: green">(-1,870.0 [-80.7%])</span> <div style='text-align: right'>448.0</div>  |
| app_proof |  |  | <div style='text-align: right'>245.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>1</div>  |  |  | <div style='text-align: right'>2,012.0</div>  |

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

| group | chip_name | rows_used |
| --- | --- | --- |
| ecrecover_program | ProgramChip | <div style='text-align: right'>8,064</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <div style='text-align: right'>2,946</div>  |
| ecrecover_program | Merkle | <div style='text-align: right'>3,264</div>  |
| ecrecover_program | AccessAdapter<4> | <div style='text-align: right'>34</div>  |
| ecrecover_program | AccessAdapter<8> | <div style='text-align: right'>6,364</div>  |
| ecrecover_program | AccessAdapter<16> | <div style='text-align: right'>2,878</div>  |
| ecrecover_program | AccessAdapter<32> | <div style='text-align: right'>1,440</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>1,271</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>726</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <div style='text-align: right'>3,194</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <div style='text-align: right'>6</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>814</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>1,508</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>1,744</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>5,220</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>11,190</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <div style='text-align: right'>4,005</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>28,448</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>8,531</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>2,026</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>37,447</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <div style='text-align: right'>45</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>6,210</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <div style='text-align: right'>27,504</div>  |
| ecrecover_program |  | AND | <div style='text-align: right'>5,000</div>  |
| ecrecover_program |  | AUIPC | <div style='text-align: right'>814</div>  |
| ecrecover_program |  | BEQ | <div style='text-align: right'>6,301</div>  |
| ecrecover_program |  | BGEU | <div style='text-align: right'>605</div>  |
| ecrecover_program |  | BLT | <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | <div style='text-align: right'>4,603</div>  |
| ecrecover_program |  | BNE | <div style='text-align: right'>4,889</div>  |
| ecrecover_program |  | EcAddNe | <div style='text-align: right'>726</div>  |
| ecrecover_program |  | EcDouble | <div style='text-align: right'>1,271</div>  |
| ecrecover_program |  | HINT_STOREW | <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | <div style='text-align: right'>3,203</div>  |
| ecrecover_program |  | JAL | <div style='text-align: right'>1,479</div>  |
| ecrecover_program |  | JALR | <div style='text-align: right'>1,508</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <div style='text-align: right'>4,005</div>  |
| ecrecover_program |  | LOADBU | <div style='text-align: right'>2,290</div>  |
| ecrecover_program |  | LOADW | <div style='text-align: right'>10,360</div>  |
| ecrecover_program |  | LUI | <div style='text-align: right'>265</div>  |
| ecrecover_program |  | MUL | <div style='text-align: right'>11</div>  |
| ecrecover_program |  | ModularAddSub | <div style='text-align: right'>7</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <div style='text-align: right'>4,037</div>  |
| ecrecover_program |  | PHANTOM | <div style='text-align: right'>45</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <div style='text-align: right'>4,100</div>  |
| ecrecover_program |  | SLTU | <div style='text-align: right'>2,026</div>  |
| ecrecover_program |  | SRL | <div style='text-align: right'>4,431</div>  |
| ecrecover_program |  | STOREB | <div style='text-align: right'>5,839</div>  |
| ecrecover_program |  | STOREW | <div style='text-align: right'>9,959</div>  |
| ecrecover_program |  | SUB | <div style='text-align: right'>886</div>  |
| ecrecover_program |  | XOR | <div style='text-align: right'>20</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>990,144</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>68</div>  |
| ecrecover_program | Boundary |  | ADD | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180,000</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>17,094</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>163,826</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>19,360</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>147,296</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>127,114</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <div style='text-align: right'>449,394</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <div style='text-align: right'>27,175</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <div style='text-align: right'>22,304</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <div style='text-align: right'>36,890</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <div style='text-align: right'>690,153</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <div style='text-align: right'>1,600</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <div style='text-align: right'>1,312</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <div style='text-align: right'>2,108</div>  |
| ecrecover_program | Boundary |  | EcDouble | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcDouble | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<16> |  | HINT_STOREW | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | HINT_STOREW | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>1,853</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>6,016</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <div style='text-align: right'>531,698</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <div style='text-align: right'>4,500</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <div style='text-align: right'>3,690</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <div style='text-align: right'>6,052</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <div style='text-align: right'>448</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>26,622</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>42,224</div>  |
| ecrecover_program | AccessAdapter<16> |  | KECCAK256 | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | KECCAK256 | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | KECCAK256 | <div style='text-align: right'>340</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <div style='text-align: right'>140,175</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>91,600</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | <div style='text-align: right'>625</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | <div style='text-align: right'>615</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <div style='text-align: right'>799</div>  |
| ecrecover_program | Boundary |  | LOADBU | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | LOADBU | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>414,400</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <div style='text-align: right'>17,125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <div style='text-align: right'>13,940</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>23,579</div>  |
| ecrecover_program | Boundary |  | LOADW | <div style='text-align: right'>480</div>  |
| ecrecover_program | Merkle |  | LOADW | <div style='text-align: right'>3,904</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>4,770</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <div style='text-align: right'>341</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <div style='text-align: right'>1,393</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <div style='text-align: right'>700</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <div style='text-align: right'>574</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <div style='text-align: right'>986</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>145,332</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>270</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>217,300</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>74,962</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>234,843</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>233,560</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <div style='text-align: right'>925</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <div style='text-align: right'>779</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>2,040</div>  |
| ecrecover_program | Boundary |  | STOREB | <div style='text-align: right'>2,040</div>  |
| ecrecover_program | Merkle |  | STOREB | <div style='text-align: right'>4,928</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>398,360</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <div style='text-align: right'>2,150</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <div style='text-align: right'>1,763</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>24,786</div>  |
| ecrecover_program | Boundary |  | STOREW | <div style='text-align: right'>51,280</div>  |
| ecrecover_program | Merkle |  | STOREW | <div style='text-align: right'>82,560</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>31,896</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>720</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>212,992</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>2,368</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | AccessAdapterAir<16> | 0 | <div style='text-align: right'>200,704</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | AccessAdapterAir<32> | 0 | <div style='text-align: right'>133,120</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>1,502,208</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>909,312</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>3,128</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>8</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>1,776</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>50,176</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>126,976</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>1,212,416</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>157,696</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>5,136,384</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| app_proof | 0 | <div style='text-align: right'>102.0</div>  | <div style='text-align: right'>1,910.0</div>  | <div style='text-align: right'>35,713,167</div>  |
| ecrecover_program | 0 | <span style="color: green">(-1.0 [-1.0%])</span> <div style='text-align: right'>98.0</div>  |  |  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/983509523fdee6f8a8b1f2caba3edf5936ce1477/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/983509523fdee6f8a8b1f2caba3edf5936ce1477

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12249198947)
