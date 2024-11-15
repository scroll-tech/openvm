| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| base64_json_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,960,360</div>  | <div style='text-align: right'>217,349</div>  | <span style="color: red">(+21.0 [+0.7%])</span> <div style='text-align: right'>2,876.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| base64_json_program | true | <span style="color: red">(+12.0 [+0.2%])</span> <div style='text-align: right'>6,023.0</div>  | <div style='text-align: right'>8,960,360</div>  | <div style='text-align: right'>217,349</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>89,113</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>575</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>16,188</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>27,336</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>16,738</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>5,001</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>1,563</div>  |
| base64_json_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>2,940</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>1,236</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>55,121</div>  |
| base64_json_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>86</div>  |
| base64_json_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>116</div>  |
| base64_json_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>1,331</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| base64_json_program | Memory AccessAdapter<8> | true | <div style='text-align: right'>2,590</div>  |
| base64_json_program | Memory Boundary | true | <div style='text-align: right'>5,180</div>  |
| base64_json_program | Memory Merkle | true | <div style='text-align: right'>5,538</div>  |
| base64_json_program | PhantomAir | true | <div style='text-align: right'>5</div>  |
| base64_json_program | ProgramChip | true | <div style='text-align: right'>19,408</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| base64_json_program | true |  | ADD | <div style='text-align: right'>69,777</div>  |
| base64_json_program | true |  | AND | <div style='text-align: right'>10,124</div>  |
| base64_json_program | true |  | AUIPC | <div style='text-align: right'>1,331</div>  |
| base64_json_program | true |  | BEQ | <div style='text-align: right'>15,568</div>  |
| base64_json_program | true |  | BGE | <div style='text-align: right'>704</div>  |
| base64_json_program | true |  | BGEU | <div style='text-align: right'>6,863</div>  |
| base64_json_program | true |  | BLT | <div style='text-align: right'>3,353</div>  |
| base64_json_program | true |  | BLTU | <div style='text-align: right'>5,818</div>  |
| base64_json_program | true |  | BNE | <div style='text-align: right'>11,768</div>  |
| base64_json_program | true |  | HINT_STOREW | <div style='text-align: right'>1,563</div>  |
| base64_json_program | true |  | JAL | <div style='text-align: right'>3,683</div>  |
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
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>404,768</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>22,528</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>219,616</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>107,296</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>186,176</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>305,968</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>40,638</div>  |
| base64_json_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>13,277</div>  |
| base64_json_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>31,240</div>  |
| base64_json_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>49,856</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>66,294</div>  |
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
| base64_json_program | <span style="color: red">(+1.0 [+7.1%])</span> <div style='text-align: right'>15.0</div>  | <span style="color: red">(+12.0 [+1.9%])</span> <div style='text-align: right'>647.0</div>  | <span style="color: green">(-2.0 [-0.4%])</span> <div style='text-align: right'>472.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>210.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8,960,360</div>  | <div style='text-align: right'>217,349</div>  | <span style="color: red">(+21.0 [+0.7%])</span> <div style='text-align: right'>2,876.0</div>  |

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
| base64_json_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| base64_json_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>425,984</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | PhantomAir | 0 | <div style='text-align: right'>144</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>131,072</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>78,848</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>1,024</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| base64_json_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>14,208</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>17,792</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>126,976</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>1</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>10,272,768</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| base64_json_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| base64_json_program | 0 | <span style="color: red">(+13.0 [+8.2%])</span> <div style='text-align: right'>171.0</div>  | <span style="color: green">(-4.0 [-0.2%])</span> <div style='text-align: right'>2,058.0</div>  | <div style='text-align: right'>49,353,492</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e2f08050bdd7dae93d09919e14b31750728ad15b/base64_json-2-2-64cpu-linux-arm64-mimalloc-base64_json_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/e2f08050bdd7dae93d09919e14b31750728ad15b

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11854621082)
