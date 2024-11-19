| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| rkyv_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  | <div style='text-align: right'>37,753.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <div style='text-align: right'>155,938,962</div>  | <div style='text-align: right'>3,869,913</div>  | <div style='text-align: right'>20,081.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| rkyv_program | true | <div style='text-align: right'>25,499.0</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  |

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
| rkyv_program | <div style='text-align: right'>3.0</div>  | <div style='text-align: right'>10,179.0</div>  | <div style='text-align: right'>5,892.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>163.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>330,018,911</div>  | <div style='text-align: right'>4,927,505</div>  | <div style='text-align: right'>37,753.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>155,938,962</div>  | <div style='text-align: right'>3,869,913</div>  | <div style='text-align: right'>20,081.0</div>  |

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

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| rkyv_program | ProgramAir | 0 | <div style='text-align: right'>73,728</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>4,096</div>  |
| rkyv_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| rkyv_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>8,388,608</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,631,488</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>10,747,904</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>4</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,523,136</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| rkyv_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>3,440,640</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| rkyv_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>29,097,984</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>77,594,624</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| rkyv_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>46,137,344</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>16,252,928</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| rkyv_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| rkyv_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>16,252,928</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| rkyv_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>328,728,576</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| rkyv_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| rkyv_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | FriMatOpeningAir | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| rkyv_program | 0 |  | <div style='text-align: right'>4,286.0</div>  |  |  |  |  | <div style='text-align: right'>23,288.0</div>  | <div style='text-align: right'>1,049,174,120</div>  |  |
| leaf_aggregation | 0 | <div style='text-align: right'>36.0</div>  | <div style='text-align: right'>5,082.0</div>  | <div style='text-align: right'>4,250.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>43.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>14,999.0</div>  | <div style='text-align: right'>398,753,816</div>  | <div style='text-align: right'>241.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <div style='text-align: right'>22,708.0</div>  | <div style='text-align: right'>155,938,962</div>  | <div style='text-align: right'>3,869,913</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>104,932</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <div style='text-align: right'>438,747</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <div style='text-align: right'>419,868</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <div style='text-align: right'>210,102</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>61,084</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>220,805</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>1,187,014</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>744,374</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <div style='text-align: right'>87,650</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <div style='text-align: right'>1,558,016</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>37,145</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 0 | <div style='text-align: right'>154,392</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>29,323</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | JAL | 0 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 0 | <div style='text-align: right'>13,797</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>182</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>546</div>  |
| leaf_aggregation | true | AddEFI | ADD | 0 | <div style='text-align: right'>332</div>  |
| leaf_aggregation | true | AddEI | ADD | 0 | <div style='text-align: right'>31,372</div>  |
| leaf_aggregation | true | AddFI | ADD | 0 | <div style='text-align: right'>69,406</div>  |
| leaf_aggregation | true | AddV | ADD | 0 | <div style='text-align: right'>16,233</div>  |
| leaf_aggregation | true | AddVI | ADD | 0 | <div style='text-align: right'>395,489</div>  |
| leaf_aggregation | true | Alloc | ADD | 0 | <div style='text-align: right'>58,979</div>  |
| leaf_aggregation | true | Alloc | LOADW | 0 | <div style='text-align: right'>58,979</div>  |
| leaf_aggregation | true | Alloc | MUL | 0 | <div style='text-align: right'>34,879</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 0 | <div style='text-align: right'>252</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 0 | <div style='text-align: right'>9,787</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 0 | <div style='text-align: right'>2,459</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>248</div>  |
| leaf_aggregation | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>7,224</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>2,940</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>1,764</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>160,944</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>11,172</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>1,764</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>4,704</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>4,704</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>84</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>6,510</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>57</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 0 | <div style='text-align: right'>228</div>  |
| leaf_aggregation | true | DivFIN | DIV | 0 | <div style='text-align: right'>135</div>  |
| leaf_aggregation | true | For | ADD | 0 | <div style='text-align: right'>470,548</div>  |
| leaf_aggregation | true | For | BNE | 0 | <div style='text-align: right'>516,025</div>  |
| leaf_aggregation | true | For | JAL | 0 | <div style='text-align: right'>45,477</div>  |
| leaf_aggregation | true | For | LOADW | 0 | <div style='text-align: right'>2,394</div>  |
| leaf_aggregation | true | For | STOREW | 0 | <div style='text-align: right'>43,083</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>5,586</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>43</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>24,100</div>  |
| leaf_aggregation | true | IfEq | BNE | 0 | <div style='text-align: right'>19,884</div>  |
| leaf_aggregation | true | IfEqI | BNE | 0 | <div style='text-align: right'>176,478</div>  |
| leaf_aggregation | true | IfEqI | JAL | 0 | <div style='text-align: right'>42,169</div>  |
| leaf_aggregation | true | IfNe | BEQ | 0 | <div style='text-align: right'>16,779</div>  |
| leaf_aggregation | true | IfNe | JAL | 0 | <div style='text-align: right'>3</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 0 | <div style='text-align: right'>2,458</div>  |
| leaf_aggregation | true | ImmE | STOREW | 0 | <div style='text-align: right'>3,908</div>  |
| leaf_aggregation | true | ImmF | STOREW | 0 | <div style='text-align: right'>38,018</div>  |
| leaf_aggregation | true | ImmV | STOREW | 0 | <div style='text-align: right'>30,827</div>  |
| leaf_aggregation | true | LoadE | LOADW | 0 | <div style='text-align: right'>25,468</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>68,964</div>  |
| leaf_aggregation | true | LoadF | LOADW | 0 | <div style='text-align: right'>27,679</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>99,839</div>  |
| leaf_aggregation | true | LoadV | LOADW | 0 | <div style='text-align: right'>28,578</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>224,860</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>11,244</div>  |
| leaf_aggregation | true | MulEF | MUL | 0 | <div style='text-align: right'>3,984</div>  |
| leaf_aggregation | true | MulEFI | MUL | 0 | <div style='text-align: right'>580</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>1,813</div>  |
| leaf_aggregation | true | MulEI | STOREW | 0 | <div style='text-align: right'>7,252</div>  |
| leaf_aggregation | true | MulF | MUL | 0 | <div style='text-align: right'>133,248</div>  |
| leaf_aggregation | true | MulFI | MUL | 0 | <div style='text-align: right'>21</div>  |
| leaf_aggregation | true | MulV | MUL | 0 | <div style='text-align: right'>1,333</div>  |
| leaf_aggregation | true | MulVI | MUL | 0 | <div style='text-align: right'>21,281</div>  |
| leaf_aggregation | true | NegE | MUL | 0 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>17,766</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>11,557</div>  |
| leaf_aggregation | true | StoreE | STOREW | 0 | <div style='text-align: right'>25,132</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>36,036</div>  |
| leaf_aggregation | true | StoreF | STOREW | 0 | <div style='text-align: right'>30,702</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>88,209</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>234,861</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>260,294</div>  |
| leaf_aggregation | true | StoreV | STOREW | 0 | <div style='text-align: right'>3,159</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>65,731</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 0 | <div style='text-align: right'>3,724</div>  |
| leaf_aggregation | true | SubEF | LOADW | 0 | <div style='text-align: right'>16,944</div>  |
| leaf_aggregation | true | SubEF | SUB | 0 | <div style='text-align: right'>5,648</div>  |
| leaf_aggregation | true | SubEFI | ADD | 0 | <div style='text-align: right'>380</div>  |
| leaf_aggregation | true | SubEI | ADD | 0 | <div style='text-align: right'>456</div>  |
| leaf_aggregation | true | SubV | SUB | 0 | <div style='text-align: right'>75,369</div>  |
| leaf_aggregation | true | SubVI | SUB | 0 | <div style='text-align: right'>2,380</div>  |
| leaf_aggregation | true | SubVIN | SUB | 0 | <div style='text-align: right'>882</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>551,880</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>249,436</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>147,394</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 0 | <div style='text-align: right'>114,576</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>7,462</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,177</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,391</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>22,386</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>1,177</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 0 | <div style='text-align: right'>9,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,562</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 0 | <div style='text-align: right'>923</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 0 | <div style='text-align: right'>941,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 0 | <div style='text-align: right'>194,656</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 0 | <div style='text-align: right'>115,024</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 0 | <div style='text-align: right'>135,564</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 0 | <div style='text-align: right'>2,082,180</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 0 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 0 | <div style='text-align: right'>486,990</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 0 | <div style='text-align: right'>11,864,670</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 0 | <div style='text-align: right'>15,895</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 0 | <div style='text-align: right'>1,769,370</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 0 | <div style='text-align: right'>2,418,139</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 0 | <div style='text-align: right'>1,133</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 0 | <div style='text-align: right'>1,046,370</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 0 | <div style='text-align: right'>39</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>5,796</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>1,386</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>819</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 0 | <div style='text-align: right'>225,101</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 0 | <div style='text-align: right'>56,557</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>5,704</div>  |
| leaf_aggregation | PhantomAir | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>43,344</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>17,640</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>10,584</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>965,664</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>67,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>10,584</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>28,224</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>28,224</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>260,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>124,300</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>73,450</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,280</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,464</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1,456</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>9,348</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>825</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>234</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 0 | <div style='text-align: right'>4,050</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 0 | <div style='text-align: right'>14,116,440</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 0 | <div style='text-align: right'>11,868,575</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 0 | <div style='text-align: right'>454,770</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 0 | <div style='text-align: right'>429</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 0 | <div style='text-align: right'>507</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 0 | <div style='text-align: right'>98,154</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 0 | <div style='text-align: right'>473</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 0 | <div style='text-align: right'>1,766,403</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 0 | <div style='text-align: right'>759</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>160,336</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>94,744</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>9,881,088</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>258</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>144,600</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 0 | <div style='text-align: right'>457,332</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 0 | <div style='text-align: right'>4,058,994</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 0 | <div style='text-align: right'>421,690</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 0 | <div style='text-align: right'>385,917</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 0 | <div style='text-align: right'>30</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 0 | <div style='text-align: right'>56,534</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 0 | <div style='text-align: right'>160,228</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 0 | <div style='text-align: right'>9,196</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 0 | <div style='text-align: right'>5,434</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 0 | <div style='text-align: right'>13,376</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,558,738</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,573</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 0 | <div style='text-align: right'>1,263,907</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 0 | <div style='text-align: right'>16,357</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 0 | <div style='text-align: right'>1,044,188</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 0 | <div style='text-align: right'>181,698</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 0 | <div style='text-align: right'>107,367</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 0 | <div style='text-align: right'>3,740</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>2,827,524</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>59,180</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>34,970</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 0 | <div style='text-align: right'>1,134,839</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 0 | <div style='text-align: right'>53,592</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 0 | <div style='text-align: right'>31,668</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 0 | <div style='text-align: right'>20,706</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 0 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>4,093,399</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>814</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>481</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>510</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>539</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 0 | <div style='text-align: right'>1,171,698</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 0 | <div style='text-align: right'>15,158</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>9,219,260</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>979</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>449,760</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>230,076</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>135,954</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>135,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 0 | <div style='text-align: right'>119,520</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 0 | <div style='text-align: right'>20,174</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 0 | <div style='text-align: right'>11,921</div>  |
| leaf_aggregation | Boundary | true | MulEF | MUL | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEFI | MUL | 0 | <div style='text-align: right'>17,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEFI | MUL | 0 | <div style='text-align: right'>3,058</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,807</div>  |
| leaf_aggregation | Boundary | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>72,520</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>82,654</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>48,841</div>  |
| leaf_aggregation | Boundary | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>4,312</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | 0 | <div style='text-align: right'>297,332</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 0 | <div style='text-align: right'>39,831</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 0 | <div style='text-align: right'>23,517</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 0 | <div style='text-align: right'>3,997,440</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 0 | <div style='text-align: right'>630</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 0 | <div style='text-align: right'>39,990</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 0 | <div style='text-align: right'>14,641</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 0 | <div style='text-align: right'>638,430</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 0 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 0 | <div style='text-align: right'>6,600</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 0 | <div style='text-align: right'>1,716</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 0 | <div style='text-align: right'>1,014</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 0 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>731,808</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>432,432</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>282,744</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>9,931,194</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>597,212</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>353,990</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>236,470</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>6,460,363</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 0 | <div style='text-align: right'>1,030,412</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 0 | <div style='text-align: right'>19,448</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 0 | <div style='text-align: right'>11,492</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 0 | <div style='text-align: right'>276,452</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>1,477,476</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>158,928</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>93,912</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>39,732</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 0 | <div style='text-align: right'>1,258,782</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 0 | <div style='text-align: right'>337,722</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>3,616,569</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>389,356</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>231,166</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>155,975</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>81,356</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>7,045,830</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>10,672,054</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>2,863,234</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 0 | <div style='text-align: right'>129,519</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 0 | <div style='text-align: right'>34,749</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>2,694,971</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>666,512</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>148,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>130,988</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>77,402</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 0 | <div style='text-align: right'>27,896</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 0 | <div style='text-align: right'>694,704</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 0 | <div style='text-align: right'>61,985</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 0 | <div style='text-align: right'>169,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 0 | <div style='text-align: right'>61,985</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 0 | <div style='text-align: right'>73,255</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 0 | <div style='text-align: right'>11,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,738</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,027</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 0 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 0 | <div style='text-align: right'>13,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 0 | <div style='text-align: right'>3,762</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 0 | <div style='text-align: right'>2,223</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 0 | <div style='text-align: right'>2,261,070</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 0 | <div style='text-align: right'>71,400</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 0 | <div style='text-align: right'>15,147</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 0 | <div style='text-align: right'>26,460</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/rkyv-2-2-64cpu-linux-arm64-mimalloc-rkyv_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/33be8678e9fdd6c2cc3fb04562271a656b2850fc

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11917092664)
