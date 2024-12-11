| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| regex_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>238,871,537</div>  | <div style='text-align: right'>4,190,904</div>  | <span style="color: red">(+192.0 [+1.2%])</span> <div style='text-align: right'>16,380.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <span style="color: green">(-5,710 [-0.0%])</span> <div style='text-align: right'>315,452,927</div>  | <span style="color: green">(-550 [-0.0%])</span> <div style='text-align: right'>7,322,532</div>  | <span style="color: red">(+143.0 [+0.5%])</span> <div style='text-align: right'>26,364.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <span style="color: red">(+1.0 [+2.4%])</span> <div style='text-align: right'>43.0</div>  | <span style="color: red">(+10.0 [+0.1%])</span> <div style='text-align: right'>7,574.0</div>  | <span style="color: green">(-21.0 [-0.4%])</span> <div style='text-align: right'>4,861.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+528.0 [+0.7%])</span> <div style='text-align: right'>74,742.0</div>  |

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
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
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>4</div>  |
| VolatileBoundaryAir | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| FriReducedOpeningAir | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>4</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| regex_program | 0 | <span style="color: red">(+192.0 [+1.2%])</span> <div style='text-align: right'>16,380.0</div>  | <div style='text-align: right'>791,770,496</div>  | <div style='text-align: right'>238,871,537</div>  | <div style='text-align: right'>4,190,904</div>  | <span style="color: red">(+18.0 [+0.7%])</span> <div style='text-align: right'>2,712.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| regex_program | ProgramChip | 0 | <div style='text-align: right'>89,914</div>  |
| regex_program | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| regex_program | Boundary | 0 | <div style='text-align: right'>69,164</div>  |
| regex_program | Merkle | 0 | <div style='text-align: right'>70,468</div>  |
| regex_program | AccessAdapter<2> | 0 | <div style='text-align: right'>42</div>  |
| regex_program | AccessAdapter<4> | 0 | <div style='text-align: right'>22</div>  |
| regex_program | AccessAdapter<8> | 0 | <div style='text-align: right'>69,164</div>  |
| regex_program | KeccakVmAir | 0 | <div style='text-align: right'>24</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>12,767</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | 0 | <div style='text-align: right'>114</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>244</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <div style='text-align: right'>52,087</div>  |
| regex_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>39,557</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>130,444</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>106,072</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>198,078</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>282,074</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <div style='text-align: right'>687</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>1,961,387</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>218,625</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>38,005</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,150,473</div>  |
| regex_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| regex_program | PhantomAir | 0 | <div style='text-align: right'>289</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>139,632</div>  |
| regex_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| regex_program |  | ADD | 0 | <div style='text-align: right'>1,008,001</div>  |
| regex_program |  | AND | 0 | <div style='text-align: right'>66,789</div>  |
| regex_program |  | AUIPC | 0 | <div style='text-align: right'>39,557</div>  |
| regex_program |  | BEQ | 0 | <div style='text-align: right'>178,501</div>  |
| regex_program |  | BGE | 0 | <div style='text-align: right'>294</div>  |
| regex_program |  | BGEU | 0 | <div style='text-align: right'>121,597</div>  |
| regex_program |  | BLT | 0 | <div style='text-align: right'>5,141</div>  |
| regex_program |  | BLTU | 0 | <div style='text-align: right'>71,046</div>  |
| regex_program |  | BNE | 0 | <div style='text-align: right'>103,573</div>  |
| regex_program |  | DIVU | 0 | <div style='text-align: right'>114</div>  |
| regex_program |  | HINT_STOREW | 0 | <div style='text-align: right'>12,767</div>  |
| regex_program |  | JAL | 0 | <div style='text-align: right'>61,576</div>  |
| regex_program |  | JALR | 0 | <div style='text-align: right'>130,444</div>  |
| regex_program |  | KECCAK256 | 0 | <div style='text-align: right'>1</div>  |
| regex_program |  | LOADB | 0 | <div style='text-align: right'>679</div>  |
| regex_program |  | LOADBU | 0 | <div style='text-align: right'>27,294</div>  |
| regex_program |  | LOADH | 0 | <div style='text-align: right'>8</div>  |
| regex_program |  | LOADHU | 0 | <div style='text-align: right'>95</div>  |
| regex_program |  | LOADW | 0 | <div style='text-align: right'>1,142,838</div>  |
| regex_program |  | LUI | 0 | <div style='text-align: right'>44,496</div>  |
| regex_program |  | MUL | 0 | <div style='text-align: right'>52,087</div>  |
| regex_program |  | MULHU | 0 | <div style='text-align: right'>244</div>  |
| regex_program |  | OR | 0 | <div style='text-align: right'>23,536</div>  |
| regex_program |  | PHANTOM | 0 | <div style='text-align: right'>289</div>  |
| regex_program |  | SLL | 0 | <div style='text-align: right'>213,542</div>  |
| regex_program |  | SLT | 0 | <div style='text-align: right'>5</div>  |
| regex_program |  | SLTU | 0 | <div style='text-align: right'>38,000</div>  |
| regex_program |  | SRA | 0 | <div style='text-align: right'>1</div>  |
| regex_program |  | SRL | 0 | <div style='text-align: right'>5,082</div>  |
| regex_program |  | STOREB | 0 | <div style='text-align: right'>12,721</div>  |
| regex_program |  | STOREH | 0 | <div style='text-align: right'>10,074</div>  |
| regex_program |  | STOREW | 0 | <div style='text-align: right'>768,365</div>  |
| regex_program |  | SUB | 0 | <div style='text-align: right'>42,583</div>  |
| regex_program |  | XOR | 0 | <div style='text-align: right'>9,564</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>36,288,036</div>  |
| regex_program | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>102</div>  |
| regex_program | Boundary |  | ADD | 0 | <div style='text-align: right'>240</div>  |
| regex_program | Merkle |  | ADD | 0 | <div style='text-align: right'>128</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>2,404,404</div>  |
| regex_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>830,697</div>  |
| regex_program | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| regex_program | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| regex_program | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>4,641,026</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | 0 | <div style='text-align: right'>9,408</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>3,891,104</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>164,512</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>2,273,472</div>  |
| regex_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>2,692,898</div>  |
| regex_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> |  | DIVU | 0 | <div style='text-align: right'>6,498</div>  |
| regex_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>331,942</div>  |
| regex_program | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>108,528</div>  |
| regex_program | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>255,360</div>  |
| regex_program | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>408,576</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>1,108,368</div>  |
| regex_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>3,652,432</div>  |
| regex_program | AccessAdapter<2> |  | KECCAK256 | 0 | <div style='text-align: right'>231</div>  |
| regex_program | AccessAdapter<4> |  | KECCAK256 | 0 | <div style='text-align: right'>143</div>  |
| regex_program | KeccakVmAir |  | KECCAK256 | 0 | <div style='text-align: right'>75,936</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <div style='text-align: right'>23,765</div>  |
| regex_program | AccessAdapter<8> |  | LOADB | 0 | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | LOADB | 0 | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>1,091,760</div>  |
| regex_program | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>187</div>  |
| regex_program | Boundary |  | LOADBU | 0 | <div style='text-align: right'>440</div>  |
| regex_program | Merkle |  | LOADBU | 0 | <div style='text-align: right'>2,624</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADH | 0 | <div style='text-align: right'>280</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | 0 | <div style='text-align: right'>3,800</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>45,713,520</div>  |
| regex_program | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>3,026</div>  |
| regex_program | Boundary |  | LOADW | 0 | <div style='text-align: right'>7,120</div>  |
| regex_program | Merkle |  | LOADW | 0 | <div style='text-align: right'>26,624</div>  |
| regex_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>800,928</div>  |
| regex_program | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| regex_program | Merkle |  | LUI | 0 | <div style='text-align: right'>64</div>  |
| regex_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <div style='text-align: right'>1,614,697</div>  |
| regex_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>9,516</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>847,296</div>  |
| regex_program | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>1,734</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>11,317,726</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | 0 | <div style='text-align: right'>185</div>  |
| regex_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>1,406,000</div>  |
| regex_program | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>17</div>  |
| regex_program | Boundary |  | SLTU | 0 | <div style='text-align: right'>40</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | 0 | <div style='text-align: right'>53</div>  |
| regex_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>269,346</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>508,840</div>  |
| regex_program | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>1,105</div>  |
| regex_program | Boundary |  | STOREB | 0 | <div style='text-align: right'>2,600</div>  |
| regex_program | Merkle |  | STOREB | 0 | <div style='text-align: right'>9,088</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | 0 | <div style='text-align: right'>402,960</div>  |
| regex_program | AccessAdapter<8> |  | STOREH | 0 | <div style='text-align: right'>85,221</div>  |
| regex_program | Boundary |  | STOREH | 0 | <div style='text-align: right'>200,520</div>  |
| regex_program | Merkle |  | STOREH | 0 | <div style='text-align: right'>321,600</div>  |
| regex_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>30,734,600</div>  |
| regex_program | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>389,640</div>  |
| regex_program | Boundary |  | STOREW | 0 | <div style='text-align: right'>916,800</div>  |
| regex_program | Merkle |  | STOREW | 0 | <div style='text-align: right'>1,482,752</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>1,532,988</div>  |
| regex_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>344,304</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| regex_program | <span style="color: green">(-14.0 [-0.3%])</span> <div style='text-align: right'>4,838.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>238,871,537</div>  | <div style='text-align: right'>4,190,904</div>  | <span style="color: red">(+192.0 [+1.2%])</span> <div style='text-align: right'>16,380.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  | <span style="color: green">(-5,710 [-0.0%])</span> <div style='text-align: right'>315,452,927</div>  | <span style="color: green">(-550 [-0.0%])</span> <div style='text-align: right'>7,322,532</div>  | <span style="color: red">(+143.0 [+0.5%])</span> <div style='text-align: right'>26,364.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| regex_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>4,194,304</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>6,815,744</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| regex_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| regex_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>5,373,952</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | KeccakVmAir | 0 | <div style='text-align: right'>142,464</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>32</div>  |
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16,384</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | <div style='text-align: right'>20,608</div>  | <div style='text-align: right'>57</div>  | <div style='text-align: right'>104</div>  |  | <div style='text-align: right'>128</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>35,584</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>256</div>  |
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>38,797,312</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>524,288</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>113,664</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>1,024</div>  |
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>27,525,120</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>65,536</div>  |
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>243,269,632</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| regex_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| regex_program | PhantomAir | 0 | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>512</div>  |
| regex_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>164,364,288</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>262,144</div>  |
| regex_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <span style="color: red">(+55.0 [+0.7%])</span> <div style='text-align: right'>7,968.0</div>  | <span style="color: green">(-5,710 [-0.0%])</span> <div style='text-align: right'>315,452,927</div>  | <span style="color: green">(-550 [-0.0%])</span> <div style='text-align: right'>7,322,532</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>311,121</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>1,058,325</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <span style="color: green">(-12 [-0.0%])</span> <div style='text-align: right'>1,111,580</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>556,000</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>115,174</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>56,262</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>570,948</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>111,763</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,870,816</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: green">(-550 [-0.6%])</span> <div style='text-align: right'>98,379</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,464,110</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,065,573</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>648,495</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,544,664</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>8,109</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>38,132</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>19,898</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,444,212</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>18,449</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>177</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>48,548</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>16,974</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,098</div>  |
| leaf_verifier |  | 0 | JAL | <span style="color: green">(-550 [-0.6%])</span> <div style='text-align: right'>98,379</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>212,271</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>666,566</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>228,568</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>37,813</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>648,495</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>513,606</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>257,184</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>415,946</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>97,407</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>76,339,920</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <span style="color: green">(-66 [-0.0%])</span> <div style='text-align: right'>634,854</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <span style="color: green">(-39 [-0.0%])</span> <div style='text-align: right'>375,141</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>767,943</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>324,360</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>161,084</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>95,186</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>352</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>1,525,280</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <span style="color: green">(-66 [-0.0%])</span> <div style='text-align: right'>1,116,588</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <span style="color: green">(-39 [-0.0%])</span> <div style='text-align: right'>659,802</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>1,037,080</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>457,654</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>33,216,876</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>1,540</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>910</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>749,892</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>443,118</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>289,731</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>10,312,991</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>5,310</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>1,941,920</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>1,370,644</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>809,926</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>1,380,324</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>678,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>550,726</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>325,429</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>574,816</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>400,708</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>236,782</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>36,540,672</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <span style="color: green">(-5,500 [-0.6%])</span> <div style='text-align: right'>983,790</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>572</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>676</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>8,703,111</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>567,347</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>289,445</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>21,607</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>382,239</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>27,329,206</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>59,994</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>35,451</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>510</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,408</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>6,857,040</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>30,041</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>17,771</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>112,376</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,764,048</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,043,757</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>689,248</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>21,137,467</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>3,890,970</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>21,057,846</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SHINTW | <div style='text-align: right'>22</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SHINTW | <div style='text-align: right'>26</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | SHINTW | <div style='text-align: right'>17</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,644,078</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>10,544,544</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>155,540</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>90,805</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>868,362</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>17,053,786</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>1,726,208</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>1,021,397</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>595,901</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>857,406</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>2,922,210</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>78,793</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>93,119</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>9,437,184</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>39,845,888</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>56,623,104</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>146,800,640</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <span style="color: red">(+143.0 [+0.5%])</span> <div style='text-align: right'>26,364.0</div>  | <div style='text-align: right'>807,209,432</div>  | <span style="color: green">(-32.0 [-1.8%])</span> <div style='text-align: right'>1,705.0</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: red">(+31.0 [+1.2%])</span> <div style='text-align: right'>2,705.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ce9c8ba40e916a59f81fff8019a00f127a21433/regex-2-2-1048476-64cpu-linux-arm64-mimalloc-regex_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/7ce9c8ba40e916a59f81fff8019a00f127a21433

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12280984404)
