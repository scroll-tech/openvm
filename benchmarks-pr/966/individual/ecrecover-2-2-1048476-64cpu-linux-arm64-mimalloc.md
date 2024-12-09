| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>100,539,955</div>  | <div style='text-align: right'>1,502,571</div>  | <span style="color: red">(+126.0 [+1.2%])</span> <div style='text-align: right'>10,526.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>8.0</div>  | <span style="color: red">(+134.0 [+5.2%])</span> <div style='text-align: right'>2,734.0</div>  | <span style="color: green">(-18.0 [-1.0%])</span> <div style='text-align: right'>1,856.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+76,380.0 [+29719.8%])</span> <div style='text-align: right'>76,637.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>100,539,955</div>  | <div style='text-align: right'>1,502,571</div>  | <span style="color: red">(+126.0 [+1.2%])</span> <div style='text-align: right'>10,526.0</div>  |

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
| ecrecover_program | ProgramChip | <div style='text-align: right'>8,431</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <div style='text-align: right'>31,622</div>  |
| ecrecover_program | Merkle | <div style='text-align: right'>31,936</div>  |
| ecrecover_program | AccessAdapter<4> | <div style='text-align: right'>34</div>  |
| ecrecover_program | AccessAdapter<8> | <div style='text-align: right'>126,872</div>  |
| ecrecover_program | AccessAdapter<16> | <div style='text-align: right'>47,670</div>  |
| ecrecover_program | AccessAdapter<32> | <div style='text-align: right'>23,836</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>1,277</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>1,268</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <div style='text-align: right'>8,018</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <div style='text-align: right'>642</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <div style='text-align: right'>1,291</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>14,568</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>29,146</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>14,707</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>76,944</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>119,280</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <div style='text-align: right'>37,173</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>510,113</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>76,595</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>50,954</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>559,012</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <div style='text-align: right'>1,325</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>63,558</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <div style='text-align: right'>386,128</div>  |
| ecrecover_program |  | AND | <div style='text-align: right'>117,837</div>  |
| ecrecover_program |  | AUIPC | <div style='text-align: right'>14,568</div>  |
| ecrecover_program |  | BEQ | <div style='text-align: right'>50,976</div>  |
| ecrecover_program |  | BGE | <div style='text-align: right'>4,504</div>  |
| ecrecover_program |  | BGEU | <div style='text-align: right'>3,930</div>  |
| ecrecover_program |  | BLT | <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | <div style='text-align: right'>68,498</div>  |
| ecrecover_program |  | BNE | <div style='text-align: right'>68,304</div>  |
| ecrecover_program |  | EcAddNe | <div style='text-align: right'>1,268</div>  |
| ecrecover_program |  | EcDouble | <div style='text-align: right'>1,277</div>  |
| ecrecover_program |  | HINT_STOREW | <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | <div style='text-align: right'>8,027</div>  |
| ecrecover_program |  | JAL | <div style='text-align: right'>8,062</div>  |
| ecrecover_program |  | JALR | <div style='text-align: right'>29,146</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <div style='text-align: right'>37,173</div>  |
| ecrecover_program |  | LOADBU | <div style='text-align: right'>5,182</div>  |
| ecrecover_program |  | LOADW | <div style='text-align: right'>211,009</div>  |
| ecrecover_program |  | LUI | <div style='text-align: right'>6,645</div>  |
| ecrecover_program |  | MUL | <div style='text-align: right'>1,291</div>  |
| ecrecover_program |  | ModularAddSub | <div style='text-align: right'>643</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <div style='text-align: right'>45,889</div>  |
| ecrecover_program |  | PHANTOM | <div style='text-align: right'>1,325</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <div style='text-align: right'>36,154</div>  |
| ecrecover_program |  | SLTU | <div style='text-align: right'>50,954</div>  |
| ecrecover_program |  | SRA | <div style='text-align: right'>1,284</div>  |
| ecrecover_program |  | SRL | <div style='text-align: right'>39,157</div>  |
| ecrecover_program |  | STOREB | <div style='text-align: right'>56,402</div>  |
| ecrecover_program |  | STOREW | <div style='text-align: right'>237,520</div>  |
| ecrecover_program |  | SUB | <div style='text-align: right'>8,502</div>  |
| ecrecover_program |  | XOR | <div style='text-align: right'>656</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>13,900,608</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary |  | ADD | <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>4,242,132</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>305,928</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>1,325,376</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | <div style='text-align: right'>144,128</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>125,760</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>2,191,936</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>1,775,904</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <div style='text-align: right'>784,892</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <div style='text-align: right'>126,975</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <div style='text-align: right'>104,140</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <div style='text-align: right'>172,618</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <div style='text-align: right'>693,411</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <div style='text-align: right'>63,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <div style='text-align: right'>52,316</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <div style='text-align: right'>86,768</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<16> |  | HINT_STOREW | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | HINT_STOREW | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>1,853</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>6,656</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <div style='text-align: right'>1,332,482</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <div style='text-align: right'>338,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <div style='text-align: right'>277,816</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <div style='text-align: right'>460,700</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <div style='text-align: right'>704</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>145,116</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>816,088</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <div style='text-align: right'>1,301,055</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>207,280</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | <div style='text-align: right'>125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <div style='text-align: right'>476</div>  |
| ecrecover_program | Boundary |  | LOADBU | <div style='text-align: right'>920</div>  |
| ecrecover_program | Merkle |  | LOADBU | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>8,440,360</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <div style='text-align: right'>323,525</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <div style='text-align: right'>265,188</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>503,115</div>  |
| ecrecover_program | Boundary |  | LOADW | <div style='text-align: right'>148,320</div>  |
| ecrecover_program | Merkle |  | LOADW | <div style='text-align: right'>197,056</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>119,610</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <div style='text-align: right'>40,021</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <div style='text-align: right'>127,957</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <div style='text-align: right'>64,300</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <div style='text-align: right'>52,726</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <div style='text-align: right'>87,482</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>1,652,004</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>7,950</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>1,916,162</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>1,885,298</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | <div style='text-align: right'>68,052</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>2,075,321</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>2,256,080</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <div style='text-align: right'>53,825</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <div style='text-align: right'>88,109</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>75,667</div>  |
| ecrecover_program | Boundary |  | STOREB | <div style='text-align: right'>91,600</div>  |
| ecrecover_program | Merkle |  | STOREB | <div style='text-align: right'>251,264</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>9,500,800</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <div style='text-align: right'>217,500</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <div style='text-align: right'>134,398</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>496,621</div>  |
| ecrecover_program | Boundary |  | STOREW | <div style='text-align: right'>386,760</div>  |
| ecrecover_program | Merkle |  | STOREW | <div style='text-align: right'>558,912</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>306,072</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>23,616</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>2,368</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <div style='text-align: right'>2,129,920</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>3,004,416</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>16</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>400,384</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>802,816</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>11,534,336</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>9,699,328</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>36,864</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>41,091,072</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: red">(+10.0 [+1.3%])</span> <div style='text-align: right'>792.0</div>  | <span style="color: green">(-18.0 [-0.3%])</span> <div style='text-align: right'>7,000.0</div>  | <div style='text-align: right'>303,080,679</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/55e2fd5120dcd5f051acbab6b37da409eb5c86a0/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/55e2fd5120dcd5f051acbab6b37da409eb5c86a0

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12244666122)
