| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>238,660,282</div>  | <div style='text-align: right'>4,181,050</div>  | <span style="color: red">(+550.0 [+1.9%])</span> <div style='text-align: right'>29,001.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| regex_program | true | <span style="color: red">(+297.0 [+0.4%])</span> <div style='text-align: right'>68,609.0</div>  | <div style='text-align: right'>238,660,282</div>  | <div style='text-align: right'>4,181,050</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| regex_program | ProgramChip | true | <div style='text-align: right'>89,500</div>  |
| regex_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| regex_program | Boundary | true | <div style='text-align: right'>69,276</div>  |
| regex_program | Merkle | true | <div style='text-align: right'>70,540</div>  |
| regex_program | AccessAdapter<2> | true | <div style='text-align: right'>42</div>  |
| regex_program | AccessAdapter<4> | true | <div style='text-align: right'>22</div>  |
| regex_program | AccessAdapter<8> | true | <div style='text-align: right'>69,276</div>  |
| regex_program | PhantomAir | true | <div style='text-align: right'>289</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>1,150,316</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>38,005</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>218,632</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>1,960,994</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>687</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>282,059</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>198,079</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>96,823</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>130,412</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>39,541</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>52,087</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>12,767</div>  |
| regex_program | KeccakVmAir | true | <div style='text-align: right'>24</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>139,816</div>  |
| regex_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| regex_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| regex_program | true |  | ADD | <div style='text-align: right'>1,007,844</div>  |
| regex_program | true |  | AND | <div style='text-align: right'>66,789</div>  |
| regex_program | true |  | AUIPC | <div style='text-align: right'>39,541</div>  |
| regex_program | true |  | BEQ | <div style='text-align: right'>160,033</div>  |
| regex_program | true |  | BGE | <div style='text-align: right'>294</div>  |
| regex_program | true |  | BGEU | <div style='text-align: right'>121,615</div>  |
| regex_program | true |  | BLT | <div style='text-align: right'>5,141</div>  |
| regex_program | true |  | BLTU | <div style='text-align: right'>71,029</div>  |
| regex_program | true |  | BNE | <div style='text-align: right'>122,026</div>  |
| regex_program | true |  | DIVU | <div style='text-align: right'>114</div>  |
| regex_program | true |  | HINT_STOREW | <div style='text-align: right'>12,767</div>  |
| regex_program | true |  | JAL | <div style='text-align: right'>52,349</div>  |
| regex_program | true |  | JALR | <div style='text-align: right'>130,412</div>  |
| regex_program | true |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| regex_program | true |  | LOADB | <div style='text-align: right'>679</div>  |
| regex_program | true |  | LOADBU | <div style='text-align: right'>27,294</div>  |
| regex_program | true |  | LOADH | <div style='text-align: right'>8</div>  |
| regex_program | true |  | LOADHU | <div style='text-align: right'>95</div>  |
| regex_program | true |  | LOADW | <div style='text-align: right'>1,142,626</div>  |
| regex_program | true |  | LUI | <div style='text-align: right'>44,474</div>  |
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
| regex_program | true |  | STOREW | <div style='text-align: right'>768,184</div>  |
| regex_program | true |  | SUB | <div style='text-align: right'>42,583</div>  |
| regex_program | true |  | XOR | <div style='text-align: right'>9,564</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>36,282,384</div>  |
| regex_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>102</div>  |
| regex_program | Boundary | true |  | ADD | <div style='text-align: right'>240</div>  |
| regex_program | Merkle | true |  | ADD | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>2,404,404</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>830,361</div>  |
| regex_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| regex_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| regex_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>4,160,858</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>3,891,680</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>2,272,928</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>3,172,676</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>108,511</div>  |
| regex_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>255,320</div>  |
| regex_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>408,640</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>942,282</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>3,651,536</div>  |
| regex_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>143</div>  |
| regex_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>23,765</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | LOADB | <div style='text-align: right'>40</div>  |
| regex_program | Merkle | true |  | LOADB | <div style='text-align: right'>192</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>170</div>  |
| regex_program | Boundary | true |  | LOADBU | <div style='text-align: right'>400</div>  |
| regex_program | Merkle | true |  | LOADBU | <div style='text-align: right'>2,112</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>45,705,040</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>2,992</div>  |
| regex_program | Boundary | true |  | LOADW | <div style='text-align: right'>7,040</div>  |
| regex_program | Merkle | true |  | LOADW | <div style='text-align: right'>24,768</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>800,532</div>  |
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
| regex_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,159</div>  |
| regex_program | Boundary | true |  | STOREB | <div style='text-align: right'>5,080</div>  |
| regex_program | Merkle | true |  | STOREB | <div style='text-align: right'>11,968</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREH | <div style='text-align: right'>85,255</div>  |
| regex_program | Boundary | true |  | STOREH | <div style='text-align: right'>200,600</div>  |
| regex_program | Merkle | true |  | STOREH | <div style='text-align: right'>321,536</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>30,727,360</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>389,572</div>  |
| regex_program | Boundary | true |  | STOREW | <div style='text-align: right'>916,640</div>  |
| regex_program | Merkle | true |  | STOREW | <div style='text-align: right'>1,484,352</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>344,304</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: red">(+1.0 [+2.3%])</span> <div style='text-align: right'>45.0</div>  | <span style="color: red">(+27.0 [+0.3%])</span> <div style='text-align: right'>7,820.0</div>  | <span style="color: green">(-52.0 [-1.0%])</span> <div style='text-align: right'>5,275.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-2.0 [-1.0%])</span> <div style='text-align: right'>206.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>238,660,282</div>  | <div style='text-align: right'>4,181,050</div>  | <span style="color: red">(+550.0 [+1.9%])</span> <div style='text-align: right'>29,001.0</div>  |

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
| regex_program | 0 | <span style="color: red">(+79.0 [+3.2%])</span> <div style='text-align: right'>2,534.0</div>  | <span style="color: red">(+444.0 [+2.4%])</span> <div style='text-align: right'>18,647.0</div>  | <div style='text-align: right'>790,590,848</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/66b9f4a5cfcac08f014257d259393cb98aa664af/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/66b9f4a5cfcac08f014257d259393cb98aa664af

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11937095775)
