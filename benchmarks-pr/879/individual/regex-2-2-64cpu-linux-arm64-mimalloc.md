| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <div style='text-align: right'>2</div>  | <span style="color: red">(+21,360 [+0.0%])</span> <div style='text-align: right'>238,661,728</div>  | <span style="color: red">(+89 [+0.0%])</span> <div style='text-align: right'>4,181,142</div>  | <span style="color: green">(-175.0 [-0.6%])</span> <div style='text-align: right'>26,926.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| regex_program | true | <span style="color: red">(+491.0 [+0.7%])</span> <div style='text-align: right'>66,765.0</div>  | <span style="color: red">(+21,360 [+0.0%])</span> <div style='text-align: right'>238,661,728</div>  | <span style="color: red">(+89 [+0.0%])</span> <div style='text-align: right'>4,181,142</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| regex_program | ProgramChip | true | <span style="color: red">(+99 [+0.1%])</span> <div style='text-align: right'>89,661</div>  |
| regex_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| regex_program | Boundary | true | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>69,284</div>  |
| regex_program | Merkle | true | <span style="color: red">(+24 [+0.0%])</span> <div style='text-align: right'>70,528</div>  |
| regex_program | AccessAdapter<2> | true | <div style='text-align: right'>42</div>  |
| regex_program | AccessAdapter<4> | true | <div style='text-align: right'>22</div>  |
| regex_program | AccessAdapter<8> | true | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>69,284</div>  |
| regex_program | PhantomAir | true | <div style='text-align: right'>289</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: red">(+38 [+0.0%])</span> <div style='text-align: right'>1,150,354</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>38,011</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: red">(+15 [+0.0%])</span> <div style='text-align: right'>218,647</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: red">(+14 [+0.0%])</span> <div style='text-align: right'>1,961,010</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <span style="color: red">(+15 [+2.2%])</span> <div style='text-align: right'>702</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: red">(+3 [+0.0%])</span> <div style='text-align: right'>282,062</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>198,078</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>96,821</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>130,413</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>39,542</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>52,087</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>12,767</div>  |
| regex_program | KeccakVmAir | true | <div style='text-align: right'>24</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | true | <span style="color: red">(+30 [+0.0%])</span> <div style='text-align: right'>139,812</div>  |
| regex_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| regex_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| regex_program | true |  | ADD | <span style="color: red">(+23 [+0.0%])</span> <div style='text-align: right'>1,007,867</div>  |
| regex_program | true |  | AND | <span style="color: red">(+7 [+0.0%])</span> <div style='text-align: right'>66,796</div>  |
| regex_program | true |  | AUIPC | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>39,542</div>  |
| regex_program | true |  | BEQ | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>160,039</div>  |
| regex_program | true |  | BGE | <div style='text-align: right'>294</div>  |
| regex_program | true |  | BGEU | <div style='text-align: right'>121,615</div>  |
| regex_program | true |  | BLT | <div style='text-align: right'>5,141</div>  |
| regex_program | true |  | BLTU | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>71,028</div>  |
| regex_program | true |  | BNE | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>122,023</div>  |
| regex_program | true |  | DIVU | <div style='text-align: right'>114</div>  |
| regex_program | true |  | HINT_STOREW | <div style='text-align: right'>12,767</div>  |
| regex_program | true |  | JAL | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>52,348</div>  |
| regex_program | true |  | JALR | <div style='text-align: right'>130,413</div>  |
| regex_program | true |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| regex_program | true |  | LOADB | <span style="color: red">(+15 [+2.2%])</span> <div style='text-align: right'>694</div>  |
| regex_program | true |  | LOADBU | <div style='text-align: right'>27,294</div>  |
| regex_program | true |  | LOADH | <div style='text-align: right'>8</div>  |
| regex_program | true |  | LOADHU | <div style='text-align: right'>95</div>  |
| regex_program | true |  | LOADW | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>1,142,628</div>  |
| regex_program | true |  | LUI | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>44,473</div>  |
| regex_program | true |  | MUL | <div style='text-align: right'>52,087</div>  |
| regex_program | true |  | MULHU | <div style='text-align: right'>244</div>  |
| regex_program | true |  | OR | <span style="color: red">(+8 [+0.0%])</span> <div style='text-align: right'>23,544</div>  |
| regex_program | true |  | PHANTOM | <div style='text-align: right'>289</div>  |
| regex_program | true |  | SLL | <span style="color: red">(+7 [+0.0%])</span> <div style='text-align: right'>213,556</div>  |
| regex_program | true |  | SLT | <div style='text-align: right'>5</div>  |
| regex_program | true |  | SLTU | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>38,006</div>  |
| regex_program | true |  | SRA | <div style='text-align: right'>1</div>  |
| regex_program | true |  | SRL | <span style="color: red">(+8 [+0.2%])</span> <div style='text-align: right'>5,090</div>  |
| regex_program | true |  | STOREB | <span style="color: red">(+16 [+0.1%])</span> <div style='text-align: right'>12,737</div>  |
| regex_program | true |  | STOREH | <div style='text-align: right'>10,074</div>  |
| regex_program | true |  | STOREW | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>768,182</div>  |
| regex_program | true |  | SUB | <div style='text-align: right'>42,583</div>  |
| regex_program | true |  | XOR | <div style='text-align: right'>9,564</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: red">(+828 [+0.0%])</span> <div style='text-align: right'>36,283,212</div>  |
| regex_program | AccessAdapter<8> | true |  | ADD | <span style="color: green">(-17 [-16.7%])</span> <div style='text-align: right'>85</div>  |
| regex_program | Boundary | true |  | ADD | <span style="color: green">(-40 [-16.7%])</span> <div style='text-align: right'>200</div>  |
| regex_program | Merkle | true |  | ADD | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: red">(+252 [+0.0%])</span> <div style='text-align: right'>2,404,656</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: red">(+21 [+0.0%])</span> <div style='text-align: right'>830,382</div>  |
| regex_program | AccessAdapter<8> | true |  | AUIPC | <span style="color: red">(+17 [+50.0%])</span> <div style='text-align: right'>51</div>  |
| regex_program | Boundary | true |  | AUIPC | <span style="color: red">(+40 [+50.0%])</span> <div style='text-align: right'>120</div>  |
| regex_program | Merkle | true |  | AUIPC | <span style="color: red">(+64 [+1.9%])</span> <div style='text-align: right'>3,520</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: red">(+156 [+0.0%])</span> <div style='text-align: right'>4,161,014</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>3,891,680</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <span style="color: green">(-32 [-0.0%])</span> <div style='text-align: right'>2,272,896</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-78 [-0.0%])</span> <div style='text-align: right'>3,172,598</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>108,511</div>  |
| regex_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>255,320</div>  |
| regex_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>408,640</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: green">(-18 [-0.0%])</span> <div style='text-align: right'>942,264</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>3,651,564</div>  |
| regex_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>143</div>  |
| regex_program | AccessAdapter<8> | true |  | KECCAK256 | <div style='text-align: right'>68</div>  |
| regex_program | Boundary | true |  | KECCAK256 | <div style='text-align: right'>160</div>  |
| regex_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| regex_program | Merkle | true |  | KECCAK256 | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <span style="color: red">(+525 [+2.2%])</span> <div style='text-align: right'>24,290</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>51</div>  |
| regex_program | Boundary | true |  | LOADB | <div style='text-align: right'>120</div>  |
| regex_program | Merkle | true |  | LOADB | <div style='text-align: right'>512</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>187</div>  |
| regex_program | Boundary | true |  | LOADBU | <div style='text-align: right'>440</div>  |
| regex_program | Merkle | true |  | LOADBU | <span style="color: red">(+576 [+28.1%])</span> <div style='text-align: right'>2,624</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: red">(+40 [+0.0%])</span> <div style='text-align: right'>45,705,120</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-51 [-1.7%])</span> <div style='text-align: right'>2,958</div>  |
| regex_program | Boundary | true |  | LOADW | <span style="color: green">(-120 [-1.7%])</span> <div style='text-align: right'>6,960</div>  |
| regex_program | Merkle | true |  | LOADW | <span style="color: red">(+256 [+1.0%])</span> <div style='text-align: right'>25,536</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <span style="color: green">(-18 [-0.0%])</span> <div style='text-align: right'>800,514</div>  |
| regex_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>1,614,697</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>9,516</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: red">(+288 [+0.0%])</span> <div style='text-align: right'>847,584</div>  |
| regex_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>1,734</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: red">(+371 [+0.0%])</span> <div style='text-align: right'>11,318,468</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>185</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: red">(+222 [+0.0%])</span> <div style='text-align: right'>1,406,222</div>  |
| regex_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>53</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: red">(+424 [+0.2%])</span> <div style='text-align: right'>269,770</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <span style="color: red">(+640 [+0.1%])</span> <div style='text-align: right'>509,480</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREB | <span style="color: red">(+51 [+2.4%])</span> <div style='text-align: right'>2,210</div>  |
| regex_program | Boundary | true |  | STOREB | <span style="color: red">(+120 [+2.4%])</span> <div style='text-align: right'>5,200</div>  |
| regex_program | Merkle | true |  | STOREB | <span style="color: red">(+1,856 [+17.5%])</span> <div style='text-align: right'>12,480</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREH | <div style='text-align: right'>85,255</div>  |
| regex_program | Boundary | true |  | STOREH | <div style='text-align: right'>200,600</div>  |
| regex_program | Merkle | true |  | STOREH | <span style="color: green">(-64 [-0.0%])</span> <div style='text-align: right'>321,344</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-120 [-0.0%])</span> <div style='text-align: right'>30,727,280</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREW | <span style="color: green">(-68 [-0.0%])</span> <div style='text-align: right'>389,504</div>  |
| regex_program | Boundary | true |  | STOREW | <span style="color: green">(-160 [-0.0%])</span> <div style='text-align: right'>916,480</div>  |
| regex_program | Merkle | true |  | STOREW | <span style="color: green">(-2,496 [-0.2%])</span> <div style='text-align: right'>1,481,920</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>344,304</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: green">(-6.0 [-13.3%])</span> <div style='text-align: right'>39.0</div>  | <span style="color: red">(+16.0 [+0.2%])</span> <div style='text-align: right'>7,568.0</div>  | <span style="color: green">(-8.0 [-0.2%])</span> <div style='text-align: right'>4,835.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-2.0 [-1.0%])</span> <div style='text-align: right'>192.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+21,360 [+0.0%])</span> <div style='text-align: right'>238,661,728</div>  | <span style="color: red">(+89 [+0.0%])</span> <div style='text-align: right'>4,181,142</div>  | <span style="color: green">(-175.0 [-0.6%])</span> <div style='text-align: right'>26,926.0</div>  |

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
| regex_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| regex_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| regex_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| regex_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| regex_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| regex_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>4,194,304</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>6,815,744</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| regex_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| regex_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | PhantomAir | 0 | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>512</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>27,525,120</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>113,664</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>1,024</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>35,584</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>256</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>20,608</div>  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  | <div style='text-align: right'>128</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16,384</div>  |
| regex_program | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>32</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>164,364,288</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| regex_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| regex_program | 0 | <span style="color: red">(+24.0 [+0.9%])</span> <div style='text-align: right'>2,723.0</div>  | <span style="color: green">(-215.0 [-1.3%])</span> <div style='text-align: right'>16,635.0</div>  | <div style='text-align: right'>790,590,848</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/3f702da87c877b0775f88d73ff77675a5d948594/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/3f702da87c877b0775f88d73ff77675a5d948594

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12074931842)
