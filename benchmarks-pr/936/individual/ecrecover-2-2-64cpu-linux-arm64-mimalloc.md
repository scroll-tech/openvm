| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-2,865,108 [-1.0%])</span> <div style='text-align: right'>284,010,873</div>  | <span style="color: green">(-69,693 [-1.3%])</span> <div style='text-align: right'>5,163,156</div>  | <span style="color: green">(-225.0 [-0.8%])</span> <div style='text-align: right'>26,467.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <span style="color: red">(+1.0 [+14.3%])</span> <div style='text-align: right'>8.0</div>  | <span style="color: green">(-98.0 [-1.3%])</span> <div style='text-align: right'>7,410.0</div>  | <span style="color: green">(-58.0 [-1.0%])</span> <div style='text-align: right'>5,725.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-1.0 [-0.4%])</span> <div style='text-align: right'>254.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-2,865,108 [-1.0%])</span> <div style='text-align: right'>284,010,873</div>  | <span style="color: green">(-69,693 [-1.3%])</span> <div style='text-align: right'>5,163,156</div>  | <span style="color: green">(-225.0 [-0.8%])</span> <div style='text-align: right'>26,467.0</div>  |

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
| ecrecover_program | ProgramChip | <span style="color: green">(-86 [-0.5%])</span> <div style='text-align: right'>16,061</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <span style="color: red">(+18 [+0.0%])</span> <div style='text-align: right'>63,454</div>  |
| ecrecover_program | Merkle | <span style="color: red">(+18 [+0.0%])</span> <div style='text-align: right'>63,808</div>  |
| ecrecover_program | AccessAdapter<2> | <div style='text-align: right'>650</div>  |
| ecrecover_program | AccessAdapter<4> | <div style='text-align: right'>364</div>  |
| ecrecover_program | AccessAdapter<8> | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>253,592</div>  |
| ecrecover_program | AccessAdapter<16> | <div style='text-align: right'>95,138</div>  |
| ecrecover_program | AccessAdapter<32> | <div style='text-align: right'>47,570</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>2,556</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <div style='text-align: right'>2,551</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <div style='text-align: right'>16,045</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <div style='text-align: right'>26</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <div style='text-align: right'>1,281</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>174</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | <div style='text-align: right'>184,740</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <div style='text-align: right'>195,166</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <span style="color: red">(+684 [+2.0%])</span> <div style='text-align: right'>34,855</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <span style="color: red">(+1,368 [+2.0%])</span> <div style='text-align: right'>69,720</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <span style="color: green">(-3,843 [-10.6%])</span> <div style='text-align: right'>32,470</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <span style="color: red">(+3,330 [+2.1%])</span> <div style='text-align: right'>162,420</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <span style="color: red">(+4,224 [+1.7%])</span> <div style='text-align: right'>257,923</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <span style="color: red">(+390 [+0.5%])</span> <div style='text-align: right'>74,687</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <span style="color: green">(-93,914 [-7.0%])</span> <div style='text-align: right'>1,252,740</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <span style="color: red">(+300 [+0.1%])</span> <div style='text-align: right'>516,041</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <span style="color: red">(+1,368 [+0.4%])</span> <div style='text-align: right'>324,183</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <span style="color: red">(+16,400 [+0.8%])</span> <div style='text-align: right'>2,032,880</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <div style='text-align: right'>2,675</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <span style="color: red">(+36 [+0.0%])</span> <div style='text-align: right'>127,262</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <span style="color: red">(+11,462 [+0.8%])</span> <div style='text-align: right'>1,496,606</div>  |
| ecrecover_program |  | AND | <span style="color: red">(+4,134 [+1.3%])</span> <div style='text-align: right'>314,883</div>  |
| ecrecover_program |  | AUIPC | <span style="color: red">(+684 [+2.0%])</span> <div style='text-align: right'>34,855</div>  |
| ecrecover_program |  | BEQ | <span style="color: red">(+1,372 [+1.3%])</span> <div style='text-align: right'>107,196</div>  |
| ecrecover_program |  | BGE | <div style='text-align: right'>9,005</div>  |
| ecrecover_program |  | BGEU | <span style="color: red">(+4 [+0.1%])</span> <div style='text-align: right'>5,265</div>  |
| ecrecover_program |  | BLT | <div style='text-align: right'>67</div>  |
| ecrecover_program |  | BLTU | <span style="color: red">(+3,326 [+2.3%])</span> <div style='text-align: right'>148,083</div>  |
| ecrecover_program |  | BNE | <span style="color: red">(+2,852 [+1.9%])</span> <div style='text-align: right'>150,727</div>  |
| ecrecover_program |  | EcAddNe | <div style='text-align: right'>2,551</div>  |
| ecrecover_program |  | EcDouble | <div style='text-align: right'>2,556</div>  |
| ecrecover_program |  | HINT_STOREW | <div style='text-align: right'>174</div>  |
| ecrecover_program |  | IS_EQ | <div style='text-align: right'>16,049</div>  |
| ecrecover_program |  | JAL | <span style="color: green">(-624 [-3.7%])</span> <div style='text-align: right'>16,025</div>  |
| ecrecover_program |  | JALR | <span style="color: red">(+1,368 [+2.0%])</span> <div style='text-align: right'>69,720</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <span style="color: red">(+390 [+0.5%])</span> <div style='text-align: right'>74,682</div>  |
| ecrecover_program |  | LOADBU | <div style='text-align: right'>13,230</div>  |
| ecrecover_program |  | LOADH | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADW | <span style="color: green">(-48,467 [-8.2%])</span> <div style='text-align: right'>540,414</div>  |
| ecrecover_program |  | LUI | <span style="color: green">(-3,219 [-16.4%])</span> <div style='text-align: right'>16,445</div>  |
| ecrecover_program |  | MUL | <div style='text-align: right'>195,166</div>  |
| ecrecover_program |  | MULHU | <div style='text-align: right'>184,740</div>  |
| ecrecover_program |  | ModularAddSub | <div style='text-align: right'>1,292</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <span style="color: red">(+804 [+0.4%])</span> <div style='text-align: right'>199,290</div>  |
| ecrecover_program |  | PHANTOM | <div style='text-align: right'>2,675</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <span style="color: red">(+120 [+0.0%])</span> <div style='text-align: right'>249,569</div>  |
| ecrecover_program |  | SLTU | <span style="color: red">(+1,368 [+0.4%])</span> <div style='text-align: right'>324,183</div>  |
| ecrecover_program |  | SRA | <div style='text-align: right'>2,562</div>  |
| ecrecover_program |  | SRL | <span style="color: red">(+180 [+0.1%])</span> <div style='text-align: right'>263,910</div>  |
| ecrecover_program |  | STOREB | <span style="color: red">(+480 [+0.4%])</span> <div style='text-align: right'>115,531</div>  |
| ecrecover_program |  | STOREH | <div style='text-align: right'>10</div>  |
| ecrecover_program |  | STOREW | <span style="color: green">(-45,927 [-7.3%])</span> <div style='text-align: right'>583,555</div>  |
| ecrecover_program |  | SUB | <div style='text-align: right'>17,421</div>  |
| ecrecover_program |  | XOR | <div style='text-align: right'>4,680</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <span style="color: red">(+412,632 [+0.8%])</span> <div style='text-align: right'>53,877,816</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary |  | ADD | <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <span style="color: red">(+148,824 [+1.3%])</span> <div style='text-align: right'>11,335,788</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <span style="color: red">(+14,364 [+2.0%])</span> <div style='text-align: right'>731,955</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <span style="color: red">(+35,672 [+1.3%])</span> <div style='text-align: right'>2,787,096</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | <div style='text-align: right'>288,160</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <span style="color: red">(+128 [+0.1%])</span> <div style='text-align: right'>168,480</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>2,144</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <span style="color: red">(+106,432 [+2.3%])</span> <div style='text-align: right'>4,738,656</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <span style="color: red">(+74,152 [+1.9%])</span> <div style='text-align: right'>3,918,902</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <div style='text-align: right'>1,579,069</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <div style='text-align: right'>255,275</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <div style='text-align: right'>209,346</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <div style='text-align: right'>347,106</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <div style='text-align: right'>1,387,908</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <div style='text-align: right'>127,750</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <div style='text-align: right'>104,755</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <div style='text-align: right'>173,740</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>4,524</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>1,513</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <span style="color: green">(-64 [-1.0%])</span> <div style='text-align: right'>6,208</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <div style='text-align: right'>2,664,134</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <div style='text-align: right'>675,250</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <div style='text-align: right'>553,705</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <div style='text-align: right'>918,272</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <span style="color: red">(+64 [+12.5%])</span> <div style='text-align: right'>576</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <span style="color: green">(-11,232 [-3.7%])</span> <div style='text-align: right'>288,450</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <span style="color: red">(+38,304 [+2.0%])</span> <div style='text-align: right'>1,952,160</div>  |
| ecrecover_program | AccessAdapter<2> |  | KECCAK256 | <div style='text-align: right'>3,575</div>  |
| ecrecover_program | AccessAdapter<4> |  | KECCAK256 | <div style='text-align: right'>2,145</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <span style="color: red">(+13,650 [+0.5%])</span> <div style='text-align: right'>2,613,870</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>529,200</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <div style='text-align: right'>697</div>  |
| ecrecover_program | Boundary |  | LOADBU | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | Merkle |  | LOADBU | <span style="color: green">(-192 [-7.1%])</span> <div style='text-align: right'>2,496</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADH | <div style='text-align: right'>175</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <span style="color: green">(-1,938,680 [-8.2%])</span> <div style='text-align: right'>21,616,560</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <div style='text-align: right'>643,350</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <div style='text-align: right'>527,547</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>997,628</div>  |
| ecrecover_program | Boundary |  | LOADW | <div style='text-align: right'>288,640</div>  |
| ecrecover_program | Merkle |  | LOADW | <span style="color: red">(+1,344 [+0.4%])</span> <div style='text-align: right'>381,312</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <span style="color: green">(-57,942 [-16.4%])</span> <div style='text-align: right'>296,010</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <div style='text-align: right'>6,050,146</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | <div style='text-align: right'>7,204,860</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <div style='text-align: right'>257,108</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <div style='text-align: right'>129,200</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <div style='text-align: right'>105,944</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <span style="color: red">(+153 [+0.1%])</span> <div style='text-align: right'>175,746</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <span style="color: red">(+40 [+5.9%])</span> <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <div style='text-align: right'>1,750</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <div style='text-align: right'>1,435</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <div style='text-align: right'>2,380</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <span style="color: red">(+28,944 [+0.4%])</span> <div style='text-align: right'>7,174,440</div>  |
| ecrecover_program | AccessAdapter<8> |  | OR | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | OR | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | OR | <div style='text-align: right'>64</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>16,050</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <span style="color: red">(+6,360 [+0.0%])</span> <div style='text-align: right'>13,227,157</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <span style="color: red">(+50,616 [+0.4%])</span> <div style='text-align: right'>11,994,771</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | <div style='text-align: right'>135,786</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <span style="color: red">(+9,540 [+0.1%])</span> <div style='text-align: right'>13,987,230</div>  |
| ecrecover_program | AccessAdapter<8> |  | SRL | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | SRL | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <span style="color: red">(+19,200 [+0.4%])</span> <div style='text-align: right'>4,621,240</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <span style="color: green">(-20,250 [-15.9%])</span> <div style='text-align: right'>107,175</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <span style="color: green">(-33,210 [-15.9%])</span> <div style='text-align: right'>175,767</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <span style="color: green">(-13,617 [-8.1%])</span> <div style='text-align: right'>154,207</div>  |
| ecrecover_program | Boundary |  | STOREB | <span style="color: red">(+360 [+0.2%])</span> <div style='text-align: right'>191,360</div>  |
| ecrecover_program | Merkle |  | STOREB | <span style="color: red">(+704 [+0.1%])</span> <div style='text-align: right'>515,712</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | <div style='text-align: right'>400</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <span style="color: green">(-1,837,080 [-7.3%])</span> <div style='text-align: right'>23,342,200</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <span style="color: red">(+20,250 [+4.8%])</span> <div style='text-align: right'>437,950</div>  |
| ecrecover_program | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>2,860</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <span style="color: red">(+33,210 [+14.0%])</span> <div style='text-align: right'>271,256</div>  |
| ecrecover_program | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>1,716</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <span style="color: red">(+13,753 [+1.4%])</span> <div style='text-align: right'>1,001,028</div>  |
| ecrecover_program | Boundary |  | STOREW | <span style="color: green">(-40 [-0.0%])</span> <div style='text-align: right'>782,480</div>  |
| ecrecover_program | Merkle |  | STOREW | <span style="color: green">(-1,280 [-0.1%])</span> <div style='text-align: right'>1,129,152</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>627,156</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>168,480</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
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
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>36,438,016</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>29,097,984</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <span style="color: green">(-2,031,616 [-50.0%])</span> <div style='text-align: right'>2,031,616</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>14,548,992</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>55,050,240</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>82,182,144</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: green">(-2.0 [-0.1%])</span> <div style='text-align: right'>1,923.0</div>  | <span style="color: green">(-125.0 [-0.7%])</span> <div style='text-align: right'>17,134.0</div>  | <span style="color: green">(-2,031,616 [-0.2%])</span> <div style='text-align: right'>843,704,289</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/bc82b6dc53fb40b51a891a2bac686e1c33a5d245/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/bc82b6dc53fb40b51a891a2bac686e1c33a5d245

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12150476122)
