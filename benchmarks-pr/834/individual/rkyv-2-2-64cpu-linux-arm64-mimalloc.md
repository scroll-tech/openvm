| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| rkyv_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  | <div style='text-align: right'>37,790.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| rkyv_program | true | <div style='text-align: right'>25,699.0</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| rkyv_program | ProgramChip | true | <div style='text-align: right'>2,987</div>  |
| rkyv_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| rkyv_program | Boundary | true | <div style='text-align: right'>134,168</div>  |
| rkyv_program | Merkle | true | <div style='text-align: right'>134,372</div>  |
| rkyv_program | AccessAdapter<8> | true | <div style='text-align: right'>134,168</div>  |
| rkyv_program | PhantomAir | true | <div style='text-align: right'>3</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>2,008,901</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>32,395</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>17,939</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>1,359,563</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>168,362</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>609,117</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>263,511</div>  |
| rkyv_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>158,120</div>  |
| rkyv_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>117,015</div>  |
| rkyv_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>58,510</div>  |
| rkyv_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>134,069</div>  |
| rkyv_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>268,540</div>  |
| rkyv_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| rkyv_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| rkyv_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| rkyv_program | true |  | ADD | <div style='text-align: right'>1,726,386</div>  |
| rkyv_program | true |  | AND | <div style='text-align: right'>157,703</div>  |
| rkyv_program | true |  | AUIPC | <div style='text-align: right'>58,510</div>  |
| rkyv_program | true |  | BEQ | <div style='text-align: right'>165,435</div>  |
| rkyv_program | true |  | BGEU | <div style='text-align: right'>73,808</div>  |
| rkyv_program | true |  | BLT | <div style='text-align: right'>168,362</div>  |
| rkyv_program | true |  | BLTU | <div style='text-align: right'>21,341</div>  |
| rkyv_program | true |  | BNE | <div style='text-align: right'>443,682</div>  |
| rkyv_program | true |  | HINT_STOREW | <div style='text-align: right'>134,069</div>  |
| rkyv_program | true |  | JAL | <div style='text-align: right'>101,603</div>  |
| rkyv_program | true |  | JALR | <div style='text-align: right'>117,015</div>  |
| rkyv_program | true |  | LOADB | <div style='text-align: right'>168,362</div>  |
| rkyv_program | true |  | LOADBU | <div style='text-align: right'>168,669</div>  |
| rkyv_program | true |  | LOADW | <div style='text-align: right'>583,578</div>  |
| rkyv_program | true |  | LUI | <div style='text-align: right'>56,517</div>  |
| rkyv_program | true |  | OR | <div style='text-align: right'>10,195</div>  |
| rkyv_program | true |  | PHANTOM | <div style='text-align: right'>3</div>  |
| rkyv_program | true |  | SLL | <div style='text-align: right'>9,970</div>  |
| rkyv_program | true |  | SLTU | <div style='text-align: right'>32,395</div>  |
| rkyv_program | true |  | SRL | <div style='text-align: right'>7,969</div>  |
| rkyv_program | true |  | STOREW | <div style='text-align: right'>607,316</div>  |
| rkyv_program | true |  | SUB | <div style='text-align: right'>114,617</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| rkyv_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>62,149,896</div>  |
| rkyv_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| rkyv_program | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| rkyv_program | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>5,677,308</div>  |
| rkyv_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>1,228,710</div>  |
| rkyv_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| rkyv_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| rkyv_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>4,301,310</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>2,361,856</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>5,387,584</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>682,912</div>  |
| rkyv_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>11,535,732</div>  |
| rkyv_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>3,485,794</div>  |
| rkyv_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>1,139,595</div>  |
| rkyv_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>2,681,400</div>  |
| rkyv_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>4,290,368</div>  |
| rkyv_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>1,828,854</div>  |
| rkyv_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>3,276,420</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>5,892,670</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>6,746,760</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>23,343,120</div>  |
| rkyv_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>68</div>  |
| rkyv_program | Boundary | true |  | LOADW | <div style='text-align: right'>160</div>  |
| rkyv_program | Merkle | true |  | LOADW | <div style='text-align: right'>2,944</div>  |
| rkyv_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>1,017,306</div>  |
| rkyv_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>51</div>  |
| rkyv_program | Boundary | true |  | LUI | <div style='text-align: right'>120</div>  |
| rkyv_program | Merkle | true |  | LUI | <div style='text-align: right'>64</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>367,020</div>  |
| rkyv_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>18</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>528,410</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>1,198,615</div>  |
| rkyv_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| rkyv_program | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| rkyv_program | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>422,357</div>  |
| rkyv_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>24,292,640</div>  |
| rkyv_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>595</div>  |
| rkyv_program | Boundary | true |  | STOREW | <div style='text-align: right'>1,400</div>  |
| rkyv_program | Merkle | true |  | STOREW | <div style='text-align: right'>2,880</div>  |
| rkyv_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>4,126,212</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| rkyv_program | <div style='text-align: right'>4.0</div>  | <div style='text-align: right'>10,268.0</div>  | <div style='text-align: right'>5,933.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>167.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  | <div style='text-align: right'>37,790.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| rkyv_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| rkyv_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| rkyv_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| rkyv_program | ProgramAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| rkyv_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| rkyv_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,631,488</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>10,747,904</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,523,136</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>32,768</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>3,440,640</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>32,768</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>29,097,984</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>77,594,624</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>46,137,344</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>16,252,928</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| rkyv_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| rkyv_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>16,252,928</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>328,728,576</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| rkyv_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| rkyv_program | 0 | <div style='text-align: right'>4,334.0</div>  | <div style='text-align: right'>23,188.0</div>  | <div style='text-align: right'>1,049,174,120</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/67bc0c0be453e2683f5f51f5991d951085ca85b8

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11906265034)
