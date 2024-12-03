| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| base64_json_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>15,125,712</div>  | <div style='text-align: right'>217,353</div>  | <span style="color: green">(-45.0 [-1.7%])</span> <div style='text-align: right'>2,608.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <span style="color: red">(+25,320 [+0.0%])</span> <div style='text-align: right'>294,353,671</div>  | <span style="color: red">(+2,434 [+0.0%])</span> <div style='text-align: right'>6,773,724</div>  | <span style="color: red">(+33.0 [+0.1%])</span> <div style='text-align: right'>34,878.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| base64_json_program | true | <div style='text-align: right'>1,533.0</div>  | <div style='text-align: right'>15,125,712</div>  | <div style='text-align: right'>217,353</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| base64_json_program | ProgramChip | true | <div style='text-align: right'>19,406</div>  |
| base64_json_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| base64_json_program | Boundary | true | <div style='text-align: right'>5,180</div>  |
| base64_json_program | Merkle | true | <div style='text-align: right'>5,536</div>  |
| base64_json_program | AccessAdapter<8> | true | <div style='text-align: right'>5,180</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>1,563</div>  |
| base64_json_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>86</div>  |
| base64_json_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>116</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| base64_json_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>1,331</div>  |
| base64_json_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>2,940</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>5,005</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>16,738</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>27,336</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>1,236</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>55,121</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>16,188</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>575</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>89,113</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| base64_json_program | PhantomAir | true | <div style='text-align: right'>5</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>10,716</div>  |
| base64_json_program | VariableRangeCheckerAir | true | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| base64_json_program | true |  | ADD | <div style='text-align: right'>69,777</div>  |
| base64_json_program | true |  | AND | <div style='text-align: right'>10,124</div>  |
| base64_json_program | true |  | AUIPC | <div style='text-align: right'>1,331</div>  |
| base64_json_program | true |  | BEQ | <div style='text-align: right'>15,566</div>  |
| base64_json_program | true |  | BGE | <div style='text-align: right'>704</div>  |
| base64_json_program | true |  | BGEU | <div style='text-align: right'>6,863</div>  |
| base64_json_program | true |  | BLT | <div style='text-align: right'>3,353</div>  |
| base64_json_program | true |  | BLTU | <div style='text-align: right'>5,818</div>  |
| base64_json_program | true |  | BNE | <div style='text-align: right'>11,770</div>  |
| base64_json_program | true |  | HINT_STOREW | <div style='text-align: right'>1,563</div>  |
| base64_json_program | true |  | JAL | <div style='text-align: right'>3,687</div>  |
| base64_json_program | true |  | JALR | <div style='text-align: right'>2,940</div>  |
| base64_json_program | true |  | LOADB | <div style='text-align: right'>1,236</div>  |
| base64_json_program | true |  | LOADBU | <div style='text-align: right'>23,858</div>  |
| base64_json_program | true |  | LOADHU | <div style='text-align: right'>3</div>  |
| base64_json_program | true |  | LOADW | <div style='text-align: right'>13,465</div>  |
| base64_json_program | true |  | LUI | <div style='text-align: right'>1,318</div>  |
| base64_json_program | true |  | MUL | <div style='text-align: right'>116</div>  |
| base64_json_program | true |  | MULHU | <div style='text-align: right'>86</div>  |
| base64_json_program | true |  | OR | <div style='text-align: right'>7,608</div>  |
| base64_json_program | true |  | PHANTOM | <div style='text-align: right'>5</div>  |
| base64_json_program | true |  | SLL | <div style='text-align: right'>7,118</div>  |
| base64_json_program | true |  | SLT | <div style='text-align: right'>5</div>  |
| base64_json_program | true |  | SLTU | <div style='text-align: right'>570</div>  |
| base64_json_program | true |  | SRA | <div style='text-align: right'>8</div>  |
| base64_json_program | true |  | SRL | <div style='text-align: right'>9,062</div>  |
| base64_json_program | true |  | STOREB | <div style='text-align: right'>5,133</div>  |
| base64_json_program | true |  | STOREH | <div style='text-align: right'>10</div>  |
| base64_json_program | true |  | STOREW | <div style='text-align: right'>12,652</div>  |
| base64_json_program | true |  | SUB | <div style='text-align: right'>1,416</div>  |
| base64_json_program | true |  | XOR | <div style='text-align: right'>188</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>2,511,972</div>  |
| base64_json_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>85</div>  |
| base64_json_program | Boundary | true |  | ADD | <div style='text-align: right'>200</div>  |
| base64_json_program | Merkle | true |  | ADD | <div style='text-align: right'>128</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>364,464</div>  |
| base64_json_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>27,951</div>  |
| base64_json_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>51</div>  |
| base64_json_program | Boundary | true |  | AUIPC | <div style='text-align: right'>120</div>  |
| base64_json_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,520</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>404,716</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>22,528</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>219,616</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>107,296</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>186,176</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>306,020</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>40,638</div>  |
| base64_json_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>13,277</div>  |
| base64_json_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>31,240</div>  |
| base64_json_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>49,792</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>66,366</div>  |
| base64_json_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>82,320</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>43,260</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>954,320</div>  |
| base64_json_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>2,873</div>  |
| base64_json_program | Boundary | true |  | LOADBU | <div style='text-align: right'>6,760</div>  |
| base64_json_program | Merkle | true |  | LOADBU | <div style='text-align: right'>12,480</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>120</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>538,600</div>  |
| base64_json_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>1,921</div>  |
| base64_json_program | Boundary | true |  | LOADW | <div style='text-align: right'>4,520</div>  |
| base64_json_program | Merkle | true |  | LOADW | <div style='text-align: right'>12,416</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>23,724</div>  |
| base64_json_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>3,596</div>  |
| base64_json_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>3,354</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>273,888</div>  |
| base64_json_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>30</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>377,254</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>185</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>21,090</div>  |
| base64_json_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>424</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>480,286</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>205,320</div>  |
| base64_json_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>10,472</div>  |
| base64_json_program | Boundary | true |  | STOREB | <div style='text-align: right'>24,640</div>  |
| base64_json_program | Merkle | true |  | STOREB | <div style='text-align: right'>39,680</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>400</div>  |
| base64_json_program | AccessAdapter<8> | true |  | STOREH | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary | true |  | STOREH | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>506,080</div>  |
| base64_json_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>15,300</div>  |
| base64_json_program | Boundary | true |  | STOREW | <div style='text-align: right'>36,000</div>  |
| base64_json_program | Merkle | true |  | STOREW | <div style='text-align: right'>59,072</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>50,976</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>6,768</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | <span style="color: red">(+1.0 [+7.7%])</span> <div style='text-align: right'>14.0</div>  | <div style='text-align: right'>506.0</div>  | <span style="color: red">(+2.0 [+0.6%])</span> <div style='text-align: right'>341.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+3.0 [+1.4%])</span> <div style='text-align: right'>219.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>15,125,712</div>  | <div style='text-align: right'>217,353</div>  | <span style="color: green">(-45.0 [-1.7%])</span> <div style='text-align: right'>2,608.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <span style="color: red">(+25,320 [+0.0%])</span> <div style='text-align: right'>294,353,671</div>  | <span style="color: red">(+2,434 [+0.0%])</span> <div style='text-align: right'>6,773,724</div>  | <span style="color: red">(+33.0 [+0.1%])</span> <div style='text-align: right'>34,878.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| base64_json_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| base64_json_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| base64_json_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| base64_json_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>262,144</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>425,984</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>335,872</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  |  |  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  |  | <div style='text-align: right'>1</div>  |
| base64_json_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>126,976</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>17,792</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>14,208</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| base64_json_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>100,352</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>4,096</div>  |
| base64_json_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,340,032</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>78,848</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | PhantomAir | 0 | <div style='text-align: right'>144</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>8</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>10,272,768</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>56,623,104</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | FriReducedOpeningAir | 0 | <div style='text-align: right'>146,800,640</div>  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | 0 |  | <span style="color: green">(-2.0 [-1.2%])</span> <div style='text-align: right'>162.0</div>  |  |  |  |  | <span style="color: green">(-43.0 [-2.2%])</span> <div style='text-align: right'>1,940.0</div>  | <div style='text-align: right'>50,533,140</div>  |  |
| leaf_aggregation | 0 | <span style="color: red">(+6.0 [+5.5%])</span> <div style='text-align: right'>115.0</div>  | <span style="color: red">(+64.0 [+0.7%])</span> <div style='text-align: right'>9,315.0</div>  | <span style="color: red">(+35.0 [+0.5%])</span> <div style='text-align: right'>7,633.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-1.0 [-1.6%])</span> <div style='text-align: right'>63.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-31.0 [-0.1%])</span> <div style='text-align: right'>25,563.0</div>  | <div style='text-align: right'>787,283,992</div>  | <span style="color: red">(+927.0 [+1.2%])</span> <div style='text-align: right'>75,335.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: red">(+355.0 [+0.9%])</span> <div style='text-align: right'>38,153.0</div>  | <span style="color: red">(+25,320 [+0.0%])</span> <div style='text-align: right'>294,353,671</div>  | <span style="color: red">(+2,434 [+0.0%])</span> <div style='text-align: right'>6,773,724</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>305,114</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <div style='text-align: right'>1,012,342</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <span style="color: red">(+56 [+0.0%])</span> <div style='text-align: right'>1,058,988</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <span style="color: red">(+28 [+0.0%])</span> <div style='text-align: right'>529,704</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>107,792</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>52,761</div>  |
| leaf_aggregation | FriReducedOpeningAir | true | 0 | <div style='text-align: right'>550,368</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>106,391</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <div style='text-align: right'>2,619,349</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: red">(+2,434 [+3.1%])</span> <div style='text-align: right'>82,179</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>1,362,708</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>1,922,916</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>621,078</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | ADD | 0 | <div style='text-align: right'>2,356,149</div>  |
| leaf_aggregation | true |  | BBE4DIV | 0 | <div style='text-align: right'>7,254</div>  |
| leaf_aggregation | true |  | BBE4MUL | 0 | <div style='text-align: right'>36,463</div>  |
| leaf_aggregation | true |  | BEQ | 0 | <div style='text-align: right'>17,774</div>  |
| leaf_aggregation | true |  | BNE | 0 | <div style='text-align: right'>1,344,934</div>  |
| leaf_aggregation | true |  | COMP_POS2 | 0 | <div style='text-align: right'>16,380</div>  |
| leaf_aggregation | true |  | DIV | 0 | <div style='text-align: right'>156</div>  |
| leaf_aggregation | true |  | FE4ADD | 0 | <div style='text-align: right'>46,074</div>  |
| leaf_aggregation | true |  | FE4SUB | 0 | <div style='text-align: right'>16,600</div>  |
| leaf_aggregation | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>6,342</div>  |
| leaf_aggregation | true |  | JAL | 0 | <span style="color: red">(+2,434 [+3.1%])</span> <div style='text-align: right'>82,179</div>  |
| leaf_aggregation | true |  | LOADW | 0 | <div style='text-align: right'>192,563</div>  |
| leaf_aggregation | true |  | LOADW2 | 0 | <div style='text-align: right'>612,893</div>  |
| leaf_aggregation | true |  | MUL | 0 | <div style='text-align: right'>185,481</div>  |
| leaf_aggregation | true |  | PERM_POS2 | 0 | <div style='text-align: right'>36,381</div>  |
| leaf_aggregation | true |  | PHANTOM | 0 | <div style='text-align: right'>621,078</div>  |
| leaf_aggregation | true |  | SHINTW | 0 | <div style='text-align: right'>479,323</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>247,198</div>  |
| leaf_aggregation | true |  | STOREW2 | 0 | <div style='text-align: right'>390,939</div>  |
| leaf_aggregation | true |  | SUB | 0 | <div style='text-align: right'>77,563</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | ADD | 0 | <div style='text-align: right'>70,684,470</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | ADD | 0 | <span style="color: red">(+308 [+0.1%])</span> <div style='text-align: right'>592,878</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | ADD | 0 | <span style="color: red">(+182 [+0.1%])</span> <div style='text-align: right'>350,337</div>  |
| leaf_aggregation | Boundary | true |  | ADD | 0 | <div style='text-align: right'>753,764</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4DIV | 0 | <div style='text-align: right'>290,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4DIV | 0 | <div style='text-align: right'>143,836</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4DIV | 0 | <div style='text-align: right'>84,994</div>  |
| leaf_aggregation | Boundary | true |  | BBE4DIV | 0 | <div style='text-align: right'>352</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4MUL | 0 | <div style='text-align: right'>1,458,520</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4MUL | 0 | <span style="color: red">(+308 [+0.0%])</span> <div style='text-align: right'>1,086,316</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4MUL | 0 | <span style="color: red">(+182 [+0.0%])</span> <div style='text-align: right'>641,914</div>  |
| leaf_aggregation | Boundary | true |  | BBE4MUL | 0 | <div style='text-align: right'>1,037,212</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BEQ | 0 | <div style='text-align: right'>408,802</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BNE | 0 | <div style='text-align: right'>30,933,482</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BNE | 0 | <div style='text-align: right'>1,474</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BNE | 0 | <div style='text-align: right'>871</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>648,648</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>383,292</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>250,614</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>9,156,420</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | DIV | 0 | <div style='text-align: right'>4,680</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4ADD | 0 | <div style='text-align: right'>1,842,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4ADD | 0 | <div style='text-align: right'>1,321,012</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4ADD | 0 | <div style='text-align: right'>780,598</div>  |
| leaf_aggregation | Boundary | true |  | FE4ADD | 0 | <div style='text-align: right'>1,365,980</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4SUB | 0 | <div style='text-align: right'>664,000</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4SUB | 0 | <div style='text-align: right'>539,198</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4SUB | 0 | <div style='text-align: right'>318,617</div>  |
| leaf_aggregation | Boundary | true |  | FE4SUB | 0 | <div style='text-align: right'>574,816</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>378,840</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>223,860</div>  |
| leaf_aggregation | FriReducedOpeningAir | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>35,223,552</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <span style="color: red">(+24,340 [+3.1%])</span> <div style='text-align: right'>821,790</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | JAL | 0 | <div style='text-align: right'>495</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | JAL | 0 | <div style='text-align: right'>585</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW | 0 | <div style='text-align: right'>7,895,083</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW | 0 | <div style='text-align: right'>529,551</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW | 0 | <div style='text-align: right'>272,012</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW | 0 | <div style='text-align: right'>19,278</div>  |
| leaf_aggregation | Boundary | true |  | LOADW | 0 | <div style='text-align: right'>382,646</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW2 | 0 | <div style='text-align: right'>25,128,613</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW2 | 0 | <div style='text-align: right'>54,406</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW2 | 0 | <div style='text-align: right'>32,149</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW2 | 0 | <div style='text-align: right'>476</div>  |
| leaf_aggregation | Boundary | true |  | LOADW2 | 0 | <div style='text-align: right'>1,672</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | MUL | 0 | <div style='text-align: right'>5,564,430</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | MUL | 0 | <div style='text-align: right'>27,401</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | MUL | 0 | <div style='text-align: right'>16,211</div>  |
| leaf_aggregation | Boundary | true |  | MUL | 0 | <div style='text-align: right'>112,376</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>1,706,584</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>1,009,801</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>665,618</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>20,336,979</div>  |
| leaf_aggregation | PhantomAir | true |  | PHANTOM | 0 | <div style='text-align: right'>3,726,468</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | SHINTW | 0 | <div style='text-align: right'>19,652,243</div>  |
| leaf_aggregation | Boundary | true |  | SHINTW | 0 | <div style='text-align: right'>5,272,553</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>10,135,118</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW | 0 | <div style='text-align: right'>139,425</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW | 0 | <div style='text-align: right'>81,380</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>864,699</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW2 | 0 | <div style='text-align: right'>16,028,499</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW2 | 0 | <div style='text-align: right'>1,642,916</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW2 | 0 | <div style='text-align: right'>972,179</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | STOREW2 | 0 | <div style='text-align: right'>570,877</div>  |
| leaf_aggregation | Boundary | true |  | STOREW2 | 0 | <div style='text-align: right'>754,501</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | SUB | 0 | <div style='text-align: right'>2,326,890</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | SUB | 0 | <div style='text-align: right'>70,400</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | SUB | 0 | <div style='text-align: right'>83,200</div>  |
| leaf_aggregation | Boundary | true |  | SUB | 0 | <div style='text-align: right'>15,180</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/base64_json-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/14a79fbb495d36c2cd970329a6d0f972943a24b3

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12131278254)
