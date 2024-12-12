| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>51,612,244</div>  | <div style='text-align: right'>1,500,137</div>  | <span style="color: red">(+876.0 [+17.0%])</span> <div style='text-align: right'>6,042.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,229,413</div>  | <div style='text-align: right'>3,519,706</div>  | <div style='text-align: right'>15,940.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <span style="color: red">(+1.0 [+25.0%])</span> <div style='text-align: right'>5.0</div>  | <span style="color: green">(-343.0 [-29.0%])</span> <div style='text-align: right'>841.0</div>  | <span style="color: green">(-250.0 [-27.0%])</span> <div style='text-align: right'>677.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+38.0 [+7.7%])</span> <div style='text-align: right'>530.0</div>  |

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>4</div>  |
| VolatileBoundaryAir | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| FriReducedOpeningAir | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>4</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 | <span style="color: red">(+876.0 [+17.0%])</span> <div style='text-align: right'>6,042.0</div>  | <div style='text-align: right'>197,696,030</div>  | <div style='text-align: right'>51,612,244</div>  | <div style='text-align: right'>1,500,137</div>  | <span style="color: green">(-70.0 [-29.2%])</span> <div style='text-align: right'>170.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | ProgramChip | 0 | <div style='text-align: right'>3,335</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program | Boundary | 0 | <div style='text-align: right'>36</div>  |
| fibonacci_program | Merkle | 0 | <div style='text-align: right'>280</div>  |
| fibonacci_program | AccessAdapter<8> | 0 | <div style='text-align: right'>36</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>9</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>13</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>100,010</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>5</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>200,009</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>28</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>300,002</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>900,054</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>316</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| fibonacci_program |  | ADD | 0 | <div style='text-align: right'>900,045</div>  |
| fibonacci_program |  | AND | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program |  | AUIPC | 0 | <div style='text-align: right'>9</div>  |
| fibonacci_program |  | BEQ | 0 | <div style='text-align: right'>100,004</div>  |
| fibonacci_program |  | BGEU | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | BLTU | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program |  | BNE | 0 | <div style='text-align: right'>100,005</div>  |
| fibonacci_program |  | HINT_STOREW | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | JAL | 0 | <div style='text-align: right'>100,001</div>  |
| fibonacci_program |  | JALR | 0 | <div style='text-align: right'>13</div>  |
| fibonacci_program |  | LOADW | 0 | <div style='text-align: right'>13</div>  |
| fibonacci_program |  | LUI | 0 | <div style='text-align: right'>9</div>  |
| fibonacci_program |  | OR | 0 | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program |  | SLL | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program |  | SLTU | 0 | <div style='text-align: right'>300,002</div>  |
| fibonacci_program |  | STOREW | 0 | <div style='text-align: right'>15</div>  |
| fibonacci_program |  | SUB | 0 | <div style='text-align: right'>4</div>  |
| fibonacci_program |  | XOR | 0 | <div style='text-align: right'>2</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>32,401,620</div>  |
| fibonacci_program | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>68</div>  |
| fibonacci_program | Boundary |  | ADD | 0 | <div style='text-align: right'>160</div>  |
| fibonacci_program | Merkle |  | ADD | 0 | <div style='text-align: right'>320</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>72</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>189</div>  |
| fibonacci_program | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>2,600,104</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>96</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>2,600,130</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>78</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>1,800,018</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>364</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>520</div>  |
| fibonacci_program | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | LOADW | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | LOADW | 0 | <div style='text-align: right'>2,304</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>162</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>36</div>  |
| fibonacci_program | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>106</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>11,100,074</div>  |
| fibonacci_program | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | SLTU | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>600</div>  |
| fibonacci_program | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>136</div>  |
| fibonacci_program | Boundary |  | STOREW | 0 | <div style='text-align: right'>320</div>  |
| fibonacci_program | Merkle |  | STOREW | 0 | <div style='text-align: right'>2,816</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>144</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>72</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | <span style="color: green">(-230.0 [-24.8%])</span> <div style='text-align: right'>697.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>51,612,244</div>  | <div style='text-align: right'>1,500,137</div>  | <span style="color: red">(+876.0 [+17.0%])</span> <div style='text-align: right'>6,042.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>144,229,413</div>  | <div style='text-align: right'>3,519,706</div>  | <div style='text-align: right'>15,940.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | ProgramAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>26,624</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>704</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>8</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>3,584</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>210</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>2</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>36</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>1,937.0</div>  | <div style='text-align: right'>144,229,413</div>  | <div style='text-align: right'>3,519,706</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>424,581</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>404,840</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>202,672</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>58,714</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>28,074</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>144,732</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>35,074</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>1,355,464</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>73,865</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>676,191</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,134,653</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>211,015</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>1,153,076</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>6,268</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>11,820</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>18,557</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>657,634</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>17,189</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>128</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>13,429</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>3,557</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>5,334</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>73,865</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>155,907</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>360,784</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>143,987</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>10,885</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>211,015</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>245,582</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>192,376</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>180,004</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>58,273</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>34,592,280</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>204,490</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>120,835</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,135</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>250,720</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>121,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>71,526</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>472,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>304,106</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>179,699</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>426,811</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>15,125,582</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>819</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>694,452</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>410,358</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>268,311</div>  |
| leaf_verifier | Boundary |  | 0 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>9,608,651</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>3,840</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>537,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>246,554</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>145,691</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>142,280</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>125,488</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>74,152</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>151,580</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>89,570</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,262,848</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>738,650</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>418</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>494</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>6,392,187</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>285,538</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>134,381</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>20,893</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>14,792,144</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>57,200</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>33,800</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>493</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>4,319,610</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>23,881</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>14,131</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>583,396</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>346,372</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>230,758</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>6,084,715</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>1,266,090</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>10,068,862</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SHINTW | <div style='text-align: right'>22</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SHINTW | <div style='text-align: right'>26</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | SHINTW | <div style='text-align: right'>17</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>2,696,870</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>7,887,416</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>69,905</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>40,391</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>735,889</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>7,380,164</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>497,530</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>295,633</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>138,856</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>739,684</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>1,748,190</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>59,235</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>70,005</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>15,940.0</div>  | <div style='text-align: right'>399,935,960</div>  | <div style='text-align: right'>565.0</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: green">(-93.0 [-36.3%])</span> <div style='text-align: right'>163.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/955b7fb27f04174fb652a9db77b45fecd0a2a294/fibonacci-38e0b9de817f645c4bec37c0d4a3e58baecccb040f5718dc069a72c7385a0bed-leaf_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/955b7fb27f04174fb652a9db77b45fecd0a2a294

Max Segment Length: 1048476

Instance Type: 64cpu-linux-x64

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12288504336)
