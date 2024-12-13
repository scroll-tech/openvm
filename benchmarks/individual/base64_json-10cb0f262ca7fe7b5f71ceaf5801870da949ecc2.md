| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| base64_json_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>15,117,985</div>  | <div style='text-align: right'>434,694</div>  | <span style="color: green">(-83.0 [-4.0%])</span> <div style='text-align: right'>2,005.0</div>  |
| leaf | <div style='text-align: right'>2</div>  | <span style="color: green">(-15,800 [-0.0%])</span> <div style='text-align: right'>294,982,579</div>  | <span style="color: green">(-2,810 [-0.0%])</span> <div style='text-align: right'>13,576,644</div>  | <span style="color: green">(-290.0 [-1.0%])</span> <div style='text-align: right'>28,291.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <div style='text-align: right'>14.0</div>  | <span style="color: green">(-21.0 [-3.5%])</span> <div style='text-align: right'>584.0</div>  | <span style="color: green">(-26.0 [-5.7%])</span> <div style='text-align: right'>431.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-20.0 [-0.0%])</span> <div style='text-align: right'>75,386.0</div>  |

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
| KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
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

| segment | total_cycles | trace_gen_time_ms |
| --- | --- | --- |
| 0 | <div style='text-align: right'>217,347</div>  | <span style="color: red">(+4.0 [+2.7%])</span> <div style='text-align: right'>151.0</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | 0 | <span style="color: green">(-83.0 [-4.0%])</span> <div style='text-align: right'>2,005.0</div>  | <div style='text-align: right'>50,533,140</div>  | <div style='text-align: right'>15,117,985</div>  | <div style='text-align: right'>434,694</div>  | <span style="color: red">(+9.0 [+6.2%])</span> <div style='text-align: right'>155.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| base64_json_program | ProgramChip | 0 | <div style='text-align: right'>18,961</div>  |
| base64_json_program | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| base64_json_program | Boundary | 0 | <div style='text-align: right'>5,178</div>  |
| base64_json_program | Merkle | 0 | <div style='text-align: right'>5,526</div>  |
| base64_json_program | AccessAdapter<8> | 0 | <div style='text-align: right'>5,178</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,563</div>  |
| base64_json_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>86</div>  |
| base64_json_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <div style='text-align: right'>116</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| base64_json_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>1,331</div>  |
| base64_json_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,940</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>5,003</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,738</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>27,336</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,236</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>55,121</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,188</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>575</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>89,109</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| base64_json_program | PhantomAir | 0 | <div style='text-align: right'>5</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>10,704</div>  |
| base64_json_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| base64_json_program |  | ADD | 0 | <div style='text-align: right'>69,773</div>  |
| base64_json_program |  | AND | 0 | <div style='text-align: right'>10,124</div>  |
| base64_json_program |  | AUIPC | 0 | <div style='text-align: right'>1,331</div>  |
| base64_json_program |  | BEQ | 0 | <div style='text-align: right'>15,568</div>  |
| base64_json_program |  | BGE | 0 | <div style='text-align: right'>703</div>  |
| base64_json_program |  | BGEU | 0 | <div style='text-align: right'>6,863</div>  |
| base64_json_program |  | BLT | 0 | <div style='text-align: right'>3,354</div>  |
| base64_json_program |  | BLTU | 0 | <div style='text-align: right'>5,818</div>  |
| base64_json_program |  | BNE | 0 | <div style='text-align: right'>11,768</div>  |
| base64_json_program |  | HINT_STOREW | 0 | <div style='text-align: right'>1,563</div>  |
| base64_json_program |  | JAL | 0 | <div style='text-align: right'>3,685</div>  |
| base64_json_program |  | JALR | 0 | <div style='text-align: right'>2,940</div>  |
| base64_json_program |  | LOADB | 0 | <div style='text-align: right'>1,236</div>  |
| base64_json_program |  | LOADBU | 0 | <div style='text-align: right'>23,858</div>  |
| base64_json_program |  | LOADHU | 0 | <div style='text-align: right'>3</div>  |
| base64_json_program |  | LOADW | 0 | <div style='text-align: right'>13,465</div>  |
| base64_json_program |  | LUI | 0 | <div style='text-align: right'>1,318</div>  |
| base64_json_program |  | MUL | 0 | <div style='text-align: right'>116</div>  |
| base64_json_program |  | MULHU | 0 | <div style='text-align: right'>86</div>  |
| base64_json_program |  | OR | 0 | <div style='text-align: right'>7,608</div>  |
| base64_json_program |  | PHANTOM | 0 | <div style='text-align: right'>5</div>  |
| base64_json_program |  | SLL | 0 | <div style='text-align: right'>7,118</div>  |
| base64_json_program |  | SLT | 0 | <div style='text-align: right'>5</div>  |
| base64_json_program |  | SLTU | 0 | <div style='text-align: right'>570</div>  |
| base64_json_program |  | SRA | 0 | <div style='text-align: right'>8</div>  |
| base64_json_program |  | SRL | 0 | <div style='text-align: right'>9,062</div>  |
| base64_json_program |  | STOREB | 0 | <div style='text-align: right'>5,133</div>  |
| base64_json_program |  | STOREH | 0 | <div style='text-align: right'>10</div>  |
| base64_json_program |  | STOREW | 0 | <div style='text-align: right'>12,652</div>  |
| base64_json_program |  | SUB | 0 | <div style='text-align: right'>1,416</div>  |
| base64_json_program |  | XOR | 0 | <div style='text-align: right'>188</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>2,511,828</div>  |
| base64_json_program | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>85</div>  |
| base64_json_program | Boundary |  | ADD | 0 | <div style='text-align: right'>200</div>  |
| base64_json_program | Merkle |  | ADD | 0 | <div style='text-align: right'>128</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>364,464</div>  |
| base64_json_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>27,951</div>  |
| base64_json_program | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>51</div>  |
| base64_json_program | Boundary |  | AUIPC | 0 | <div style='text-align: right'>120</div>  |
| base64_json_program | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,520</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>404,768</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | 0 | <div style='text-align: right'>22,496</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>219,616</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>107,328</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>186,176</div>  |
| base64_json_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>305,968</div>  |
| base64_json_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>40,638</div>  |
| base64_json_program | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>13,277</div>  |
| base64_json_program | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>31,240</div>  |
| base64_json_program | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>50,304</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>66,330</div>  |
| base64_json_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>82,320</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <div style='text-align: right'>43,260</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>954,320</div>  |
| base64_json_program | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>2,856</div>  |
| base64_json_program | Boundary |  | LOADBU | 0 | <div style='text-align: right'>6,720</div>  |
| base64_json_program | Merkle |  | LOADBU | 0 | <div style='text-align: right'>12,288</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | 0 | <div style='text-align: right'>120</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>538,600</div>  |
| base64_json_program | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>1,921</div>  |
| base64_json_program | Boundary |  | LOADW | 0 | <div style='text-align: right'>4,520</div>  |
| base64_json_program | Merkle |  | LOADW | 0 | <div style='text-align: right'>12,224</div>  |
| base64_json_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>23,724</div>  |
| base64_json_program | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <div style='text-align: right'>3,596</div>  |
| base64_json_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>3,354</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>273,888</div>  |
| base64_json_program | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>30</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>377,254</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | 0 | <div style='text-align: right'>185</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>21,090</div>  |
| base64_json_program | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary |  | SLTU | 0 | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | 0 | <div style='text-align: right'>424</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>480,286</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>205,320</div>  |
| base64_json_program | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>10,472</div>  |
| base64_json_program | Boundary |  | STOREB | 0 | <div style='text-align: right'>24,640</div>  |
| base64_json_program | Merkle |  | STOREB | 0 | <div style='text-align: right'>39,296</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | 0 | <div style='text-align: right'>400</div>  |
| base64_json_program | AccessAdapter<8> |  | STOREH | 0 | <div style='text-align: right'>17</div>  |
| base64_json_program | Boundary |  | STOREH | 0 | <div style='text-align: right'>40</div>  |
| base64_json_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>506,080</div>  |
| base64_json_program | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>15,300</div>  |
| base64_json_program | Boundary |  | STOREW | 0 | <div style='text-align: right'>36,000</div>  |
| base64_json_program | Merkle |  | STOREW | 0 | <div style='text-align: right'>59,008</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>50,976</div>  |
| base64_json_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>6,768</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | <span style="color: green">(-32.0 [-6.7%])</span> <div style='text-align: right'>446.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>15,117,985</div>  | <div style='text-align: right'>434,694</div>  | <span style="color: green">(-83.0 [-4.0%])</span> <div style='text-align: right'>2,005.0</div>  |
| leaf |  | <div style='text-align: right'>2</div>  |  | <span style="color: green">(-15,800 [-0.0%])</span> <div style='text-align: right'>294,982,579</div>  | <span style="color: green">(-2,810 [-0.0%])</span> <div style='text-align: right'>13,576,644</div>  | <span style="color: green">(-290.0 [-1.0%])</span> <div style='text-align: right'>28,291.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| base64_json_program | ProgramAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| base64_json_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>425,984</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>1</div>  |
| base64_json_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>126,976</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>17,792</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>14,208</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>128</div>  |
| base64_json_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| base64_json_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| base64_json_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>32,768</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>2,048</div>  |
| base64_json_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>78,848</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>1,024</div>  |
| base64_json_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>131,072</div>  |
| base64_json_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| base64_json_program | PhantomAir | 0 | <div style='text-align: right'>144</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  |
| base64_json_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>10,272,768</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>16,384</div>  |
| base64_json_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | idx | execute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | <span style="color: green">(-3,186.0 [-26.3%])</span> <div style='text-align: right'>8,946.0</div>  | <span style="color: green">(-290.0 [-1.0%])</span> <div style='text-align: right'>28,291.0</div>  | <div style='text-align: right'>787,286,488</div>  | <span style="color: green">(-15,800 [-0.0%])</span> <div style='text-align: right'>294,982,579</div>  | <span style="color: green">(-1,405 [-0.0%])</span> <div style='text-align: right'>6,788,322</div>  |

