| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,227,298</div>  | <div style='text-align: right'>2,596</div>  | <div style='text-align: right'>1,120.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>139,481,396</div>  | <div style='text-align: right'>3,388,559</div>  | <div style='text-align: right'>13,035.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
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
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |
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
| fibonacci_program | 0 | <div style='text-align: right'>1,120.0</div>  | <div style='text-align: right'>8,575,641</div>  | <div style='text-align: right'>1,227,298</div>  | <div style='text-align: right'>2,596</div>  | <div style='text-align: right'>17.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | ProgramChip | 0 | <div style='text-align: right'>5,874</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program | Boundary | 0 | <div style='text-align: right'>56</div>  |
| fibonacci_program | Merkle | 0 | <div style='text-align: right'>310</div>  |
| fibonacci_program | AccessAdapter<8> | 0 | <div style='text-align: right'>56</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>11</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>17</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>171</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>9</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>330</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>51</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>1</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>484</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,516</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>366</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| fibonacci_program |  | ADD | 0 | <div style='text-align: right'>1,502</div>  |
| fibonacci_program |  | AND | 0 | <div style='text-align: right'>5</div>  |
| fibonacci_program |  | AUIPC | 0 | <div style='text-align: right'>11</div>  |
| fibonacci_program |  | BEQ | 0 | <div style='text-align: right'>164</div>  |
| fibonacci_program |  | BGEU | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | BLTU | 0 | <div style='text-align: right'>6</div>  |
| fibonacci_program |  | BNE | 0 | <div style='text-align: right'>166</div>  |
| fibonacci_program |  | HINT_STOREW | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | JAL | 0 | <div style='text-align: right'>161</div>  |
| fibonacci_program |  | JALR | 0 | <div style='text-align: right'>17</div>  |
| fibonacci_program |  | LOADBU | 0 | <div style='text-align: right'>2</div>  |
| fibonacci_program |  | LOADW | 0 | <div style='text-align: right'>22</div>  |
| fibonacci_program |  | LUI | 0 | <div style='text-align: right'>10</div>  |
| fibonacci_program |  | OR | 0 | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | PHANTOM | 0 | <div style='text-align: right'>3</div>  |
| fibonacci_program |  | SLTU | 0 | <div style='text-align: right'>484</div>  |
| fibonacci_program |  | SRL | 0 | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | STOREB | 0 | <div style='text-align: right'>1</div>  |
| fibonacci_program |  | STOREW | 0 | <div style='text-align: right'>26</div>  |
| fibonacci_program |  | SUB | 0 | <div style='text-align: right'>4</div>  |
| fibonacci_program |  | XOR | 0 | <div style='text-align: right'>4</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>54,072</div>  |
| fibonacci_program | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>51</div>  |
| fibonacci_program | Boundary |  | ADD | 0 | <div style='text-align: right'>120</div>  |
| fibonacci_program | Merkle |  | ADD | 0 | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>231</div>  |
| fibonacci_program | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>4,264</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>96</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>192</div>  |
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>4,316</div>  |
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>78</div>  |
| fibonacci_program | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>128</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>2,898</div>  |
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>476</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>880</div>  |
| fibonacci_program | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | LOADW | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | LOADW | 0 | <div style='text-align: right'>2,432</div>  |
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>180</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>36</div>  |
| fibonacci_program | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>18</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>17,908</div>  |
| fibonacci_program | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>34</div>  |
| fibonacci_program | Boundary |  | SLTU | 0 | <div style='text-align: right'>80</div>  |
| fibonacci_program | Merkle |  | SLTU | 0 | <div style='text-align: right'>64</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>53</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>40</div>  |
| fibonacci_program | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>17</div>  |
| fibonacci_program | Boundary |  | STOREB | 0 | <div style='text-align: right'>40</div>  |
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>1,040</div>  |
| fibonacci_program | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>272</div>  |
| fibonacci_program | Boundary |  | STOREW | 0 | <div style='text-align: right'>640</div>  |
| fibonacci_program | Merkle |  | STOREW | 0 | <div style='text-align: right'>3,712</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>144</div>  |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>144</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | <div style='text-align: right'>39.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1,227,298</div>  | <div style='text-align: right'>2,596</div>  | <div style='text-align: right'>1,120.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>139,481,396</div>  | <div style='text-align: right'>3,388,559</div>  | <div style='text-align: right'>13,035.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>26,624</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>37,888</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>105</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>39,424</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>237,568</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,048</div>  |
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>3,473.0</div>  | <div style='text-align: right'>139,481,396</div>  | <div style='text-align: right'>3,388,559</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>412,092</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>395,684</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>198,094</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>56,780</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>27,149</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>144,732</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>34,478</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>1,288,865</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>68,140</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>649,637</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,105,334</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>209,586</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>1,105,928</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>6,226</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>11,479</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>17,545</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>632,092</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>16,223</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>128</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>13,342</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>3,431</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>5,334</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>68,140</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>150,600</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>351,958</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>130,294</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>10,926</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>209,586</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>236,501</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>189,015</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>177,260</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>52,515</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>33,177,840</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>201,476</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>119,054</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,135</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>249,040</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>121,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>71,526</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>459,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>300,960</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>177,840</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>403,535</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>14,538,116</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>819</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>651,948</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>385,242</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>251,889</div>  |
| leaf_verifier | Boundary |  | 0 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>9,068,657</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>3,840</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>533,680</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>244,662</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>144,573</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>137,240</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>71,422</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>151,580</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>89,570</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,262,848</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>681,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>418</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>494</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>6,174,600</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>281,798</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>132,145</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>20,179</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>14,430,278</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>54,406</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>32,149</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>476</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>3,908,820</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>22,957</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>13,585</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>585,200</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>347,438</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>230,741</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>6,107,634</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>1,257,516</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>9,696,541</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SHINTW | <div style='text-align: right'>22</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SHINTW | <div style='text-align: right'>26</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | SHINTW | <div style='text-align: right'>17</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>2,596,979</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>7,749,615</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>68,849</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>39,793</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>723,415</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>7,267,660</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>503,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>298,896</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>140,284</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>714,670</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>1,575,450</div>  |
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
| leaf_verifier | 0 | 0 | <div style='text-align: right'>13,035.0</div>  | <div style='text-align: right'>399,935,960</div>  | <div style='text-align: right'>849.0</div>  |

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms |
| --- | --- | --- | --- |
| <span style="color: red">(+1.0 [+25.0%])</span> <div style='text-align: right'>5.0</div>  | <span style="color: green">(-1,168.0 [-95.2%])</span> <div style='text-align: right'>59.0</div>  | <span style="color: green">(-921.0 [-95.7%])</span> <div style='text-align: right'>41.0</div>  | <span style="color: red">(+5.0 [+1.0%])</span> <div style='text-align: right'>490.0</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: green">(-247.0 [-93.6%])</span> <div style='text-align: right'>17.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c052d500fd423616de4bd658d68081e6ba05055f/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/c052d500fd423616de4bd658d68081e6ba05055f

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12271799273)
