| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| revm_100_transfers | <div style='text-align: right'>2</div>  | <span style="color: red">(+273,786 [+0.3%])</span> <div style='text-align: right'>90,156,770</div>  | <span style="color: red">(+7,551 [+0.3%])</span> <div style='text-align: right'>2,329,515</div>  | <span style="color: red">(+223.0 [+1.3%])</span> <div style='text-align: right'>16,775.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true | <span style="color: red">(+191.0 [+0.3%])</span> <div style='text-align: right'>62,228.0</div>  | <span style="color: red">(+273,786 [+0.3%])</span> <div style='text-align: right'>90,156,770</div>  | <span style="color: red">(+7,551 [+0.3%])</span> <div style='text-align: right'>2,329,515</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: red">(+1,714 [+0.2%])</span> <div style='text-align: right'>815,634</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>29,893</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: red">(+389 [+0.3%])</span> <div style='text-align: right'>121,158</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: red">(+580 [+0.4%])</span> <div style='text-align: right'>146,189</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <span style="color: green">(-8 [-0.0%])</span> <div style='text-align: right'>73,629</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: red">(+906 [+2.0%])</span> <div style='text-align: right'>47,161</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>36,987</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: red">(+3,975 [+0.4%])</span> <div style='text-align: right'>961,063</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>41,134</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>17,170</div>  |
| revm_100_transfers | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_100_transfers | Memory AccessAdapter<8> | true | <span style="color: red">(+198 [+1.0%])</span> <div style='text-align: right'>19,764</div>  |
| revm_100_transfers | Memory Boundary | true | <span style="color: red">(+396 [+1.0%])</span> <div style='text-align: right'>39,528</div>  |
| revm_100_transfers | Memory Merkle | true | <span style="color: green">(-334 [-0.7%])</span> <div style='text-align: right'>48,390</div>  |
| revm_100_transfers | PhantomAir | true | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | ProgramChip | true | <span style="color: red">(+898 [+0.4%])</span> <div style='text-align: right'>247,615</div>  |
| revm_100_transfers | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_100_transfers | true |  | ADD | <span style="color: red">(+347 [+0.1%])</span> <div style='text-align: right'>524,550</div>  |
| revm_100_transfers | true |  | AND | <span style="color: red">(+678 [+0.8%])</span> <div style='text-align: right'>90,184</div>  |
| revm_100_transfers | true |  | AUIPC | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>17,170</div>  |
| revm_100_transfers | true |  | BEQ | <span style="color: red">(+286 [+0.6%])</span> <div style='text-align: right'>50,692</div>  |
| revm_100_transfers | true |  | BGE | <div style='text-align: right'>41</div>  |
| revm_100_transfers | true |  | BGEU | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>5,680</div>  |
| revm_100_transfers | true |  | BLT | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>3,139</div>  |
| revm_100_transfers | true |  | BLTU | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>64,769</div>  |
| revm_100_transfers | true |  | BNE | <span style="color: red">(+294 [+0.3%])</span> <div style='text-align: right'>95,497</div>  |
| revm_100_transfers | true |  | JAL | <span style="color: red">(+400 [+3.4%])</span> <div style='text-align: right'>12,185</div>  |
| revm_100_transfers | true |  | JALR | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>36,987</div>  |
| revm_100_transfers | true |  | LOADB | <div style='text-align: right'>15,137</div>  |
| revm_100_transfers | true |  | LOADBU | <span style="color: red">(+391 [+0.2%])</span> <div style='text-align: right'>190,794</div>  |
| revm_100_transfers | true |  | LOADW | <span style="color: red">(+2,399 [+0.6%])</span> <div style='text-align: right'>384,465</div>  |
| revm_100_transfers | true |  | LUI | <span style="color: red">(+506 [+1.5%])</span> <div style='text-align: right'>34,976</div>  |
| revm_100_transfers | true |  | MUL | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>41,134</div>  |
| revm_100_transfers | true |  | MULHU | <div style='text-align: right'>23,081</div>  |
| revm_100_transfers | true |  | OR | <span style="color: red">(+289 [+0.3%])</span> <div style='text-align: right'>107,041</div>  |
| revm_100_transfers | true |  | PHANTOM | <div style='text-align: right'>1,279</div>  |
| revm_100_transfers | true |  | SLL | <span style="color: red">(+387 [+0.4%])</span> <div style='text-align: right'>100,003</div>  |
| revm_100_transfers | true |  | SLT | <div style='text-align: right'>100</div>  |
| revm_100_transfers | true |  | SLTU | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>29,793</div>  |
| revm_100_transfers | true |  | SRA | <div style='text-align: right'>40</div>  |
| revm_100_transfers | true |  | SRL | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>21,115</div>  |
| revm_100_transfers | true |  | STOREB | <div style='text-align: right'>28,966</div>  |
| revm_100_transfers | true |  | STOREH | <div style='text-align: right'>300</div>  |
| revm_100_transfers | true |  | STOREW | <span style="color: red">(+1,185 [+0.3%])</span> <div style='text-align: right'>356,538</div>  |
| revm_100_transfers | true |  | SUB | <span style="color: red">(+3 [+0.0%])</span> <div style='text-align: right'>30,343</div>  |
| revm_100_transfers | true |  | XOR | <span style="color: red">(+397 [+0.6%])</span> <div style='text-align: right'>63,516</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: red">(+12,492 [+0.1%])</span> <div style='text-align: right'>18,883,800</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | ADD | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: red">(+24,408 [+0.8%])</span> <div style='text-align: right'>3,246,624</div>  |
| revm_100_transfers | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>360,570</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>51</div>  |
| revm_100_transfers | Boundary | true |  | AUIPC | <div style='text-align: right'>120</div>  |
| revm_100_transfers | Merkle | true |  | AUIPC | <div style='text-align: right'>3,520</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: red">(+7,436 [+0.6%])</span> <div style='text-align: right'>1,317,992</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>1,312</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <span style="color: green">(-32 [-0.0%])</span> <div style='text-align: right'>181,760</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <span style="color: green">(-32 [-0.0%])</span> <div style='text-align: right'>100,448</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <span style="color: green">(-192 [-0.0%])</span> <div style='text-align: right'>2,072,608</div>  |
| revm_100_transfers | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: red">(+7,644 [+0.3%])</span> <div style='text-align: right'>2,482,922</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: red">(+7,200 [+3.4%])</span> <div style='text-align: right'>219,330</div>  |
| revm_100_transfers | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-56 [-0.0%])</span> <div style='text-align: right'>1,035,636</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>529,795</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>68</div>  |
| revm_100_transfers | Boundary | true |  | LOADB | <div style='text-align: right'>160</div>  |
| revm_100_transfers | Merkle | true |  | LOADB | <span style="color: green">(-320 [-33.3%])</span> <div style='text-align: right'>640</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: red">(+15,640 [+0.2%])</span> <div style='text-align: right'>7,631,760</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADBU | <span style="color: red">(+68 [+50.0%])</span> <div style='text-align: right'>204</div>  |
| revm_100_transfers | Boundary | true |  | LOADBU | <span style="color: red">(+160 [+50.0%])</span> <div style='text-align: right'>480</div>  |
| revm_100_transfers | Merkle | true |  | LOADBU | <span style="color: red">(+896 [+70.0%])</span> <div style='text-align: right'>2,176</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: red">(+95,960 [+0.6%])</span> <div style='text-align: right'>15,378,600</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-102 [-1.1%])</span> <div style='text-align: right'>9,248</div>  |
| revm_100_transfers | Boundary | true |  | LOADW | <span style="color: green">(-240 [-1.1%])</span> <div style='text-align: right'>21,760</div>  |
| revm_100_transfers | Merkle | true |  | LOADW | <span style="color: green">(-832 [-1.7%])</span> <div style='text-align: right'>46,720</div>  |
| revm_100_transfers | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <span style="color: red">(+9,108 [+1.5%])</span> <div style='text-align: right'>629,568</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: red">(+62 [+0.0%])</span> <div style='text-align: right'>1,275,154</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>34</div>  |
| revm_100_transfers | Boundary | true |  | MUL | <div style='text-align: right'>80</div>  |
| revm_100_transfers | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_100_transfers | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>900,159</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_100_transfers | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: red">(+10,404 [+0.3%])</span> <div style='text-align: right'>3,853,476</div>  |
| revm_100_transfers | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>7,674</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: red">(+20,511 [+0.4%])</span> <div style='text-align: right'>5,300,159</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>3,700</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: green">(-148 [-0.0%])</span> <div style='text-align: right'>1,102,341</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>2,120</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: red">(+106 [+0.0%])</span> <div style='text-align: right'>1,119,095</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>1,158,640</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>39,576</div>  |
| revm_100_transfers | Boundary | true |  | STOREB | <div style='text-align: right'>93,120</div>  |
| revm_100_transfers | Merkle | true |  | STOREB | <span style="color: red">(+9,856 [+2.9%])</span> <div style='text-align: right'>354,176</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>12,000</div>  |
| revm_100_transfers | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: red">(+47,400 [+0.3%])</span> <div style='text-align: right'>14,261,520</div>  |
| revm_100_transfers | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+3,400 [+1.2%])</span> <div style='text-align: right'>286,722</div>  |
| revm_100_transfers | Boundary | true |  | STOREW | <span style="color: red">(+8,000 [+1.2%])</span> <div style='text-align: right'>674,640</div>  |
| revm_100_transfers | Merkle | true |  | STOREW | <span style="color: green">(-20,288 [-1.7%])</span> <div style='text-align: right'>1,141,056</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: red">(+108 [+0.0%])</span> <div style='text-align: right'>1,092,348</div>  |
| revm_100_transfers | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: red">(+14,292 [+0.6%])</span> <div style='text-align: right'>2,286,576</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_100_transfers | <span style="color: green">(-5.0 [-4.4%])</span> <div style='text-align: right'>108.0</div>  | <span style="color: red">(+156.0 [+2.6%])</span> <div style='text-align: right'>6,092.0</div>  | <span style="color: red">(+15.0 [+0.3%])</span> <div style='text-align: right'>4,714.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-8.0 [-3.7%])</span> <div style='text-align: right'>209.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+273,786 [+0.3%])</span> <div style='text-align: right'>90,156,770</div>  | <span style="color: red">(+7,551 [+0.3%])</span> <div style='text-align: right'>2,329,515</div>  | <span style="color: red">(+223.0 [+1.3%])</span> <div style='text-align: right'>16,775.0</div>  |

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
| revm_100_transfers | 0 | <span style="color: red">(+142.0 [+11.9%])</span> <div style='text-align: right'>1,338.0</div>  | <span style="color: green">(-75.0 [-0.8%])</span> <div style='text-align: right'>9,345.0</div>  | <div style='text-align: right'>411,492,740</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6/revm_transfer-revm_100_transfers.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/7a794c3ee59e8cdd0cd6e1cd943ed7b9cd4590a6
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11829393947)
