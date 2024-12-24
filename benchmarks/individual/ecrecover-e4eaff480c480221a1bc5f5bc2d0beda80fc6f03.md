| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: red">(+7,918,417 [+108.3%])</span> <div style='text-align: right'>15,230,037</div>  | <span style="color: red">(+384,966 [+197.4%])</span> <div style='text-align: right'>580,032</div>  | <span style="color: red">(+488.0 [+25.9%])</span> <div style='text-align: right'>2,373.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- |
| <span style="color: green">(-1.0 [-11.1%])</span> <div style='text-align: right'>8.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-1.0 [-0.1%])</span> <div style='text-align: right'>1,191.0</div>  |

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
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>449</div>  | <div style='text-align: right'>411</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>456</div>  | <div style='text-align: right'>422</div>  | <div style='text-align: right'>2</div>  |
| KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | <div style='text-align: right'>223</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | <div style='text-align: right'>188</div>  | <div style='text-align: right'>156</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | <div style='text-align: right'>126</div>  | <div style='text-align: right'>94</div>  | <div style='text-align: right'>2</div>  |
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
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | <div style='text-align: right'>286</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: red">(+488.0 [+25.9%])</span> <div style='text-align: right'>2,373.0</div>  | <span style="color: red">(+25,624,080 [+84.6%])</span> <div style='text-align: right'>55,907,135</div>  | <span style="color: red">(+7,918,417 [+108.3%])</span> <div style='text-align: right'>15,230,037</div>  | <span style="color: red">(+384,966 [+197.4%])</span> <div style='text-align: right'>580,032</div>  | <span style="color: red">(+28.0 [+62.2%])</span> <div style='text-align: right'>73.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| ecrecover_program | ProgramChip | 0 | <span style="color: red">(+178 [+2.1%])</span> <div style='text-align: right'>8,576</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | 0 | <span style="color: green">(-54 [-1.8%])</span> <div style='text-align: right'>2,982</div>  |
| ecrecover_program | Merkle | 0 | <span style="color: green">(-66 [-2.0%])</span> <div style='text-align: right'>3,274</div>  |
| ecrecover_program | AccessAdapter<2> | 0 | <div style='text-align: right'>132</div>  |
| ecrecover_program | AccessAdapter<4> | 0 | <div style='text-align: right'>68</div>  |
| ecrecover_program | AccessAdapter<8> | 0 | <span style="color: red">(+20,680 [+324.6%])</span> <div style='text-align: right'>27,050</div>  |
| ecrecover_program | AccessAdapter<16> | 0 | <span style="color: red">(+10,348 [+359.6%])</span> <div style='text-align: right'>13,226</div>  |
| ecrecover_program | AccessAdapter<32> | 0 | <span style="color: red">(+5,174 [+359.3%])</span> <div style='text-align: right'>6,614</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | 0 | <div style='text-align: right'>1,271</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | 0 | <div style='text-align: right'>726</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | 0 | <div style='text-align: right'>3,194</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | 0 | <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | 0 | <div style='text-align: right'>6</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | 0 | <div style='text-align: right'>5</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>5</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <span style="color: red">(+2,543 [+15893.8%])</span> <div style='text-align: right'>2,559</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <span style="color: red">(+2,548 [+305.1%])</span> <div style='text-align: right'>3,383</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <span style="color: red">(+5,096 [+329.0%])</span> <div style='text-align: right'>6,645</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <span style="color: red">(+2,783 [+218.6%])</span> <div style='text-align: right'>4,056</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <span style="color: red">(+17,541 [+298.1%])</span> <div style='text-align: right'>23,426</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <span style="color: red">(+3,453 [+28.9%])</span> <div style='text-align: right'>15,389</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <span style="color: red">(+3,640 [+2600.0%])</span> <div style='text-align: right'>3,780</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <span style="color: red">(+78,680 [+254.4%])</span> <div style='text-align: right'>109,606</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <span style="color: red">(+6,500 [+282.2%])</span> <div style='text-align: right'>8,803</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <span style="color: red">(+260 [+14.8%])</span> <div style='text-align: right'>2,011</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <span style="color: red">(+69,439 [+196.1%])</span> <div style='text-align: right'>104,848</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>45</div>  |
| ecrecover_program | Arc<BabyBearParameters>, 1> | 0 | <span style="color: green">(-19 [-0.9%])</span> <div style='text-align: right'>2,061</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| ecrecover_program |  | ADD | 0 | <span style="color: red">(+45,450 [+162.2%])</span> <div style='text-align: right'>73,466</div>  |
| ecrecover_program |  | AND | 0 | <span style="color: red">(+10,697 [+220.8%])</span> <div style='text-align: right'>15,542</div>  |
| ecrecover_program |  | AUIPC | 0 | <span style="color: red">(+2,548 [+305.1%])</span> <div style='text-align: right'>3,383</div>  |
| ecrecover_program |  | BEQ | 0 | <span style="color: red">(+2,673 [+33.7%])</span> <div style='text-align: right'>10,612</div>  |
| ecrecover_program |  | BGEU | 0 | <div style='text-align: right'>925</div>  |
| ecrecover_program |  | BLT | 0 | <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | 0 | <span style="color: red">(+17,541 [+354.5%])</span> <div style='text-align: right'>22,489</div>  |
| ecrecover_program |  | BNE | 0 | <span style="color: red">(+780 [+19.5%])</span> <div style='text-align: right'>4,777</div>  |
| ecrecover_program |  | DIVU | 0 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | EcAddNe | 0 | <div style='text-align: right'>726</div>  |
| ecrecover_program |  | EcDouble | 0 | <div style='text-align: right'>1,271</div>  |
| ecrecover_program |  | HINT_STOREW | 0 | <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | 0 | <div style='text-align: right'>3,203</div>  |
| ecrecover_program |  | JAL | 0 | <span style="color: red">(+260 [+25.9%])</span> <div style='text-align: right'>1,263</div>  |
| ecrecover_program |  | JALR | 0 | <span style="color: red">(+5,096 [+329.0%])</span> <div style='text-align: right'>6,645</div>  |
| ecrecover_program |  | KECCAK256 | 0 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | 0 | <span style="color: red">(+3,640 [+2600.0%])</span> <div style='text-align: right'>3,780</div>  |
| ecrecover_program |  | LOADBU | 0 | <div style='text-align: right'>2,450</div>  |
| ecrecover_program |  | LOADW | 0 | <span style="color: green">(-760 [-5.2%])</span> <div style='text-align: right'>13,766</div>  |
| ecrecover_program |  | LUI | 0 | <span style="color: red">(+2,523 [+934.4%])</span> <div style='text-align: right'>2,793</div>  |
| ecrecover_program |  | MUL | 0 | <span style="color: red">(+2,543 [+15893.8%])</span> <div style='text-align: right'>2,559</div>  |
| ecrecover_program |  | MULHU | 0 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | ModularAddSub | 0 | <div style='text-align: right'>7</div>  |
| ecrecover_program |  | ModularMulDiv | 0 | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | 0 | <span style="color: red">(+5,663 [+434.9%])</span> <div style='text-align: right'>6,965</div>  |
| ecrecover_program |  | PHANTOM | 0 | <div style='text-align: right'>45</div>  |
| ecrecover_program |  | SETUP_ISEQ | 0 | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | 0 | <span style="color: red">(+3,120 [+261.7%])</span> <div style='text-align: right'>4,312</div>  |
| ecrecover_program |  | SLTU | 0 | <span style="color: red">(+260 [+14.8%])</span> <div style='text-align: right'>2,011</div>  |
| ecrecover_program |  | SRL | 0 | <span style="color: red">(+3,380 [+304.2%])</span> <div style='text-align: right'>4,491</div>  |
| ecrecover_program |  | STOREB | 0 | <span style="color: red">(+24,504 [+1708.8%])</span> <div style='text-align: right'>25,938</div>  |
| ecrecover_program |  | STOREW | 0 | <span style="color: red">(+54,936 [+438.9%])</span> <div style='text-align: right'>67,452</div>  |
| ecrecover_program |  | SUB | 0 | <span style="color: red">(+7,629 [+624.8%])</span> <div style='text-align: right'>8,850</div>  |
| ecrecover_program |  | XOR | 0 | <div style='text-align: right'>25</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <span style="color: red">(+1,636,200 [+162.2%])</span> <div style='text-align: right'>2,644,776</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>68</div>  |
| ecrecover_program | Boundary |  | ADD | 0 | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | ADD | 0 | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <span style="color: red">(+385,092 [+220.8%])</span> <div style='text-align: right'>559,512</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <span style="color: red">(+53,508 [+305.1%])</span> <div style='text-align: right'>71,043</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary |  | AUIPC | 0 | <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,520</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <span style="color: red">(+69,498 [+33.7%])</span> <div style='text-align: right'>275,912</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>29,600</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <span style="color: red">(+561,312 [+354.5%])</span> <div style='text-align: right'>719,648</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <span style="color: red">(+20,280 [+19.5%])</span> <div style='text-align: right'>124,202</div>  |
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> |  | DIVU | 0 | <div style='text-align: right'>285</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | 0 | <div style='text-align: right'>449,394</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | 0 | <div style='text-align: right'>27,175</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | 0 | <div style='text-align: right'>22,304</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | 0 | <div style='text-align: right'>36,890</div>  |
| ecrecover_program | Boundary |  | EcAddNe | 0 | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | 0 | <div style='text-align: right'>256</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | 0 | <div style='text-align: right'>690,153</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | 0 | <div style='text-align: right'>1,600</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | 0 | <div style='text-align: right'>1,312</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | 0 | <span style="color: red">(+136 [+6.7%])</span> <div style='text-align: right'>2,176</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | 0 | <span style="color: red">(+68 [+4.5%])</span> <div style='text-align: right'>1,581</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | 0 | <span style="color: red">(+160 [+4.5%])</span> <div style='text-align: right'>3,720</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | 0 | <span style="color: green">(-128 [-1.9%])</span> <div style='text-align: right'>6,464</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | 0 | <div style='text-align: right'>531,698</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | 0 | <span style="color: red">(+129,350 [+2874.4%])</span> <div style='text-align: right'>133,850</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | 0 | <span style="color: red">(+106,067 [+2874.4%])</span> <div style='text-align: right'>109,757</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | 0 | <span style="color: red">(+175,984 [+2907.9%])</span> <div style='text-align: right'>182,036</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <span style="color: red">(+4,680 [+25.9%])</span> <div style='text-align: right'>22,734</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <span style="color: red">(+142,688 [+329.0%])</span> <div style='text-align: right'>186,060</div>  |
| ecrecover_program | AccessAdapter<8> |  | KECCAK256 | 0 | <div style='text-align: right'>68</div>  |
| ecrecover_program | Boundary |  | KECCAK256 | 0 | <div style='text-align: right'>160</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | 0 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | Merkle |  | KECCAK256 | 0 | <div style='text-align: right'>128</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <span style="color: red">(+127,400 [+2600.0%])</span> <div style='text-align: right'>132,300</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>98,000</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | 0 | <div style='text-align: right'>125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | 0 | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>85</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <span style="color: green">(-30,400 [-5.2%])</span> <div style='text-align: right'>550,640</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | 0 | <div style='text-align: right'>17,625</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | 0 | <div style='text-align: right'>14,350</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>24,276</div>  |
| ecrecover_program | Boundary |  | LOADW | 0 | <div style='text-align: right'>520</div>  |
| ecrecover_program | Merkle |  | LOADW | 0 | <span style="color: green">(-64 [-1.8%])</span> <div style='text-align: right'>3,584</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <span style="color: red">(+45,414 [+934.4%])</span> <div style='text-align: right'>50,274</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <span style="color: red">(+78,833 [+15893.8%])</span> <div style='text-align: right'>79,329</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>195</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | 0 | <div style='text-align: right'>1,393</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | 0 | <div style='text-align: right'>700</div>  |
| ecrecover_program | AccessAdapter<2> |  | ModularAddSub | 0 | <div style='text-align: right'>726</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | 0 | <div style='text-align: right'>574</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | 0 | <div style='text-align: right'>442</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | 0 | <div style='text-align: right'>969</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | 0 | <div style='text-align: right'>680</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | 0 | <div style='text-align: right'>2,624</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | 0 | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | 0 | <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | 0 | <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | 0 | <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <span style="color: red">(+203,868 [+434.9%])</span> <div style='text-align: right'>250,740</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>270</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | 0 | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <span style="color: red">(+165,360 [+261.7%])</span> <div style='text-align: right'>228,536</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <span style="color: red">(+9,620 [+14.8%])</span> <div style='text-align: right'>74,407</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <span style="color: red">(+179,140 [+304.2%])</span> <div style='text-align: right'>238,023</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <span style="color: red">(+980,160 [+1708.8%])</span> <div style='text-align: right'>1,037,520</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | 0 | <span style="color: red">(+64,800 [+10368.0%])</span> <div style='text-align: right'>65,425</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | 0 | <span style="color: red">(+105,903 [+15194.1%])</span> <div style='text-align: right'>106,600</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | 0 | <span style="color: red">(+44,285 [+3888.1%])</span> <div style='text-align: right'>45,424</div>  |
| ecrecover_program | Boundary |  | STOREB | 0 | <span style="color: red">(+40 [+3.3%])</span> <div style='text-align: right'>1,240</div>  |
| ecrecover_program | Merkle |  | STOREB | 0 | <span style="color: red">(+128 [+4.9%])</span> <div style='text-align: right'>2,752</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <span style="color: red">(+2,197,440 [+438.9%])</span> <div style='text-align: right'>2,698,080</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | 0 | <span style="color: red">(+64,650 [+2416.8%])</span> <div style='text-align: right'>67,325</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | 0 | <span style="color: red">(+246 [+12.0%])</span> <div style='text-align: right'>2,296</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | 0 | <span style="color: red">(+131,427 [+493.4%])</span> <div style='text-align: right'>158,066</div>  |
| ecrecover_program | Boundary |  | STOREW | 0 | <span style="color: green">(-800 [-1.5%])</span> <div style='text-align: right'>52,840</div>  |
| ecrecover_program | Merkle |  | STOREW | 0 | <span style="color: green">(-1,152 [-1.3%])</span> <div style='text-align: right'>85,312</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <span style="color: red">(+274,644 [+624.8%])</span> <div style='text-align: right'>318,600</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>900</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <span style="color: red">(+256.0 [+101.2%])</span> <div style='text-align: right'>509.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+7,918,417 [+108.3%])</span> <div style='text-align: right'>15,230,037</div>  | <span style="color: red">(+384,966 [+197.4%])</span> <div style='text-align: right'>580,032</div>  | <span style="color: red">(+488.0 [+25.9%])</span> <div style='text-align: right'>2,373.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>212,992</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>8,960</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>4,736</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <span style="color: red">(+1,007,616 [+300.0%])</span> <div style='text-align: right'>1,343,488</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: red">(+24,576 [+300.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <span style="color: red">(+602,112 [+300.0%])</span> <div style='text-align: right'>802,816</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: red">(+12,288 [+300.0%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <span style="color: red">(+399,360 [+300.0%])</span> <div style='text-align: right'>532,480</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: red">(+6,144 [+300.0%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>1,502,208</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>909,312</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>16</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>3,128</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>8</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>1,288</div>  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  | <div style='text-align: right'>8</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>1,112</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>8</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <span style="color: red">(+452,880 [+25500.0%])</span> <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: red">(+4,080 [+25500.0%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <span style="color: red">(+150,528 [+300.0%])</span> <div style='text-align: right'>200,704</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <span style="color: red">(+3,072 [+300.0%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: red">(+393,216 [+300.0%])</span> <div style='text-align: right'>524,288</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <span style="color: red">(+6,144 [+300.0%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <span style="color: red">(+126,976 [+100.0%])</span> <div style='text-align: right'>253,952</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <span style="color: red">(+2,048 [+100.0%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <span style="color: red">(+2,162,688 [+300.0%])</span> <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: red">(+24,576 [+300.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>1,212,416</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <span style="color: red">(+426,240 [+1500.0%])</span> <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <span style="color: red">(+3,840 [+1500.0%])</span> <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: red">(+11,010,048 [+300.0%])</span> <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <span style="color: red">(+98,304 [+300.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: red">(+1,290,240 [+300.0%])</span> <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: red">(+12,288 [+300.0%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>157,696</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <span style="color: red">(+7,602,176 [+100.0%])</span> <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: red">(+65,536 [+100.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| ecrecover_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | <div style='text-align: right'>1,261,568</div>  | <div style='text-align: right'>300</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03/ecrecover-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/e4eaff480c480221a1bc5f5bc2d0beda80fc6f03

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12477477726)
