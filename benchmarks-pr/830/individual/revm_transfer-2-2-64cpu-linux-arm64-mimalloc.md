| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>2</div>  | <span style="color: red">(+48,924,992 [+54.4%])</span> <div style='text-align: right'>138,799,555</div>  | <span style="color: red">(+3,068 [+0.1%])</span> <div style='text-align: right'>2,325,156</div>  | <span style="color: green">(-1,063.0 [-6.3%])</span> <div style='text-align: right'>15,917.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true | <span style="color: green">(-51,304.0 [-78.9%])</span> <div style='text-align: right'>13,706.0</div>  | <span style="color: red">(+48,924,992 [+54.4%])</span> <div style='text-align: right'>138,799,555</div>  | <span style="color: red">(+3,068 [+0.1%])</span> <div style='text-align: right'>2,325,156</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_100_transfers | ProgramChip | true | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>247,613</div>  |
| revm_100_transfers | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| revm_100_transfers | Boundary | true | <div style='text-align: right'>39,492</div>  |
| revm_100_transfers | Merkle | true | <div style='text-align: right'>47,274</div>  |
| revm_100_transfers | AccessAdapter<8> | true | <div style='text-align: right'>39,492</div>  |
| revm_100_transfers | PhantomAir | true | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: red">(+1,873 [+0.2%])</span> <div style='text-align: right'>813,386</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: red">(+100 [+0.3%])</span> <div style='text-align: right'>29,993</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: red">(+411 [+0.3%])</span> <div style='text-align: right'>120,799</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: red">(+612 [+0.1%])</span> <div style='text-align: right'>960,017</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-11 [-0.0%])</span> <div style='text-align: right'>145,486</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>73,629</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: red">(+104 [+0.2%])</span> <div style='text-align: right'>47,072</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-9 [-0.0%])</span> <div style='text-align: right'>36,981</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>17,166</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>41,130</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>86,766</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| revm_100_transfers | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true |  | ADD | <span style="color: red">(+854 [+0.2%])</span> <div style='text-align: right'>523,208</div>  |
| revm_100_transfers | true |  | AND | <span style="color: red">(+610 [+0.7%])</span> <div style='text-align: right'>89,832</div>  |
| revm_100_transfers | true |  | AUIPC | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>17,166</div>  |
| revm_100_transfers | true |  | BEQ | <span style="color: red">(+9 [+0.0%])</span> <div style='text-align: right'>49,931</div>  |
| revm_100_transfers | true |  | BGE | <div style='text-align: right'>41</div>  |
| revm_100_transfers | true |  | BGEU | <div style='text-align: right'>5,680</div>  |
| revm_100_transfers | true |  | BLT | <div style='text-align: right'>3,139</div>  |
| revm_100_transfers | true |  | BLTU | <div style='text-align: right'>64,769</div>  |
| revm_100_transfers | true |  | BNE | <span style="color: green">(-20 [-0.0%])</span> <div style='text-align: right'>95,555</div>  |
| revm_100_transfers | true |  | JAL | <span style="color: red">(+104 [+0.9%])</span> <div style='text-align: right'>12,096</div>  |
| revm_100_transfers | true |  | JALR | <span style="color: green">(-9 [-0.0%])</span> <div style='text-align: right'>36,981</div>  |
| revm_100_transfers | true |  | LOADB | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | true |  | LOADBU | <span style="color: red">(+293 [+0.2%])</span> <div style='text-align: right'>190,394</div>  |
| revm_100_transfers | true |  | LOADW | <span style="color: red">(+211 [+0.1%])</span> <div style='text-align: right'>384,097</div>  |
| revm_100_transfers | true |  | LUI | <div style='text-align: right'>34,976</div>  |
| revm_100_transfers | true |  | MUL | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>41,130</div>  |
| revm_100_transfers | true |  | MULHU | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | true |  | OR | <span style="color: red">(+312 [+0.3%])</span> <div style='text-align: right'>106,774</div>  |
| revm_100_transfers | true |  | PHANTOM | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | true |  | SLL | <span style="color: red">(+417 [+0.4%])</span> <div style='text-align: right'>99,648</div>  |
| revm_100_transfers | true |  | SLT | <div style='text-align: right'>100</div>  |
| revm_100_transfers | true |  | SLTU | <span style="color: red">(+100 [+0.3%])</span> <div style='text-align: right'>29,893</div>  |
| revm_100_transfers | true |  | SRA | <div style='text-align: right'>40</div>  |
| revm_100_transfers | true |  | SRL | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>21,111</div>  |
| revm_100_transfers | true |  | STOREB | <div style='text-align: right'>28,966</div>  |
| revm_100_transfers | true |  | STOREH | <div style='text-align: right'>300</div>  |
| revm_100_transfers | true |  | STOREW | <span style="color: red">(+108 [+0.0%])</span> <div style='text-align: right'>356,260</div>  |
| revm_100_transfers | true |  | SUB | <span style="color: green">(-9 [-0.0%])</span> <div style='text-align: right'>30,337</div>  |
| revm_100_transfers | true |  | XOR | <span style="color: red">(+106 [+0.2%])</span> <div style='text-align: right'>63,235</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: red">(+30,744 [+0.2%])</span> <div style='text-align: right'>18,835,488</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: red">(+21,960 [+0.7%])</span> <div style='text-align: right'>3,233,952</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-126 [-0.0%])</span> <div style='text-align: right'>360,486</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | AUIPC | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | AUIPC | <div style='text-align: right'>3,520</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: red">(+234 [+0.0%])</span> <div style='text-align: right'>1,298,206</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>1,312</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>181,760</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>100,448</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>2,072,608</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-520 [-0.0%])</span> <div style='text-align: right'>2,484,430</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: red">(+1,872 [+0.9%])</span> <div style='text-align: right'>217,728</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-252 [-0.0%])</span> <div style='text-align: right'>1,035,468</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>529,795</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>68</div>  |
| revm_100_transfers | Boundary | true |  | LOADB | <div style='text-align: right'>160</div>  |
| revm_100_transfers | Merkle | true |  | LOADB | <span style="color: red">(+256 [+40.0%])</span> <div style='text-align: right'>896</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: red">(+11,720 [+0.2%])</span> <div style='text-align: right'>7,615,760</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>204</div>  |
| revm_100_transfers | Boundary | true |  | LOADBU | <div style='text-align: right'>480</div>  |
| revm_100_transfers | Merkle | true |  | LOADBU | <span style="color: red">(+256 [+11.4%])</span> <div style='text-align: right'>2,496</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: red">(+8,440 [+0.1%])</span> <div style='text-align: right'>15,363,880</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>9,265</div>  |
| revm_100_transfers | Boundary | true |  | LOADW | <div style='text-align: right'>21,800</div>  |
| revm_100_transfers | Merkle | true |  | LOADW | <span style="color: green">(-768 [-1.6%])</span> <div style='text-align: right'>46,592</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>629,568</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: green">(-186 [-0.0%])</span> <div style='text-align: right'>1,275,030</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>34</div>  |
| revm_100_transfers | Boundary | true |  | MUL | <div style='text-align: right'>80</div>  |
| revm_100_transfers | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>900,159</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: red">(+11,232 [+0.3%])</span> <div style='text-align: right'>3,843,864</div>  |
| revm_100_transfers | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>7,674</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: red">(+22,101 [+0.4%])</span> <div style='text-align: right'>5,281,344</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>3,700</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: red">(+3,700 [+0.3%])</span> <div style='text-align: right'>1,106,041</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>2,120</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: green">(-318 [-0.0%])</span> <div style='text-align: right'>1,118,883</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>1,158,640</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>39,576</div>  |
| revm_100_transfers | Boundary | true |  | STOREB | <div style='text-align: right'>93,120</div>  |
| revm_100_transfers | Merkle | true |  | STOREB | <span style="color: green">(-7,040 [-1.9%])</span> <div style='text-align: right'>354,176</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>12,000</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: red">(+4,320 [+0.0%])</span> <div style='text-align: right'>14,250,400</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREW | <span style="color: green">(-1,870 [-0.6%])</span> <div style='text-align: right'>286,399</div>  |
| revm_100_transfers | Boundary | true |  | STOREW | <span style="color: green">(-4,400 [-0.6%])</span> <div style='text-align: right'>673,880</div>  |
| revm_100_transfers | Merkle | true |  | STOREW | <span style="color: green">(-17,728 [-1.6%])</span> <div style='text-align: right'>1,104,896</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: green">(-324 [-0.0%])</span> <div style='text-align: right'>1,092,132</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: red">(+3,816 [+0.2%])</span> <div style='text-align: right'>2,276,460</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>112.0</div>  | <span style="color: green">(-1,008.0 [-15.9%])</span> <div style='text-align: right'>5,315.0</div>  | <span style="color: green">(-1,009.0 [-19.8%])</span> <div style='text-align: right'>4,082.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-22.0 [-10.4%])</span> <div style='text-align: right'>190.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+48,924,992 [+54.4%])</span> <div style='text-align: right'>138,799,555</div>  | <span style="color: red">(+3,068 [+0.1%])</span> <div style='text-align: right'>2,325,156</div>  | <span style="color: green">(-1,063.0 [-6.3%])</span> <div style='text-align: right'>15,917.0</div>  |

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
| revm_100_transfers | 0 | <span style="color: green">(-3.0 [-0.3%])</span> <div style='text-align: right'>1,189.0</div>  | <span style="color: green">(-52.0 [-0.5%])</span> <div style='text-align: right'>9,413.0</div>  | <div style='text-align: right'>411,492,740</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/351d629ff6031ebe39217a9ec2d307f7350c4768/revm_transfer-2-2-64cpu-linux-arm64-mimalloc-revm_100_transfers.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/351d629ff6031ebe39217a9ec2d307f7350c4768

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11867621255)