| group | chip_name | idx | rows_used |
| --- | --- | --- | --- |
| leaf | ProgramChip | 0 | <div style='text-align: right'>311,121</div>  |
| leaf | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf | Boundary | 0 | <div style='text-align: right'>1,015,583</div>  |
| leaf | AccessAdapter<2> | 0 | <span style="color: green">(-100 [-0.0%])</span> <div style='text-align: right'>1,064,808</div>  |
| leaf | AccessAdapter<4> | 0 | <span style="color: green">(-50 [-0.0%])</span> <div style='text-align: right'>532,614</div>  |
| leaf | AccessAdapter<8> | 0 | <div style='text-align: right'>108,110</div>  |
| leaf | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>52,814</div>  |
| leaf | FriReducedOpeningAir | 0 | <div style='text-align: right'>550,368</div>  |
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>107,074</div>  |
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,624,831</div>  |
| leaf | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: green">(-1,405 [-1.6%])</span> <div style='text-align: right'>83,860</div>  |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,363,651</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,928,200</div>  |
| leaf | PhantomAir | 0 | <div style='text-align: right'>621,514</div>  |
| leaf | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | idx | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf |  | 0 | ADD | <div style='text-align: right'>2,361,192</div>  |
| leaf |  | 0 | BBE4DIV | <div style='text-align: right'>7,254</div>  |
| leaf |  | 0 | BBE4MUL | <div style='text-align: right'>36,463</div>  |
| leaf |  | 0 | BEQ | <div style='text-align: right'>17,775</div>  |
| leaf |  | 0 | BNE | <div style='text-align: right'>1,345,876</div>  |
| leaf |  | 0 | COMP_POS2 | <div style='text-align: right'>16,433</div>  |
| leaf |  | 0 | DIV | <div style='text-align: right'>156</div>  |
| leaf |  | 0 | FE4ADD | <div style='text-align: right'>46,757</div>  |
| leaf |  | 0 | FE4SUB | <div style='text-align: right'>16,600</div>  |
| leaf |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>6,342</div>  |
| leaf |  | 0 | JAL | <span style="color: green">(-1,405 [-1.6%])</span> <div style='text-align: right'>83,860</div>  |
| leaf |  | 0 | LOADW | <div style='text-align: right'>195,212</div>  |
| leaf |  | 0 | LOADW2 | <div style='text-align: right'>612,920</div>  |
| leaf |  | 0 | MUL | <div style='text-align: right'>185,920</div>  |
| leaf |  | 0 | PERM_POS2 | <div style='text-align: right'>36,381</div>  |
| leaf |  | 0 | PHANTOM | <div style='text-align: right'>621,514</div>  |
| leaf |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf |  | 0 | SHINTW | <div style='text-align: right'>480,191</div>  |
| leaf |  | 0 | STOREW | <div style='text-align: right'>248,487</div>  |
| leaf |  | 0 | STOREW2 | <div style='text-align: right'>391,390</div>  |
| leaf |  | 0 | SUB | <div style='text-align: right'>77,563</div>  |

