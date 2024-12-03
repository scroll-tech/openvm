| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>51,645,664</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+40.0 [+0.6%])</span> <div style='text-align: right'>6,968.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <span style="color: green">(-4,310 [-0.0%])</span> <div style='text-align: right'>143,611,669</div>  | <span style="color: green">(-284 [-0.0%])</span> <div style='text-align: right'>3,505,975</div>  | <span style="color: red">(+176.0 [+0.9%])</span> <div style='text-align: right'>19,452.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| fibonacci_program | true | <span style="color: green">(-5.0 [-0.1%])</span> <div style='text-align: right'>5,957.0</div>  | <div style='text-align: right'>51,645,664</div>  | <div style='text-align: right'>1,500,219</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | ProgramChip | true | <div style='text-align: right'>6,547</div>  |
| fibonacci_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| fibonacci_program | Boundary | true | <div style='text-align: right'>56</div>  |
| fibonacci_program | Merkle | true | <div style='text-align: right'>306</div>  |
| fibonacci_program | AccessAdapter<8> | true | <div style='text-align: right'>56</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>17</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>100,012</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>200,012</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>57</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>4</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>300,004</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>900,085</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | true | <div style='text-align: right'>3</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>362</div>  |
| fibonacci_program | VariableRangeCheckerAir | true | <div style='text-align: right'>262,144</div>  |

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
| fibonacci_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>64</div>  |
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
| fibonacci_program | <span style="color: red">(+1.0 [+16.7%])</span> <div style='text-align: right'>7.0</div>  | <span style="color: red">(+30.0 [+3.1%])</span> <div style='text-align: right'>994.0</div>  | <span style="color: red">(+22.0 [+2.8%])</span> <div style='text-align: right'>815.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-6.0 [-2.8%])</span> <div style='text-align: right'>209.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>51,645,664</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+40.0 [+0.6%])</span> <div style='text-align: right'>6,968.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-4,310 [-0.0%])</span> <div style='text-align: right'>143,611,669</div>  | <span style="color: green">(-284 [-0.0%])</span> <div style='text-align: right'>3,505,975</div>  | <span style="color: red">(+176.0 [+0.9%])</span> <div style='text-align: right'>19,452.0</div>  |

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
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>26,624</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,048</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>32</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>420</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| leaf_aggregation | FriReducedOpeningAir | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 |  | <span style="color: red">(+8.0 [+4.7%])</span> <div style='text-align: right'>178.0</div>  |  |  |  |  | <span style="color: red">(+2.0 [+0.0%])</span> <div style='text-align: right'>5,796.0</div>  | <div style='text-align: right'>197,775,316</div>  |  |
| leaf_aggregation | 0 | <span style="color: red">(+1.0 [+1.9%])</span> <div style='text-align: right'>54.0</div>  | <span style="color: red">(+92.0 [+2.3%])</span> <div style='text-align: right'>4,017.0</div>  | <span style="color: red">(+80.0 [+2.4%])</span> <div style='text-align: right'>3,422.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+3.0 [+5.2%])</span> <div style='text-align: right'>61.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+84.0 [+0.5%])</span> <div style='text-align: right'>15,435.0</div>  | <div style='text-align: right'>399,933,464</div>  | <span style="color: green">(-15.0 [-6.1%])</span> <div style='text-align: right'>232.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: red">(+167.0 [+1.0%])</span> <div style='text-align: right'>17,072.0</div>  | <span style="color: green">(-4,310 [-0.0%])</span> <div style='text-align: right'>143,611,669</div>  | <span style="color: green">(-284 [-0.0%])</span> <div style='text-align: right'>3,505,975</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>104,521</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <div style='text-align: right'>421,828</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <span style="color: green">(-84 [-0.0%])</span> <div style='text-align: right'>401,080</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <span style="color: green">(-42 [-0.0%])</span> <div style='text-align: right'>200,792</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>58,312</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>27,979</div>  |
| leaf_aggregation | FriReducedOpeningAir | true | 0 | <div style='text-align: right'>144,732</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>34,795</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <div style='text-align: right'>1,356,374</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: green">(-284 [-0.4%])</span> <div style='text-align: right'>72,601</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>674,446</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>1,124,581</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>209,865</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | ADD | 0 | <div style='text-align: right'>1,151,443</div>  |
| leaf_aggregation | true |  | BBE4DIV | 0 | <div style='text-align: right'>6,268</div>  |
| leaf_aggregation | true |  | BBE4MUL | 0 | <div style='text-align: right'>11,846</div>  |
| leaf_aggregation | true |  | BEQ | 0 | <div style='text-align: right'>18,472</div>  |
| leaf_aggregation | true |  | BNE | 0 | <div style='text-align: right'>655,974</div>  |
| leaf_aggregation | true |  | COMP_POS2 | 0 | <div style='text-align: right'>17,052</div>  |
| leaf_aggregation | true |  | DIV | 0 | <div style='text-align: right'>128</div>  |
| leaf_aggregation | true |  | FE4ADD | 0 | <div style='text-align: right'>13,124</div>  |
| leaf_aggregation | true |  | FE4SUB | 0 | <div style='text-align: right'>3,557</div>  |
| leaf_aggregation | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>5,334</div>  |
| leaf_aggregation | true |  | JAL | 0 | <span style="color: green">(-284 [-0.4%])</span> <div style='text-align: right'>72,601</div>  |
| leaf_aggregation | true |  | LOADW | 0 | <div style='text-align: right'>153,132</div>  |
| leaf_aggregation | true |  | LOADW2 | 0 | <div style='text-align: right'>360,337</div>  |
| leaf_aggregation | true |  | MUL | 0 | <div style='text-align: right'>145,522</div>  |
| leaf_aggregation | true |  | PERM_POS2 | 0 | <div style='text-align: right'>10,927</div>  |
| leaf_aggregation | true |  | PHANTOM | 0 | <div style='text-align: right'>209,865</div>  |
| leaf_aggregation | true |  | SHINTW | 0 | <div style='text-align: right'>245,092</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>186,383</div>  |
| leaf_aggregation | true |  | STOREW2 | 0 | <div style='text-align: right'>179,637</div>  |
| leaf_aggregation | true |  | SUB | 0 | <div style='text-align: right'>59,281</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | ADD | 0 | <div style='text-align: right'>34,543,290</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | ADD | 0 | <span style="color: green">(-462 [-0.2%])</span> <div style='text-align: right'>195,008</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | ADD | 0 | <span style="color: green">(-273 [-0.2%])</span> <div style='text-align: right'>115,232</div>  |
| leaf_aggregation | Boundary | true |  | ADD | 0 | <div style='text-align: right'>141,559</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4DIV | 0 | <div style='text-align: right'>250,720</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4DIV | 0 | <div style='text-align: right'>121,022</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4DIV | 0 | <div style='text-align: right'>71,513</div>  |
| leaf_aggregation | Boundary | true |  | BBE4DIV | 0 | <div style='text-align: right'>704</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4MUL | 0 | <div style='text-align: right'>473,840</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4MUL | 0 | <span style="color: green">(-462 [-0.2%])</span> <div style='text-align: right'>302,170</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4MUL | 0 | <span style="color: green">(-273 [-0.2%])</span> <div style='text-align: right'>178,555</div>  |
| leaf_aggregation | Boundary | true |  | BBE4MUL | 0 | <div style='text-align: right'>141,240</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BEQ | 0 | <div style='text-align: right'>424,856</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BNE | 0 | <div style='text-align: right'>15,087,402</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BNE | 0 | <div style='text-align: right'>1,386</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BNE | 0 | <div style='text-align: right'>819</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>689,304</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>407,316</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>266,322</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>9,532,068</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | DIV | 0 | <div style='text-align: right'>3,840</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4ADD | 0 | <div style='text-align: right'>524,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4ADD | 0 | <div style='text-align: right'>237,490</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4ADD | 0 | <div style='text-align: right'>140,335</div>  |
| leaf_aggregation | Boundary | true |  | FE4ADD | 0 | <div style='text-align: right'>111,804</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4SUB | 0 | <div style='text-align: right'>142,280</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4SUB | 0 | <div style='text-align: right'>125,598</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4SUB | 0 | <div style='text-align: right'>74,217</div>  |
| leaf_aggregation | Boundary | true |  | FE4SUB | 0 | <div style='text-align: right'>26,224</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>151,580</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>89,570</div>  |
| leaf_aggregation | FriReducedOpeningAir | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>9,262,848</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <span style="color: green">(-2,840 [-0.4%])</span> <div style='text-align: right'>726,010</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | JAL | 0 | <div style='text-align: right'>407</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | JAL | 0 | <div style='text-align: right'>481</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW | 0 | <div style='text-align: right'>6,278,412</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW | 0 | <div style='text-align: right'>283,349</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW | 0 | <div style='text-align: right'>133,081</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW | 0 | <div style='text-align: right'>19,992</div>  |
| leaf_aggregation | Boundary | true |  | LOADW | 0 | <div style='text-align: right'>21,747</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW2 | 0 | <div style='text-align: right'>14,773,817</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW2 | 0 | <div style='text-align: right'>57,200</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW2 | 0 | <div style='text-align: right'>33,800</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW2 | 0 | <div style='text-align: right'>493</div>  |
| leaf_aggregation | Boundary | true |  | LOADW2 | 0 | <div style='text-align: right'>1,672</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | MUL | 0 | <div style='text-align: right'>4,365,660</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | MUL | 0 | <div style='text-align: right'>24,057</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | MUL | 0 | <div style='text-align: right'>14,235</div>  |
| leaf_aggregation | Boundary | true |  | MUL | 0 | <div style='text-align: right'>32,736</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>578,776</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>343,642</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>229,330</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>6,108,193</div>  |
| leaf_aggregation | PhantomAir | true |  | PHANTOM | 0 | <div style='text-align: right'>1,259,190</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | SHINTW | 0 | <div style='text-align: right'>10,048,772</div>  |
| leaf_aggregation | Boundary | true |  | SHINTW | 0 | <div style='text-align: right'>2,696,012</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>7,641,703</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW | 0 | <div style='text-align: right'>65,285</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW | 0 | <div style='text-align: right'>37,674</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>713,680</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW2 | 0 | <div style='text-align: right'>7,365,117</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW2 | 0 | <div style='text-align: right'>503,998</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW2 | 0 | <div style='text-align: right'>299,455</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | STOREW2 | 0 | <div style='text-align: right'>141,712</div>  |
| leaf_aggregation | Boundary | true |  | STOREW2 | 0 | <div style='text-align: right'>737,539</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | SUB | 0 | <div style='text-align: right'>1,778,430</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | SUB | 0 | <div style='text-align: right'>59,224</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | SUB | 0 | <div style='text-align: right'>69,992</div>  |
| leaf_aggregation | Boundary | true |  | SUB | 0 | <div style='text-align: right'>15,180</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/14a79fbb495d36c2cd970329a6d0f972943a24b3/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/14a79fbb495d36c2cd970329a6d0f972943a24b3

Instance Type: 64cpu-linux-x64

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12131278254)
