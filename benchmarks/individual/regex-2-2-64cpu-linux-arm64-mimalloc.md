| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-78,094 [-0.0%])</span> <div style='text-align: right'>238,735,511</div>  | <span style="color: green">(-16 [-0.0%])</span> <div style='text-align: right'>4,181,198</div>  | <span style="color: red">(+46.0 [+0.2%])</span> <div style='text-align: right'>26,991.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <span style="color: green">(-43,196 [-0.0%])</span> <div style='text-align: right'>314,813,914</div>  | <span style="color: red">(+752 [+0.0%])</span> <div style='text-align: right'>7,308,612</div>  | <span style="color: green">(-1,047.0 [-2.9%])</span> <div style='text-align: right'>35,651.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| regex_program | true | <span style="color: red">(+1,939.0 [+2.9%])</span> <div style='text-align: right'>68,363.0</div>  | <span style="color: green">(-78,094 [-0.0%])</span> <div style='text-align: right'>238,735,511</div>  | <span style="color: green">(-16 [-0.0%])</span> <div style='text-align: right'>4,181,198</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| regex_program | ProgramChip | true | <span style="color: green">(-181 [-0.2%])</span> <div style='text-align: right'>85,436</div>  |
| regex_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| regex_program | Boundary | true | <span style="color: green">(-98 [-0.1%])</span> <div style='text-align: right'>69,200</div>  |
| regex_program | Merkle | true | <span style="color: green">(-32 [-0.0%])</span> <div style='text-align: right'>70,522</div>  |
| regex_program | AccessAdapter<2> | true | <div style='text-align: right'>42</div>  |
| regex_program | AccessAdapter<4> | true | <div style='text-align: right'>22</div>  |
| regex_program | AccessAdapter<8> | true | <span style="color: green">(-98 [-0.1%])</span> <div style='text-align: right'>69,200</div>  |
| regex_program | KeccakVmAir | true | <div style='text-align: right'>24</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>12,767</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>52,087</div>  |
| regex_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>39,542</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>130,415</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>96,897</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>198,079</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>282,064</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>687</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-10 [-0.0%])</span> <div style='text-align: right'>1,961,056</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>218,632</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>38,005</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>1,150,319</div>  |
| regex_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| regex_program | PhantomAir | true | <div style='text-align: right'>289</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | true | <span style="color: green">(-130 [-0.1%])</span> <div style='text-align: right'>139,722</div>  |
| regex_program | VariableRangeCheckerAir | true | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| regex_program | true |  | ADD | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>1,007,847</div>  |
| regex_program | true |  | AND | <div style='text-align: right'>66,789</div>  |
| regex_program | true |  | AUIPC | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>39,542</div>  |
| regex_program | true |  | BEQ | <div style='text-align: right'>159,878</div>  |
| regex_program | true |  | BGE | <div style='text-align: right'>294</div>  |
| regex_program | true |  | BGEU | <div style='text-align: right'>121,615</div>  |
| regex_program | true |  | BLT | <div style='text-align: right'>5,141</div>  |
| regex_program | true |  | BLTU | <div style='text-align: right'>71,029</div>  |
| regex_program | true |  | BNE | <div style='text-align: right'>122,186</div>  |
| regex_program | true |  | DIVU | <div style='text-align: right'>114</div>  |
| regex_program | true |  | HINT_STOREW | <div style='text-align: right'>12,767</div>  |
| regex_program | true |  | JAL | <div style='text-align: right'>52,424</div>  |
| regex_program | true |  | JALR | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>130,415</div>  |
| regex_program | true |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| regex_program | true |  | LOADB | <div style='text-align: right'>679</div>  |
| regex_program | true |  | LOADBU | <div style='text-align: right'>27,294</div>  |
| regex_program | true |  | LOADH | <div style='text-align: right'>8</div>  |
| regex_program | true |  | LOADHU | <div style='text-align: right'>95</div>  |
| regex_program | true |  | LOADW | <span style="color: green">(-5 [-0.0%])</span> <div style='text-align: right'>1,142,653</div>  |
| regex_program | true |  | LUI | <div style='text-align: right'>44,473</div>  |
| regex_program | true |  | MUL | <div style='text-align: right'>52,087</div>  |
| regex_program | true |  | MULHU | <div style='text-align: right'>244</div>  |
| regex_program | true |  | OR | <div style='text-align: right'>23,536</div>  |
| regex_program | true |  | PHANTOM | <div style='text-align: right'>289</div>  |
| regex_program | true |  | SLL | <div style='text-align: right'>213,549</div>  |
| regex_program | true |  | SLT | <div style='text-align: right'>5</div>  |
| regex_program | true |  | SLTU | <div style='text-align: right'>38,000</div>  |
| regex_program | true |  | SRA | <div style='text-align: right'>1</div>  |
| regex_program | true |  | SRL | <div style='text-align: right'>5,082</div>  |
| regex_program | true |  | STOREB | <div style='text-align: right'>12,721</div>  |
| regex_program | true |  | STOREH | <div style='text-align: right'>10,074</div>  |
| regex_program | true |  | STOREW | <span style="color: green">(-5 [-0.0%])</span> <div style='text-align: right'>768,219</div>  |
| regex_program | true |  | SUB | <div style='text-align: right'>42,583</div>  |
| regex_program | true |  | XOR | <div style='text-align: right'>9,564</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-144 [-0.0%])</span> <div style='text-align: right'>36,282,492</div>  |
| regex_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>102</div>  |
| regex_program | Boundary | true |  | ADD | <div style='text-align: right'>240</div>  |
| regex_program | Merkle | true |  | ADD | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>2,404,404</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>830,382</div>  |
| regex_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| regex_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| regex_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>4,156,828</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>3,891,680</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>2,272,928</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>3,176,836</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> | true |  | HINT_STOREW | <span style="color: red">(+17 [+0.0%])</span> <div style='text-align: right'>108,528</div>  |
| regex_program | Boundary | true |  | HINT_STOREW | <span style="color: red">(+40 [+0.0%])</span> <div style='text-align: right'>255,360</div>  |
| regex_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>408,576</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>943,632</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-28 [-0.0%])</span> <div style='text-align: right'>3,651,620</div>  |
| regex_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>143</div>  |
| regex_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>23,765</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADBU | <span style="color: red">(+68 [+44.4%])</span> <div style='text-align: right'>221</div>  |
| regex_program | Boundary | true |  | LOADBU | <span style="color: red">(+160 [+44.4%])</span> <div style='text-align: right'>520</div>  |
| regex_program | Merkle | true |  | LOADBU | <span style="color: red">(+384 [+20.0%])</span> <div style='text-align: right'>2,304</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-200 [-0.0%])</span> <div style='text-align: right'>45,706,120</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADW | <span style="color: red">(+34 [+1.1%])</span> <div style='text-align: right'>3,043</div>  |
| regex_program | Boundary | true |  | LOADW | <span style="color: red">(+80 [+1.1%])</span> <div style='text-align: right'>7,160</div>  |
| regex_program | Merkle | true |  | LOADW | <span style="color: red">(+640 [+2.5%])</span> <div style='text-align: right'>25,920</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>800,514</div>  |
| regex_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| regex_program | Merkle | true |  | LUI | <div style='text-align: right'>64</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>1,614,697</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>9,516</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>847,296</div>  |
| regex_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>1,734</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>11,318,097</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>185</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>1,406,000</div>  |
| regex_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>53</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>269,346</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>508,840</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREB | <span style="color: green">(-1,054 [-48.8%])</span> <div style='text-align: right'>1,105</div>  |
| regex_program | Boundary | true |  | STOREB | <span style="color: green">(-2,480 [-48.8%])</span> <div style='text-align: right'>2,600</div>  |
| regex_program | Merkle | true |  | STOREB | <span style="color: green">(-2,624 [-23.8%])</span> <div style='text-align: right'>8,384</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREH | <span style="color: green">(-34 [-0.0%])</span> <div style='text-align: right'>85,255</div>  |
| regex_program | Boundary | true |  | STOREH | <span style="color: green">(-80 [-0.0%])</span> <div style='text-align: right'>200,600</div>  |
| regex_program | Merkle | true |  | STOREH | <span style="color: green">(-384 [-0.1%])</span> <div style='text-align: right'>321,088</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-200 [-0.0%])</span> <div style='text-align: right'>30,728,760</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+153 [+0.0%])</span> <div style='text-align: right'>389,878</div>  |
| regex_program | Boundary | true |  | STOREW | <span style="color: red">(+360 [+0.0%])</span> <div style='text-align: right'>917,360</div>  |
| regex_program | Merkle | true |  | STOREW | <span style="color: red">(+1,216 [+0.1%])</span> <div style='text-align: right'>1,486,720</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>344,304</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: green">(-4.0 [-8.5%])</span> <div style='text-align: right'>43.0</div>  | <span style="color: green">(-58.0 [-0.8%])</span> <div style='text-align: right'>7,529.0</div>  | <span style="color: green">(-23.0 [-0.5%])</span> <div style='text-align: right'>4,851.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-1.0 [-0.5%])</span> <div style='text-align: right'>213.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-78,094 [-0.0%])</span> <div style='text-align: right'>238,735,511</div>  | <span style="color: green">(-16 [-0.0%])</span> <div style='text-align: right'>4,181,198</div>  | <span style="color: red">(+46.0 [+0.2%])</span> <div style='text-align: right'>26,991.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-43,196 [-0.0%])</span> <div style='text-align: right'>314,813,914</div>  | <span style="color: red">(+752 [+0.0%])</span> <div style='text-align: right'>7,308,612</div>  | <span style="color: green">(-1,047.0 [-2.9%])</span> <div style='text-align: right'>35,651.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| regex_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| regex_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| regex_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| regex_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| regex_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| regex_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| regex_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| regex_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| regex_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>4,194,304</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>6,815,744</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>2,240</div>  |  |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>64</div>  |
| regex_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,184</div>  |  |  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>32</div>  |
| regex_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  |  |  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  |  | <div style='text-align: right'>32</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>20,608</div>  |  |  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  |  | <div style='text-align: right'>128</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>35,584</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>256</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>113,664</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>27,525,120</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | PhantomAir | 0 | <div style='text-align: right'>9,216</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>512</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>164,364,288</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>39,845,888</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
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
| regex_program | 0 |  | <span style="color: green">(-36.0 [-1.3%])</span> <div style='text-align: right'>2,668.0</div>  |  |  |  |  | <span style="color: red">(+140.0 [+0.8%])</span> <div style='text-align: right'>16,794.0</div>  | <div style='text-align: right'>791,770,496</div>  |  |
| leaf_aggregation | 0 | <span style="color: green">(-100.0 [-40.5%])</span> <div style='text-align: right'>147.0</div>  | <span style="color: green">(-17.0 [-0.2%])</span> <div style='text-align: right'>9,861.0</div>  | <span style="color: green">(-36.0 [-0.4%])</span> <div style='text-align: right'>8,103.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-9.0 [-12.5%])</span> <div style='text-align: right'>63.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-1,030.0 [-3.8%])</span> <div style='text-align: right'>25,790.0</div>  | <div style='text-align: right'>807,206,936</div>  | <span style="color: red">(+1,001.0 [+1.4%])</span> <div style='text-align: right'>74,525.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: red">(+801.0 [+2.0%])</span> <div style='text-align: right'>40,487.0</div>  | <span style="color: green">(-43,196 [-0.0%])</span> <div style='text-align: right'>314,813,914</div>  | <span style="color: red">(+752 [+0.0%])</span> <div style='text-align: right'>7,308,612</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <span style="color: green">(-228 [-0.1%])</span> <div style='text-align: right'>305,094</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <span style="color: green">(-18 [-0.0%])</span> <div style='text-align: right'>1,055,116</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <span style="color: green">(-2,488 [-0.2%])</span> <div style='text-align: right'>1,105,680</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <span style="color: green">(-1,244 [-0.2%])</span> <div style='text-align: right'>553,050</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>114,850</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>56,206</div>  |
| leaf_aggregation | FriReducedOpeningAir | true | 0 | <div style='text-align: right'>570,948</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <span style="color: green">(-225 [-0.2%])</span> <div style='text-align: right'>111,046</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <div style='text-align: right'>2,865,063</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: red">(+977 [+1.0%])</span> <div style='text-align: right'>97,962</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>1,463,112</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>2,060,066</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>648,059</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | ADD | 0 | <div style='text-align: right'>2,539,350</div>  |
| leaf_aggregation | true |  | BBE4DIV | 0 | <div style='text-align: right'>8,109</div>  |
| leaf_aggregation | true |  | BBE4MUL | 0 | <div style='text-align: right'>38,132</div>  |
| leaf_aggregation | true |  | BEQ | 0 | <div style='text-align: right'>19,897</div>  |
| leaf_aggregation | true |  | BNE | 0 | <div style='text-align: right'>1,443,215</div>  |
| leaf_aggregation | true |  | COMP_POS2 | 0 | <div style='text-align: right'>18,396</div>  |
| leaf_aggregation | true |  | DIV | 0 | <div style='text-align: right'>177</div>  |
| leaf_aggregation | true |  | FE4ADD | 0 | <div style='text-align: right'>47,831</div>  |
| leaf_aggregation | true |  | FE4SUB | 0 | <div style='text-align: right'>16,974</div>  |
| leaf_aggregation | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>7,098</div>  |
| leaf_aggregation | true |  | JAL | 0 | <span style="color: red">(+97,961 [+9796100.0%])</span> <div style='text-align: right'>97,962</div>  |
| leaf_aggregation | true |  | LOADW | 0 | <div style='text-align: right'>209,622</div>  |
| leaf_aggregation | true |  | LOADW2 | 0 | <div style='text-align: right'>666,377</div>  |
| leaf_aggregation | true |  | MUL | 0 | <div style='text-align: right'>228,129</div>  |
| leaf_aggregation | true |  | PERM_POS2 | 0 | <div style='text-align: right'>37,810</div>  |
| leaf_aggregation | true |  | PHANTOM | 0 | <div style='text-align: right'>648,059</div>  |
| leaf_aggregation | true |  | SHINTW | 0 | <div style='text-align: right'>512,738</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <span style="color: red">(+255,859 [+12792950.0%])</span> <div style='text-align: right'>255,861</div>  |
| leaf_aggregation | true |  | STOREW2 | 0 | <div style='text-align: right'>415,468</div>  |
| leaf_aggregation | true |  | SUB | 0 | <div style='text-align: right'>97,407</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | ADD | 0 | <div style='text-align: right'>76,180,500</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | ADD | 0 | <div style='text-align: right'>615,318</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | ADD | 0 | <div style='text-align: right'>363,597</div>  |
| leaf_aggregation | Boundary | true |  | ADD | 0 | <div style='text-align: right'>753,786</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4DIV | 0 | <div style='text-align: right'>324,360</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4DIV | 0 | <div style='text-align: right'>160,996</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4DIV | 0 | <div style='text-align: right'>95,134</div>  |
| leaf_aggregation | Boundary | true |  | BBE4DIV | 0 | <div style='text-align: right'>352</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | BBE4MUL | 0 | <div style='text-align: right'>1,525,280</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BBE4MUL | 0 | <div style='text-align: right'>1,116,214</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BBE4MUL | 0 | <div style='text-align: right'>659,581</div>  |
| leaf_aggregation | Boundary | true |  | BBE4MUL | 0 | <div style='text-align: right'>1,037,212</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BEQ | 0 | <div style='text-align: right'>457,631</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true |  | BNE | 0 | <div style='text-align: right'>33,193,945</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | BNE | 0 | <div style='text-align: right'>1,540</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | BNE | 0 | <div style='text-align: right'>910</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>742,896</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>438,984</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>287,028</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | COMP_POS2 | 0 | <div style='text-align: right'>10,283,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | DIV | 0 | <div style='text-align: right'>5,310</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4ADD | 0 | <div style='text-align: right'>1,913,240</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4ADD | 0 | <div style='text-align: right'>1,345,652</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4ADD | 0 | <div style='text-align: right'>795,158</div>  |
| leaf_aggregation | Boundary | true |  | FE4ADD | 0 | <div style='text-align: right'>1,365,980</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true |  | FE4SUB | 0 | <div style='text-align: right'>678,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FE4SUB | 0 | <div style='text-align: right'>550,880</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FE4SUB | 0 | <div style='text-align: right'>325,520</div>  |
| leaf_aggregation | Boundary | true |  | FE4SUB | 0 | <div style='text-align: right'>574,816</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>400,708</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>236,782</div>  |
| leaf_aggregation | FriReducedOpeningAir | true |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>36,540,672</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <span style="color: red">(+979,610 [+9796100.0%])</span> <div style='text-align: right'>979,620</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | JAL | 0 | <div style='text-align: right'>561</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | JAL | 0 | <div style='text-align: right'>663</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW | 0 | <div style='text-align: right'>8,594,502</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW | 0 | <div style='text-align: right'>565,202</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW | 0 | <div style='text-align: right'>288,223</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW | 0 | <div style='text-align: right'>20,706</div>  |
| leaf_aggregation | Boundary | true |  | LOADW | 0 | <div style='text-align: right'>382,646</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | LOADW2 | 0 | <div style='text-align: right'>27,321,457</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | LOADW2 | 0 | <div style='text-align: right'>59,994</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | LOADW2 | 0 | <div style='text-align: right'>35,451</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | LOADW2 | 0 | <div style='text-align: right'>510</div>  |
| leaf_aggregation | Boundary | true |  | LOADW2 | 0 | <div style='text-align: right'>1,661</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | MUL | 0 | <div style='text-align: right'>6,843,870</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | MUL | 0 | <div style='text-align: right'>30,415</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | MUL | 0 | <div style='text-align: right'>17,992</div>  |
| leaf_aggregation | Boundary | true |  | MUL | 0 | <div style='text-align: right'>112,376</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>1,763,894</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>1,043,666</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>689,197</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true |  | PERM_POS2 | 0 | <div style='text-align: right'>21,135,790</div>  |
| leaf_aggregation | PhantomAir | true |  | PHANTOM | 0 | <div style='text-align: right'>3,888,354</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | SHINTW | 0 | <div style='text-align: right'>21,022,258</div>  |
| leaf_aggregation | Boundary | true |  | SHINTW | 0 | <div style='text-align: right'>5,640,118</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <span style="color: red">(+10,490,219 [+12792950.0%])</span> <div style='text-align: right'>10,490,301</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW | 0 | <div style='text-align: right'>151,030</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW | 0 | <div style='text-align: right'>88,101</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <span style="color: red">(+868,318 [+3946900.0%])</span> <div style='text-align: right'>868,340</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW2 | 0 | <div style='text-align: right'>17,034,188</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | STOREW2 | 0 | <div style='text-align: right'>1,726,054</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | STOREW2 | 0 | <div style='text-align: right'>1,021,306</div>  |
| leaf_aggregation | AccessAdapter<8> | true |  | STOREW2 | 0 | <div style='text-align: right'>595,850</div>  |
| leaf_aggregation | Boundary | true |  | STOREW2 | 0 | <div style='text-align: right'>853,798</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true |  | SUB | 0 | <div style='text-align: right'>2,922,210</div>  |
| leaf_aggregation | AccessAdapter<2> | true |  | SUB | 0 | <div style='text-align: right'>78,782</div>  |
| leaf_aggregation | AccessAdapter<4> | true |  | SUB | 0 | <div style='text-align: right'>93,106</div>  |
| leaf_aggregation | Boundary | true |  | SUB | 0 | <div style='text-align: right'>15,180</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/15f5656ab6f107aeba63a31ae5be61ca72c6741e/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/15f5656ab6f107aeba63a31ae5be61ca72c6741e

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12112721893)
