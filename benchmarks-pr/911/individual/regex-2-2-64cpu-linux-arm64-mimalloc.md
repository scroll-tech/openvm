| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>238,817,559</div>  | <div style='text-align: right'>4,181,198</div>  | <span style="color: red">(+345.0 [+1.3%])</span> <div style='text-align: right'>27,349.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <span style="color: green">(-37,280 [-0.0%])</span> <div style='text-align: right'>314,789,754</div>  | <span style="color: green">(-3,581 [-0.0%])</span> <div style='text-align: right'>7,306,378</div>  | <span style="color: green">(-7,565.0 [-21.0%])</span> <div style='text-align: right'>28,541.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: green">(-11.0 [-21.6%])</span> <div style='text-align: right'>40.0</div>  | <span style="color: red">(+76.0 [+1.0%])</span> <div style='text-align: right'>7,614.0</div>  | <span style="color: green">(-6.0 [-0.1%])</span> <div style='text-align: right'>4,838.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+22.0 [+10.1%])</span> <div style='text-align: right'>239.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>238,817,559</div>  | <div style='text-align: right'>4,181,198</div>  | <span style="color: red">(+345.0 [+1.3%])</span> <div style='text-align: right'>27,349.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-37,280 [-0.0%])</span> <div style='text-align: right'>314,789,754</div>  | <span style="color: green">(-3,581 [-0.0%])</span> <div style='text-align: right'>7,306,378</div>  | <span style="color: green">(-7,565.0 [-21.0%])</span> <div style='text-align: right'>28,541.0</div>  |

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

