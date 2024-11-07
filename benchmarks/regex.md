| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <div style='text-align: right'>160,106,995</div>  | <div style='text-align: right'>4,191,023</div>  | <span style="color: red">(+5,514.0 [+24.9%])</span> <div style='text-align: right'>27,626.0</div>  |
| leaf_aggregation | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <span style="color: green">(-375,024,297 [-54.8%])</span> <div style='text-align: right'>309,643,883</div>  | <span style="color: green">(-9,054,390 [-55.4%])</span> <div style='text-align: right'>7,301,059</div>  | <span style="color: green">(-35,860.0 [-44.4%])</span> <div style='text-align: right'>44,992.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| regex_program | true | <span style="color: red">(+215.0 [+0.2%])</span> <div style='text-align: right'>107,804.0</div>  | <div style='text-align: right'>160,106,995</div>  | <div style='text-align: right'>4,191,023</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>1,150,485</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>38,005</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>218,625</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>282,074</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>198,078</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>106,071</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>12,767</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>130,454</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>687</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>1,961,480</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>52,087</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>39,562</div>  |
| regex_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| regex_program | KeccakVmAir | true | <div style='text-align: right'>24</div>  |
| regex_program | Memory AccessAdapter<2> | true | <div style='text-align: right'>21</div>  |
| regex_program | Memory AccessAdapter<4> | true | <div style='text-align: right'>11</div>  |
| regex_program | Memory AccessAdapter<8> | true | <div style='text-align: right'>34,637</div>  |
| regex_program | Memory Boundary | true | <div style='text-align: right'>69,274</div>  |
| regex_program | Memory Merkle | true | <div style='text-align: right'>70,524</div>  |
| regex_program | PhantomAir | true | <div style='text-align: right'>289</div>  |
| regex_program | ProgramChip | true | <div style='text-align: right'>89,554</div>  |
| regex_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| regex_program | true |  | ADD | <div style='text-align: right'>1,008,013</div>  |
| regex_program | true |  | AND | <div style='text-align: right'>66,789</div>  |
| regex_program | true |  | AUIPC | <div style='text-align: right'>39,562</div>  |
| regex_program | true |  | BEQ | <div style='text-align: right'>178,501</div>  |
| regex_program | true |  | BGE | <div style='text-align: right'>294</div>  |
| regex_program | true |  | BGEU | <div style='text-align: right'>121,597</div>  |
| regex_program | true |  | BLT | <div style='text-align: right'>5,141</div>  |
| regex_program | true |  | BLTU | <div style='text-align: right'>71,046</div>  |
| regex_program | true |  | BNE | <div style='text-align: right'>103,573</div>  |
| regex_program | true |  | DIVU | <div style='text-align: right'>114</div>  |
| regex_program | true |  | HINT_STOREW | <div style='text-align: right'>12,767</div>  |
| regex_program | true |  | JAL | <div style='text-align: right'>61,575</div>  |
| regex_program | true |  | JALR | <div style='text-align: right'>130,454</div>  |
| regex_program | true |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| regex_program | true |  | LOADB | <div style='text-align: right'>679</div>  |
| regex_program | true |  | LOADBU | <div style='text-align: right'>27,294</div>  |
| regex_program | true |  | LOADH | <div style='text-align: right'>8</div>  |
| regex_program | true |  | LOADHU | <div style='text-align: right'>95</div>  |
| regex_program | true |  | LOADW | <div style='text-align: right'>1,142,883</div>  |
| regex_program | true |  | LUI | <div style='text-align: right'>44,496</div>  |
| regex_program | true |  | MUL | <div style='text-align: right'>52,087</div>  |
| regex_program | true |  | MULHU | <div style='text-align: right'>244</div>  |
| regex_program | true |  | OR | <div style='text-align: right'>23,536</div>  |
| regex_program | true |  | PHANTOM | <div style='text-align: right'>289</div>  |
| regex_program | true |  | SLL | <div style='text-align: right'>213,542</div>  |
| regex_program | true |  | SLT | <div style='text-align: right'>5</div>  |
| regex_program | true |  | SLTU | <div style='text-align: right'>38,000</div>  |
| regex_program | true |  | SRA | <div style='text-align: right'>1</div>  |
| regex_program | true |  | SRL | <div style='text-align: right'>5,082</div>  |
| regex_program | true |  | STOREB | <div style='text-align: right'>12,721</div>  |
| regex_program | true |  | STOREH | <div style='text-align: right'>10,074</div>  |
| regex_program | true |  | STOREW | <div style='text-align: right'>768,413</div>  |
| regex_program | true |  | SUB | <div style='text-align: right'>42,583</div>  |
| regex_program | true |  | XOR | <div style='text-align: right'>9,564</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>36,288,468</div>  |
| regex_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>102</div>  |
| regex_program | Boundary | true |  | ADD | <div style='text-align: right'>240</div>  |
| regex_program | Merkle | true |  | ADD | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>2,404,404</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>830,802</div>  |
| regex_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| regex_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| regex_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>4,641,026</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>3,891,104</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>2,273,472</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>2,692,898</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | true |  | DIVU | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>108,511</div>  |
| regex_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>255,320</div>  |
| regex_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>408,640</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>1,108,350</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>3,652,712</div>  |
| regex_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>143</div>  |
| regex_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>23,765</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>187</div>  |
| regex_program | Boundary | true |  | LOADBU | <div style='text-align: right'>440</div>  |
| regex_program | Merkle | true |  | LOADBU | <div style='text-align: right'>1,920</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>45,715,320</div>  |
| regex_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>3,009</div>  |
| regex_program | Boundary | true |  | LOADW | <div style='text-align: right'>7,080</div>  |
| regex_program | Merkle | true |  | LOADW | <div style='text-align: right'>24,832</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>800,928</div>  |
| regex_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| regex_program | Merkle | true |  | LUI | <div style='text-align: right'>64</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>1,614,697</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>9,516</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>847,296</div>  |
| regex_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>1,734</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>11,317,726</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>185</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>1,406,000</div>  |
| regex_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>17</div>  |
| regex_program | Boundary | true |  | SLTU | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>53</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>269,346</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>508,840</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,159</div>  |
| regex_program | Boundary | true |  | STOREB | <div style='text-align: right'>5,080</div>  |
| regex_program | Merkle | true |  | STOREB | <div style='text-align: right'>12,288</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREH | <div style='text-align: right'>85,255</div>  |
| regex_program | Boundary | true |  | STOREH | <div style='text-align: right'>200,600</div>  |
| regex_program | Merkle | true |  | STOREH | <div style='text-align: right'>320,896</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>30,736,520</div>  |
| regex_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>389,538</div>  |
| regex_program | Boundary | true |  | STOREW | <div style='text-align: right'>916,560</div>  |
| regex_program | Merkle | true |  | STOREW | <div style='text-align: right'>1,484,480</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>344,304</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: red">(+17.0 [+58.6%])</span> <div style='text-align: right'>46.0</div>  | <span style="color: red">(+51.0 [+0.6%])</span> <div style='text-align: right'>9,089.0</div>  | <span style="color: red">(+66.0 [+1.0%])</span> <div style='text-align: right'>6,593.0</div>  | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <span style="color: red">(+79.0 [+59.8%])</span> <div style='text-align: right'>211.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>160,106,995</div>  | <div style='text-align: right'>4,191,023</div>  | <span style="color: red">(+5,514.0 [+24.9%])</span> <div style='text-align: right'>27,626.0</div>  |
| leaf_aggregation |  |  |  | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-375,024,297 [-54.8%])</span> <div style='text-align: right'>309,643,883</div>  | <span style="color: green">(-9,054,390 [-55.4%])</span> <div style='text-align: right'>7,301,059</div>  | <span style="color: green">(-35,860.0 [-44.4%])</span> <div style='text-align: right'>44,992.0</div>  |

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

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| regex_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>4,194,304</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>6,815,744</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>2,240</div>  |  |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>64</div>  |
| regex_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,184</div>  |  |  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>32</div>  |
| regex_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | PhantomAir | 0 | <div style='text-align: right'>9,216</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>512</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>27,525,120</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>113,664</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>35,584</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>256</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>20,608</div>  |  |  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  |  | <div style='text-align: right'>128</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| regex_program | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  |  |  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  |  | <div style='text-align: right'>32</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>164,364,288</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <span style="color: green">(-8 [-25.0%])</span> <div style='text-align: right'>24</div>  | <span style="color: green">(-1 [-11.1%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <span style="color: green">(-8,388,608 [-17.4%])</span> <div style='text-align: right'>39,845,888</div>  | <span style="color: green">(-1 [-5.9%])</span> <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <span style="color: green">(-16,777,216 [-22.9%])</span> <div style='text-align: right'>56,623,104</div>  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <span style="color: green">(-8,388,608 [-21.6%])</span> <div style='text-align: right'>30,408,704</div>  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <span style="color: green">(-17,170,432 [-79.9%])</span> <div style='text-align: right'>4,325,376</div>  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <span style="color: green">(-8 [-33.3%])</span> <div style='text-align: right'>16</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-393,216 [-75.0%])</span> <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | AccessAdapterAir<16> | 0 |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<32> | 0 |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<64> | 0 |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 0 | <span style="color: green">(-23,068,672 [-61.1%])</span> <div style='text-align: right'>14,680,064</div>  | <span style="color: green">(-1 [-20.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <span style="color: green">(-4 [-33.3%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-1,048,576 [-50.0%])</span> <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <span style="color: green">(-576,716,800 [-80.9%])</span> <div style='text-align: right'>136,314,880</div>  | <span style="color: green">(-5 [-13.9%])</span> <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <span style="color: green">(-20 [-45.5%])</span> <div style='text-align: right'>24</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-6,291,456 [-75.0%])</span> <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <span style="color: green">(-190,840,832 [-64.1%])</span> <div style='text-align: right'>106,954,752</div>  | <span style="color: green">(-5 [-17.9%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <span style="color: green">(-20 [-41.7%])</span> <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <span style="color: green">(-2,097,152 [-50.0%])</span> <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <span style="color: green">(-4,980,736 [-63.3%])</span> <div style='text-align: right'>2,883,584</div>  | <span style="color: green">(-2 [-25.0%])</span> <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <span style="color: green">(-8 [-40.0%])</span> <div style='text-align: right'>12</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <span style="color: green">(-343,932,928 [-62.1%])</span> <div style='text-align: right'>209,715,200</div>  | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <span style="color: green">(-16 [-44.4%])</span> <div style='text-align: right'>20</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-4,194,304 [-50.0%])</span> <div style='text-align: right'>4,194,304</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <span style="color: green">(-12,058,624 [-60.5%])</span> <div style='text-align: right'>7,864,320</div>  | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <span style="color: green">(-16 [-44.4%])</span> <div style='text-align: right'>20</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | FriMatOpeningAir | 0 | <span style="color: green">(-289,406,976 [-66.3%])</span> <div style='text-align: right'>146,800,640</div>  | <span style="color: green">(-17 [-22.4%])</span> <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <span style="color: green">(-68 [-47.2%])</span> <div style='text-align: right'>76</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-1,048,576 [-50.0%])</span> <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-43,188,224 [-52.6%])</span> <div style='text-align: right'>38,993,920</div>  | <span style="color: green">(-8 [-1.5%])</span> <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <span style="color: green">(-32 [-47.1%])</span> <div style='text-align: right'>36</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-65,536 [-50.0%])</span> <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | 0 |  |  |  |  |  |  | <span style="color: red">(+5,463.0 [+41.8%])</span> <div style='text-align: right'>18,537.0</div>  | <div style='text-align: right'>790,590,848</div>  |  |
| leaf_aggregation | 0 | <span style="color: red">(+58.0 [+47.2%])</span> <div style='text-align: right'>181.0</div>  | <span style="color: green">(-18,646.0 [-56.1%])</span> <div style='text-align: right'>14,562.0</div>  | <span style="color: green">(-16,933.0 [-56.8%])</span> <div style='text-align: right'>12,875.0</div>  | <span style="color: red">(+1 [+100.0%])</span> <div style='text-align: right'>2</div>  | <span style="color: red">(+4.0 [+7.3%])</span> <div style='text-align: right'>59.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-17,214.0 [-36.1%])</span> <div style='text-align: right'>30,430.0</div>  | <span style="color: green">(-1,534,918,664 [-65.6%])</span> <div style='text-align: right'>806,027,288</div>  | <span style="color: red">(+108.0 [+0.2%])</span> <div style='text-align: right'>48,179.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <span style="color: green">(-145,811.0 [-55.3%])</span> <div style='text-align: right'>117,766.0</div>  | <span style="color: green">(-375,024,297 [-54.8%])</span> <div style='text-align: right'>309,643,883</div>  | <span style="color: green">(-9,054,390 [-55.4%])</span> <div style='text-align: right'>7,301,059</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <span style="color: green">(-1,875,184 [-56.2%])</span> <div style='text-align: right'>1,463,812</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <span style="color: green">(-121,076 [-55.8%])</span> <div style='text-align: right'>96,064</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <span style="color: green">(-3,488,753 [-55.0%])</span> <div style='text-align: right'>2,857,293</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <span style="color: green">(-2,562,052 [-55.4%])</span> <div style='text-align: right'>2,062,184</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <span style="color: green">(-31,816 [-22.4%])</span> <div style='text-align: right'>110,473</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 0 | <span style="color: green">(-788,452 [-58.0%])</span> <div style='text-align: right'>570,948</div>  |
| leaf_aggregation | Memory AccessAdapter<2> | true | 0 | <span style="color: green">(-637,505 [-42.9%])</span> <div style='text-align: right'>849,498</div>  |
| leaf_aggregation | Memory AccessAdapter<4> | true | 0 | <span style="color: green">(-319,043 [-42.9%])</span> <div style='text-align: right'>424,959</div>  |
| leaf_aggregation | Memory AccessAdapter<8> | true | 0 | <span style="color: green">(-126,222 [-57.4%])</span> <div style='text-align: right'>93,597</div>  |
| leaf_aggregation | Memory Boundary | true | 0 | <span style="color: green">(-809,738 [-43.4%])</span> <div style='text-align: right'>1,057,556</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <span style="color: green">(-891,170 [-57.9%])</span> <div style='text-align: right'>648,013</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <span style="color: green">(-74,537 [-57.0%])</span> <div style='text-align: right'>56,122</div>  |
| leaf_aggregation | ProgramChip | true | 0 | <span style="color: green">(-17,806 [-5.5%])</span> <div style='text-align: right'>303,075</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | JAL | 0 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 0 | <span style="color: green">(-12,238 [-20.5%])</span> <div style='text-align: right'>47,319</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 0 | <span style="color: green">(-58 [-23.0%])</span> <div style='text-align: right'>194</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 0 | <span style="color: green">(-174 [-23.0%])</span> <div style='text-align: right'>582</div>  |
| leaf_aggregation | true | AddEFI | ADD | 0 | <div style='text-align: right'>556</div>  |
| leaf_aggregation | true | AddEI | ADD | 0 | <span style="color: green">(-14,912 [-13.3%])</span> <div style='text-align: right'>96,940</div>  |
| leaf_aggregation | true | AddFI | ADD | 0 | <span style="color: green">(-98,655 [-55.3%])</span> <div style='text-align: right'>79,836</div>  |
| leaf_aggregation | true | AddV | ADD | 0 | <span style="color: green">(-20,578 [-54.2%])</span> <div style='text-align: right'>17,371</div>  |
| leaf_aggregation | true | AddVI | ADD | 0 | <span style="color: green">(-1,112,012 [-57.3%])</span> <div style='text-align: right'>828,630</div>  |
| leaf_aggregation | true | Alloc | ADD | 0 | <span style="color: green">(-79,866 [-55.7%])</span> <div style='text-align: right'>63,623</div>  |
| leaf_aggregation | true | Alloc | LOADW | 0 | <span style="color: green">(-79,866 [-55.7%])</span> <div style='text-align: right'>63,623</div>  |
| leaf_aggregation | true | Alloc | MUL | 0 | <span style="color: green">(-48,430 [-55.9%])</span> <div style='text-align: right'>38,191</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 0 | <span style="color: green">(-232 [-45.7%])</span> <div style='text-align: right'>276</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 0 | <span style="color: green">(-13,514 [-58.0%])</span> <div style='text-align: right'>9,787</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 0 | <span style="color: green">(-3,016 [-54.4%])</span> <div style='text-align: right'>2,525</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>314</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 0 | <span style="color: green">(-928 [-58.0%])</span> <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <span style="color: green">(-12,760 [-58.0%])</span> <div style='text-align: right'>9,240</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 0 | <span style="color: green">(-5,104 [-58.0%])</span> <div style='text-align: right'>3,696</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <span style="color: green">(-2,436 [-58.0%])</span> <div style='text-align: right'>1,764</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <span style="color: green">(-800,284 [-58.0%])</span> <div style='text-align: right'>579,516</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <span style="color: green">(-19,604 [-58.0%])</span> <div style='text-align: right'>14,196</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 0 | <span style="color: green">(-928 [-58.0%])</span> <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 0 | <span style="color: green">(-2,436 [-58.0%])</span> <div style='text-align: right'>1,764</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <span style="color: green">(-7,540 [-58.0%])</span> <div style='text-align: right'>5,460</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <span style="color: green">(-7,540 [-58.0%])</span> <div style='text-align: right'>5,460</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 0 | <span style="color: green">(-116 [-58.0%])</span> <div style='text-align: right'>84</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 0 | <span style="color: green">(-11,020 [-57.8%])</span> <div style='text-align: right'>8,034</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>75</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 0 | <div style='text-align: right'>300</div>  |
| leaf_aggregation | true | DivFIN | DIV | 0 | <div style='text-align: right'>177</div>  |
| leaf_aggregation | true | For | ADD | 0 | <span style="color: green">(-1,196,374 [-55.6%])</span> <div style='text-align: right'>955,222</div>  |
| leaf_aggregation | true | For | BNE | 0 | <span style="color: green">(-1,262,030 [-55.6%])</span> <div style='text-align: right'>1,005,913</div>  |
| leaf_aggregation | true | For | JAL | 0 | <span style="color: green">(-65,656 [-56.4%])</span> <div style='text-align: right'>50,691</div>  |
| leaf_aggregation | true | For | LOADW | 0 | <span style="color: green">(-3,828 [-58.0%])</span> <div style='text-align: right'>2,772</div>  |
| leaf_aggregation | true | For | STOREW | 0 | <span style="color: green">(-61,828 [-56.3%])</span> <div style='text-align: right'>47,919</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 0 | <span style="color: green">(-9,802 [-58.0%])</span> <div style='text-align: right'>7,098</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 0 | <span style="color: green">(-58 [-57.4%])</span> <div style='text-align: right'>43</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 0 | <span style="color: green">(-31,436 [-55.3%])</span> <div style='text-align: right'>25,432</div>  |
| leaf_aggregation | true | IfEq | BNE | 0 | <span style="color: green">(-39,498 [-57.9%])</span> <div style='text-align: right'>28,704</div>  |
| leaf_aggregation | true | IfEqI | BNE | 0 | <span style="color: green">(-532,708 [-57.3%])</span> <div style='text-align: right'>396,434</div>  |
| leaf_aggregation | true | IfEqI | JAL | 0 | <span style="color: green">(-55,420 [-55.0%])</span> <div style='text-align: right'>45,369</div>  |
| leaf_aggregation | true | IfNe | BEQ | 0 | <span style="color: green">(-20,416 [-54.6%])</span> <div style='text-align: right'>16,989</div>  |
| leaf_aggregation | true | IfNe | JAL | 0 | <div style='text-align: right'>3</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 0 | <span style="color: green">(-3,770 [-56.8%])</span> <div style='text-align: right'>2,866</div>  |
| leaf_aggregation | true | ImmE | STOREW | 0 | <span style="color: green">(-1,160 [-6.6%])</span> <div style='text-align: right'>16,420</div>  |
| leaf_aggregation | true | ImmF | STOREW | 0 | <span style="color: green">(-63,394 [-57.9%])</span> <div style='text-align: right'>46,094</div>  |
| leaf_aggregation | true | ImmV | STOREW | 0 | <span style="color: green">(-81,372 [-57.5%])</span> <div style='text-align: right'>60,185</div>  |
| leaf_aggregation | true | LoadE | LOADW | 0 | <span style="color: green">(-14,616 [-18.3%])</span> <div style='text-align: right'>65,188</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 0 | <span style="color: green">(-114,608 [-58.0%])</span> <div style='text-align: right'>83,100</div>  |
| leaf_aggregation | true | LoadF | LOADW | 0 | <span style="color: green">(-37,874 [-57.8%])</span> <div style='text-align: right'>27,679</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 0 | <span style="color: green">(-433,086 [-57.9%])</span> <div style='text-align: right'>314,285</div>  |
| leaf_aggregation | true | LoadV | LOADW | 0 | <span style="color: green">(-36,888 [-55.1%])</span> <div style='text-align: right'>30,036</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 0 | <span style="color: green">(-346,934 [-56.3%])</span> <div style='text-align: right'>268,866</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 0 | <span style="color: green">(-4,904 [-12.9%])</span> <div style='text-align: right'>32,969</div>  |
| leaf_aggregation | true | MulEF | MUL | 0 | <span style="color: green">(-4,872 [-54.1%])</span> <div style='text-align: right'>4,128</div>  |
| leaf_aggregation | true | MulEFI | MUL | 0 | <div style='text-align: right'>2,136</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>5,119</div>  |
| leaf_aggregation | true | MulEI | STOREW | 0 | <div style='text-align: right'>20,476</div>  |
| leaf_aggregation | true | MulF | MUL | 0 | <span style="color: green">(-189,120 [-55.3%])</span> <div style='text-align: right'>153,132</div>  |
| leaf_aggregation | true | MulFI | MUL | 0 | <div style='text-align: right'>27</div>  |
| leaf_aggregation | true | MulV | MUL | 0 | <span style="color: green">(-1,798 [-57.4%])</span> <div style='text-align: right'>1,333</div>  |
| leaf_aggregation | true | MulVI | MUL | 0 | <span style="color: green">(-27,956 [-55.4%])</span> <div style='text-align: right'>22,541</div>  |
| leaf_aggregation | true | NegE | MUL | 0 | <div style='text-align: right'>460</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <span style="color: green">(-22,446 [-55.0%])</span> <div style='text-align: right'>18,354</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <span style="color: green">(-52,091 [-58.0%])</span> <div style='text-align: right'>37,768</div>  |
| leaf_aggregation | true | StoreE | STOREW | 0 | <span style="color: green">(-34,336 [-57.7%])</span> <div style='text-align: right'>25,204</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 0 | <span style="color: green">(-60,784 [-58.0%])</span> <div style='text-align: right'>44,100</div>  |
| leaf_aggregation | true | StoreF | STOREW | 0 | <span style="color: green">(-50,576 [-57.9%])</span> <div style='text-align: right'>36,750</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 0 | <span style="color: green">(-409,886 [-57.9%])</span> <div style='text-align: right'>297,615</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 0 | <span style="color: green">(-571,358 [-54.1%])</span> <div style='text-align: right'>485,595</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 0 | <span style="color: green">(-604,592 [-54.1%])</span> <div style='text-align: right'>512,360</div>  |
| leaf_aggregation | true | StoreV | STOREW | 0 | <span style="color: green">(-3,770 [-53.7%])</span> <div style='text-align: right'>3,249</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 0 | <span style="color: green">(-93,016 [-55.8%])</span> <div style='text-align: right'>73,669</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 0 | <span style="color: green">(-3,654 [-17.7%])</span> <div style='text-align: right'>16,957</div>  |
| leaf_aggregation | true | SubEF | LOADW | 0 | <span style="color: green">(-29,406 [-57.7%])</span> <div style='text-align: right'>21,516</div>  |
| leaf_aggregation | true | SubEF | SUB | 0 | <span style="color: green">(-9,802 [-57.7%])</span> <div style='text-align: right'>7,172</div>  |
| leaf_aggregation | true | SubEFI | ADD | 0 | <div style='text-align: right'>9,556</div>  |
| leaf_aggregation | true | SubEI | ADD | 0 | <div style='text-align: right'>600</div>  |
| leaf_aggregation | true | SubV | SUB | 0 | <span style="color: green">(-108,728 [-55.6%])</span> <div style='text-align: right'>86,799</div>  |
| leaf_aggregation | true | SubVI | SUB | 0 | <span style="color: green">(-3,074 [-56.3%])</span> <div style='text-align: right'>2,386</div>  |
| leaf_aggregation | true | SubVIN | SUB | 0 | <span style="color: green">(-1,218 [-58.0%])</span> <div style='text-align: right'>882</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 0 | <span style="color: green">(-489,520 [-20.5%])</span> <div style='text-align: right'>1,892,760</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 0 | <span style="color: green">(-195,228 [-12.6%])</span> <div style='text-align: right'>1,348,908</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 0 | <span style="color: green">(-115,362 [-12.6%])</span> <div style='text-align: right'>797,082</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 0 | <div style='text-align: right'>1,365,496</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 0 | <span style="color: green">(-2,378 [-23.0%])</span> <div style='text-align: right'>7,954</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,298</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,534</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 0 | <span style="color: green">(-7,134 [-23.0%])</span> <div style='text-align: right'>23,862</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>1,298</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 0 | <div style='text-align: right'>16,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 0 | <div style='text-align: right'>2,618</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,547</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 0 | <div style='text-align: right'>2,112</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 0 | <span style="color: green">(-447,360 [-13.3%])</span> <div style='text-align: right'>2,908,200</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 0 | <span style="color: green">(-66,506 [-9.8%])</span> <div style='text-align: right'>610,588</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 0 | <span style="color: green">(-39,299 [-9.8%])</span> <div style='text-align: right'>360,802</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 0 | <div style='text-align: right'>708,752</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 0 | <span style="color: green">(-2,959,650 [-55.3%])</span> <div style='text-align: right'>2,395,080</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 0 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 0 | <span style="color: green">(-617,340 [-54.2%])</span> <div style='text-align: right'>521,130</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 0 | <span style="color: green">(-33,360,360 [-57.3%])</span> <div style='text-align: right'>24,858,900</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 0 | <span style="color: green">(-19,778 [-55.4%])</span> <div style='text-align: right'>15,950</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 0 | <span style="color: green">(-2,395,980 [-55.7%])</span> <div style='text-align: right'>1,908,690</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 0 | <span style="color: green">(-3,274,506 [-55.7%])</span> <div style='text-align: right'>2,608,543</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 0 | <span style="color: green">(-638 [-36.2%])</span> <div style='text-align: right'>1,122</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 0 | <span style="color: green">(-1,452,900 [-55.9%])</span> <div style='text-align: right'>1,145,730</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 0 | <div style='text-align: right'>39</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 0 | <span style="color: green">(-5,336 [-45.7%])</span> <div style='text-align: right'>6,348</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 0 | <span style="color: green">(-1,276 [-45.7%])</span> <div style='text-align: right'>1,518</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 0 | <span style="color: green">(-754 [-45.7%])</span> <div style='text-align: right'>897</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 0 | <span style="color: green">(-310,822 [-58.0%])</span> <div style='text-align: right'>225,101</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 0 | <span style="color: green">(-69,368 [-54.4%])</span> <div style='text-align: right'>58,075</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>7,222</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 0 | <span style="color: green">(-5,568 [-58.0%])</span> <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <span style="color: green">(-76,560 [-58.0%])</span> <div style='text-align: right'>55,440</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 0 | <span style="color: green">(-30,624 [-58.0%])</span> <div style='text-align: right'>22,176</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <span style="color: green">(-14,616 [-58.0%])</span> <div style='text-align: right'>10,584</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <span style="color: green">(-4,801,704 [-58.0%])</span> <div style='text-align: right'>3,477,096</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <span style="color: green">(-117,624 [-58.0%])</span> <div style='text-align: right'>85,176</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 0 | <span style="color: green">(-5,568 [-58.0%])</span> <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 0 | <span style="color: green">(-14,616 [-58.0%])</span> <div style='text-align: right'>10,584</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <span style="color: green">(-45,240 [-58.0%])</span> <div style='text-align: right'>32,760</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <span style="color: green">(-45,240 [-58.0%])</span> <div style='text-align: right'>32,760</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 0 | <span style="color: green">(-696 [-58.0%])</span> <div style='text-align: right'>504</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 0 | <span style="color: green">(-440,800 [-57.8%])</span> <div style='text-align: right'>321,360</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 0 | <span style="color: green">(-215,644 [-57.7%])</span> <div style='text-align: right'>157,960</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 0 | <span style="color: green">(-127,426 [-57.7%])</span> <div style='text-align: right'>93,340</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>3,000</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>3,168</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1,872</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>12,300</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>1,089</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>312</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 0 | <div style='text-align: right'>5,310</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 0 | <span style="color: green">(-35,891,220 [-55.6%])</span> <div style='text-align: right'>28,656,660</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 0 | <span style="color: green">(-29,026,690 [-55.6%])</span> <div style='text-align: right'>23,135,999</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 0 | <span style="color: green">(-656,560 [-56.4%])</span> <div style='text-align: right'>506,910</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 0 | <div style='text-align: right'>561</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 0 | <div style='text-align: right'>663</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 0 | <span style="color: green">(-156,948 [-58.0%])</span> <div style='text-align: right'>113,652</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 0 | <span style="color: green">(-638 [-58.0%])</span> <div style='text-align: right'>462</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 0 | <span style="color: green">(-2,534,948 [-56.3%])</span> <div style='text-align: right'>1,964,679</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 0 | <div style='text-align: right'>847</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 0 | <span style="color: green">(-140,360 [-25.9%])</span> <div style='text-align: right'>400,708</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 0 | <span style="color: green">(-82,940 [-25.9%])</span> <div style='text-align: right'>236,782</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 0 | <span style="color: green">(-50,460,928 [-58.0%])</span> <div style='text-align: right'>36,540,672</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 0 | <span style="color: green">(-348 [-57.4%])</span> <div style='text-align: right'>258</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 0 | <span style="color: green">(-188,616 [-55.3%])</span> <div style='text-align: right'>152,592</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 0 | <span style="color: green">(-908,454 [-57.9%])</span> <div style='text-align: right'>660,192</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 0 | <span style="color: green">(-12,252,284 [-57.3%])</span> <div style='text-align: right'>9,117,982</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 0 | <span style="color: green">(-554,200 [-55.0%])</span> <div style='text-align: right'>453,690</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 0 | <span style="color: green">(-469,568 [-54.6%])</span> <div style='text-align: right'>390,747</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 0 | <div style='text-align: right'>30</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 0 | <span style="color: green">(-86,710 [-56.8%])</span> <div style='text-align: right'>65,918</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 0 | <span style="color: green">(-47,560 [-6.6%])</span> <div style='text-align: right'>673,220</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 0 | <span style="color: green">(-1,276 [-7.4%])</span> <div style='text-align: right'>15,862</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 0 | <span style="color: green">(-754 [-7.4%])</span> <div style='text-align: right'>9,373</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 0 | <div style='text-align: right'>133,584</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 0 | <span style="color: green">(-2,599,154 [-57.9%])</span> <div style='text-align: right'>1,889,854</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 0 | <span style="color: green">(-638 [-28.9%])</span> <div style='text-align: right'>1,573</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 0 | <span style="color: green">(-3,336,252 [-57.5%])</span> <div style='text-align: right'>2,467,585</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 0 | <span style="color: green">(-21,054 [-56.3%])</span> <div style='text-align: right'>16,324</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 0 | <span style="color: green">(-599,256 [-18.3%])</span> <div style='text-align: right'>2,672,708</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 0 | <span style="color: green">(-53,592 [-10.6%])</span> <div style='text-align: right'>450,252</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 0 | <span style="color: green">(-31,668 [-10.6%])</span> <div style='text-align: right'>266,058</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 0 | <div style='text-align: right'>305,052</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 0 | <span style="color: green">(-4,698,928 [-58.0%])</span> <div style='text-align: right'>3,407,100</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 0 | <span style="color: green">(-81,664 [-58.0%])</span> <div style='text-align: right'>59,180</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 0 | <span style="color: green">(-48,256 [-58.0%])</span> <div style='text-align: right'>34,970</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 0 | <span style="color: green">(-1,552,834 [-57.8%])</span> <div style='text-align: right'>1,134,839</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 0 | <span style="color: green">(-74,008 [-58.0%])</span> <div style='text-align: right'>53,592</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 0 | <span style="color: green">(-43,732 [-58.0%])</span> <div style='text-align: right'>31,668</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 0 | <span style="color: green">(-28,594 [-58.0%])</span> <div style='text-align: right'>20,706</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 0 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 0 | <span style="color: green">(-17,756,526 [-57.9%])</span> <div style='text-align: right'>12,885,685</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 0 | <span style="color: green">(-319 [-28.2%])</span> <div style='text-align: right'>814</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 0 | <span style="color: green">(-195 [-28.8%])</span> <div style='text-align: right'>481</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 0 | <span style="color: green">(-119 [-18.9%])</span> <div style='text-align: right'>510</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 0 | <span style="color: green">(-638 [-54.2%])</span> <div style='text-align: right'>539</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 0 | <span style="color: green">(-1,512,408 [-55.1%])</span> <div style='text-align: right'>1,231,476</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 0 | <span style="color: green">(-19,778 [-56.7%])</span> <div style='text-align: right'>15,114</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 0 | <span style="color: green">(-14,224,294 [-56.3%])</span> <div style='text-align: right'>11,023,506</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>968</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 0 | <span style="color: green">(-196,160 [-12.9%])</span> <div style='text-align: right'>1,318,760</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 0 | <span style="color: green">(-69,058 [-7.6%])</span> <div style='text-align: right'>834,570</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 0 | <span style="color: green">(-40,807 [-7.6%])</span> <div style='text-align: right'>493,155</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>935,792</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 0 | <span style="color: green">(-146,160 [-54.1%])</span> <div style='text-align: right'>123,840</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 0 | <span style="color: green">(-26,796 [-56.5%])</span> <div style='text-align: right'>20,636</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 0 | <span style="color: green">(-15,834 [-56.5%])</span> <div style='text-align: right'>12,194</div>  |
| leaf_aggregation | Boundary | true | MulEF | MUL | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEFI | MUL | 0 | <div style='text-align: right'>64,080</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEFI | MUL | 0 | <div style='text-align: right'>5,126</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEFI | MUL | 0 | <div style='text-align: right'>3,029</div>  |
| leaf_aggregation | Boundary | true | MulEFI | MUL | 0 | <div style='text-align: right'>16,104</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>204,760</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>286,660</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>169,390</div>  |
| leaf_aggregation | Boundary | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>117,656</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | 0 | <div style='text-align: right'>839,516</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 0 | <div style='text-align: right'>112,563</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 0 | <div style='text-align: right'>66,495</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 0 | <span style="color: green">(-5,673,600 [-55.3%])</span> <div style='text-align: right'>4,593,960</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 0 | <div style='text-align: right'>810</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 0 | <span style="color: green">(-53,940 [-57.4%])</span> <div style='text-align: right'>39,990</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 0 | <span style="color: green">(-19,778 [-57.5%])</span> <div style='text-align: right'>14,641</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 0 | <span style="color: green">(-838,680 [-55.4%])</span> <div style='text-align: right'>676,230</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 0 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 0 | <div style='text-align: right'>13,800</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 0 | <div style='text-align: right'>3,388</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 0 | <div style='text-align: right'>2,002</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 0 | <div style='text-align: right'>2,420</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <span style="color: green">(-895,752 [-54.7%])</span> <div style='text-align: right'>741,048</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <span style="color: green">(-529,308 [-54.7%])</span> <div style='text-align: right'>437,892</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <span style="color: green">(-346,086 [-54.7%])</span> <div style='text-align: right'>286,314</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <span style="color: green">(-12,547,314 [-55.0%])</span> <div style='text-align: right'>10,259,886</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <span style="color: green">(-2,433,002 [-58.0%])</span> <div style='text-align: right'>1,763,894</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <span style="color: green">(-1,439,568 [-58.0%])</span> <div style='text-align: right'>1,043,666</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <span style="color: green">(-949,637 [-58.0%])</span> <div style='text-align: right'>688,483</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <span style="color: green">(-29,118,869 [-58.0%])</span> <div style='text-align: right'>21,112,312</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 0 | <span style="color: green">(-1,407,776 [-57.7%])</span> <div style='text-align: right'>1,033,364</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 0 | <span style="color: green">(-26,796 [-57.9%])</span> <div style='text-align: right'>19,448</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 0 | <span style="color: green">(-15,834 [-57.9%])</span> <div style='text-align: right'>11,492</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 0 | <span style="color: green">(-377,696 [-57.7%])</span> <div style='text-align: right'>277,244</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 0 | <span style="color: green">(-2,492,144 [-58.0%])</span> <div style='text-align: right'>1,808,100</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 0 | <span style="color: green">(-280,720 [-58.0%])</span> <div style='text-align: right'>203,280</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 0 | <span style="color: green">(-165,880 [-58.0%])</span> <div style='text-align: right'>120,120</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 0 | <span style="color: green">(-53,592 [-57.4%])</span> <div style='text-align: right'>39,732</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 0 | <span style="color: green">(-2,073,616 [-57.9%])</span> <div style='text-align: right'>1,506,750</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 0 | <span style="color: green">(-556,336 [-57.9%])</span> <div style='text-align: right'>404,250</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 0 | <span style="color: green">(-16,805,326 [-57.9%])</span> <div style='text-align: right'>12,202,215</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 0 | <span style="color: green">(-2,100,934 [-58.0%])</span> <div style='text-align: right'>1,522,774</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 0 | <span style="color: green">(-1,243,346 [-58.0%])</span> <div style='text-align: right'>901,186</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 0 | <span style="color: green">(-821,338 [-58.0%])</span> <div style='text-align: right'>595,136</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 0 | <span style="color: green">(-107,184 [-56.5%])</span> <div style='text-align: right'>82,412</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 0 | <span style="color: green">(-17,140,740 [-54.1%])</span> <div style='text-align: right'>14,567,850</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 0 | <span style="color: green">(-24,788,272 [-54.1%])</span> <div style='text-align: right'>21,006,760</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 0 | <span style="color: green">(-6,650,512 [-54.1%])</span> <div style='text-align: right'>5,635,960</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 0 | <span style="color: green">(-154,570 [-53.7%])</span> <div style='text-align: right'>133,209</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 0 | <span style="color: green">(-41,470 [-53.7%])</span> <div style='text-align: right'>35,739</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 0 | <span style="color: green">(-3,813,656 [-55.8%])</span> <div style='text-align: right'>3,020,429</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 0 | <span style="color: green">(-1,016,972 [-55.8%])</span> <div style='text-align: right'>806,498</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 0 | <span style="color: green">(-146,160 [-17.7%])</span> <div style='text-align: right'>678,280</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 0 | <span style="color: green">(-133,980 [-19.6%])</span> <div style='text-align: right'>550,704</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 0 | <span style="color: green">(-79,170 [-19.6%])</span> <div style='text-align: right'>325,416</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 0 | <div style='text-align: right'>577,324</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 0 | <span style="color: green">(-1,205,646 [-57.7%])</span> <div style='text-align: right'>882,156</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 0 | <span style="color: green">(-107,822 [-57.8%])</span> <div style='text-align: right'>78,749</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 0 | <span style="color: green">(-294,060 [-57.7%])</span> <div style='text-align: right'>215,160</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 0 | <span style="color: green">(-107,822 [-57.8%])</span> <div style='text-align: right'>78,749</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 0 | <span style="color: green">(-127,426 [-57.8%])</span> <div style='text-align: right'>93,067</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 0 | <div style='text-align: right'>286,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 0 | <div style='text-align: right'>8,228</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 0 | <div style='text-align: right'>4,862</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 0 | <div style='text-align: right'>99,616</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 0 | <div style='text-align: right'>18,000</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 0 | <div style='text-align: right'>5,192</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 0 | <div style='text-align: right'>3,068</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 0 | <span style="color: green">(-3,261,840 [-55.6%])</span> <div style='text-align: right'>2,603,970</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 0 | <span style="color: green">(-92,220 [-56.3%])</span> <div style='text-align: right'>71,580</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 0 | <span style="color: green">(-20,416 [-57.4%])</span> <div style='text-align: right'>15,147</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 0 | <span style="color: green">(-36,540 [-58.0%])</span> <div style='text-align: right'>26,460</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-leaf_aggregation.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/2f1818cbda5e2b77846508bcafb51062f63b4413/regex-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/2f1818cbda5e2b77846508bcafb51062f63b4413
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11730338268)
