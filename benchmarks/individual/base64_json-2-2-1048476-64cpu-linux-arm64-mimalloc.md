| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <div style='text-align: right'>881,896,337</div>  | <div style='text-align: right'>6,995,780</div>  | <span style="color: red">(+999.0 [+2.5%])</span> <div style='text-align: right'>40,440.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-8.0 [-0.0%])</span> <div style='text-align: right'>23,324.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <span style="color: green">(-1.0 [-6.7%])</span> <div style='text-align: right'>14.0</div>  | <span style="color: red">(+13.0 [+2.7%])</span> <div style='text-align: right'>497.0</div>  | <span style="color: red">(+8.0 [+2.5%])</span> <div style='text-align: right'>324.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-966.0 [-1.3%])</span> <div style='text-align: right'>74,441.0</div>  |

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
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
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 |  |  | <span style="color: green">(-20.0 [-0.1%])</span> <div style='text-align: right'>13,879.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>1</div>  | <span style="color: red">(+999.0 [+2.5%])</span> <div style='text-align: right'>40,440.0</div>  | <div style='text-align: right'>1,465,057,304</div>  | <div style='text-align: right'>881,896,337</div>  | <div style='text-align: right'>6,995,780</div>  | <span style="color: green">(-63.0 [-0.6%])</span> <div style='text-align: right'>9,878.0</div>  |  |
| leaf_aggregation | 0 | <span style="color: green">(-5.0 [-2.8%])</span> <div style='text-align: right'>171.0</div>  | <span style="color: green">(-8.0 [-0.0%])</span> <div style='text-align: right'>23,324.0</div>  | <span style="color: red">(+73.0 [+0.5%])</span> <div style='text-align: right'>13,591.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+2.0 [+0.6%])</span> <div style='text-align: right'>363.0</div>  |  |  |  |  |  | <span style="color: green">(-81.0 [-0.8%])</span> <div style='text-align: right'>9,718.0</div>  | <span style="color: red">(+22.0 [+0.0%])</span> <div style='text-align: right'>74,466.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <div style='text-align: right'>308,776</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <div style='text-align: right'>407,254</div>  |
| app_proof | Merkle | 0 | <div style='text-align: right'>594,448</div>  |
| app_proof | AccessAdapter<2> | 0 | <div style='text-align: right'>1,329,318</div>  |
| app_proof | AccessAdapter<4> | 0 | <div style='text-align: right'>698,006</div>  |
| app_proof | AccessAdapter<8> | 0 | <div style='text-align: right'>476,740</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,563</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>86</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <div style='text-align: right'>116</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>1,331</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,940</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>5,003</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,738</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>27,336</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,236</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>55,121</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,188</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>575</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>89,109</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>621,078</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>1,001,702</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>550,368</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>107,074</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,622,083</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>83,471</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,362,708</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>1,922,916</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <div style='text-align: right'>2,358,883</div>  |
| app_proof |  | AND | 0 | <div style='text-align: right'>10,124</div>  |
| app_proof |  | AUIPC | 0 | <div style='text-align: right'>1,331</div>  |
| app_proof |  | BEQ | 0 | <div style='text-align: right'>17,774</div>  |
| app_proof |  | BGE | 0 | <div style='text-align: right'>703</div>  |
| app_proof |  | BGEU | 0 | <div style='text-align: right'>6,863</div>  |
| app_proof |  | BLT | 0 | <div style='text-align: right'>3,354</div>  |
| app_proof |  | BLTU | 0 | <div style='text-align: right'>5,818</div>  |
| app_proof |  | BNE | 0 | <div style='text-align: right'>1,344,934</div>  |
| app_proof |  | HINT_STOREW | 0 | <div style='text-align: right'>1,563</div>  |
| app_proof |  | JAL | 0 | <div style='text-align: right'>83,471</div>  |
| app_proof |  | JALR | 0 | <div style='text-align: right'>2,940</div>  |
| app_proof |  | LOADB | 0 | <div style='text-align: right'>1,236</div>  |
| app_proof |  | LOADBU | 0 | <div style='text-align: right'>23,858</div>  |
| app_proof |  | LOADHU | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | LOADW | 0 | <div style='text-align: right'>192,563</div>  |
| app_proof |  | LUI | 0 | <div style='text-align: right'>1,318</div>  |
| app_proof |  | MUL | 0 | <div style='text-align: right'>185,481</div>  |
| app_proof |  | MULHU | 0 | <div style='text-align: right'>86</div>  |
| app_proof |  | OR | 0 | <div style='text-align: right'>7,608</div>  |
| app_proof |  | PHANTOM | 0 | <div style='text-align: right'>621,078</div>  |
| app_proof |  | SLL | 0 | <div style='text-align: right'>7,118</div>  |
| app_proof |  | SLT | 0 | <div style='text-align: right'>5</div>  |
| app_proof |  | SLTU | 0 | <div style='text-align: right'>570</div>  |
| app_proof |  | SRA | 0 | <div style='text-align: right'>8</div>  |
| app_proof |  | SRL | 0 | <div style='text-align: right'>9,062</div>  |
| app_proof |  | STOREB | 0 | <div style='text-align: right'>5,133</div>  |
| app_proof |  | STOREH | 0 | <div style='text-align: right'>10</div>  |
| app_proof |  | STOREW | 0 | <div style='text-align: right'>247,198</div>  |
| app_proof |  | SUB | 0 | <div style='text-align: right'>77,563</div>  |
| app_proof |  | XOR | 0 | <div style='text-align: right'>188</div>  |
| app_proof |  | BBE4DIV | 0 | <div style='text-align: right'>7,254</div>  |
| app_proof |  | BBE4MUL | 0 | <div style='text-align: right'>36,463</div>  |
| app_proof |  | COMP_POS2 | 0 | <div style='text-align: right'>16,380</div>  |
| app_proof |  | DIV | 0 | <div style='text-align: right'>156</div>  |
| app_proof |  | FE4ADD | 0 | <div style='text-align: right'>46,757</div>  |
| app_proof |  | FE4SUB | 0 | <div style='text-align: right'>16,600</div>  |
| app_proof |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>6,342</div>  |
| app_proof |  | LOADW2 | 0 | <div style='text-align: right'>612,893</div>  |
| app_proof |  | PERM_POS2 | 0 | <div style='text-align: right'>36,381</div>  |
| app_proof |  | SHINTW | 0 | <div style='text-align: right'>479,323</div>  |
| app_proof |  | STOREW2 | 0 | <div style='text-align: right'>390,939</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>2,511,828</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>294,355</div>  |
| app_proof | Boundary |  | ADD | 0 | <div style='text-align: right'>692,600</div>  |
| app_proof | Merkle |  | ADD | 0 | <div style='text-align: right'>2,039,040</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>364,464</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>27,951</div>  |
| app_proof | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>51</div>  |
| app_proof | Boundary |  | AUIPC | 0 | <div style='text-align: right'>120</div>  |
| app_proof | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,520</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>404,768</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | 0 | <div style='text-align: right'>22,496</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>219,616</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>107,328</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>186,176</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>305,968</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>40,638</div>  |
| app_proof | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>13,277</div>  |
| app_proof | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>31,240</div>  |
| app_proof | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>50,240</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>66,330</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>82,320</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <div style='text-align: right'>43,260</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>954,320</div>  |
| app_proof | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>2,856</div>  |
| app_proof | Boundary |  | LOADBU | 0 | <div style='text-align: right'>6,720</div>  |
| app_proof | Merkle |  | LOADBU | 0 | <div style='text-align: right'>12,352</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | 0 | <div style='text-align: right'>120</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>538,600</div>  |
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>124,763</div>  |
| app_proof | Boundary |  | LOADW | 0 | <div style='text-align: right'>248,200</div>  |
| app_proof | Merkle |  | LOADW | 0 | <div style='text-align: right'>397,504</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>23,724</div>  |
| app_proof | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <div style='text-align: right'>3,596</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>3,354</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>273,888</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>3,726,468</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>377,254</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | 0 | <div style='text-align: right'>185</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>21,090</div>  |
| app_proof | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | SLTU | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | 0 | <div style='text-align: right'>424</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>480,286</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>205,320</div>  |
| app_proof | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>10,472</div>  |
| app_proof | Boundary |  | STOREB | 0 | <div style='text-align: right'>24,640</div>  |
| app_proof | Merkle |  | STOREB | 0 | <div style='text-align: right'>39,232</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | 0 | <div style='text-align: right'>400</div>  |
| app_proof | AccessAdapter<8> |  | STOREH | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | STOREH | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>506,080</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>238,901</div>  |
| app_proof | Boundary |  | STOREW | 0 | <div style='text-align: right'>562,120</div>  |
| app_proof | Merkle |  | STOREW | 0 | <div style='text-align: right'>2,794,880</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>50,976</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>6,768</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <div style='text-align: right'>70,766,490</div>  |
| app_proof | AccessAdapter<2> |  | ADD | 0 | <div style='text-align: right'>995,863</div>  |
| app_proof | AccessAdapter<4> |  | ADD | 0 | <div style='text-align: right'>588,666</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>290,160</div>  |
| app_proof | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>143,682</div>  |
| app_proof | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>84,903</div>  |
| app_proof | AccessAdapter<8> |  | BBE4DIV | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | BBE4DIV | 0 | <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | BBE4DIV | 0 | <div style='text-align: right'>384</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>1,458,520</div>  |
| app_proof | AccessAdapter<2> |  | BBE4MUL | 0 | <div style='text-align: right'>568,634</div>  |
| app_proof | AccessAdapter<4> |  | BBE4MUL | 0 | <div style='text-align: right'>336,011</div>  |
| app_proof | AccessAdapter<8> |  | BBE4MUL | 0 | <div style='text-align: right'>395,947</div>  |
| app_proof | Boundary |  | BBE4MUL | 0 | <div style='text-align: right'>931,640</div>  |
| app_proof | Merkle |  | BBE4MUL | 0 | <div style='text-align: right'>458,752</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>408,802</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>30,933,482</div>  |
| app_proof | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>1,474</div>  |
| app_proof | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>871</div>  |
| app_proof | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>648,648</div>  |
| app_proof | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>383,292</div>  |
| app_proof | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>250,614</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>9,156,420</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>4,680</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>1,870,280</div>  |
| app_proof | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>654,918</div>  |
| app_proof | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>386,997</div>  |
| app_proof | AccessAdapter<8> |  | FE4ADD | 0 | <div style='text-align: right'>530,825</div>  |
| app_proof | Boundary |  | FE4ADD | 0 | <div style='text-align: right'>1,249,000</div>  |
| app_proof | Merkle |  | FE4ADD | 0 | <div style='text-align: right'>2,400,064</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>664,000</div>  |
| app_proof | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>251,790</div>  |
| app_proof | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>148,785</div>  |
| app_proof | AccessAdapter<8> |  | FE4SUB | 0 | <div style='text-align: right'>221,697</div>  |
| app_proof | Boundary |  | FE4SUB | 0 | <div style='text-align: right'>521,640</div>  |
| app_proof | Merkle |  | FE4SUB | 0 | <div style='text-align: right'>252,160</div>  |
| app_proof | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>378,840</div>  |
| app_proof | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>223,860</div>  |
| app_proof | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>35,223,552</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <div style='text-align: right'>834,710</div>  |
| app_proof | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>506</div>  |
| app_proof | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>598</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>7,895,083</div>  |
| app_proof | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>720,720</div>  |
| app_proof | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>389,129</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>25,128,613</div>  |
| app_proof | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>55,275</div>  |
| app_proof | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>32,799</div>  |
| app_proof | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>1,309</div>  |
| app_proof | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,960</div>  |
| app_proof | Merkle |  | LOADW2 | 0 | <div style='text-align: right'>3,008</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>5,564,430</div>  |
| app_proof | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>76,065</div>  |
| app_proof | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>48,997</div>  |
| app_proof | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>42,789</div>  |
| app_proof | Boundary |  | MUL | 0 | <div style='text-align: right'>100,680</div>  |
| app_proof | Merkle |  | MUL | 0 | <div style='text-align: right'>168,576</div>  |
| app_proof | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,706,584</div>  |
| app_proof | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,009,801</div>  |
| app_proof | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>665,618</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>20,336,979</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>19,652,243</div>  |
| app_proof | AccessAdapter<2> |  | SHINTW | 0 | <div style='text-align: right'>2,778,094</div>  |
| app_proof | AccessAdapter<4> |  | SHINTW | 0 | <div style='text-align: right'>1,809,470</div>  |
| app_proof | AccessAdapter<8> |  | SHINTW | 0 | <div style='text-align: right'>1,431,298</div>  |
| app_proof | Boundary |  | SHINTW | 0 | <div style='text-align: right'>3,367,760</div>  |
| app_proof | Merkle |  | SHINTW | 0 | <div style='text-align: right'>9,614,336</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>10,135,118</div>  |
| app_proof | AccessAdapter<2> |  | STOREW | 0 | <div style='text-align: right'>576,400</div>  |
| app_proof | AccessAdapter<4> |  | STOREW | 0 | <div style='text-align: right'>354,614</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>16,028,499</div>  |
| app_proof | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>2,021,558</div>  |
| app_proof | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>1,211,964</div>  |
| app_proof | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>748,510</div>  |
| app_proof | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>417,960</div>  |
| app_proof | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>811,264</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>2,326,890</div>  |
| app_proof | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>85,294</div>  |
| app_proof | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>99,931</div>  |
| app_proof | AccessAdapter<8> |  | SUB | 0 | <div style='text-align: right'>21,862</div>  |
| app_proof | Boundary |  | SUB | 0 | <div style='text-align: right'>51,440</div>  |
| app_proof | Merkle |  | SUB | 0 | <div style='text-align: right'>82,304</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: red">(+12.0 [+3.8%])</span> <div style='text-align: right'>325.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>881,896,337</div>  | <div style='text-align: right'>6,995,780</div>  | <span style="color: red">(+999.0 [+2.5%])</span> <div style='text-align: right'>40,440.0</div>  |
| leaf_aggregation |  | <div style='text-align: right'>2</div>  |  |  |  | <span style="color: green">(-8.0 [-0.0%])</span> <div style='text-align: right'>23,324.0</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>14,680,064</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>46,137,344</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>17,301,504</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  |  |  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  |  | <div style='text-align: right'>1</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>126,976</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>17,792</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>128</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>14,208</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>128</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>100,352</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,340,032</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>78,848</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>14,680,064</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>623,902,720</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| leaf_aggregation | VmConnectorAir | 0 |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PersistentBoundaryAir<8> | 0 |  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | MemoryMerkleAir<8> | 0 |  | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<2> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<4> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<8> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 |  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | FriReducedOpeningAir | 0 |  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  |  |  | <div style='text-align: right'>2</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VariableRangeCheckerAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| app_proof | AccessAdapterAir<2> | 0 | <div style='text-align: right'>56,623,104</div>  |  |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>30,408,704</div>  |  |  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>146,800,640</div>  |  |  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>7,864,320</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>209,715,200</div>  |  |  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>4,194,304</div>  |
| app_proof | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>106,954,752</div>  |  |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  |  |  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: red">(+6.0 [+3.6%])</span> <div style='text-align: right'>172.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/17afa111e7a7a6fedffa5a64f119479e7a11da07

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12263798777)
