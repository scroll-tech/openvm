| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <span style="color: green">(-12,960 [-0.0%])</span> <div style='text-align: right'>942,137,881</div>  | <span style="color: green">(-1,261 [-0.0%])</span> <div style='text-align: right'>11,500,504</div>  | <span style="color: green">(-841.0 [-1.6%])</span> <div style='text-align: right'>51,972.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-306.0 [-0.9%])</span> <div style='text-align: right'>32,813.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <span style="color: red">(+2.0 [+4.8%])</span> <div style='text-align: right'>44.0</div>  | <span style="color: green">(-10.0 [-0.1%])</span> <div style='text-align: right'>7,511.0</div>  | <span style="color: green">(-9.0 [-0.2%])</span> <div style='text-align: right'>4,814.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-49.0 [-0.1%])</span> <div style='text-align: right'>74,373.0</div>  |

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
| KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
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
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 |  |  | <span style="color: red">(+13.0 [+0.1%])</span> <div style='text-align: right'>14,748.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>1</div>  | <span style="color: green">(-841.0 [-1.6%])</span> <div style='text-align: right'>51,972.0</div>  | <div style='text-align: right'>2,088,960,024</div>  | <span style="color: green">(-12,960 [-0.0%])</span> <div style='text-align: right'>942,137,881</div>  | <span style="color: green">(-1,261 [-0.0%])</span> <div style='text-align: right'>11,500,504</div>  | <span style="color: green">(-88.0 [-0.5%])</span> <div style='text-align: right'>18,444.0</div>  |  |
| leaf_aggregation | 0 | <span style="color: green">(-11.0 [-6.2%])</span> <div style='text-align: right'>166.0</div>  | <span style="color: green">(-306.0 [-0.9%])</span> <div style='text-align: right'>32,813.0</div>  | <span style="color: green">(-90.0 [-0.6%])</span> <div style='text-align: right'>14,553.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-2.0 [-0.6%])</span> <div style='text-align: right'>361.0</div>  |  |  |  |  |  | <span style="color: green">(-215.0 [-1.2%])</span> <div style='text-align: right'>18,244.0</div>  | <span style="color: red">(+331.0 [+0.4%])</span> <div style='text-align: right'>74,675.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <div style='text-align: right'>308,776</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <div style='text-align: right'>424,724</div>  |
| app_proof | Merkle | 0 | <div style='text-align: right'>644,104</div>  |
| app_proof | AccessAdapter<2> | 0 | <span style="color: green">(-20 [-0.0%])</span> <div style='text-align: right'>1,406,470</div>  |
| app_proof | AccessAdapter<4> | 0 | <span style="color: green">(-10 [-0.0%])</span> <div style='text-align: right'>740,308</div>  |
| app_proof | AccessAdapter<8> | 0 | <div style='text-align: right'>497,326</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>24</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>12,767</div>  |
| app_proof | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | 0 | <div style='text-align: right'>114</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>244</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <div style='text-align: right'>52,087</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>39,557</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>130,444</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>106,072</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>198,078</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>282,074</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <div style='text-align: right'>687</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>1,961,387</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>218,625</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>38,005</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,150,473</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>648,059</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>1,068,828</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>570,948</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>111,763</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,868,068</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: green">(-1,261 [-1.3%])</span> <div style='text-align: right'>94,947</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,463,167</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,060,289</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <div style='text-align: right'>2,542,355</div>  |
| app_proof |  | AND | 0 | <div style='text-align: right'>66,789</div>  |
| app_proof |  | AUIPC | 0 | <div style='text-align: right'>39,557</div>  |
| app_proof |  | BEQ | 0 | <div style='text-align: right'>178,501</div>  |
| app_proof |  | BGE | 0 | <div style='text-align: right'>294</div>  |
| app_proof |  | BGEU | 0 | <div style='text-align: right'>121,597</div>  |
| app_proof |  | BLT | 0 | <div style='text-align: right'>5,141</div>  |
| app_proof |  | BLTU | 0 | <div style='text-align: right'>71,046</div>  |
| app_proof |  | BNE | 0 | <div style='text-align: right'>1,443,270</div>  |
| app_proof |  | DIVU | 0 | <div style='text-align: right'>114</div>  |
| app_proof |  | HINT_STOREW | 0 | <div style='text-align: right'>12,767</div>  |
| app_proof |  | JAL | 0 | <span style="color: green">(-1,261 [-1.3%])</span> <div style='text-align: right'>94,947</div>  |
| app_proof |  | JALR | 0 | <div style='text-align: right'>130,444</div>  |
| app_proof |  | KECCAK256 | 0 | <div style='text-align: right'>1</div>  |
| app_proof |  | LOADB | 0 | <div style='text-align: right'>679</div>  |
| app_proof |  | LOADBU | 0 | <div style='text-align: right'>27,294</div>  |
| app_proof |  | LOADH | 0 | <div style='text-align: right'>8</div>  |
| app_proof |  | LOADHU | 0 | <div style='text-align: right'>95</div>  |
| app_proof |  | LOADW | 0 | <div style='text-align: right'>1,142,838</div>  |
| app_proof |  | LUI | 0 | <div style='text-align: right'>44,496</div>  |
| app_proof |  | MUL | 0 | <div style='text-align: right'>228,129</div>  |
| app_proof |  | MULHU | 0 | <div style='text-align: right'>244</div>  |
| app_proof |  | OR | 0 | <div style='text-align: right'>23,536</div>  |
| app_proof |  | PHANTOM | 0 | <div style='text-align: right'>648,059</div>  |
| app_proof |  | SLL | 0 | <div style='text-align: right'>213,542</div>  |
| app_proof |  | SLT | 0 | <div style='text-align: right'>5</div>  |
| app_proof |  | SLTU | 0 | <div style='text-align: right'>38,000</div>  |
| app_proof |  | SRA | 0 | <div style='text-align: right'>1</div>  |
| app_proof |  | SRL | 0 | <div style='text-align: right'>5,082</div>  |
| app_proof |  | STOREB | 0 | <div style='text-align: right'>12,721</div>  |
| app_proof |  | STOREH | 0 | <div style='text-align: right'>10,074</div>  |
| app_proof |  | STOREW | 0 | <div style='text-align: right'>768,365</div>  |
| app_proof |  | SUB | 0 | <div style='text-align: right'>97,407</div>  |
| app_proof |  | XOR | 0 | <div style='text-align: right'>9,564</div>  |
| app_proof |  | BBE4DIV | 0 | <div style='text-align: right'>8,109</div>  |
| app_proof |  | BBE4MUL | 0 | <div style='text-align: right'>38,132</div>  |
| app_proof |  | COMP_POS2 | 0 | <div style='text-align: right'>18,396</div>  |
| app_proof |  | DIV | 0 | <div style='text-align: right'>177</div>  |
| app_proof |  | FE4ADD | 0 | <div style='text-align: right'>48,548</div>  |
| app_proof |  | FE4SUB | 0 | <div style='text-align: right'>16,974</div>  |
| app_proof |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>7,098</div>  |
| app_proof |  | LOADW2 | 0 | <div style='text-align: right'>666,539</div>  |
| app_proof |  | PERM_POS2 | 0 | <div style='text-align: right'>37,813</div>  |
| app_proof |  | SHINTW | 0 | <div style='text-align: right'>512,738</div>  |
| app_proof |  | STOREW2 | 0 | <div style='text-align: right'>415,495</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>36,288,036</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>294,372</div>  |
| app_proof | Boundary |  | ADD | 0 | <div style='text-align: right'>692,640</div>  |
| app_proof | Merkle |  | ADD | 0 | <div style='text-align: right'>2,039,040</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>2,404,404</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>830,697</div>  |
| app_proof | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>4,641,026</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | 0 | <div style='text-align: right'>9,408</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>3,891,104</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>164,512</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>2,273,472</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>2,692,898</div>  |
| app_proof | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> |  | DIVU | 0 | <div style='text-align: right'>6,498</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>331,942</div>  |
| app_proof | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>108,511</div>  |
| app_proof | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>255,320</div>  |
| app_proof | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>408,576</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>1,108,368</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>3,652,432</div>  |
| app_proof | AccessAdapter<2> |  | KECCAK256 | 0 | <div style='text-align: right'>231</div>  |
| app_proof | AccessAdapter<4> |  | KECCAK256 | 0 | <div style='text-align: right'>143</div>  |
| app_proof | KeccakVmAir |  | KECCAK256 | 0 | <div style='text-align: right'>75,936</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <div style='text-align: right'>23,765</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>1,091,760</div>  |
| app_proof | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>170</div>  |
| app_proof | Boundary |  | LOADBU | 0 | <div style='text-align: right'>400</div>  |
| app_proof | Merkle |  | LOADBU | 0 | <div style='text-align: right'>2,688</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADH | 0 | <div style='text-align: right'>280</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | 0 | <div style='text-align: right'>3,800</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>45,713,520</div>  |
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>126,191</div>  |
| app_proof | Boundary |  | LOADW | 0 | <div style='text-align: right'>248,200</div>  |
| app_proof | Merkle |  | LOADW | 0 | <div style='text-align: right'>397,504</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>800,928</div>  |
| app_proof | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| app_proof | Merkle |  | LUI | 0 | <div style='text-align: right'>64</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <div style='text-align: right'>1,614,697</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>9,516</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>847,296</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>3,888,354</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>11,317,726</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | 0 | <div style='text-align: right'>185</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>1,406,000</div>  |
| app_proof | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | SLTU | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | 0 | <div style='text-align: right'>53</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>269,346</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>508,840</div>  |
| app_proof | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>2,159</div>  |
| app_proof | Boundary |  | STOREB | 0 | <div style='text-align: right'>5,080</div>  |
| app_proof | Merkle |  | STOREB | 0 | <div style='text-align: right'>10,496</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | 0 | <div style='text-align: right'>402,960</div>  |
| app_proof | AccessAdapter<8> |  | STOREH | 0 | <div style='text-align: right'>85,255</div>  |
| app_proof | Boundary |  | STOREH | 0 | <div style='text-align: right'>200,600</div>  |
| app_proof | Merkle |  | STOREH | 0 | <div style='text-align: right'>321,792</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>30,734,600</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>389,487</div>  |
| app_proof | Boundary |  | STOREW | 0 | <div style='text-align: right'>916,440</div>  |
| app_proof | Merkle |  | STOREW | 0 | <div style='text-align: right'>3,290,112</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>1,532,988</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>344,304</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <div style='text-align: right'>76,270,650</div>  |
| app_proof | AccessAdapter<2> |  | ADD | 0 | <span style="color: green">(-110 [-0.0%])</span> <div style='text-align: right'>1,018,534</div>  |
| app_proof | AccessAdapter<4> |  | ADD | 0 | <span style="color: green">(-65 [-0.0%])</span> <div style='text-align: right'>602,069</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>324,360</div>  |
| app_proof | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>160,908</div>  |
| app_proof | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>95,082</div>  |
| app_proof | AccessAdapter<8> |  | BBE4DIV | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | BBE4DIV | 0 | <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | BBE4DIV | 0 | <div style='text-align: right'>384</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <div style='text-align: right'>1,525,280</div>  |
| app_proof | AccessAdapter<2> |  | BBE4MUL | 0 | <span style="color: green">(-110 [-0.0%])</span> <div style='text-align: right'>597,608</div>  |
| app_proof | AccessAdapter<4> |  | BBE4MUL | 0 | <span style="color: green">(-65 [-0.0%])</span> <div style='text-align: right'>353,132</div>  |
| app_proof | AccessAdapter<8> |  | BBE4MUL | 0 | <div style='text-align: right'>395,947</div>  |
| app_proof | Boundary |  | BBE4MUL | 0 | <div style='text-align: right'>931,640</div>  |
| app_proof | Merkle |  | BBE4MUL | 0 | <div style='text-align: right'>458,752</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <div style='text-align: right'>457,631</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <div style='text-align: right'>33,195,210</div>  |
| app_proof | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>1,540</div>  |
| app_proof | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>910</div>  |
| app_proof | AccessAdapter<2> |  | COMP_POS2 | 0 | <div style='text-align: right'>742,896</div>  |
| app_proof | AccessAdapter<4> |  | COMP_POS2 | 0 | <div style='text-align: right'>438,984</div>  |
| app_proof | AccessAdapter<8> |  | COMP_POS2 | 0 | <div style='text-align: right'>287,028</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <div style='text-align: right'>10,283,364</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>5,310</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>1,941,920</div>  |
| app_proof | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>680,482</div>  |
| app_proof | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>402,103</div>  |
| app_proof | AccessAdapter<8> |  | FE4ADD | 0 | <div style='text-align: right'>530,825</div>  |
| app_proof | Boundary |  | FE4ADD | 0 | <div style='text-align: right'>1,249,000</div>  |
| app_proof | Merkle |  | FE4ADD | 0 | <div style='text-align: right'>2,400,064</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>678,960</div>  |
| app_proof | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>263,318</div>  |
| app_proof | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>155,597</div>  |
| app_proof | AccessAdapter<8> |  | FE4SUB | 0 | <div style='text-align: right'>221,697</div>  |
| app_proof | Boundary |  | FE4SUB | 0 | <div style='text-align: right'>521,640</div>  |
| app_proof | Merkle |  | FE4SUB | 0 | <div style='text-align: right'>252,160</div>  |
| app_proof | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>400,708</div>  |
| app_proof | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>236,782</div>  |
| app_proof | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>36,540,672</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: green">(-12,610 [-1.3%])</span> <div style='text-align: right'>949,470</div>  |
| app_proof | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>572</div>  |
| app_proof | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>676</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | 0 | <div style='text-align: right'>8,594,502</div>  |
| app_proof | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>756,294</div>  |
| app_proof | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>405,288</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <div style='text-align: right'>27,328,099</div>  |
| app_proof | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>60,863</div>  |
| app_proof | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>36,101</div>  |
| app_proof | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>1,343</div>  |
| app_proof | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,960</div>  |
| app_proof | Merkle |  | LOADW2 | 0 | <div style='text-align: right'>3,008</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <div style='text-align: right'>6,843,870</div>  |
| app_proof | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>78,903</div>  |
| app_proof | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>50,674</div>  |
| app_proof | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>42,789</div>  |
| app_proof | Boundary |  | MUL | 0 | <div style='text-align: right'>100,680</div>  |
| app_proof | Merkle |  | MUL | 0 | <div style='text-align: right'>168,576</div>  |
| app_proof | AccessAdapter<2> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,764,048</div>  |
| app_proof | AccessAdapter<4> |  | PERM_POS2 | 0 | <div style='text-align: right'>1,043,757</div>  |
| app_proof | AccessAdapter<8> |  | PERM_POS2 | 0 | <div style='text-align: right'>689,248</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <div style='text-align: right'>21,137,467</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | 0 | <div style='text-align: right'>21,022,258</div>  |
| app_proof | AccessAdapter<2> |  | SHINTW | 0 | <div style='text-align: right'>2,979,944</div>  |
| app_proof | AccessAdapter<4> |  | SHINTW | 0 | <div style='text-align: right'>1,949,818</div>  |
| app_proof | AccessAdapter<8> |  | SHINTW | 0 | <div style='text-align: right'>1,553,562</div>  |
| app_proof | Boundary |  | SHINTW | 0 | <div style='text-align: right'>3,655,440</div>  |
| app_proof | Merkle |  | SHINTW | 0 | <div style='text-align: right'>10,604,032</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <div style='text-align: right'>10,491,695</div>  |
| app_proof | AccessAdapter<2> |  | STOREW | 0 | <div style='text-align: right'>590,282</div>  |
| app_proof | AccessAdapter<4> |  | STOREW | 0 | <div style='text-align: right'>363,870</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <div style='text-align: right'>17,035,295</div>  |
| app_proof | AccessAdapter<2> |  | STOREW2 | 0 | <div style='text-align: right'>2,154,977</div>  |
| app_proof | AccessAdapter<4> |  | STOREW2 | 0 | <div style='text-align: right'>1,292,759</div>  |
| app_proof | AccessAdapter<8> |  | STOREW2 | 0 | <div style='text-align: right'>795,022</div>  |
| app_proof | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>468,520</div>  |
| app_proof | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>915,328</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <div style='text-align: right'>2,922,210</div>  |
| app_proof | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>93,676</div>  |
| app_proof | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>109,837</div>  |
| app_proof | AccessAdapter<8> |  | SUB | 0 | <div style='text-align: right'>21,862</div>  |
| app_proof | Boundary |  | SUB | 0 | <div style='text-align: right'>51,440</div>  |
| app_proof | Merkle |  | SUB | 0 | <div style='text-align: right'>82,304</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: red">(+26.0 [+0.5%])</span> <div style='text-align: right'>4,847.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-12,960 [-0.0%])</span> <div style='text-align: right'>942,137,881</div>  | <span style="color: green">(-1,261 [-0.0%])</span> <div style='text-align: right'>11,500,504</div>  | <span style="color: green">(-841.0 [-1.6%])</span> <div style='text-align: right'>51,972.0</div>  |
| leaf_aggregation |  | <div style='text-align: right'>2</div>  |  |  |  | <span style="color: green">(-306.0 [-0.9%])</span> <div style='text-align: right'>32,813.0</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>9,437,184</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>14,680,064</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>46,137,344</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | AccessAdapterAir<2> | 0 | <div style='text-align: right'>56,623,104</div>  |  |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>30,408,704</div>  |  |  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>17,301,504</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  |  |  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  |  | <div style='text-align: right'>32</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>20,608</div>  |  |  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  |  | <div style='text-align: right'>128</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>35,584</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>256</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>113,664</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>27,525,120</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>14,680,064</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>1,247,805,440</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| leaf_aggregation | VmConnectorAir | 0 |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PersistentBoundaryAir<8> | 0 |  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | MemoryMerkleAir<8> | 0 |  | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<2> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<4> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<8> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 |  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | FriReducedOpeningAir | 0 |  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  |  |  | <div style='text-align: right'>2</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VariableRangeCheckerAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>146,800,640</div>  |  |  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>7,864,320</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>209,715,200</div>  |  |  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>4,194,304</div>  |
| app_proof | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>106,954,752</div>  |  |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  |  |  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: red">(+1.0 [+0.0%])</span> <div style='text-align: right'>2,690.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/17afa111e7a7a6fedffa5a64f119479e7a11da07/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/17afa111e7a7a6fedffa5a64f119479e7a11da07

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12263798777)
