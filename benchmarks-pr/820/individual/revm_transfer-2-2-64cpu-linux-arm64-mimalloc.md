| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>2</div>  | <span style="color: green">(-523,721 [-0.4%])</span> <div style='text-align: right'>139,128,415</div>  | <span style="color: green">(-18,499 [-0.8%])</span> <div style='text-align: right'>2,329,938</div>  | <span style="color: green">(-73.0 [-0.5%])</span> <div style='text-align: right'>15,983.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true | <span style="color: green">(-112.0 [-0.8%])</span> <div style='text-align: right'>13,680.0</div>  | <span style="color: green">(-523,721 [-0.4%])</span> <div style='text-align: right'>139,128,415</div>  | <span style="color: green">(-18,499 [-0.8%])</span> <div style='text-align: right'>2,329,938</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_100_transfers | ProgramChip | true | <span style="color: red">(+3 [+0.0%])</span> <div style='text-align: right'>247,619</div>  |
| revm_100_transfers | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| revm_100_transfers | Boundary | true | <span style="color: red">(+206 [+0.5%])</span> <div style='text-align: right'>39,708</div>  |
| revm_100_transfers | Merkle | true | <span style="color: red">(+22 [+0.0%])</span> <div style='text-align: right'>47,346</div>  |
| revm_100_transfers | AccessAdapter<8> | true | <span style="color: red">(+206 [+0.5%])</span> <div style='text-align: right'>39,708</div>  |
| revm_100_transfers | PhantomAir | true | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-8,671 [-1.1%])</span> <div style='text-align: right'>815,535</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>29,893</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: green">(-520 [-0.4%])</span> <div style='text-align: right'>121,205</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-4,972 [-0.5%])</span> <div style='text-align: right'>960,800</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-4,641 [-3.1%])</span> <div style='text-align: right'>145,875</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>73,629</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: red">(+952 [+2.0%])</span> <div style='text-align: right'>48,209</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-277 [-0.7%])</span> <div style='text-align: right'>36,989</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-185 [-1.1%])</span> <div style='text-align: right'>17,171</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: green">(-185 [-0.4%])</span> <div style='text-align: right'>41,135</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | Poseidon2VmAir<BabyBearParameters> | true | <span style="color: red">(+228 [+0.3%])</span> <div style='text-align: right'>87,054</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| revm_100_transfers | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true |  | ADD | <span style="color: green">(-7,267 [-1.4%])</span> <div style='text-align: right'>524,342</div>  |
| revm_100_transfers | true |  | AND | <span style="color: green">(-699 [-0.8%])</span> <div style='text-align: right'>90,242</div>  |
| revm_100_transfers | true |  | AUIPC | <span style="color: green">(-185 [-1.1%])</span> <div style='text-align: right'>17,171</div>  |
| revm_100_transfers | true |  | BEQ | <span style="color: green">(-2,162 [-4.2%])</span> <div style='text-align: right'>49,099</div>  |
| revm_100_transfers | true |  | BGE | <div style='text-align: right'>41</div>  |
| revm_100_transfers | true |  | BGEU | <span style="color: green">(-1,040 [-18.3%])</span> <div style='text-align: right'>4,640</div>  |
| revm_100_transfers | true |  | BLT | <div style='text-align: right'>3,139</div>  |
| revm_100_transfers | true |  | BLTU | <span style="color: red">(+1,040 [+1.6%])</span> <div style='text-align: right'>65,809</div>  |
| revm_100_transfers | true |  | BNE | <span style="color: green">(-2,479 [-2.5%])</span> <div style='text-align: right'>96,776</div>  |
| revm_100_transfers | true |  | JAL | <span style="color: red">(+952 [+7.8%])</span> <div style='text-align: right'>13,233</div>  |
| revm_100_transfers | true |  | JALR | <span style="color: green">(-277 [-0.7%])</span> <div style='text-align: right'>36,989</div>  |
| revm_100_transfers | true |  | LOADB | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | true |  | LOADBU | <span style="color: green">(-4,146 [-2.1%])</span> <div style='text-align: right'>190,879</div>  |
| revm_100_transfers | true |  | LOADW | <span style="color: green">(-456 [-0.1%])</span> <div style='text-align: right'>384,297</div>  |
| revm_100_transfers | true |  | LUI | <div style='text-align: right'>34,976</div>  |
| revm_100_transfers | true |  | MUL | <span style="color: green">(-185 [-0.4%])</span> <div style='text-align: right'>41,135</div>  |
| revm_100_transfers | true |  | MULHU | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | true |  | OR | <span style="color: green">(-252 [-0.2%])</span> <div style='text-align: right'>107,074</div>  |
| revm_100_transfers | true |  | PHANTOM | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | true |  | SLL | <span style="color: green">(-336 [-0.3%])</span> <div style='text-align: right'>100,048</div>  |
| revm_100_transfers | true |  | SLT | <div style='text-align: right'>100</div>  |
| revm_100_transfers | true |  | SLTU | <div style='text-align: right'>29,793</div>  |
| revm_100_transfers | true |  | SRA | <div style='text-align: right'>40</div>  |
| revm_100_transfers | true |  | SRL | <span style="color: green">(-184 [-0.9%])</span> <div style='text-align: right'>21,117</div>  |
| revm_100_transfers | true |  | STOREB | <div style='text-align: right'>28,966</div>  |
| revm_100_transfers | true |  | STOREH | <div style='text-align: right'>300</div>  |
| revm_100_transfers | true |  | STOREW | <span style="color: green">(-370 [-0.1%])</span> <div style='text-align: right'>356,358</div>  |
| revm_100_transfers | true |  | SUB | <span style="color: green">(-276 [-0.9%])</span> <div style='text-align: right'>30,346</div>  |
| revm_100_transfers | true |  | XOR | <span style="color: green">(-177 [-0.3%])</span> <div style='text-align: right'>63,531</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-261,612 [-1.4%])</span> <div style='text-align: right'>18,876,312</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: green">(-25,164 [-0.8%])</span> <div style='text-align: right'>3,248,712</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-3,885 [-1.1%])</span> <div style='text-align: right'>360,591</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | AUIPC | <span style="color: green">(-17 [-33.3%])</span> <div style='text-align: right'>34</div>  |
| revm_100_transfers | Boundary | true |  | AUIPC | <span style="color: green">(-40 [-33.3%])</span> <div style='text-align: right'>80</div>  |
| revm_100_transfers | Merkle | true |  | AUIPC | <span style="color: green">(-64 [-1.8%])</span> <div style='text-align: right'>3,456</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: green">(-56,212 [-4.2%])</span> <div style='text-align: right'>1,276,574</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>1,312</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <span style="color: green">(-33,280 [-18.3%])</span> <div style='text-align: right'>148,480</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>100,448</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <span style="color: red">(+33,280 [+1.6%])</span> <div style='text-align: right'>2,105,888</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-64,454 [-2.5%])</span> <div style='text-align: right'>2,516,176</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: red">(+17,136 [+7.8%])</span> <div style='text-align: right'>238,194</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-7,756 [-0.7%])</span> <div style='text-align: right'>1,035,692</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>529,795</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>68</div>  |
| revm_100_transfers | Boundary | true |  | LOADB | <div style='text-align: right'>160</div>  |
| revm_100_transfers | Merkle | true |  | LOADB | <div style='text-align: right'>640</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: green">(-165,840 [-2.1%])</span> <div style='text-align: right'>7,635,160</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>204</div>  |
| revm_100_transfers | Boundary | true |  | LOADBU | <div style='text-align: right'>480</div>  |
| revm_100_transfers | Merkle | true |  | LOADBU | <span style="color: red">(+192 [+8.8%])</span> <div style='text-align: right'>2,368</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-18,240 [-0.1%])</span> <div style='text-align: right'>15,371,880</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-34 [-0.4%])</span> <div style='text-align: right'>9,231</div>  |
| revm_100_transfers | Boundary | true |  | LOADW | <span style="color: green">(-80 [-0.4%])</span> <div style='text-align: right'>21,720</div>  |
| revm_100_transfers | Merkle | true |  | LOADW | <span style="color: green">(-384 [-0.8%])</span> <div style='text-align: right'>46,912</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>629,568</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: green">(-5,735 [-0.4%])</span> <div style='text-align: right'>1,275,185</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MUL | <span style="color: red">(+17 [+50.0%])</span> <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | MUL | <span style="color: red">(+40 [+50.0%])</span> <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>900,159</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_100_transfers | Merkle | true |  | MULHU | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: green">(-9,072 [-0.2%])</span> <div style='text-align: right'>3,854,664</div>  |
| revm_100_transfers | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>7,674</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: green">(-17,808 [-0.3%])</span> <div style='text-align: right'>5,302,544</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>3,700</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>1,102,341</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>2,120</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: green">(-9,752 [-0.9%])</span> <div style='text-align: right'>1,119,201</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>1,158,640</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>39,576</div>  |
| revm_100_transfers | Boundary | true |  | STOREB | <div style='text-align: right'>93,120</div>  |
| revm_100_transfers | Merkle | true |  | STOREB | <span style="color: green">(-28,032 [-7.8%])</span> <div style='text-align: right'>333,184</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>12,000</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-14,800 [-0.1%])</span> <div style='text-align: right'>14,254,320</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+1,785 [+0.6%])</span> <div style='text-align: right'>288,269</div>  |
| revm_100_transfers | Boundary | true |  | STOREW | <span style="color: red">(+4,200 [+0.6%])</span> <div style='text-align: right'>678,280</div>  |
| revm_100_transfers | Merkle | true |  | STOREW | <span style="color: red">(+28,928 [+2.6%])</span> <div style='text-align: right'>1,128,256</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: green">(-9,936 [-0.9%])</span> <div style='text-align: right'>1,092,456</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: green">(-6,372 [-0.3%])</span> <div style='text-align: right'>2,287,116</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>108.0</div>  | <span style="color: green">(-40.0 [-0.7%])</span> <div style='text-align: right'>5,307.0</div>  | <span style="color: green">(-65.0 [-1.6%])</span> <div style='text-align: right'>4,060.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-7.0 [-3.5%])</span> <div style='text-align: right'>194.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-523,721 [-0.4%])</span> <div style='text-align: right'>139,128,415</div>  | <span style="color: green">(-18,499 [-0.8%])</span> <div style='text-align: right'>2,329,938</div>  | <span style="color: green">(-73.0 [-0.5%])</span> <div style='text-align: right'>15,983.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| revm_100_transfers | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| revm_100_transfers | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| revm_100_transfers | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| revm_100_transfers | ProgramAir | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| revm_100_transfers | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| revm_100_transfers | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>3,407,872</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,686,976</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | PhantomAir | 0 | <div style='text-align: right'>36,864</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2,048</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,523,136</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>131,072</div>  |
| revm_100_transfers | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>117,440,512</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| revm_100_transfers | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| revm_100_transfers | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>11,534,336</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>131,072</div>  |
| revm_100_transfers | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>4,063,232</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>4,194,304</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>1,605,632</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_100_transfers | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>4,554,752</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_100_transfers | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>1</div>  |
| revm_100_transfers | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>82,182,144</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>131,072</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| revm_100_transfers | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| revm_100_transfers | 0 | <span style="color: red">(+26.0 [+2.2%])</span> <div style='text-align: right'>1,205.0</div>  | <span style="color: green">(-59.0 [-0.6%])</span> <div style='text-align: right'>9,471.0</div>  | <div style='text-align: right'>411,492,740</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5c5bfa3973416ec9c0adf5c50172b52c596d9b03/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/5c5bfa3973416ec9c0adf5c50172b52c596d9b03

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11907256882)