| group | chip_name | rows_used |
| --- | --- | --- |
| regex_program | ProgramChip | <div style='text-align: right'>85,436</div>  |
| regex_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| regex_program | Boundary | <div style='text-align: right'>69,298</div>  |
| regex_program | Merkle | <div style='text-align: right'>70,562</div>  |
| regex_program | AccessAdapter<2> | <div style='text-align: right'>42</div>  |
| regex_program | AccessAdapter<4> | <div style='text-align: right'>22</div>  |
| regex_program | AccessAdapter<8> | <div style='text-align: right'>69,298</div>  |
| regex_program | KeccakVmAir | <div style='text-align: right'>24</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>12,767</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <div style='text-align: right'>52,087</div>  |
| regex_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>39,542</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>130,415</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>96,897</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>198,079</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>282,064</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <div style='text-align: right'>687</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>1,961,056</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>218,632</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>38,005</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>1,150,319</div>  |
| regex_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| regex_program | PhantomAir | <div style='text-align: right'>289</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>139,860</div>  |
| regex_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| regex_program |  | ADD | <div style='text-align: right'>1,007,847</div>  |
| regex_program |  | AND | <div style='text-align: right'>66,789</div>  |
| regex_program |  | AUIPC | <div style='text-align: right'>39,542</div>  |
| regex_program |  | BEQ | <div style='text-align: right'>159,878</div>  |
| regex_program |  | BGE | <div style='text-align: right'>294</div>  |
| regex_program |  | BGEU | <div style='text-align: right'>121,615</div>  |
| regex_program |  | BLT | <div style='text-align: right'>5,141</div>  |
| regex_program |  | BLTU | <div style='text-align: right'>71,029</div>  |
| regex_program |  | BNE | <div style='text-align: right'>122,186</div>  |
| regex_program |  | DIVU | <div style='text-align: right'>114</div>  |
| regex_program |  | HINT_STOREW | <div style='text-align: right'>12,767</div>  |
| regex_program |  | JAL | <div style='text-align: right'>52,424</div>  |
| regex_program |  | JALR | <div style='text-align: right'>130,415</div>  |
| regex_program |  | KECCAK256 | <div style='text-align: right'>1</div>  |
| regex_program |  | LOADB | <div style='text-align: right'>679</div>  |
| regex_program |  | LOADBU | <div style='text-align: right'>27,294</div>  |
| regex_program |  | LOADH | <div style='text-align: right'>8</div>  |
| regex_program |  | LOADHU | <div style='text-align: right'>95</div>  |
| regex_program |  | LOADW | <div style='text-align: right'>1,142,653</div>  |
| regex_program |  | LUI | <div style='text-align: right'>44,473</div>  |
| regex_program |  | MUL | <div style='text-align: right'>52,087</div>  |
| regex_program |  | MULHU | <div style='text-align: right'>244</div>  |
| regex_program |  | OR | <div style='text-align: right'>23,536</div>  |
| regex_program |  | PHANTOM | <div style='text-align: right'>289</div>  |
| regex_program |  | SLL | <div style='text-align: right'>213,549</div>  |
| regex_program |  | SLT | <div style='text-align: right'>5</div>  |
| regex_program |  | SLTU | <div style='text-align: right'>38,000</div>  |
| regex_program |  | SRA | <div style='text-align: right'>1</div>  |
| regex_program |  | SRL | <div style='text-align: right'>5,082</div>  |
| regex_program |  | STOREB | <div style='text-align: right'>12,721</div>  |
| regex_program |  | STOREH | <div style='text-align: right'>10,074</div>  |
| regex_program |  | STOREW | <div style='text-align: right'>768,219</div>  |
| regex_program |  | SUB | <div style='text-align: right'>42,583</div>  |
| regex_program |  | XOR | <div style='text-align: right'>9,564</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>36,282,492</div>  |
| regex_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>102</div>  |
| regex_program | Boundary |  | ADD | <div style='text-align: right'>240</div>  |
| regex_program | Merkle |  | ADD | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>2,404,404</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>830,382</div>  |
| regex_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| regex_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| regex_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>4,156,828</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>3,891,680</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>2,272,928</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>3,176,836</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> |  | DIVU | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>108,511</div>  |
| regex_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>255,320</div>  |
| regex_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>408,576</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>943,632</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>3,651,620</div>  |
| regex_program | AccessAdapter<2> |  | KECCAK256 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> |  | KECCAK256 | <div style='text-align: right'>143</div>  |
| regex_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>75,936</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <div style='text-align: right'>23,765</div>  |
| regex_program | AccessAdapter<8> |  | LOADB | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | LOADB | <div style='text-align: right'>40</div>  |
| regex_program | Merkle |  | LOADB | <div style='text-align: right'>64</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> |  | LOADBU | <div style='text-align: right'>170</div>  |
| regex_program | Boundary |  | LOADBU | <div style='text-align: right'>400</div>  |
| regex_program | Merkle |  | LOADBU | <div style='text-align: right'>2,240</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADH | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>45,706,120</div>  |
| regex_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>2,992</div>  |
| regex_program | Boundary |  | LOADW | <div style='text-align: right'>7,040</div>  |
| regex_program | Merkle |  | LOADW | <div style='text-align: right'>25,344</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>800,514</div>  |
| regex_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| regex_program | Merkle |  | LUI | <div style='text-align: right'>64</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <div style='text-align: right'>1,614,697</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | <div style='text-align: right'>9,516</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>847,296</div>  |
| regex_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,734</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>11,318,097</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | <div style='text-align: right'>185</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>1,406,000</div>  |
| regex_program | AccessAdapter<8> |  | SLTU | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | SLTU | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | <div style='text-align: right'>53</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>269,346</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>508,840</div>  |
| regex_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>2,159</div>  |
| regex_program | Boundary |  | STOREB | <div style='text-align: right'>5,080</div>  |
| regex_program | Merkle |  | STOREB | <div style='text-align: right'>11,584</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> |  | STOREH | <div style='text-align: right'>85,289</div>  |
| regex_program | Boundary |  | STOREH | <div style='text-align: right'>200,680</div>  |
| regex_program | Merkle |  | STOREH | <div style='text-align: right'>321,408</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>30,728,760</div>  |
| regex_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>389,725</div>  |
| regex_program | Boundary |  | STOREW | <div style='text-align: right'>917,000</div>  |
| regex_program | Merkle |  | STOREW | <div style='text-align: right'>1,485,056</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>344,304</div>  |

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

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | 0 |  | <span style="color: red">(+43.0 [+1.6%])</span> <div style='text-align: right'>2,728.0</div>  |  |  |  |  | <span style="color: red">(+226.0 [+1.3%])</span> <div style='text-align: right'>17,007.0</div>  | <div style='text-align: right'>791,770,496</div>  |  |  |  |
| leaf_aggregation | 0 | <span style="color: red">(+72.0 [+63.7%])</span> <div style='text-align: right'>185.0</div>  | <span style="color: green">(-8,213.0 [-82.5%])</span> <div style='text-align: right'>1,737.0</div>  | <span style="color: green">(-109.0 [-1.3%])</span> <div style='text-align: right'>8,118.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+5.0 [+6.8%])</span> <div style='text-align: right'>78.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+648.0 [+2.5%])</span> <div style='text-align: right'>26,804.0</div>  | <div style='text-align: right'>807,206,936</div>  | <div style='text-align: right'>314,789,754</div>  | <div style='text-align: right'>7,306,378</div>  | <span style="color: green">(-797.0 [-1.1%])</span> <div style='text-align: right'>73,757.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | 0 | <div style='text-align: right'>305,094</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | 0 | <div style='text-align: right'>1,055,116</div>  |
| leaf_aggregation | AccessAdapter<2> | 0 | <div style='text-align: right'>1,105,576</div>  |
| leaf_aggregation | AccessAdapter<4> | 0 | <div style='text-align: right'>552,998</div>  |
| leaf_aggregation | AccessAdapter<8> | 0 | <div style='text-align: right'>114,850</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>56,206</div>  |
| leaf_aggregation | FriReducedOpeningAir | 0 | <div style='text-align: right'>570,948</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>111,046</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,865,063</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>95,728</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,463,112</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,060,066</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>648,059</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| leaf_aggregation |  | ADD | 0 | <div style='text-align: right'>2,539,350</div>  |
| leaf_aggregation |  | BBE4DIV | 0 | <div style='text-align: right'>8,109</div>  |
| leaf_aggregation |  | BBE4MUL | 0 | <div style='text-align: right'>38,132</div>  |
| leaf_aggregation |  | BEQ | 0 | <div style='text-align: right'>19,897</div>  |
| leaf_aggregation |  | BNE | 0 | <div style='text-align: right'>1,443,215</div>  |
| leaf_aggregation |  | COMP_POS2 | 0 | <div style='text-align: right'>18,396</div>  |
| leaf_aggregation |  | DIV | 0 | <div style='text-align: right'>177</div>  |
| leaf_aggregation |  | FE4ADD | 0 | <div style='text-align: right'>47,831</div>  |
| leaf_aggregation |  | FE4SUB | 0 | <div style='text-align: right'>16,974</div>  |
| leaf_aggregation |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>7,098</div>  |
| leaf_aggregation |  | JAL | 0 | <div style='text-align: right'>95,728</div>  |
| leaf_aggregation |  | LOADW | 0 | <div style='text-align: right'>209,622</div>  |
| leaf_aggregation |  | LOADW2 | 0 | <div style='text-align: right'>666,377</div>  |
| leaf_aggregation |  | MUL | 0 | <div style='text-align: right'>228,129</div>  |
| leaf_aggregation |  | PERM_POS2 | 0 | <div style='text-align: right'>37,810</div>  |
| leaf_aggregation |  | PHANTOM | 0 | <div style='text-align: right'>648,059</div>  |
| leaf_aggregation |  | SHINTW | 0 | <div style='text-align: right'>512,738</div>  |
| leaf_aggregation |  | STOREW | 0 | <div style='text-align: right'>255,861</div>  |
| leaf_aggregation |  | STOREW2 | 0 | <div style='text-align: right'>415,468</div>  |
| leaf_aggregation |  | SUB | 0 | <div style='text-align: right'>97,407</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <div style='text-align: right'>76,180,500</div>  |
| leaf_aggregation | AccessAdapter<2> |  | ADD | 0 | <div style='text-align: right'>614,746</div>  |
| leaf_aggregation | AccessAdapter<4> |  | ADD | 0 | <div style='text-align: right'>363,259</div>  |
| leaf_aggregation | Boundary |  | ADD | 0 | <div style='text-align: right'>753,786</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>324,360</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>160,996</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>95,134</div>  |
| leaf_aggregation | Boundary |  | BBE4DIV | 0 | <div style='text-align: right'>352</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>1,525,280</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BBE4MUL | 0 | <div style='text-align: right'>1,115,642</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BBE4MUL | 0 | <div style='text-align: right'>659,243</div>  |
| leaf_aggregation | Boundary |  | BBE4MUL | 0 | <div style='text-align: right'>1,037,212</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>457,631</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>33,193,945</div>  |
| leaf_aggregation | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>1,540</div>  |
| leaf_aggregation | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>910</div>  |
| leaf_aggregation | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>742,896</div>  |
| leaf_aggregation | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>438,984</div>  |
| leaf_aggregation | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>287,028</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>10,283,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>5,310</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>1,913,240</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>1,345,652</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>795,158</div>  |
| leaf_aggregation | Boundary |  | FE4ADD | 0 | <div style='text-align: right'>1,365,980</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>678,960</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>550,880</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>325,520</div>  |
| leaf_aggregation | Boundary |  | FE4SUB | 0 | <div style='text-align: right'>574,816</div>  |
| leaf_aggregation | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>400,708</div>  |
| leaf_aggregation | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>236,782</div>  |
| leaf_aggregation | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>36,540,672</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <div style='text-align: right'>957,280</div>  |
| leaf_aggregation | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>561</div>  |
| leaf_aggregation | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>663</div>  |
| leaf_aggregation | Boundary |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>8,594,502</div>  |
| leaf_aggregation | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>565,202</div>  |
| leaf_aggregation | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>288,223</div>  |
| leaf_aggregation | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>20,706</div>  |
| leaf_aggregation | Boundary |  | LOADW | 0 | <div style='text-align: right'>382,646</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>27,321,457</div>  |
| leaf_aggregation | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>59,994</div>  |
| leaf_aggregation | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>35,451</div>  |
| leaf_aggregation | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>510</div>  |
| leaf_aggregation | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,661</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>6,843,870</div>  |
| leaf_aggregation | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>30,415</div>  |
| leaf_aggregation | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>17,992</div>  |
| leaf_aggregation | Boundary |  | MUL | 0 | <div style='text-align: right'>112,376</div>  |
| leaf_aggregation | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,763,894</div>  |
| leaf_aggregation | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,043,666</div>  |
| leaf_aggregation | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>689,197</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>21,135,790</div>  |
| leaf_aggregation | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>3,888,354</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>21,022,258</div>  |
| leaf_aggregation | Boundary |  | SHINTW | 0 | <div style='text-align: right'>5,640,118</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>10,490,301</div>  |
| leaf_aggregation | AccessAdapter<2> |  | STOREW | 0 | <div style='text-align: right'>151,030</div>  |
| leaf_aggregation | AccessAdapter<4> |  | STOREW | 0 | <div style='text-align: right'>88,101</div>  |
| leaf_aggregation | Boundary |  | STOREW | 0 | <div style='text-align: right'>868,340</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>17,034,188</div>  |
| leaf_aggregation | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>1,726,054</div>  |
| leaf_aggregation | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>1,021,306</div>  |
| leaf_aggregation | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>595,850</div>  |
| leaf_aggregation | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>853,798</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>2,922,210</div>  |
| leaf_aggregation | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>78,782</div>  |
| leaf_aggregation | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>93,106</div>  |
| leaf_aggregation | Boundary |  | SUB | 0 | <div style='text-align: right'>15,180</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/regex-2-2-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12130528540)
