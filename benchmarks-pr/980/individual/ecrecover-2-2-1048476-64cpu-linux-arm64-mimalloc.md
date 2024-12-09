| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-90,021,582 [-89.5%])</span> <div style='text-align: right'>10,518,373</div>  | <span style="color: green">(-1,397,331 [-93.0%])</span> <div style='text-align: right'>105,240</div>  | <span style="color: green">(-8,192.0 [-78.2%])</span> <div style='text-align: right'>2,286.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <span style="color: green">(-2.0 [-28.6%])</span> <div style='text-align: right'>5.0</div>  | <span style="color: green">(-2,273.0 [-86.8%])</span> <div style='text-align: right'>347.0</div>  | <span style="color: green">(-1,625.0 [-86.9%])</span> <div style='text-align: right'>245.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>255.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-90,021,582 [-89.5%])</span> <div style='text-align: right'>10,518,373</div>  | <span style="color: green">(-1,397,331 [-93.0%])</span> <div style='text-align: right'>105,240</div>  | <span style="color: green">(-8,192.0 [-78.2%])</span> <div style='text-align: right'>2,286.0</div>  |

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
| ecrecover_program | ProgramChip | <span style="color: green">(-448 [-5.3%])</span> <div style='text-align: right'>7,983</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <span style="color: green">(-28,680 [-90.7%])</span> <div style='text-align: right'>2,942</div>  |
| ecrecover_program | Merkle | <span style="color: green">(-28,682 [-89.8%])</span> <div style='text-align: right'>3,254</div>  |
| ecrecover_program | AccessAdapter<4> | <div style='text-align: right'>34</div>  |
| ecrecover_program | AccessAdapter<8> | <span style="color: green">(-120,512 [-95.0%])</span> <div style='text-align: right'>6,360</div>  |
| ecrecover_program | AccessAdapter<16> | <span style="color: green">(-44,792 [-94.0%])</span> <div style='text-align: right'>2,878</div>  |
| ecrecover_program | AccessAdapter<32> | <span style="color: green">(-22,396 [-94.0%])</span> <div style='text-align: right'>1,440</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: green">(-6 [-0.5%])</span> <div style='text-align: right'>1,271</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: green">(-542 [-42.7%])</span> <div style='text-align: right'>726</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <span style="color: green">(-4,824 [-60.2%])</span> <div style='text-align: right'>3,194</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <span style="color: green">(-636 [-99.1%])</span> <div style='text-align: right'>6</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <span style="color: green">(-1,280 [-99.1%])</span> <div style='text-align: right'>11</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <span style="color: green">(-13,764 [-94.5%])</span> <div style='text-align: right'>804</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <span style="color: green">(-27,653 [-94.9%])</span> <div style='text-align: right'>1,493</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <span style="color: green">(-12,963 [-88.1%])</span> <div style='text-align: right'>1,744</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <span style="color: green">(-71,724 [-93.2%])</span> <div style='text-align: right'>5,220</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <span style="color: green">(-108,425 [-90.9%])</span> <div style='text-align: right'>10,855</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <span style="color: green">(-33,168 [-89.2%])</span> <div style='text-align: right'>4,005</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <span style="color: green">(-481,999 [-94.5%])</span> <div style='text-align: right'>28,114</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <span style="color: green">(-68,064 [-88.9%])</span> <div style='text-align: right'>8,531</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <span style="color: green">(-48,928 [-96.0%])</span> <div style='text-align: right'>2,026</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <span style="color: green">(-522,075 [-93.4%])</span> <div style='text-align: right'>36,937</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <span style="color: green">(-1,280 [-96.6%])</span> <div style='text-align: right'>45</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <span style="color: green">(-57,362 [-90.3%])</span> <div style='text-align: right'>6,196</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <span style="color: green">(-359,134 [-93.0%])</span> <div style='text-align: right'>26,994</div>  |
| ecrecover_program |  | AND | <span style="color: green">(-112,837 [-95.8%])</span> <div style='text-align: right'>5,000</div>  |
| ecrecover_program |  | AUIPC | <span style="color: green">(-13,764 [-94.5%])</span> <div style='text-align: right'>804</div>  |
| ecrecover_program |  | BEQ | <span style="color: green">(-44,680 [-87.6%])</span> <div style='text-align: right'>6,296</div>  |
| ecrecover_program |  | BGEU | <span style="color: green">(-3,325 [-84.6%])</span> <div style='text-align: right'>605</div>  |
| ecrecover_program |  | BLT | <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | <span style="color: green">(-63,895 [-93.3%])</span> <div style='text-align: right'>4,603</div>  |
| ecrecover_program |  | BNE | <span style="color: green">(-63,745 [-93.3%])</span> <div style='text-align: right'>4,559</div>  |
| ecrecover_program |  | EcAddNe | <span style="color: green">(-542 [-42.7%])</span> <div style='text-align: right'>726</div>  |
| ecrecover_program |  | EcDouble | <span style="color: green">(-6 [-0.5%])</span> <div style='text-align: right'>1,271</div>  |
| ecrecover_program |  | HINT_STOREW | <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | <span style="color: green">(-4,824 [-60.1%])</span> <div style='text-align: right'>3,203</div>  |
| ecrecover_program |  | JAL | <span style="color: green">(-6,583 [-81.7%])</span> <div style='text-align: right'>1,479</div>  |
| ecrecover_program |  | JALR | <span style="color: green">(-27,653 [-94.9%])</span> <div style='text-align: right'>1,493</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <span style="color: green">(-33,168 [-89.2%])</span> <div style='text-align: right'>4,005</div>  |
| ecrecover_program |  | LOADBU | <span style="color: green">(-3,212 [-62.0%])</span> <div style='text-align: right'>1,970</div>  |
| ecrecover_program |  | LOADW | <span style="color: green">(-200,652 [-95.1%])</span> <div style='text-align: right'>10,357</div>  |
| ecrecover_program |  | LUI | <span style="color: green">(-6,380 [-96.0%])</span> <div style='text-align: right'>265</div>  |
| ecrecover_program |  | MUL | <span style="color: green">(-1,280 [-99.1%])</span> <div style='text-align: right'>11</div>  |
| ecrecover_program |  | ModularAddSub | <span style="color: green">(-636 [-98.9%])</span> <div style='text-align: right'>7</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <span style="color: green">(-41,852 [-91.2%])</span> <div style='text-align: right'>4,037</div>  |
| ecrecover_program |  | PHANTOM | <span style="color: green">(-1,280 [-96.6%])</span> <div style='text-align: right'>45</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <span style="color: green">(-32,054 [-88.7%])</span> <div style='text-align: right'>4,100</div>  |
| ecrecover_program |  | SLTU | <span style="color: green">(-48,928 [-96.0%])</span> <div style='text-align: right'>2,026</div>  |
| ecrecover_program |  | SRL | <span style="color: green">(-34,726 [-88.7%])</span> <div style='text-align: right'>4,431</div>  |
| ecrecover_program |  | STOREB | <span style="color: green">(-50,563 [-89.6%])</span> <div style='text-align: right'>5,839</div>  |
| ecrecover_program |  | STOREW | <span style="color: green">(-227,572 [-95.8%])</span> <div style='text-align: right'>9,948</div>  |
| ecrecover_program |  | SUB | <span style="color: green">(-7,616 [-89.6%])</span> <div style='text-align: right'>886</div>  |
| ecrecover_program |  | XOR | <span style="color: green">(-636 [-97.0%])</span> <div style='text-align: right'>20</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <span style="color: green">(-12,928,824 [-93.0%])</span> <div style='text-align: right'>971,784</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <span style="color: red">(+17 [+33.3%])</span> <div style='text-align: right'>68</div>  |
| ecrecover_program | Boundary |  | ADD | <span style="color: red">(+40 [+33.3%])</span> <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <span style="color: green">(-4,062,132 [-95.8%])</span> <div style='text-align: right'>180,000</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <span style="color: green">(-289,044 [-94.5%])</span> <div style='text-align: right'>16,884</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <span style="color: green">(-1,161,680 [-87.6%])</span> <div style='text-align: right'>163,696</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <span style="color: green">(-106,400 [-84.6%])</span> <div style='text-align: right'>19,360</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <span style="color: green">(-2,044,640 [-93.3%])</span> <div style='text-align: right'>147,296</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <span style="color: green">(-1,657,370 [-93.3%])</span> <div style='text-align: right'>118,534</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <span style="color: green">(-335,498 [-42.7%])</span> <div style='text-align: right'>449,394</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <span style="color: green">(-99,800 [-78.6%])</span> <div style='text-align: right'>27,175</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <span style="color: green">(-81,836 [-78.6%])</span> <div style='text-align: right'>22,304</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <span style="color: green">(-135,728 [-78.6%])</span> <div style='text-align: right'>36,890</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <span style="color: green">(-3,258 [-0.5%])</span> <div style='text-align: right'>690,153</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <span style="color: green">(-62,200 [-97.5%])</span> <div style='text-align: right'>1,600</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <span style="color: green">(-51,004 [-97.5%])</span> <div style='text-align: right'>1,312</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <span style="color: green">(-84,660 [-97.6%])</span> <div style='text-align: right'>2,108</div>  |
| ecrecover_program | Boundary |  | EcDouble | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcDouble | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<16> |  | HINT_STOREW | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | HINT_STOREW | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>1,853</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>6,656</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <span style="color: green">(-800,784 [-60.1%])</span> <div style='text-align: right'>531,698</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <span style="color: green">(-334,300 [-98.7%])</span> <div style='text-align: right'>4,500</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <span style="color: green">(-274,126 [-98.7%])</span> <div style='text-align: right'>3,690</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <span style="color: green">(-454,648 [-98.7%])</span> <div style='text-align: right'>6,052</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <span style="color: green">(-128 [-18.2%])</span> <div style='text-align: right'>576</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <span style="color: green">(-118,494 [-81.7%])</span> <div style='text-align: right'>26,622</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <span style="color: green">(-774,284 [-94.9%])</span> <div style='text-align: right'>41,804</div>  |
| ecrecover_program | AccessAdapter<16> |  | KECCAK256 | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | KECCAK256 | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | KECCAK256 | <div style='text-align: right'>340</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <span style="color: green">(-1,160,880 [-89.2%])</span> <div style='text-align: right'>140,175</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <span style="color: green">(-128,480 [-62.0%])</span> <div style='text-align: right'>78,800</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | <span style="color: red">(+500 [+400.0%])</span> <div style='text-align: right'>625</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | <span style="color: red">(+410 [+200.0%])</span> <div style='text-align: right'>615</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <span style="color: red">(+323 [+67.9%])</span> <div style='text-align: right'>799</div>  |
| ecrecover_program | Boundary |  | LOADBU | <span style="color: green">(-840 [-91.3%])</span> <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | LOADBU | <span style="color: green">(-960 [-93.8%])</span> <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <span style="color: green">(-8,026,080 [-95.1%])</span> <div style='text-align: right'>414,280</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <span style="color: green">(-306,400 [-94.7%])</span> <div style='text-align: right'>17,125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <span style="color: green">(-251,248 [-94.7%])</span> <div style='text-align: right'>13,940</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <span style="color: green">(-479,536 [-95.3%])</span> <div style='text-align: right'>23,579</div>  |
| ecrecover_program | Boundary |  | LOADW | <span style="color: green">(-147,840 [-99.7%])</span> <div style='text-align: right'>480</div>  |
| ecrecover_program | Merkle |  | LOADW | <span style="color: green">(-193,600 [-98.2%])</span> <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <span style="color: green">(-114,840 [-96.0%])</span> <div style='text-align: right'>4,770</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <span style="color: green">(-39,680 [-99.1%])</span> <div style='text-align: right'>341</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <span style="color: green">(-126,564 [-98.9%])</span> <div style='text-align: right'>1,393</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <span style="color: green">(-63,600 [-98.9%])</span> <div style='text-align: right'>700</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <span style="color: green">(-52,152 [-98.9%])</span> <div style='text-align: right'>574</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <span style="color: green">(-86,496 [-98.9%])</span> <div style='text-align: right'>986</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <span style="color: red">(+384 [+15.0%])</span> <div style='text-align: right'>2,944</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <span style="color: green">(-1,506,672 [-91.2%])</span> <div style='text-align: right'>145,332</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <span style="color: green">(-7,680 [-96.6%])</span> <div style='text-align: right'>270</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <span style="color: green">(-1,698,862 [-88.7%])</span> <div style='text-align: right'>217,300</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <span style="color: green">(-1,810,336 [-96.0%])</span> <div style='text-align: right'>74,962</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <span style="color: green">(-1,840,478 [-88.7%])</span> <div style='text-align: right'>234,843</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <span style="color: green">(-2,022,520 [-89.6%])</span> <div style='text-align: right'>233,560</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <span style="color: green">(-52,900 [-98.3%])</span> <div style='text-align: right'>925</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <span style="color: green">(-87,330 [-99.1%])</span> <div style='text-align: right'>779</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <span style="color: green">(-73,627 [-97.3%])</span> <div style='text-align: right'>2,040</div>  |
| ecrecover_program | Boundary |  | STOREB | <span style="color: green">(-89,560 [-97.8%])</span> <div style='text-align: right'>2,040</div>  |
| ecrecover_program | Merkle |  | STOREB | <span style="color: green">(-246,848 [-98.2%])</span> <div style='text-align: right'>4,416</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <span style="color: green">(-9,102,880 [-95.8%])</span> <div style='text-align: right'>397,920</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <span style="color: green">(-215,350 [-99.0%])</span> <div style='text-align: right'>2,150</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <span style="color: green">(-132,635 [-98.7%])</span> <div style='text-align: right'>1,763</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <span style="color: green">(-471,869 [-95.0%])</span> <div style='text-align: right'>24,752</div>  |
| ecrecover_program | Boundary |  | STOREW | <span style="color: green">(-335,560 [-86.8%])</span> <div style='text-align: right'>51,200</div>  |
| ecrecover_program | Merkle |  | STOREW | <span style="color: green">(-476,864 [-85.3%])</span> <div style='text-align: right'>82,048</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <span style="color: green">(-274,176 [-89.6%])</span> <div style='text-align: right'>31,896</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <span style="color: green">(-22,896 [-97.0%])</span> <div style='text-align: right'>720</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <span style="color: green">(-147,456 [-50.0%])</span> <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-8,192 [-50.0%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <span style="color: green">(-917,504 [-87.5%])</span> <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-28,672 [-87.5%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <span style="color: green">(-1,490,944 [-87.5%])</span> <div style='text-align: right'>212,992</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <span style="color: green">(-28,672 [-87.5%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>2,368</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <span style="color: green">(-5,038,080 [-93.8%])</span> <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-122,880 [-93.8%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <span style="color: green">(-3,010,560 [-93.8%])</span> <div style='text-align: right'>200,704</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-61,440 [-93.8%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <span style="color: green">(-1,996,800 [-93.8%])</span> <div style='text-align: right'>133,120</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-30,720 [-93.8%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <span style="color: green">(-1,502,208 [-50.0%])</span> <div style='text-align: right'>1,502,208</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <span style="color: green">(-1,024 [-50.0%])</span> <div style='text-align: right'>1,024</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <span style="color: green">(-909,312 [-50.0%])</span> <div style='text-align: right'>909,312</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: green">(-4,096 [-50.0%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>16</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <span style="color: green">(-397,256 [-99.2%])</span> <div style='text-align: right'>3,128</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <span style="color: green">(-1,016 [-99.2%])</span> <div style='text-align: right'>8</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <span style="color: green">(-225,552 [-99.2%])</span> <div style='text-align: right'>1,776</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-2,032 [-99.2%])</span> <div style='text-align: right'>16</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <span style="color: green">(-752,640 [-93.8%])</span> <div style='text-align: right'>50,176</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <span style="color: green">(-15,360 [-93.8%])</span> <div style='text-align: right'>1,024</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: green">(-1,966,080 [-93.8%])</span> <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <span style="color: green">(-30,720 [-93.8%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <span style="color: green">(-888,832 [-87.5%])</span> <div style='text-align: right'>126,976</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <span style="color: green">(-14,336 [-87.5%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <span style="color: green">(-10,813,440 [-93.8%])</span> <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: green">(-122,880 [-93.8%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <span style="color: green">(-8,486,912 [-87.5%])</span> <div style='text-align: right'>1,212,416</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <span style="color: green">(-114,688 [-87.5%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <span style="color: green">(-6,819,840 [-93.8%])</span> <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <span style="color: green">(-61,440 [-93.8%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: green">(-55,050,240 [-93.8%])</span> <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <span style="color: green">(-491,520 [-93.8%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-12,042,240 [-87.5%])</span> <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: green">(-114,688 [-87.5%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <span style="color: green">(-4,888,576 [-96.9%])</span> <div style='text-align: right'>157,696</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <span style="color: green">(-63,488 [-96.9%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <span style="color: green">(-114,032,640 [-93.8%])</span> <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-983,040 [-93.8%])</span> <div style='text-align: right'>65,536</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <span style="color: green">(-35,712 [-96.9%])</span> <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-1,984 [-96.9%])</span> <div style='text-align: right'>64</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-35,954,688 [-87.5%])</span> <div style='text-align: right'>5,136,384</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <span style="color: green">(-57,344 [-87.5%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: green">(-715.0 [-87.5%])</span> <div style='text-align: right'>102.0</div>  | <span style="color: green">(-5,204.0 [-73.9%])</span> <div style='text-align: right'>1,837.0</div>  | <span style="color: green">(-267,367,512 [-88.2%])</span> <div style='text-align: right'>35,713,167</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a84d0cd744e53268750eabc45a6748ceb440f08b/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/a84d0cd744e53268750eabc45a6748ceb440f08b

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12245856400)
