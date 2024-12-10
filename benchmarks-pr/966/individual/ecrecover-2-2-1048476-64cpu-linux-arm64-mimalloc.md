| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: red">(+89,972,147 [+851.4%])</span> <div style='text-align: right'>100,539,955</div>  | <span style="color: red">(+1,396,127 [+1311.6%])</span> <div style='text-align: right'>1,502,571</div>  | <span style="color: red">(+1,173.0 [+50.6%])</span> <div style='text-align: right'>3,491.0</div>  |
| app_proof | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>7,805.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>7.0</div>  | <span style="color: red">(+2,317.0 [+658.2%])</span> <div style='text-align: right'>2,669.0</div>  | <span style="color: red">(+9,694.0 [+3877.6%])</span> <div style='text-align: right'>9,944.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+75,958.0 [+28991.6%])</span> <div style='text-align: right'>76,220.0</div>  |  | <span style="color: red">(+89,972,147 [+851.4%])</span> <div style='text-align: right'>100,539,955</div>  | <span style="color: red">(+1,396,127 [+1311.6%])</span> <div style='text-align: right'>1,502,571</div>  | <span style="color: red">(+1,173.0 [+50.6%])</span> <div style='text-align: right'>3,491.0</div>  |
| app_proof |  |  | <div style='text-align: right'>1,842.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>1</div>  |  |  | <div style='text-align: right'>7,805.0</div>  |

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
| ecrecover_program | ProgramChip | <span style="color: red">(+367 [+4.6%])</span> <div style='text-align: right'>8,431</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <span style="color: red">(+28,676 [+973.4%])</span> <div style='text-align: right'>31,622</div>  |
| ecrecover_program | Merkle | <span style="color: red">(+28,672 [+878.4%])</span> <div style='text-align: right'>31,936</div>  |
| ecrecover_program | AccessAdapter<4> | <div style='text-align: right'>34</div>  |
| ecrecover_program | AccessAdapter<8> | <span style="color: red">(+120,508 [+1893.6%])</span> <div style='text-align: right'>126,872</div>  |
| ecrecover_program | AccessAdapter<16> | <span style="color: red">(+44,792 [+1556.4%])</span> <div style='text-align: right'>47,670</div>  |
| ecrecover_program | AccessAdapter<32> | <span style="color: red">(+22,396 [+1555.3%])</span> <div style='text-align: right'>23,836</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: red">(+6 [+0.5%])</span> <div style='text-align: right'>1,277</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: red">(+542 [+74.7%])</span> <div style='text-align: right'>1,268</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <span style="color: red">(+4,824 [+151.0%])</span> <div style='text-align: right'>8,018</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <span style="color: red">(+636 [+10600.0%])</span> <div style='text-align: right'>642</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <span style="color: red">(+1,280 [+11636.4%])</span> <div style='text-align: right'>1,291</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <span style="color: red">(+13,754 [+1689.7%])</span> <div style='text-align: right'>14,568</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <span style="color: red">(+27,638 [+1832.8%])</span> <div style='text-align: right'>29,146</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <span style="color: red">(+12,963 [+743.3%])</span> <div style='text-align: right'>14,707</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <span style="color: red">(+71,724 [+1374.0%])</span> <div style='text-align: right'>76,944</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <span style="color: red">(+108,090 [+966.0%])</span> <div style='text-align: right'>119,280</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <span style="color: red">(+33,168 [+828.2%])</span> <div style='text-align: right'>37,173</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <span style="color: red">(+481,665 [+1693.1%])</span> <div style='text-align: right'>510,113</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <span style="color: red">(+68,064 [+797.8%])</span> <div style='text-align: right'>76,595</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <span style="color: red">(+48,928 [+2415.0%])</span> <div style='text-align: right'>50,954</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <span style="color: red">(+521,565 [+1392.8%])</span> <div style='text-align: right'>559,012</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <span style="color: red">(+1,280 [+2844.4%])</span> <div style='text-align: right'>1,325</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <span style="color: red">(+57,348 [+923.5%])</span> <div style='text-align: right'>63,558</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <span style="color: red">(+358,624 [+1303.9%])</span> <div style='text-align: right'>386,128</div>  |
| ecrecover_program |  | AND | <span style="color: red">(+112,837 [+2256.7%])</span> <div style='text-align: right'>117,837</div>  |
| ecrecover_program |  | AUIPC | <span style="color: red">(+13,754 [+1689.7%])</span> <div style='text-align: right'>14,568</div>  |
| ecrecover_program |  | BEQ | <span style="color: red">(+44,675 [+709.0%])</span> <div style='text-align: right'>50,976</div>  |
| ecrecover_program |  | BGE | <div style='text-align: right'>4,504</div>  |
| ecrecover_program |  | BGEU | <span style="color: red">(+3,325 [+549.6%])</span> <div style='text-align: right'>3,930</div>  |
| ecrecover_program |  | BLT | <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | <span style="color: red">(+63,895 [+1388.1%])</span> <div style='text-align: right'>68,498</div>  |
| ecrecover_program |  | BNE | <span style="color: red">(+63,415 [+1297.1%])</span> <div style='text-align: right'>68,304</div>  |
| ecrecover_program |  | EcAddNe | <span style="color: red">(+542 [+74.7%])</span> <div style='text-align: right'>1,268</div>  |
| ecrecover_program |  | EcDouble | <span style="color: red">(+6 [+0.5%])</span> <div style='text-align: right'>1,277</div>  |
| ecrecover_program |  | HINT_STOREW | <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | <span style="color: red">(+4,824 [+150.6%])</span> <div style='text-align: right'>8,027</div>  |
| ecrecover_program |  | JAL | <span style="color: red">(+6,583 [+445.1%])</span> <div style='text-align: right'>8,062</div>  |
| ecrecover_program |  | JALR | <span style="color: red">(+27,638 [+1832.8%])</span> <div style='text-align: right'>29,146</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <span style="color: red">(+33,168 [+828.2%])</span> <div style='text-align: right'>37,173</div>  |
| ecrecover_program |  | LOADBU | <span style="color: red">(+2,892 [+126.3%])</span> <div style='text-align: right'>5,182</div>  |
| ecrecover_program |  | LOADW | <span style="color: red">(+200,649 [+1936.8%])</span> <div style='text-align: right'>211,009</div>  |
| ecrecover_program |  | LUI | <span style="color: red">(+6,380 [+2407.5%])</span> <div style='text-align: right'>6,645</div>  |
| ecrecover_program |  | MUL | <span style="color: red">(+1,280 [+11636.4%])</span> <div style='text-align: right'>1,291</div>  |
| ecrecover_program |  | ModularAddSub | <span style="color: red">(+636 [+9085.7%])</span> <div style='text-align: right'>643</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <span style="color: red">(+41,852 [+1036.7%])</span> <div style='text-align: right'>45,889</div>  |
| ecrecover_program |  | PHANTOM | <span style="color: red">(+1,280 [+2844.4%])</span> <div style='text-align: right'>1,325</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <span style="color: red">(+32,054 [+781.8%])</span> <div style='text-align: right'>36,154</div>  |
| ecrecover_program |  | SLTU | <span style="color: red">(+48,928 [+2415.0%])</span> <div style='text-align: right'>50,954</div>  |
| ecrecover_program |  | SRA | <div style='text-align: right'>1,284</div>  |
| ecrecover_program |  | SRL | <span style="color: red">(+34,726 [+783.7%])</span> <div style='text-align: right'>39,157</div>  |
| ecrecover_program |  | STOREB | <span style="color: red">(+50,563 [+866.0%])</span> <div style='text-align: right'>56,402</div>  |
| ecrecover_program |  | STOREW | <span style="color: red">(+227,561 [+2285.0%])</span> <div style='text-align: right'>237,520</div>  |
| ecrecover_program |  | SUB | <span style="color: red">(+7,616 [+859.6%])</span> <div style='text-align: right'>8,502</div>  |
| ecrecover_program |  | XOR | <span style="color: red">(+636 [+3180.0%])</span> <div style='text-align: right'>656</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <span style="color: red">(+12,910,464 [+1303.9%])</span> <div style='text-align: right'>13,900,608</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <span style="color: green">(-17 [-25.0%])</span> <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary |  | ADD | <span style="color: green">(-40 [-25.0%])</span> <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <span style="color: red">(+4,062,132 [+2256.7%])</span> <div style='text-align: right'>4,242,132</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <span style="color: red">(+288,834 [+1689.7%])</span> <div style='text-align: right'>305,928</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <span style="color: red">(+1,161,550 [+709.0%])</span> <div style='text-align: right'>1,325,376</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | <div style='text-align: right'>144,128</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <span style="color: red">(+106,400 [+549.6%])</span> <div style='text-align: right'>125,760</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <span style="color: red">(+2,044,640 [+1388.1%])</span> <div style='text-align: right'>2,191,936</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <span style="color: red">(+1,648,790 [+1297.1%])</span> <div style='text-align: right'>1,775,904</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <span style="color: red">(+335,498 [+74.7%])</span> <div style='text-align: right'>784,892</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <span style="color: red">(+99,800 [+367.2%])</span> <div style='text-align: right'>126,975</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <span style="color: red">(+81,836 [+366.9%])</span> <div style='text-align: right'>104,140</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <span style="color: red">(+135,728 [+367.9%])</span> <div style='text-align: right'>172,618</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <span style="color: red">(+3,258 [+0.5%])</span> <div style='text-align: right'>693,411</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <span style="color: red">(+62,200 [+3887.5%])</span> <div style='text-align: right'>63,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <span style="color: red">(+51,004 [+3887.5%])</span> <div style='text-align: right'>52,316</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <span style="color: red">(+84,660 [+4016.1%])</span> <div style='text-align: right'>86,768</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<16> |  | HINT_STOREW | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | HINT_STOREW | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>1,853</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <span style="color: red">(+640 [+10.6%])</span> <div style='text-align: right'>6,656</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <span style="color: red">(+800,784 [+150.6%])</span> <div style='text-align: right'>1,332,482</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <span style="color: red">(+334,300 [+7428.9%])</span> <div style='text-align: right'>338,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <span style="color: red">(+274,126 [+7428.9%])</span> <div style='text-align: right'>277,816</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <span style="color: red">(+454,648 [+7512.4%])</span> <div style='text-align: right'>460,700</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <span style="color: red">(+256 [+57.1%])</span> <div style='text-align: right'>704</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <span style="color: red">(+118,494 [+445.1%])</span> <div style='text-align: right'>145,116</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <span style="color: red">(+773,864 [+1832.8%])</span> <div style='text-align: right'>816,088</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <span style="color: red">(+1,160,880 [+828.2%])</span> <div style='text-align: right'>1,301,055</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <span style="color: red">(+115,680 [+126.3%])</span> <div style='text-align: right'>207,280</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | <span style="color: green">(-500 [-80.0%])</span> <div style='text-align: right'>125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | <span style="color: green">(-410 [-66.7%])</span> <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <span style="color: green">(-323 [-40.4%])</span> <div style='text-align: right'>476</div>  |
| ecrecover_program | Boundary |  | LOADBU | <span style="color: red">(+840 [+1050.0%])</span> <div style='text-align: right'>920</div>  |
| ecrecover_program | Merkle |  | LOADBU | <span style="color: red">(+960 [+1500.0%])</span> <div style='text-align: right'>1,024</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <span style="color: red">(+8,025,960 [+1936.8%])</span> <div style='text-align: right'>8,440,360</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <span style="color: red">(+306,400 [+1789.2%])</span> <div style='text-align: right'>323,525</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <span style="color: red">(+251,248 [+1802.4%])</span> <div style='text-align: right'>265,188</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <span style="color: red">(+479,536 [+2033.7%])</span> <div style='text-align: right'>503,115</div>  |
| ecrecover_program | Boundary |  | LOADW | <span style="color: red">(+147,840 [+30800.0%])</span> <div style='text-align: right'>148,320</div>  |
| ecrecover_program | Merkle |  | LOADW | <span style="color: red">(+193,152 [+4947.5%])</span> <div style='text-align: right'>197,056</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <span style="color: red">(+114,840 [+2407.5%])</span> <div style='text-align: right'>119,610</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <span style="color: red">(+39,680 [+11636.4%])</span> <div style='text-align: right'>40,021</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <span style="color: red">(+126,564 [+9085.7%])</span> <div style='text-align: right'>127,957</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <span style="color: red">(+63,600 [+9085.7%])</span> <div style='text-align: right'>64,300</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <span style="color: red">(+52,152 [+9085.7%])</span> <div style='text-align: right'>52,726</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <span style="color: red">(+86,496 [+8772.4%])</span> <div style='text-align: right'>87,482</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <span style="color: red">(+1,506,672 [+1036.7%])</span> <div style='text-align: right'>1,652,004</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <span style="color: red">(+7,680 [+2844.4%])</span> <div style='text-align: right'>7,950</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <span style="color: red">(+1,698,862 [+781.8%])</span> <div style='text-align: right'>1,916,162</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <span style="color: red">(+1,810,336 [+2415.0%])</span> <div style='text-align: right'>1,885,298</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | <div style='text-align: right'>68,052</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <span style="color: red">(+1,840,478 [+783.7%])</span> <div style='text-align: right'>2,075,321</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <span style="color: red">(+2,022,520 [+866.0%])</span> <div style='text-align: right'>2,256,080</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <span style="color: red">(+52,900 [+5718.9%])</span> <div style='text-align: right'>53,825</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <span style="color: red">(+87,330 [+11210.5%])</span> <div style='text-align: right'>88,109</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <span style="color: red">(+73,627 [+3609.2%])</span> <div style='text-align: right'>75,667</div>  |
| ecrecover_program | Boundary |  | STOREB | <span style="color: red">(+89,560 [+4390.2%])</span> <div style='text-align: right'>91,600</div>  |
| ecrecover_program | Merkle |  | STOREB | <span style="color: red">(+246,336 [+4998.7%])</span> <div style='text-align: right'>251,264</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <span style="color: red">(+9,102,440 [+2285.0%])</span> <div style='text-align: right'>9,500,800</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <span style="color: red">(+215,350 [+10016.3%])</span> <div style='text-align: right'>217,500</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <span style="color: red">(+132,635 [+7523.3%])</span> <div style='text-align: right'>134,398</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <span style="color: red">(+471,835 [+1903.6%])</span> <div style='text-align: right'>496,621</div>  |
| ecrecover_program | Boundary |  | STOREW | <span style="color: red">(+335,480 [+654.2%])</span> <div style='text-align: right'>386,760</div>  |
| ecrecover_program | Merkle |  | STOREW | <span style="color: red">(+476,352 [+577.0%])</span> <div style='text-align: right'>558,912</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <span style="color: red">(+274,176 [+859.6%])</span> <div style='text-align: right'>306,072</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <span style="color: red">(+22,896 [+3180.0%])</span> <div style='text-align: right'>23,616</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>2,368</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | AccessAdapterAir<16> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | AccessAdapterAir<32> | 0 | <div style='text-align: right'>2,129,920</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>3,004,416</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>400,384</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>802,816</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>11,534,336</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>9,699,328</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>36,864</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>41,091,072</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| app_proof | 0 | <div style='text-align: right'>787.0</div>  | <div style='text-align: right'>7,018.0</div>  | <div style='text-align: right'>303,080,679</div>  |
| ecrecover_program | 0 | <span style="color: red">(+723.0 [+730.3%])</span> <div style='text-align: right'>822.0</div>  |  |  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/44c469030ecc074a5dd998b90d586639c4767436/ecrecover-2-2-1048476-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/44c469030ecc074a5dd998b90d586639c4767436

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12246948915)