| group | air_name | dsl_ir | idx | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>70,835,760</div>  |
| leaf | AccessAdapter<2> |  | 0 | ADD | <span style="color: green">(-550 [-0.1%])</span> <div style='text-align: right'>611,820</div>  |
| leaf | AccessAdapter<4> |  | 0 | ADD | <span style="color: green">(-325 [-0.1%])</span> <div style='text-align: right'>361,530</div>  |
| leaf | Boundary |  | 0 | ADD | <div style='text-align: right'>767,943</div>  |
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>290,160</div>  |
| leaf | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>143,858</div>  |
| leaf | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>85,007</div>  |
| leaf | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>352</div>  |
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>1,458,520</div>  |
| leaf | AccessAdapter<2> |  | 0 | BBE4MUL | <span style="color: green">(-550 [-0.1%])</span> <div style='text-align: right'>1,087,240</div>  |
| leaf | AccessAdapter<4> |  | 0 | BBE4MUL | <span style="color: green">(-325 [-0.1%])</span> <div style='text-align: right'>642,460</div>  |
| leaf | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>1,037,080</div>  |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>408,825</div>  |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>30,955,148</div>  |
| leaf | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>1,474</div>  |
| leaf | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>871</div>  |
| leaf | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>655,644</div>  |
| leaf | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>387,426</div>  |
| leaf | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>253,317</div>  |
| leaf | Boundary |  | 0 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>9,186,047</div>  |
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>4,680</div>  |
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>1,870,280</div>  |
| leaf | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>1,345,080</div>  |
| leaf | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>794,820</div>  |
| leaf | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>1,380,324</div>  |
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>664,000</div>  |
| leaf | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>539,198</div>  |
| leaf | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>318,617</div>  |
| leaf | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>574,816</div>  |
| leaf | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>378,840</div>  |
| leaf | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>223,860</div>  |
| leaf | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>35,223,552</div>  |
| leaf | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <span style="color: green">(-14,050 [-1.6%])</span> <div style='text-align: right'>838,600</div>  |
| leaf | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>506</div>  |
| leaf | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>598</div>  |
| leaf | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>8,003,692</div>  |
| leaf | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>531,773</div>  |
| leaf | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>273,286</div>  |
| leaf | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>20,179</div>  |
| leaf | Boundary |  | 0 | LOADW | <div style='text-align: right'>382,239</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>25,129,720</div>  |
| leaf | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>54,406</div>  |
| leaf | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>32,149</div>  |
| leaf | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>476</div>  |
| leaf | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,408</div>  |
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>5,577,600</div>  |
| leaf | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>27,203</div>  |
| leaf | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>16,094</div>  |
| leaf | Boundary |  | 0 | MUL | <div style='text-align: right'>112,376</div>  |
| leaf | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,706,584</div>  |
| leaf | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,009,801</div>  |
| leaf | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>665,618</div>  |
| leaf | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>20,336,979</div>  |
| leaf | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>3,729,084</div>  |
| leaf | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>19,687,831</div>  |
| leaf | AccessAdapter<2> |  | 0 | SHINTW | <div style='text-align: right'>22</div>  |
| leaf | AccessAdapter<4> |  | 0 | SHINTW | <div style='text-align: right'>26</div>  |
| leaf | AccessAdapter<8> |  | 0 | SHINTW | <div style='text-align: right'>17</div>  |
| leaf | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,276,931</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>10,187,967</div>  |
| leaf | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>143,990</div>  |
| leaf | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>84,110</div>  |
| leaf | AccessAdapter<8> |  | 0 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf | Boundary |  | 0 | STOREW | <div style='text-align: right'>864,787</div>  |
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>16,046,990</div>  |
| leaf | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>1,642,916</div>  |
| leaf | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>972,179</div>  |
| leaf | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>570,877</div>  |
| leaf | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>757,878</div>  |
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>2,326,890</div>  |
| leaf | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>70,411</div>  |
| leaf | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>83,213</div>  |
| leaf | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |

| group | idx | segment | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- |
| leaf | 0 | 0 | <span style="color: green">(-1,405 [-0.0%])</span> <div style='text-align: right'>6,788,322</div>  | <span style="color: red">(+34.0 [+2.2%])</span> <div style='text-align: right'>1,577.0</div>  |

| group | air_name | idx | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf | VolatileBoundaryAir | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf | AccessAdapterAir<2> | 0 | <div style='text-align: right'>56,623,104</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf | AccessAdapterAir<4> | 0 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf | AccessAdapterAir<8> | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf | FriReducedOpeningAir | 0 | <div style='text-align: right'>146,800,640</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf | PhantomAir | 0 | <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-base64_json_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2/base64_json-leaf.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/10cb0f262ca7fe7b5f71ceaf5801870da949ecc2

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12308371010)
