| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>51,310,997</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+71.0 [+0.8%])</span> <div style='text-align: right'>8,492.0</div>  |
| leaf_aggregation | <span style="color: red">(+25,860 [+0.0%])</span> <div style='text-align: right'>308,296,941</div>  | <span style="color: red">(+2,307 [+0.0%])</span> <div style='text-align: right'>7,885,939</div>  | <span style="color: red">(+306.0 [+0.3%])</span> <div style='text-align: right'>103,374.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| fibonacci_program | true | <span style="color: green">(-147.0 [-0.4%])</span> <div style='text-align: right'>37,271.0</div>  | <div style='text-align: right'>51,310,997</div>  | <div style='text-align: right'>1,500,219</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>900,085</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>300,004</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>4</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>200,012</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>100,012</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>17</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>57</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | Memory AccessAdapter<8> | true | <div style='text-align: right'>28</div>  |
| fibonacci_program | Memory Boundary | true | <div style='text-align: right'>56</div>  |
| fibonacci_program | Memory Merkle | true | <div style='text-align: right'>310</div>  |
| fibonacci_program | PhantomAir | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | ProgramChip | true | <div style='text-align: right'>5,658</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

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
| fibonacci_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>128</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>1,800,036</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>476</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>880</div>  |
| fibonacci_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary | true |  | LOADW | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle | true |  | LOADW | <div style='text-align: right'>2,432</div>  |
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
| fibonacci_program | Merkle | true |  | STOREW | <div style='text-align: right'>3,712</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>144</div>  |

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
| fibonacci_program | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| fibonacci_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>3.0</div>  | <span style="color: red">(+47.0 [+2.6%])</span> <div style='text-align: right'>1,828.0</div>  | <span style="color: red">(+50.0 [+3.3%])</span> <div style='text-align: right'>1,579.0</div>  | <span style="color: red">(+4.0 [+3.2%])</span> <div style='text-align: right'>128.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>51,310,997</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+71.0 [+0.8%])</span> <div style='text-align: right'>8,492.0</div>  |
| leaf_aggregation |  |  |  |  |  | <span style="color: red">(+25,860 [+0.0%])</span> <div style='text-align: right'>308,296,941</div>  | <span style="color: red">(+2,307 [+0.0%])</span> <div style='text-align: right'>7,885,939</div>  | <span style="color: red">(+306.0 [+0.3%])</span> <div style='text-align: right'>103,374.0</div>  |

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
| fibonacci_program | Poseidon2VmAir<BabyBear> | 0 | <div style='text-align: right'>321,024</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | AccessAdapterAir<16> | 0 |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<32> | 0 |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<64> | 0 |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>255,852,544</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>90,177,536</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>192,937,984</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | FriMatOpeningAir | 0 | <div style='text-align: right'>60,817,408</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBear> | 0 | <div style='text-align: right'>29,229,056</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 |  |  |  |  |  | <span style="color: red">(+24.0 [+0.4%])</span> <div style='text-align: right'>6,664.0</div>  | <div style='text-align: right'>196,595,668</div>  |  |
| leaf_aggregation | 0 | <span style="color: green">(-9.0 [-10.5%])</span> <div style='text-align: right'>77.0</div>  | <span style="color: red">(+166.0 [+1.2%])</span> <div style='text-align: right'>14,215.0</div>  | <span style="color: red">(+92.0 [+0.7%])</span> <div style='text-align: right'>12,572.0</div>  | <span style="color: red">(+5.0 [+8.3%])</span> <div style='text-align: right'>65.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+140.0 [+0.2%])</span> <div style='text-align: right'>89,159.0</div>  | <div style='text-align: right'>714,080,280</div>  | <span style="color: red">(+6.0 [+2.2%])</span> <div style='text-align: right'>276.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: green">(-643.0 [-0.5%])</span> <div style='text-align: right'>122,675.0</div>  | <span style="color: red">(+25,860 [+0.0%])</span> <div style='text-align: right'>308,296,941</div>  | <span style="color: red">(+2,307 [+0.0%])</span> <div style='text-align: right'>7,885,939</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>1,527,586</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: red">(+2,255 [+1.4%])</span> <div style='text-align: right'>165,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <span style="color: red">(+52 [+0.0%])</span> <div style='text-align: right'>3,003,572</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>2,555,249</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>61,127</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 0 | <div style='text-align: right'>344,600</div>  |
| leaf_aggregation | Memory AccessAdapter<2> | true | 0 | <span style="color: red">(+100 [+0.0%])</span> <div style='text-align: right'>618,533</div>  |
| leaf_aggregation | Memory AccessAdapter<4> | true | 0 | <span style="color: red">(+50 [+0.0%])</span> <div style='text-align: right'>309,868</div>  |
| leaf_aggregation | Memory AccessAdapter<8> | true | 0 | <div style='text-align: right'>89,151</div>  |
| leaf_aggregation | Memory Boundary | true | 0 | <div style='text-align: right'>904,533</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>496,150</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBear> | true | 0 | <div style='text-align: right'>63,639</div>  |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>124,795</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | JAL | 0 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 0 | <div style='text-align: right'>22,785</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>234</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>702</div>  |
| leaf_aggregation | true | AddEFI | ADD | 0 | <div style='text-align: right'>312</div>  |
| leaf_aggregation | true | AddEI | ADD | 0 | <div style='text-align: right'>44,372</div>  |
| leaf_aggregation | true | AddFI | ADD | 0 | <span style="color: red">(+52 [+0.1%])</span> <div style='text-align: right'>98,394</div>  |
| leaf_aggregation | true | AddV | ADD | 0 | <div style='text-align: right'>32,526</div>  |
| leaf_aggregation | true | AddVI | ADD | 0 | <div style='text-align: right'>801,330</div>  |
| leaf_aggregation | true | Alloc | ADD | 0 | <div style='text-align: right'>129,920</div>  |
| leaf_aggregation | true | Alloc | LOADW | 0 | <div style='text-align: right'>129,920</div>  |
| leaf_aggregation | true | Alloc | MUL | 0 | <div style='text-align: right'>79,085</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 0 | <div style='text-align: right'>480</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 0 | <div style='text-align: right'>22,501</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 0 | <div style='text-align: right'>5,364</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>237</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>1,600</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>16,400</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>8,200</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>4,000</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>359,400</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>25,400</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>1,600</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>4,000</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>12,200</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>12,200</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>200</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>14,740</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>54</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 0 | <div style='text-align: right'>216</div>  |
| leaf_aggregation | true | DivFIN | DIV | 0 | <div style='text-align: right'>128</div>  |
| leaf_aggregation | true | For | ADD | 0 | <div style='text-align: right'>960,108</div>  |
| leaf_aggregation | true | For | BNE | 0 | <div style='text-align: right'>1,060,742</div>  |
| leaf_aggregation | true | For | JAL | 0 | <div style='text-align: right'>100,634</div>  |
| leaf_aggregation | true | For | LOADW | 0 | <div style='text-align: right'>6,200</div>  |
| leaf_aggregation | true | For | STOREW | 0 | <div style='text-align: right'>94,434</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>12,700</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>101</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>50,835</div>  |
| leaf_aggregation | true | IfEq | BNE | 0 | <div style='text-align: right'>62,777</div>  |
| leaf_aggregation | true | IfEqI | BNE | 0 | <div style='text-align: right'>334,679</div>  |
| leaf_aggregation | true | IfEqI | JAL | 0 | <span style="color: red">(+2,255 [+3.6%])</span> <div style='text-align: right'>65,257</div>  |
| leaf_aggregation | true | IfNe | BEQ | 0 | <div style='text-align: right'>34,601</div>  |
| leaf_aggregation | true | IfNe | JAL | 0 | <div style='text-align: right'>24</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 0 | <div style='text-align: right'>6,201</div>  |
| leaf_aggregation | true | ImmE | STOREW | 0 | <div style='text-align: right'>4,916</div>  |
| leaf_aggregation | true | ImmF | STOREW | 0 | <div style='text-align: right'>98,306</div>  |
| leaf_aggregation | true | ImmV | STOREW | 0 | <div style='text-align: right'>74,713</div>  |
| leaf_aggregation | true | LoadE | LOADW | 0 | <div style='text-align: right'>37,956</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>156,480</div>  |
| leaf_aggregation | true | LoadF | LOADW | 0 | <div style='text-align: right'>63,045</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>223,348</div>  |
| leaf_aggregation | true | LoadV | LOADW | 0 | <div style='text-align: right'>60,545</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>457,138</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>14,865</div>  |
| leaf_aggregation | true | MulEF | MUL | 0 | <div style='text-align: right'>8,432</div>  |
| leaf_aggregation | true | MulEFI | MUL | 0 | <div style='text-align: right'>520</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>1,646</div>  |
| leaf_aggregation | true | MulEI | STOREW | 0 | <div style='text-align: right'>6,584</div>  |
| leaf_aggregation | true | MulF | MUL | 0 | <div style='text-align: right'>184,182</div>  |
| leaf_aggregation | true | MulFI | MUL | 0 | <div style='text-align: right'>20</div>  |
| leaf_aggregation | true | MulV | MUL | 0 | <div style='text-align: right'>3,131</div>  |
| leaf_aggregation | true | MulVI | MUL | 0 | <div style='text-align: right'>44,748</div>  |
| leaf_aggregation | true | NegE | MUL | 0 | <div style='text-align: right'>204</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>37,700</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>25,939</div>  |
| leaf_aggregation | true | StoreE | STOREW | 0 | <div style='text-align: right'>57,852</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>81,680</div>  |
| leaf_aggregation | true | StoreF | STOREW | 0 | <div style='text-align: right'>81,726</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>196,706</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>483,042</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>537,008</div>  |
| leaf_aggregation | true | StoreV | STOREW | 0 | <div style='text-align: right'>6,614</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>140,644</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 0 | <div style='text-align: right'>7,037</div>  |
| leaf_aggregation | true | SubEF | LOADW | 0 | <div style='text-align: right'>38,280</div>  |
| leaf_aggregation | true | SubEF | SUB | 0 | <div style='text-align: right'>12,760</div>  |
| leaf_aggregation | true | SubEFI | ADD | 0 | <div style='text-align: right'>356</div>  |
| leaf_aggregation | true | SubEI | ADD | 0 | <div style='text-align: right'>432</div>  |
| leaf_aggregation | true | SubV | SUB | 0 | <div style='text-align: right'>112,020</div>  |
| leaf_aggregation | true | SubVI | SUB | 0 | <div style='text-align: right'>5,550</div>  |
| leaf_aggregation | true | SubVIN | SUB | 0 | <div style='text-align: right'>2,000</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>911,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>396,946</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>234,559</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 0 | <div style='text-align: right'>114,576</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>9,594</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,100</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,300</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>28,782</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>1,100</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 0 | <div style='text-align: right'>9,360</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,452</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 0 | <div style='text-align: right'>858</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 0 | <div style='text-align: right'>1,331,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 0 | <span style="color: red">(+550 [+0.2%])</span> <div style='text-align: right'>249,480</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 0 | <span style="color: red">(+325 [+0.2%])</span> <div style='text-align: right'>147,420</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 0 | <div style='text-align: right'>135,564</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 0 | <span style="color: red">(+1,560 [+0.1%])</span> <div style='text-align: right'>2,951,820</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 0 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 0 | <div style='text-align: right'>975,780</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 0 | <div style='text-align: right'>24,039,900</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 0 | <div style='text-align: right'>35,673</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 0 | <div style='text-align: right'>3,897,600</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 0 | <div style='text-align: right'>5,326,720</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 0 | <div style='text-align: right'>1,837</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 0 | <div style='text-align: right'>2,372,550</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 0 | <div style='text-align: right'>26</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>11,040</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>2,640</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>1,560</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 0 | <div style='text-align: right'>517,523</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 0 | <div style='text-align: right'>123,372</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>5,451</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>9,600</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>98,400</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>49,200</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>24,000</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>2,156,400</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>152,400</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>9,600</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>24,000</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>73,200</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>73,200</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>1,200</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>589,600</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>280,742</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>165,893</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,288</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1,352</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>8,856</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>781</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>221</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 0 | <div style='text-align: right'>3,840</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 0 | <div style='text-align: right'>28,803,240</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 0 | <div style='text-align: right'>24,397,066</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 0 | <div style='text-align: right'>1,006,340</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 0 | <div style='text-align: right'>638</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 0 | <div style='text-align: right'>754</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 0 | <div style='text-align: right'>254,200</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 0 | <div style='text-align: right'>1,100</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 0 | <div style='text-align: right'>3,871,794</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 0 | <div style='text-align: right'>913</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>256,212</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>151,398</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>22,054,400</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>606</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>305,010</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 0 | <div style='text-align: right'>1,443,871</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 0 | <div style='text-align: right'>7,697,617</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 0 | <span style="color: red">(+22,550 [+3.6%])</span> <div style='text-align: right'>652,570</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 0 | <div style='text-align: right'>795,823</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 0 | <div style='text-align: right'>240</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 0 | <div style='text-align: right'>142,623</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 0 | <div style='text-align: right'>201,556</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 0 | <div style='text-align: right'>9,922</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 0 | <div style='text-align: right'>5,863</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 0 | <div style='text-align: right'>13,376</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 0 | <div style='text-align: right'>4,030,546</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 0 | <div style='text-align: right'>2,222</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 0 | <div style='text-align: right'>3,063,233</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 0 | <div style='text-align: right'>37,444</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 0 | <div style='text-align: right'>1,556,196</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 0 | <div style='text-align: right'>221,518</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 0 | <div style='text-align: right'>130,897</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 0 | <div style='text-align: right'>3,740</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>6,415,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>134,244</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>79,326</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 0 | <div style='text-align: right'>2,584,845</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 0 | <div style='text-align: right'>123,200</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 0 | <div style='text-align: right'>72,800</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 0 | <div style='text-align: right'>47,600</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 0 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>9,157,268</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>1,111</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>663</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>612</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>1,166</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 0 | <div style='text-align: right'>2,482,345</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 0 | <div style='text-align: right'>34,958</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>18,742,658</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>935</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>594,600</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 0 | <span style="color: red">(+550 [+0.2%])</span> <div style='text-align: right'>290,598</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 0 | <span style="color: red">(+325 [+0.2%])</span> <div style='text-align: right'>171,717</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>135,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 0 | <div style='text-align: right'>252,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 0 | <div style='text-align: right'>44,638</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 0 | <div style='text-align: right'>26,377</div>  |
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
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 0 | <div style='text-align: right'>35,937</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 0 | <div style='text-align: right'>21,086</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 0 | <div style='text-align: right'>5,525,460</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 0 | <div style='text-align: right'>600</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 0 | <div style='text-align: right'>93,930</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 0 | <div style='text-align: right'>34,408</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 0 | <div style='text-align: right'>1,342,440</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 0 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 0 | <div style='text-align: right'>6,120</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 0 | <div style='text-align: right'>1,562</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 0 | <div style='text-align: right'>923</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 0 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>1,513,600</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>894,400</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>584,800</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBear> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>15,758,600</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>1,375,627</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>816,777</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>545,292</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBear> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>10,842,502</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 0 | <div style='text-align: right'>2,371,932</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 0 | <div style='text-align: right'>44,044</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 0 | <div style='text-align: right'>26,026</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 0 | <div style='text-align: right'>636,372</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>3,348,880</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>360,800</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>213,200</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>88,880</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 0 | <div style='text-align: right'>3,350,766</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 0 | <div style='text-align: right'>898,986</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>8,064,946</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>837,661</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>498,888</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>337,263</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>179,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>14,491,260</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>22,017,328</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>5,907,088</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 0 | <div style='text-align: right'>271,174</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 0 | <div style='text-align: right'>72,754</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>5,766,404</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>1,535,919</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>281,480</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>253,352</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>149,708</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 0 | <div style='text-align: right'>27,896</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 0 | <div style='text-align: right'>1,569,480</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 0 | <div style='text-align: right'>140,217</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 0 | <div style='text-align: right'>382,800</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 0 | <div style='text-align: right'>140,217</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 0 | <div style='text-align: right'>165,711</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 0 | <div style='text-align: right'>10,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,650</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 0 | <div style='text-align: right'>975</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 0 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 0 | <div style='text-align: right'>12,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 0 | <div style='text-align: right'>3,520</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 0 | <div style='text-align: right'>2,080</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 0 | <div style='text-align: right'>3,360,600</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 0 | <div style='text-align: right'>166,500</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 0 | <div style='text-align: right'>35,486</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 0 | <div style='text-align: right'>60,000</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/fibonacci-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/64fa08bc809386a4f6ba9de679c3d68a342af0a9
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11691040106)
