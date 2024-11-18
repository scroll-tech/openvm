| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>3</div>  | <span style="color: red">(+204,018 [+0.4%])</span> <div style='text-align: right'>51,515,770</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: green">(-454.0 [-4.1%])</span> <div style='text-align: right'>10,591.0</div>  |
| leaf_aggregation | <div style='text-align: right'>3</div>  | <span style="color: red">(+1,440,712 [+1.5%])</span> <div style='text-align: right'>98,061,331</div>  | <span style="color: green">(-997 [-0.0%])</span> <div style='text-align: right'>2,455,027</div>  | <span style="color: green">(-556.0 [-2.8%])</span> <div style='text-align: right'>19,405.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| fibonacci_program | true | <span style="color: green">(-29,284.0 [-82.3%])</span> <div style='text-align: right'>6,279.0</div>  | <span style="color: red">(+204,018 [+0.4%])</span> <div style='text-align: right'>51,515,770</div>  | <div style='text-align: right'>1,500,219</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | ProgramChip | true | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>6,543</div>  |
| fibonacci_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| fibonacci_program | Boundary | true | <div style='text-align: right'>56</div>  |
| fibonacci_program | Merkle | true | <div style='text-align: right'>308</div>  |
| fibonacci_program | AccessAdapter<8> | true | <div style='text-align: right'>56</div>  |
| fibonacci_program | PhantomAir | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>900,085</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>300,004</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>4</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>57</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>200,012</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>100,012</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>17</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>364</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| fibonacci_program | true |  | ADD | <div style='text-align: right'>900,068</div>  |
| fibonacci_program | true |  | AND | <div style='text-align: right'>5</div>  |
| fibonacci_program | true |  | AUIPC | <div style='text-align: right'>11</div>  |
| fibonacci_program | true |  | BEQ | <div style='text-align: right'>100,005</div>  |
| fibonacci_program | true |  | BGEU | <div style='text-align: right'>3</div>  |
| fibonacci_program | true |  | BLT | <div style='text-align: right'>1</div>  |
| fibonacci_program | true |  | BLTU | <div style='text-align: right'>7</div>  |
| fibonacci_program | true |  | BNE | <div style='text-align: right'>100,007</div>  |
| fibonacci_program | true |  | HINT_STOREW | <div style='text-align: right'>3</div>  |
| fibonacci_program | true |  | JAL | <div style='text-align: right'>100,002</div>  |
| fibonacci_program | true |  | JALR | <div style='text-align: right'>17</div>  |
| fibonacci_program | true |  | LOADBU | <div style='text-align: right'>6</div>  |
| fibonacci_program | true |  | LOADW | <div style='text-align: right'>22</div>  |
| fibonacci_program | true |  | LUI | <div style='text-align: right'>10</div>  |
| fibonacci_program | true |  | OR | <div style='text-align: right'>4</div>  |
| fibonacci_program | true |  | PHANTOM | <div style='text-align: right'>3</div>  |
| fibonacci_program | true |  | SLL | <div style='text-align: right'>3</div>  |
| fibonacci_program | true |  | SLTU | <div style='text-align: right'>300,004</div>  |
| fibonacci_program | true |  | SRL | <div style='text-align: right'>1</div>  |
| fibonacci_program | true |  | STOREB | <div style='text-align: right'>1</div>  |
| fibonacci_program | true |  | STOREW | <div style='text-align: right'>28</div>  |
| fibonacci_program | true |  | SUB | <div style='text-align: right'>4</div>  |
| fibonacci_program | true |  | XOR | <div style='text-align: right'>4</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>32,402,448</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| fibonacci_program | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| fibonacci_program | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>231</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>2,600,130</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>96</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>32</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>224</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>2,600,182</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle | true |  | HINT_STOREW | <span style="color: red">(+64 [+100.0%])</span> <div style='text-align: right'>128</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>1,800,036</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>476</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>880</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary | true |  | LOADW | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle | true |  | LOADW | <div style='text-align: right'>2,304</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>144</div>  |
| fibonacci_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>18</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>159</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>11,100,148</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>53</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>17</div>  |
| fibonacci_program | Boundary | true |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>1,120</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>272</div>  |
| fibonacci_program | Boundary | true |  | STOREW | <div style='text-align: right'>640</div>  |
| fibonacci_program | Merkle | true |  | STOREW | <div style='text-align: right'>3,776</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>144</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | <span style="color: green">(-3.0 [-21.4%])</span> <div style='text-align: right'>11.0</div>  | <span style="color: green">(-743.0 [-42.7%])</span> <div style='text-align: right'>997.0</div>  | <span style="color: green">(-738.0 [-47.0%])</span> <div style='text-align: right'>833.0</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+1.0 [+0.3%])</span> <div style='text-align: right'>364.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+204,018 [+0.4%])</span> <div style='text-align: right'>51,515,770</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: green">(-454.0 [-4.1%])</span> <div style='text-align: right'>10,591.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>3</div>  |  |  | <span style="color: red">(+1,440,712 [+1.5%])</span> <div style='text-align: right'>98,061,331</div>  | <span style="color: green">(-997 [-0.0%])</span> <div style='text-align: right'>2,455,027</div>  | <span style="color: green">(-556.0 [-2.8%])</span> <div style='text-align: right'>19,405.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| fibonacci_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| fibonacci_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| fibonacci_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>26,624</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>420</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,048</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>32</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>12,058,624</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>63,963,136</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>22,544,384</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>1,441,792</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>48,234,496</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>1,835,008</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| leaf_aggregation | FriMatOpeningAir | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 |  | <span style="color: green">(-5.0 [-3.0%])</span> <div style='text-align: right'>162.0</div>  |  |  |  |  | <span style="color: red">(+294.0 [+3.2%])</span> <div style='text-align: right'>9,432.0</div>  | <div style='text-align: right'>196,595,668</div>  |  |
| leaf_aggregation | 0 | <span style="color: red">(+2.0 [+2.8%])</span> <div style='text-align: right'>74.0</div>  | <span style="color: green">(-1,013.0 [-28.1%])</span> <div style='text-align: right'>2,597.0</div>  | <span style="color: green">(-1,018.0 [-31.9%])</span> <div style='text-align: right'>2,175.0</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+9.0 [+12.2%])</span> <div style='text-align: right'>83.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+457.0 [+2.8%])</span> <div style='text-align: right'>16,808.0</div>  | <div style='text-align: right'>205,520,920</div>  | <span style="color: red">(+15.0 [+5.5%])</span> <div style='text-align: right'>288.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: green">(-21,745.0 [-67.0%])</span> <div style='text-align: right'>10,726.0</div>  | <span style="color: red">(+1,440,712 [+1.5%])</span> <div style='text-align: right'>98,061,331</div>  | <span style="color: green">(-997 [-0.0%])</span> <div style='text-align: right'>2,455,027</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>100,634</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <div style='text-align: right'>309,123</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <div style='text-align: right'>303,056</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <div style='text-align: right'>151,696</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>40,412</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>140,817</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>781,810</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>471,558</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: green">(-967 [-1.9%])</span> <div style='text-align: right'>51,079</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <span style="color: green">(-30 [-0.0%])</span> <div style='text-align: right'>958,379</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>28,407</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 0 | <div style='text-align: right'>96,488</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>19,421</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | JAL | 0 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 0 | <div style='text-align: right'>10,761</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>162</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>486</div>  |
| leaf_aggregation | true | AddEFI | ADD | 0 | <div style='text-align: right'>312</div>  |
| leaf_aggregation | true | AddEI | ADD | 0 | <div style='text-align: right'>26,740</div>  |
| leaf_aggregation | true | AddFI | ADD | 0 | <span style="color: green">(-30 [-0.1%])</span> <div style='text-align: right'>32,331</div>  |
| leaf_aggregation | true | AddV | ADD | 0 | <div style='text-align: right'>10,606</div>  |
| leaf_aggregation | true | AddVI | ADD | 0 | <div style='text-align: right'>243,567</div>  |
| leaf_aggregation | true | Alloc | ADD | 0 | <div style='text-align: right'>39,464</div>  |
| leaf_aggregation | true | Alloc | LOADW | 0 | <div style='text-align: right'>39,464</div>  |
| leaf_aggregation | true | Alloc | MUL | 0 | <div style='text-align: right'>23,350</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 0 | <div style='text-align: right'>192</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 0 | <div style='text-align: right'>6,301</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 0 | <div style='text-align: right'>1,692</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>237</div>  |
| leaf_aggregation | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>448</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>4,592</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>2,296</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>1,120</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>100,632</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>7,112</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>448</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>1,120</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>3,416</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>3,416</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>56</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>4,156</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>54</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 0 | <div style='text-align: right'>216</div>  |
| leaf_aggregation | true | DivFIN | DIV | 0 | <div style='text-align: right'>128</div>  |
| leaf_aggregation | true | For | ADD | 0 | <div style='text-align: right'>301,758</div>  |
| leaf_aggregation | true | For | BNE | 0 | <div style='text-align: right'>332,064</div>  |
| leaf_aggregation | true | For | JAL | 0 | <div style='text-align: right'>30,306</div>  |
| leaf_aggregation | true | For | LOADW | 0 | <div style='text-align: right'>1,736</div>  |
| leaf_aggregation | true | For | STOREW | 0 | <div style='text-align: right'>28,570</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>3,556</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>29</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>16,114</div>  |
| leaf_aggregation | true | IfEq | BNE | 0 | <div style='text-align: right'>17,655</div>  |
| leaf_aggregation | true | IfEqI | BNE | 0 | <div style='text-align: right'>100,275</div>  |
| leaf_aggregation | true | IfEqI | JAL | 0 | <span style="color: green">(-967 [-4.4%])</span> <div style='text-align: right'>20,770</div>  |
| leaf_aggregation | true | IfNe | BEQ | 0 | <div style='text-align: right'>11,329</div>  |
| leaf_aggregation | true | IfNe | JAL | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 0 | <div style='text-align: right'>1,809</div>  |
| leaf_aggregation | true | ImmE | STOREW | 0 | <div style='text-align: right'>3,476</div>  |
| leaf_aggregation | true | ImmF | STOREW | 0 | <div style='text-align: right'>27,642</div>  |
| leaf_aggregation | true | ImmV | STOREW | 0 | <div style='text-align: right'>20,756</div>  |
| leaf_aggregation | true | LoadE | LOADW | 0 | <div style='text-align: right'>20,676</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>43,872</div>  |
| leaf_aggregation | true | LoadF | LOADW | 0 | <div style='text-align: right'>17,829</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>62,827</div>  |
| leaf_aggregation | true | LoadV | LOADW | 0 | <div style='text-align: right'>19,199</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>138,802</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>9,073</div>  |
| leaf_aggregation | true | MulEF | MUL | 0 | <div style='text-align: right'>2,672</div>  |
| leaf_aggregation | true | MulEFI | MUL | 0 | <div style='text-align: right'>520</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>1,646</div>  |
| leaf_aggregation | true | MulEI | STOREW | 0 | <div style='text-align: right'>6,584</div>  |
| leaf_aggregation | true | MulF | MUL | 0 | <div style='text-align: right'>60,886</div>  |
| leaf_aggregation | true | MulFI | MUL | 0 | <div style='text-align: right'>20</div>  |
| leaf_aggregation | true | MulV | MUL | 0 | <div style='text-align: right'>899</div>  |
| leaf_aggregation | true | MulVI | MUL | 0 | <div style='text-align: right'>14,204</div>  |
| leaf_aggregation | true | NegE | MUL | 0 | <div style='text-align: right'>204</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>12,124</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>7,297</div>  |
| leaf_aggregation | true | StoreE | STOREW | 0 | <div style='text-align: right'>16,384</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>22,928</div>  |
| leaf_aggregation | true | StoreF | STOREW | 0 | <div style='text-align: right'>22,974</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>55,449</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>158,121</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>175,134</div>  |
| leaf_aggregation | true | StoreV | STOREW | 0 | <div style='text-align: right'>2,150</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>43,644</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 0 | <div style='text-align: right'>2,717</div>  |
| leaf_aggregation | true | SubEF | LOADW | 0 | <div style='text-align: right'>10,848</div>  |
| leaf_aggregation | true | SubEF | SUB | 0 | <div style='text-align: right'>3,616</div>  |
| leaf_aggregation | true | SubEFI | ADD | 0 | <div style='text-align: right'>356</div>  |
| leaf_aggregation | true | SubEI | ADD | 0 | <div style='text-align: right'>432</div>  |
| leaf_aggregation | true | SubV | SUB | 0 | <div style='text-align: right'>36,028</div>  |
| leaf_aggregation | true | SubVI | SUB | 0 | <div style='text-align: right'>1,605</div>  |
| leaf_aggregation | true | SubVIN | SUB | 0 | <div style='text-align: right'>560</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>430,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>202,114</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>119,431</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 0 | <div style='text-align: right'>114,576</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>6,642</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,100</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,300</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>19,926</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>1,100</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 0 | <div style='text-align: right'>9,360</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,452</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 0 | <div style='text-align: right'>858</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 0 | <div style='text-align: right'>802,200</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 0 | <span style="color: green">(-264 [-0.2%])</span> <div style='text-align: right'>170,082</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 0 | <span style="color: green">(-156 [-0.2%])</span> <div style='text-align: right'>100,503</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 0 | <div style='text-align: right'>135,564</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 0 | <span style="color: green">(-900 [-0.1%])</span> <div style='text-align: right'>969,930</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 0 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 0 | <div style='text-align: right'>318,180</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 0 | <div style='text-align: right'>7,307,010</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 0 | <div style='text-align: right'>11,110</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 0 | <div style='text-align: right'>1,183,920</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 0 | <div style='text-align: right'>1,618,024</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 0 | <div style='text-align: right'>979</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 0 | <div style='text-align: right'>700,500</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 0 | <div style='text-align: right'>39</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>4,416</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>624</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 0 | <div style='text-align: right'>144,923</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 0 | <div style='text-align: right'>38,916</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>5,451</div>  |
| leaf_aggregation | PhantomAir | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>2,688</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>27,552</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>13,776</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>6,720</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>603,792</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>42,672</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>2,688</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>6,720</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>20,496</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>20,496</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>336</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>166,240</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>79,574</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>47,021</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,288</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1,352</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>8,856</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>781</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>221</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 0 | <div style='text-align: right'>3,840</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 0 | <div style='text-align: right'>9,052,740</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 0 | <div style='text-align: right'>7,637,472</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 0 | <div style='text-align: right'>303,060</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 0 | <div style='text-align: right'>407</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 0 | <div style='text-align: right'>481</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 0 | <div style='text-align: right'>71,176</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 0 | <div style='text-align: right'>319</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 0 | <div style='text-align: right'>1,171,370</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 0 | <div style='text-align: right'>759</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>126,324</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>74,646</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>6,175,232</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>174</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>96,684</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 0 | <div style='text-align: right'>406,065</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 0 | <div style='text-align: right'>2,306,325</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 0 | <span style="color: green">(-9,670 [-4.4%])</span> <div style='text-align: right'>207,700</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 0 | <div style='text-align: right'>260,567</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 0 | <div style='text-align: right'>20</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 0 | <div style='text-align: right'>41,607</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 0 | <div style='text-align: right'>142,516</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 0 | <div style='text-align: right'>8,338</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 0 | <div style='text-align: right'>4,927</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 0 | <div style='text-align: right'>13,376</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,133,322</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,419</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 0 | <div style='text-align: right'>850,996</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 0 | <div style='text-align: right'>11,286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 0 | <div style='text-align: right'>847,716</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 0 | <div style='text-align: right'>158,158</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 0 | <div style='text-align: right'>93,457</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 0 | <div style='text-align: right'>3,740</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>1,798,752</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>37,620</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>22,230</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 0 | <div style='text-align: right'>730,989</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 0 | <div style='text-align: right'>34,496</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 0 | <div style='text-align: right'>20,384</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 0 | <div style='text-align: right'>13,328</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 0 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>2,575,907</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>715</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>429</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>459</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>385</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 0 | <div style='text-align: right'>787,159</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 0 | <div style='text-align: right'>10,384</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>5,690,882</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>979</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>362,920</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 0 | <span style="color: green">(-264 [-0.1%])</span> <div style='text-align: right'>208,032</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 0 | <span style="color: green">(-156 [-0.1%])</span> <div style='text-align: right'>122,928</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>135,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 0 | <div style='text-align: right'>80,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 0 | <div style='text-align: right'>12,958</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 0 | <div style='text-align: right'>7,657</div>  |
| leaf_aggregation | Boundary | true | MulEF | MUL | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEFI | MUL | 0 | <div style='text-align: right'>15,600</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEFI | MUL | 0 | <div style='text-align: right'>2,684</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,586</div>  |
| leaf_aggregation | Boundary | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>65,840</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>74,338</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>43,927</div>  |
| leaf_aggregation | Boundary | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>4,312</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | 0 | <div style='text-align: right'>269,944</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 0 | <div style='text-align: right'>36,157</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 0 | <div style='text-align: right'>21,346</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 0 | <div style='text-align: right'>1,826,580</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 0 | <div style='text-align: right'>600</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 0 | <div style='text-align: right'>26,970</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 0 | <div style='text-align: right'>9,867</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 0 | <div style='text-align: right'>426,120</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 0 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 0 | <div style='text-align: right'>6,120</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 0 | <div style='text-align: right'>1,562</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 0 | <div style='text-align: right'>923</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 0 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>492,800</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>291,200</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>190,400</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>5,067,832</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>386,386</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>229,411</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>153,102</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>3,050,146</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 0 | <div style='text-align: right'>671,744</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 0 | <div style='text-align: right'>12,364</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 0 | <div style='text-align: right'>7,306</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 0 | <div style='text-align: right'>180,224</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>940,048</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>101,024</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>59,696</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>25,520</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 0 | <div style='text-align: right'>941,934</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 0 | <div style='text-align: right'>252,714</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>2,273,409</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>235,312</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>140,140</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>94,554</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>52,844</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>4,743,630</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>7,180,494</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>1,926,474</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 0 | <div style='text-align: right'>88,150</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 0 | <div style='text-align: right'>23,650</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>1,789,404</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>437,415</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>108,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>94,952</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>56,108</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 0 | <div style='text-align: right'>27,896</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 0 | <div style='text-align: right'>444,768</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 0 | <div style='text-align: right'>39,633</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 0 | <div style='text-align: right'>108,480</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 0 | <div style='text-align: right'>39,633</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 0 | <div style='text-align: right'>46,839</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 0 | <div style='text-align: right'>10,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,650</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 0 | <div style='text-align: right'>975</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 0 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 0 | <div style='text-align: right'>12,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 0 | <div style='text-align: right'>3,520</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 0 | <div style='text-align: right'>2,080</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 0 | <div style='text-align: right'>1,080,840</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 0 | <div style='text-align: right'>48,150</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 0 | <div style='text-align: right'>10,219</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 0 | <div style='text-align: right'>16,800</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d45a93e7498f567133092c9192f024d0042b9949/fibonacci-3-3-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/d45a93e7498f567133092c9192f024d0042b9949

Instance Type: 64cpu-linux-x64

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11885516793)
