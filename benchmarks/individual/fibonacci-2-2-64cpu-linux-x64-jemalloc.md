| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>51,645,721</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+33.0 [+0.5%])</span> <div style='text-align: right'>7,002.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <span style="color: green">(-1,770 [-0.0%])</span> <div style='text-align: right'>461,059,797</div>  | <span style="color: green">(-72 [-0.0%])</span> <div style='text-align: right'>3,506,780</div>  | <span style="color: green">(-658.0 [-1.8%])</span> <div style='text-align: right'>35,387.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>6.0</div>  | <span style="color: green">(-24.0 [-2.4%])</span> <div style='text-align: right'>961.0</div>  | <span style="color: green">(-19.0 [-2.3%])</span> <div style='text-align: right'>807.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+6.0 [+2.7%])</span> <div style='text-align: right'>225.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>51,645,721</div>  | <div style='text-align: right'>1,500,219</div>  | <span style="color: red">(+33.0 [+0.5%])</span> <div style='text-align: right'>7,002.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-1,770 [-0.0%])</span> <div style='text-align: right'>461,059,797</div>  | <span style="color: green">(-72 [-0.0%])</span> <div style='text-align: right'>3,506,780</div>  | <span style="color: green">(-658.0 [-1.8%])</span> <div style='text-align: right'>35,387.0</div>  |

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

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program | ProgramChip | <div style='text-align: right'>6,614</div>  |
| fibonacci_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| fibonacci_program | Boundary | <div style='text-align: right'>54</div>  |
| fibonacci_program | Merkle | <div style='text-align: right'>308</div>  |
| fibonacci_program | AccessAdapter<8> | <div style='text-align: right'>54</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>3</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>17</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>100,012</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>200,012</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>57</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>4</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>300,004</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>900,085</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | <div style='text-align: right'>3</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>362</div>  |
| fibonacci_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_program |  | ADD | <div style='text-align: right'>900,068</div>  |
| fibonacci_program |  | AND | <div style='text-align: right'>5</div>  |
| fibonacci_program |  | AUIPC | <div style='text-align: right'>11</div>  |
| fibonacci_program |  | BEQ | <div style='text-align: right'>100,005</div>  |
| fibonacci_program |  | BGEU | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | BLT | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | BLTU | <div style='text-align: right'>7</div>  |
| fibonacci_program |  | BNE | <div style='text-align: right'>100,007</div>  |
| fibonacci_program |  | HINT_STOREW | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | JAL | <div style='text-align: right'>100,002</div>  |
| fibonacci_program |  | JALR | <div style='text-align: right'>17</div>  |
| fibonacci_program |  | LOADBU | <div style='text-align: right'>6</div>  |
| fibonacci_program |  | LOADW | <div style='text-align: right'>22</div>  |
| fibonacci_program |  | LUI | <div style='text-align: right'>10</div>  |
| fibonacci_program |  | OR | <div style='text-align: right'>4</div>  |
| fibonacci_program |  | PHANTOM | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | SLL | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | SLTU | <div style='text-align: right'>300,004</div>  |
| fibonacci_program |  | SRL | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | STOREB | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | STOREW | <div style='text-align: right'>28</div>  |
| fibonacci_program |  | SUB | <div style='text-align: right'>4</div>  |
| fibonacci_program |  | XOR | <div style='text-align: right'>4</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>32,402,448</div>  |
| fibonacci_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>51</div>  |
| fibonacci_program | Boundary |  | ADD | <div style='text-align: right'>120</div>  |
| fibonacci_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>231</div>  |
| fibonacci_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>2,600,130</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>96</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>32</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>224</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>2,600,182</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| fibonacci_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>17</div>  |
| fibonacci_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>40</div>  |
| fibonacci_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>128</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>1,800,036</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>476</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>880</div>  |
| fibonacci_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | LOADW | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | LOADW | <div style='text-align: right'>2,304</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>144</div>  |
| fibonacci_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>18</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>159</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>11,100,148</div>  |
| fibonacci_program | AccessAdapter<8> |  | SLTU | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | SLTU | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | SLTU | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>53</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>17</div>  |
| fibonacci_program | Boundary |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>1,120</div>  |
| fibonacci_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>272</div>  |
| fibonacci_program | Boundary |  | STOREW | <div style='text-align: right'>640</div>  |
| fibonacci_program | Merkle |  | STOREW | <div style='text-align: right'>3,776</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>144</div>  |

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
| leaf_aggregation | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>8,650,752</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>623,902,720</div>  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | FriReducedOpeningAir | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 |  | <span style="color: green">(-17.0 [-9.2%])</span> <div style='text-align: right'>167.0</div>  |  |  |  |  | <span style="color: red">(+74.0 [+1.3%])</span> <div style='text-align: right'>5,874.0</div>  | <div style='text-align: right'>197,775,316</div>  |  |  |  |
| leaf_aggregation | 0 | <span style="color: green">(-2.0 [-4.3%])</span> <div style='text-align: right'>44.0</div>  | <span style="color: green">(-24.0 [-0.3%])</span> <div style='text-align: right'>7,235.0</div>  | <span style="color: red">(+288.0 [+5.1%])</span> <div style='text-align: right'>5,982.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-3.0 [-4.9%])</span> <div style='text-align: right'>58.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-634.0 [-2.2%])</span> <div style='text-align: right'>28,152.0</div>  | <div style='text-align: right'>1,072,529,432</div>  | <span style="color: green">(-1,770 [-0.0%])</span> <div style='text-align: right'>461,059,797</div>  | <span style="color: green">(-72 [-0.0%])</span> <div style='text-align: right'>3,506,780</div>  | <span style="color: green">(-5.0 [-2.0%])</span> <div style='text-align: right'>245.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | 0 | <div style='text-align: right'>104,521</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | 0 | <div style='text-align: right'>170,804</div>  |
| leaf_aggregation | Merkle | 0 | <div style='text-align: right'>366,044</div>  |
| leaf_aggregation | AccessAdapter<2> | 0 | <span style="color: green">(-60 [-0.0%])</span> <div style='text-align: right'>639,930</div>  |
| leaf_aggregation | AccessAdapter<4> | 0 | <span style="color: green">(-30 [-0.0%])</span> <div style='text-align: right'>353,758</div>  |
| leaf_aggregation | AccessAdapter<8> | 0 | <div style='text-align: right'>189,886</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>536,848</div>  |
| leaf_aggregation | FriReducedOpeningAir | 0 | <div style='text-align: right'>144,732</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>34,795</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>1,356,374</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: green">(-72 [-0.1%])</span> <div style='text-align: right'>73,406</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>674,446</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,124,581</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>209,865</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| leaf_aggregation |  | ADD | 0 | <div style='text-align: right'>1,151,443</div>  |
| leaf_aggregation |  | BBE4DIV | 0 | <div style='text-align: right'>6,268</div>  |
| leaf_aggregation |  | BBE4MUL | 0 | <div style='text-align: right'>11,846</div>  |
| leaf_aggregation |  | BEQ | 0 | <div style='text-align: right'>18,472</div>  |
| leaf_aggregation |  | BNE | 0 | <div style='text-align: right'>655,974</div>  |
| leaf_aggregation |  | COMP_POS2 | 0 | <div style='text-align: right'>17,052</div>  |
| leaf_aggregation |  | DIV | 0 | <div style='text-align: right'>128</div>  |
| leaf_aggregation |  | FE4ADD | 0 | <div style='text-align: right'>13,124</div>  |
| leaf_aggregation |  | FE4SUB | 0 | <div style='text-align: right'>3,557</div>  |
| leaf_aggregation |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>5,334</div>  |
| leaf_aggregation |  | JAL | 0 | <span style="color: green">(-72 [-0.1%])</span> <div style='text-align: right'>73,406</div>  |
| leaf_aggregation |  | LOADW | 0 | <div style='text-align: right'>153,132</div>  |
| leaf_aggregation |  | LOADW2 | 0 | <div style='text-align: right'>360,337</div>  |
| leaf_aggregation |  | MUL | 0 | <div style='text-align: right'>145,522</div>  |
| leaf_aggregation |  | PERM_POS2 | 0 | <div style='text-align: right'>10,927</div>  |
| leaf_aggregation |  | PHANTOM | 0 | <div style='text-align: right'>209,865</div>  |
| leaf_aggregation |  | SHINTW | 0 | <div style='text-align: right'>245,092</div>  |
| leaf_aggregation |  | STOREW | 0 | <div style='text-align: right'>186,383</div>  |
| leaf_aggregation |  | STOREW2 | 0 | <div style='text-align: right'>179,637</div>  |
| leaf_aggregation |  | SUB | 0 | <div style='text-align: right'>59,281</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <div style='text-align: right'>34,543,290</div>  |
| leaf_aggregation | AccessAdapter<2> |  | ADD | 0 | <span style="color: green">(-330 [-0.1%])</span> <div style='text-align: right'>266,090</div>  |
| leaf_aggregation | AccessAdapter<4> |  | ADD | 0 | <span style="color: green">(-195 [-0.1%])</span> <div style='text-align: right'>157,456</div>  |
| leaf_aggregation | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>40,630</div>  |
| leaf_aggregation | Boundary |  | ADD | 0 | <div style='text-align: right'>95,600</div>  |
| leaf_aggregation | Merkle |  | ADD | 0 | <div style='text-align: right'>304,512</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>250,720</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>120,670</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>71,305</div>  |
| leaf_aggregation | AccessAdapter<8> |  | BBE4DIV | 0 | <div style='text-align: right'>34</div>  |
| leaf_aggregation | Boundary |  | BBE4DIV | 0 | <div style='text-align: right'>80</div>  |
| leaf_aggregation | Merkle |  | BBE4DIV | 0 | <div style='text-align: right'>192</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>473,840</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BBE4MUL | 0 | <span style="color: green">(-330 [-0.1%])</span> <div style='text-align: right'>231,770</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BBE4MUL | 0 | <span style="color: green">(-195 [-0.1%])</span> <div style='text-align: right'>136,955</div>  |
| leaf_aggregation | AccessAdapter<8> |  | BBE4MUL | 0 | <div style='text-align: right'>34,221</div>  |
| leaf_aggregation | Boundary |  | BBE4MUL | 0 | <div style='text-align: right'>80,520</div>  |
| leaf_aggregation | Merkle |  | BBE4MUL | 0 | <div style='text-align: right'>33,088</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>424,856</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>15,087,402</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>1,386</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>819</div>  |
| leaf_aggregation | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>689,304</div>  |
| leaf_aggregation | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>407,316</div>  |
| leaf_aggregation | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>266,322</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>9,532,068</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>3,840</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>524,960</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>181,588</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>107,302</div>  |
| leaf_aggregation | AccessAdapter<8> |  | FE4ADD | 0 | <div style='text-align: right'>26,078</div>  |
| leaf_aggregation | Boundary |  | FE4ADD | 0 | <div style='text-align: right'>61,360</div>  |
| leaf_aggregation | Merkle |  | FE4ADD | 0 | <div style='text-align: right'>55,296</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>142,280</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>112,486</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>66,469</div>  |
| leaf_aggregation | AccessAdapter<8> |  | FE4SUB | 0 | <div style='text-align: right'>8,381</div>  |
| leaf_aggregation | Boundary |  | FE4SUB | 0 | <div style='text-align: right'>19,720</div>  |
| leaf_aggregation | Merkle |  | FE4SUB | 0 | <div style='text-align: right'>1,536</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>151,580</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>89,570</div>  |
| leaf_aggregation | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>9,262,848</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: green">(-720 [-0.1%])</span> <div style='text-align: right'>734,060</div>  |
| leaf_aggregation | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>418</div>  |
| leaf_aggregation | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>494</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>6,278,412</div>  |
| leaf_aggregation | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>294,404</div>  |
| leaf_aggregation | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>143,858</div>  |
| leaf_aggregation | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>31,994</div>  |
| leaf_aggregation | Boundary |  | LOADW | 0 | <div style='text-align: right'>28,240</div>  |
| leaf_aggregation | Merkle |  | LOADW | 0 | <div style='text-align: right'>46,016</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>14,773,817</div>  |
| leaf_aggregation | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>58,069</div>  |
| leaf_aggregation | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>34,450</div>  |
| leaf_aggregation | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>1,326</div>  |
| leaf_aggregation | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,960</div>  |
| leaf_aggregation | Merkle |  | LOADW2 | 0 | <div style='text-align: right'>2,816</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>4,365,660</div>  |
| leaf_aggregation | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>33,099</div>  |
| leaf_aggregation | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>23,608</div>  |
| leaf_aggregation | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>11,407</div>  |
| leaf_aggregation | Boundary |  | MUL | 0 | <div style='text-align: right'>26,840</div>  |
| leaf_aggregation | Merkle |  | MUL | 0 | <div style='text-align: right'>43,840</div>  |
| leaf_aggregation | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>578,776</div>  |
| leaf_aggregation | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>343,642</div>  |
| leaf_aggregation | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>229,330</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>6,108,193</div>  |
| leaf_aggregation | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>1,259,190</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>10,048,772</div>  |
| leaf_aggregation | AccessAdapter<2> |  | SHINTW | 0 | <div style='text-align: right'>1,491,017</div>  |
| leaf_aggregation | AccessAdapter<4> |  | SHINTW | 0 | <div style='text-align: right'>1,051,154</div>  |
| leaf_aggregation | AccessAdapter<8> |  | SHINTW | 0 | <div style='text-align: right'>934,388</div>  |
| leaf_aggregation | Boundary |  | SHINTW | 0 | <div style='text-align: right'>2,198,560</div>  |
| leaf_aggregation | Merkle |  | SHINTW | 0 | <div style='text-align: right'>7,699,136</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>7,641,703</div>  |
| leaf_aggregation | AccessAdapter<2> |  | STOREW | 0 | <div style='text-align: right'>426,866</div>  |
| leaf_aggregation | AccessAdapter<4> |  | STOREW | 0 | <div style='text-align: right'>266,669</div>  |
| leaf_aggregation | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>186,626</div>  |
| leaf_aggregation | Boundary |  | STOREW | 0 | <div style='text-align: right'>439,120</div>  |
| leaf_aggregation | Merkle |  | STOREW | 0 | <div style='text-align: right'>2,744,064</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>7,365,117</div>  |
| leaf_aggregation | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>874,159</div>  |
| leaf_aggregation | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>534,196</div>  |
| leaf_aggregation | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>317,067</div>  |
| leaf_aggregation | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>412,600</div>  |
| leaf_aggregation | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>700,608</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>1,778,430</div>  |
| leaf_aggregation | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>74,107</div>  |
| leaf_aggregation | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>86,723</div>  |
| leaf_aggregation | AccessAdapter<8> |  | SUB | 0 | <div style='text-align: right'>21,879</div>  |
| leaf_aggregation | Boundary |  | SUB | 0 | <div style='text-align: right'>51,480</div>  |
| leaf_aggregation | Merkle |  | SUB | 0 | <div style='text-align: right'>82,240</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e65e535dcbdc37d163179bccb4f6ffa8704e4618/fibonacci-2-2-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/e65e535dcbdc37d163179bccb4f6ffa8704e4618

Instance Type: 64cpu-linux-x64

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12184466081)
