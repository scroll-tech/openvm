| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>2</div>  | <span style="color: green">(-292,755 [-0.3%])</span> <div style='text-align: right'>89,667,190</div>  | <span style="color: green">(-8,018 [-0.3%])</span> <div style='text-align: right'>2,315,638</div>  | <span style="color: green">(-21.0 [-0.1%])</span> <div style='text-align: right'>15,522.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true | <span style="color: green">(-310.0 [-0.5%])</span> <div style='text-align: right'>61,852.0</div>  | <span style="color: green">(-292,755 [-0.3%])</span> <div style='text-align: right'>89,667,190</div>  | <span style="color: green">(-8,018 [-0.3%])</span> <div style='text-align: right'>2,315,638</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-4,370 [-0.5%])</span> <div style='text-align: right'>810,491</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>29,897</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: green">(-776 [-0.6%])</span> <div style='text-align: right'>120,007</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-853 [-0.6%])</span> <div style='text-align: right'>145,012</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>73,637</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: green">(-192 [-0.4%])</span> <div style='text-align: right'>46,270</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>36,986</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-1,814 [-0.2%])</span> <div style='text-align: right'>955,542</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>41,130</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>17,169</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | Memory AccessAdapter<8> | true | <span style="color: red">(+8 [+0.0%])</span> <div style='text-align: right'>19,762</div>  |
| revm_100_transfers | Memory Boundary | true | <span style="color: red">(+16 [+0.0%])</span> <div style='text-align: right'>39,524</div>  |
| revm_100_transfers | Memory Merkle | true | <span style="color: red">(+118 [+0.2%])</span> <div style='text-align: right'>49,166</div>  |
| revm_100_transfers | PhantomAir | true | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | ProgramChip | true | <div style='text-align: right'>246,717</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true |  | ADD | <span style="color: green">(-2,433 [-0.5%])</span> <div style='text-align: right'>522,270</div>  |
| revm_100_transfers | true |  | AND | <span style="color: green">(-968 [-1.1%])</span> <div style='text-align: right'>88,758</div>  |
| revm_100_transfers | true |  | AUIPC | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>17,169</div>  |
| revm_100_transfers | true |  | BEQ | <span style="color: green">(-773 [-1.5%])</span> <div style='text-align: right'>49,849</div>  |
| revm_100_transfers | true |  | BGE | <div style='text-align: right'>41</div>  |
| revm_100_transfers | true |  | BGEU | <div style='text-align: right'>5,681</div>  |
| revm_100_transfers | true |  | BLT | <div style='text-align: right'>3,140</div>  |
| revm_100_transfers | true |  | BLTU | <div style='text-align: right'>64,775</div>  |
| revm_100_transfers | true |  | BNE | <span style="color: green">(-80 [-0.1%])</span> <div style='text-align: right'>95,163</div>  |
| revm_100_transfers | true |  | JAL | <span style="color: green">(-192 [-1.6%])</span> <div style='text-align: right'>11,800</div>  |
| revm_100_transfers | true |  | JALR | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>36,986</div>  |
| revm_100_transfers | true |  | LOADB | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | true |  | LOADBU | <span style="color: green">(-854 [-0.4%])</span> <div style='text-align: right'>189,602</div>  |
| revm_100_transfers | true |  | LOADW | <span style="color: green">(-576 [-0.2%])</span> <div style='text-align: right'>381,701</div>  |
| revm_100_transfers | true |  | LUI | <div style='text-align: right'>34,470</div>  |
| revm_100_transfers | true |  | MUL | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>41,130</div>  |
| revm_100_transfers | true |  | MULHU | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | true |  | OR | <span style="color: green">(-579 [-0.5%])</span> <div style='text-align: right'>106,182</div>  |
| revm_100_transfers | true |  | PHANTOM | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | true |  | SLL | <span style="color: green">(-772 [-0.8%])</span> <div style='text-align: right'>98,856</div>  |
| revm_100_transfers | true |  | SLT | <div style='text-align: right'>100</div>  |
| revm_100_transfers | true |  | SLTU | <div style='text-align: right'>29,797</div>  |
| revm_100_transfers | true |  | SRA | <div style='text-align: right'>40</div>  |
| revm_100_transfers | true |  | SRL | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>21,111</div>  |
| revm_100_transfers | true |  | STOREB | <div style='text-align: right'>28,966</div>  |
| revm_100_transfers | true |  | STOREH | <div style='text-align: right'>300</div>  |
| revm_100_transfers | true |  | STOREW | <span style="color: green">(-384 [-0.1%])</span> <div style='text-align: right'>354,973</div>  |
| revm_100_transfers | true |  | SUB | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>30,337</div>  |
| revm_100_transfers | true |  | XOR | <span style="color: green">(-384 [-0.6%])</span> <div style='text-align: right'>62,944</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-87,588 [-0.5%])</span> <div style='text-align: right'>18,801,720</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: green">(-34,848 [-1.1%])</span> <div style='text-align: right'>3,195,288</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-84 [-0.0%])</span> <div style='text-align: right'>360,549</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | AUIPC | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | AUIPC | <div style='text-align: right'>3,520</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: green">(-20,098 [-1.5%])</span> <div style='text-align: right'>1,296,074</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>1,312</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>181,792</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>100,480</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>2,072,800</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-2,080 [-0.1%])</span> <div style='text-align: right'>2,474,238</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: green">(-3,456 [-1.6%])</span> <div style='text-align: right'>212,400</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-168 [-0.0%])</span> <div style='text-align: right'>1,035,608</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>529,795</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>68</div>  |
| revm_100_transfers | Boundary | true |  | LOADB | <div style='text-align: right'>160</div>  |
| revm_100_transfers | Merkle | true |  | LOADB | <div style='text-align: right'>960</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: green">(-34,160 [-0.4%])</span> <div style='text-align: right'>7,584,080</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>136</div>  |
| revm_100_transfers | Boundary | true |  | LOADBU | <div style='text-align: right'>320</div>  |
| revm_100_transfers | Merkle | true |  | LOADBU | <div style='text-align: right'>1,024</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-23,040 [-0.2%])</span> <div style='text-align: right'>15,268,040</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>9,367</div>  |
| revm_100_transfers | Boundary | true |  | LOADW | <div style='text-align: right'>22,040</div>  |
| revm_100_transfers | Merkle | true |  | LOADW | <span style="color: red">(+64 [+0.1%])</span> <div style='text-align: right'>48,000</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>620,460</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: green">(-93 [-0.0%])</span> <div style='text-align: right'>1,275,030</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>34</div>  |
| revm_100_transfers | Boundary | true |  | MUL | <div style='text-align: right'>80</div>  |
| revm_100_transfers | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>900,159</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: green">(-20,844 [-0.5%])</span> <div style='text-align: right'>3,822,552</div>  |
| revm_100_transfers | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>7,674</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: green">(-40,916 [-0.8%])</span> <div style='text-align: right'>5,239,368</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>3,700</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>1,102,489</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>2,120</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: green">(-212 [-0.0%])</span> <div style='text-align: right'>1,118,883</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>1,158,640</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>39,576</div>  |
| revm_100_transfers | Boundary | true |  | STOREB | <div style='text-align: right'>93,120</div>  |
| revm_100_transfers | Merkle | true |  | STOREB | <div style='text-align: right'>344,320</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>12,000</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-15,360 [-0.1%])</span> <div style='text-align: right'>14,198,920</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+136 [+0.0%])</span> <div style='text-align: right'>286,637</div>  |
| revm_100_transfers | Boundary | true |  | STOREW | <span style="color: red">(+320 [+0.0%])</span> <div style='text-align: right'>674,440</div>  |
| revm_100_transfers | Merkle | true |  | STOREW | <span style="color: red">(+3,712 [+0.3%])</span> <div style='text-align: right'>1,175,296</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: green">(-216 [-0.0%])</span> <div style='text-align: right'>1,092,132</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: green">(-13,824 [-0.6%])</span> <div style='text-align: right'>2,265,984</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <span style="color: red">(+1.0 [+0.9%])</span> <div style='text-align: right'>112.0</div>  | <span style="color: green">(-29.0 [-0.5%])</span> <div style='text-align: right'>5,921.0</div>  | <span style="color: green">(-14.0 [-0.3%])</span> <div style='text-align: right'>4,690.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-15.0 [-6.6%])</span> <div style='text-align: right'>213.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-292,755 [-0.3%])</span> <div style='text-align: right'>89,667,190</div>  | <span style="color: green">(-8,018 [-0.3%])</span> <div style='text-align: right'>2,315,638</div>  | <span style="color: green">(-21.0 [-0.1%])</span> <div style='text-align: right'>15,522.0</div>  |

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

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| revm_100_transfers | 0 | <span style="color: red">(+8.0 [+0.1%])</span> <div style='text-align: right'>9,601.0</div>  | <div style='text-align: right'>411,492,740</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a04f089f00b73abfeae9403dec4e7c2daa32f84a/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/a04f089f00b73abfeae9403dec4e7c2daa32f84a
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11789190178)
