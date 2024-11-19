| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| bincode_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  | <div style='text-align: right'>90,593.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  | <div style='text-align: right'>330,861,719</div>  | <div style='text-align: right'>8,211,795</div>  | <div style='text-align: right'>21,132.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| bincode_program | true | <div style='text-align: right'>65,904.0</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| bincode_program | ProgramChip | true | <div style='text-align: right'>10,116</div>  |
| bincode_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| bincode_program | Boundary | true | <div style='text-align: right'>268,430</div>  |
| bincode_program | Merkle | true | <div style='text-align: right'>268,678</div>  |
| bincode_program | AccessAdapter<8> | true | <div style='text-align: right'>268,430</div>  |
| bincode_program | PhantomAir | true | <div style='text-align: right'>25,952</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>4,194,222</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>255,446</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>122,307</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>3,410,792</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>475,761</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>1,000,190</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>548,411</div>  |
| bincode_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>317,239</div>  |
| bincode_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>314,613</div>  |
| bincode_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>157,312</div>  |
| bincode_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>3,287</div>  |
| bincode_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>872</div>  |
| bincode_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>83,515</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | true | <div style='text-align: right'>537,108</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| bincode_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| bincode_program | VariableRangeCheckerAir | true | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| bincode_program | true |  | ADD | <div style='text-align: right'>3,313,863</div>  |
| bincode_program | true |  | AND | <div style='text-align: right'>548,441</div>  |
| bincode_program | true |  | AUIPC | <div style='text-align: right'>157,312</div>  |
| bincode_program | true |  | BEQ | <div style='text-align: right'>328,434</div>  |
| bincode_program | true |  | BGE | <div style='text-align: right'>25,426</div>  |
| bincode_program | true |  | BGEU | <div style='text-align: right'>174,800</div>  |
| bincode_program | true |  | BLT | <div style='text-align: right'>149,669</div>  |
| bincode_program | true |  | BLTU | <div style='text-align: right'>198,516</div>  |
| bincode_program | true |  | BNE | <div style='text-align: right'>671,756</div>  |
| bincode_program | true |  | HINT_STOREW | <div style='text-align: right'>83,515</div>  |
| bincode_program | true |  | JAL | <div style='text-align: right'>157,257</div>  |
| bincode_program | true |  | JALR | <div style='text-align: right'>314,613</div>  |
| bincode_program | true |  | LOADB | <div style='text-align: right'>473,676</div>  |
| bincode_program | true |  | LOADBU | <div style='text-align: right'>200,502</div>  |
| bincode_program | true |  | LOADH | <div style='text-align: right'>2,085</div>  |
| bincode_program | true |  | LOADHU | <div style='text-align: right'>2,989</div>  |
| bincode_program | true |  | LOADW | <div style='text-align: right'>1,212,973</div>  |
| bincode_program | true |  | LUI | <div style='text-align: right'>159,982</div>  |
| bincode_program | true |  | MUL | <div style='text-align: right'>3,287</div>  |
| bincode_program | true |  | MULHU | <div style='text-align: right'>872</div>  |
| bincode_program | true |  | OR | <div style='text-align: right'>139,921</div>  |
| bincode_program | true |  | PHANTOM | <div style='text-align: right'>25,952</div>  |
| bincode_program | true |  | SLL | <div style='text-align: right'>107,183</div>  |
| bincode_program | true |  | SLTU | <div style='text-align: right'>255,446</div>  |
| bincode_program | true |  | SRA | <div style='text-align: right'>3,021</div>  |
| bincode_program | true |  | SRL | <div style='text-align: right'>12,103</div>  |
| bincode_program | true |  | STOREB | <div style='text-align: right'>557,641</div>  |
| bincode_program | true |  | STOREH | <div style='text-align: right'>7,501</div>  |
| bincode_program | true |  | STOREW | <div style='text-align: right'>1,429,186</div>  |
| bincode_program | true |  | SUB | <div style='text-align: right'>186,796</div>  |
| bincode_program | true |  | XOR | <div style='text-align: right'>5,201</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>119,299,068</div>  |
| bincode_program | AccessAdapter<8> | true |  | ADD | <div style='text-align: right'>170</div>  |
| bincode_program | Boundary | true |  | ADD | <div style='text-align: right'>400</div>  |
| bincode_program | Merkle | true |  | ADD | <div style='text-align: right'>320</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>19,743,876</div>  |
| bincode_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>3,303,552</div>  |
| bincode_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| bincode_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| bincode_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>8,539,284</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>813,632</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>5,593,600</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,789,408</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>6,352,512</div>  |
| bincode_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>17,465,656</div>  |
| bincode_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>2,171,390</div>  |
| bincode_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>709,886</div>  |
| bincode_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>1,670,320</div>  |
| bincode_program | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>2,672,256</div>  |
| bincode_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>2,830,626</div>  |
| bincode_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>8,809,164</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>16,578,660</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>8,020,080</div>  |
| bincode_program | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>45,543</div>  |
| bincode_program | Boundary | true |  | LOADBU | <div style='text-align: right'>107,160</div>  |
| bincode_program | Merkle | true |  | LOADBU | <div style='text-align: right'>171,392</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>72,975</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>119,560</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>48,518,920</div>  |
| bincode_program | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>9,962</div>  |
| bincode_program | Boundary | true |  | LOADW | <div style='text-align: right'>23,440</div>  |
| bincode_program | Merkle | true |  | LOADW | <div style='text-align: right'>35,200</div>  |
| bincode_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>2,879,676</div>  |
| bincode_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| bincode_program | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| bincode_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>101,897</div>  |
| bincode_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>34,008</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>5,037,156</div>  |
| bincode_program | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>155,712</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>5,680,699</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>9,451,502</div>  |
| bincode_program | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| bincode_program | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| bincode_program | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>160,113</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>641,459</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>22,305,640</div>  |
| bincode_program | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>694,433</div>  |
| bincode_program | Boundary | true |  | STOREB | <div style='text-align: right'>1,633,960</div>  |
| bincode_program | Merkle | true |  | STOREB | <div style='text-align: right'>2,872,960</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>300,040</div>  |
| bincode_program | AccessAdapter<8> | true |  | STOREH | <div style='text-align: right'>7,395</div>  |
| bincode_program | Boundary | true |  | STOREH | <div style='text-align: right'>17,400</div>  |
| bincode_program | Merkle | true |  | STOREH | <div style='text-align: right'>27,584</div>  |
| bincode_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>57,167,440</div>  |
| bincode_program | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>869,550</div>  |
| bincode_program | Boundary | true |  | STOREW | <div style='text-align: right'>2,046,000</div>  |
| bincode_program | Merkle | true |  | STOREW | <div style='text-align: right'>3,018,496</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>6,724,656</div>  |
| bincode_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>187,236</div>  |
| bincode_program | AccessAdapter<8> | true |  | BNE | <div style='text-align: right'>34</div>  |
| bincode_program | Boundary | true |  | BNE | <div style='text-align: right'>80</div>  |
| bincode_program | Merkle | true |  | BNE | <div style='text-align: right'>3,584</div>  |
| bincode_program | AccessAdapter<8> | true |  | LOADB | <div style='text-align: right'>36,193</div>  |
| bincode_program | Boundary | true |  | LOADB | <div style='text-align: right'>85,160</div>  |
| bincode_program | Merkle | true |  | LOADB | <div style='text-align: right'>139,072</div>  |
| bincode_program | AccessAdapter<8> | true |  | LOADH | <div style='text-align: right'>17</div>  |
| bincode_program | Boundary | true |  | LOADH | <div style='text-align: right'>40</div>  |
| bincode_program | Merkle | true |  | LOADH | <div style='text-align: right'>64</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bincode_program | <div style='text-align: right'>8.0</div>  | <div style='text-align: right'>26,690.0</div>  | <div style='text-align: right'>16,648.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>179.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  | <div style='text-align: right'>90,593.0</div>  |
| leaf_aggregation |  |  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>330,861,719</div>  | <div style='text-align: right'>8,211,795</div>  | <div style='text-align: right'>21,132.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| bincode_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| bincode_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| bincode_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bincode_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| bincode_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| bincode_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>16,777,216</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>27,262,976</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>21,495,808</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | PhantomAir | 0 | <div style='text-align: right'>589,824</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>486,539,264</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>4,194,304</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>20,185,088</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>469,762,048</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>4,194,304</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>58,195,968</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>77,594,624</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>92,274,688</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>32,505,856</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>33,554,432</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>12,845,056</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>454,656</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>4,096</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>142,336</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>1,024</div>  |
| bincode_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>657,457,152</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | ProgramAir | 1 | <div style='text-align: right'>294,912</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>16,384</div>  |
| bincode_program | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| bincode_program | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>2,097,152</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>3,407,872</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | AccessAdapterAir<8> | 1 | <div style='text-align: right'>2,686,976</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | PhantomAir | 1 | <div style='text-align: right'>73,728</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>4,096</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>5,046,272</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 1 | <div style='text-align: right'>3,440,640</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 1 | <div style='text-align: right'>58,720,256</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 1 | <div style='text-align: right'>14,548,992</div>  |  |  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>11,534,336</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>4,063,232</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 1 | <div style='text-align: right'>4,194,304</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 1 | <div style='text-align: right'>1,605,632</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 1 | <div style='text-align: right'>56,832</div>  |  |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>512</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 1 | <div style='text-align: right'>35,584</div>  |  |  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  |  | <div style='text-align: right'>256</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>82,182,144</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VariableRangeCheckerAir | 1 | <div style='text-align: right'>1,179,648</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| leaf_aggregation | VolatileBoundaryAir | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<2> | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| leaf_aggregation | AccessAdapterAir<4> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | AccessAdapterAir<8> | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,048,576</div>  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| leaf_aggregation | FriMatOpeningAir | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| leaf_aggregation | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramAir | 1 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| leaf_aggregation | VmConnectorAir | 1 |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VolatileBoundaryAir | 1 |  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<2> | 1 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<4> | 1 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<8> | 1 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 1 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  |  |  | <div style='text-align: right'>2</div>  |  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | FriMatOpeningAir | 1 |  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 1 |  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VariableRangeCheckerAir | 1 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bincode_program | 0 |  | <div style='text-align: right'>8,833.0</div>  |  |  |  |  | <div style='text-align: right'>45,645.0</div>  | <div style='text-align: right'>2,036,374,560</div>  |  |
| bincode_program | 1 |  | <div style='text-align: right'>1,207.0</div>  |  |  |  |  | <div style='text-align: right'>8,218.0</div>  | <div style='text-align: right'>341,575,968</div>  |  |
| leaf_aggregation | 0 | <div style='text-align: right'>37.0</div>  | <div style='text-align: right'>810.0</div>  | <div style='text-align: right'>4,692.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>46.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>15,200.0</div>  | <div style='text-align: right'>400,916,504</div>  | <div style='text-align: right'>254.0</div>  |
| leaf_aggregation | 1 | <div style='text-align: right'>52.0</div>  | <div style='text-align: right'>5,122.0</div>  | <div style='text-align: right'>4,302.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>55.0</div>  | <div style='text-align: right'>1</div>  |  |  | <div style='text-align: right'>249.0</div>  |

| group | collect_metrics | segment | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true | 0 | <div style='text-align: right'>25,519.0</div>  | <div style='text-align: right'>171,682,061</div>  | <div style='text-align: right'>4,267,254</div>  |
| leaf_aggregation | true | 1 | <div style='text-align: right'>24,335.0</div>  | <div style='text-align: right'>159,179,658</div>  | <div style='text-align: right'>3,944,541</div>  |

| group | chip_name | collect_metrics | segment | rows_used |
| --- | --- | --- | --- | --- |
| leaf_aggregation | ProgramChip | true | 0 | <div style='text-align: right'>104,932</div>  |
| leaf_aggregation | VmConnectorAir | true | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 0 | <div style='text-align: right'>471,712</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 0 | <div style='text-align: right'>457,172</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 0 | <div style='text-align: right'>228,712</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 0 | <div style='text-align: right'>66,632</div>  |
| leaf_aggregation | PhantomAir | true | 0 | <div style='text-align: right'>247,598</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 0 | <div style='text-align: right'>1,295,951</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 0 | <div style='text-align: right'>823,135</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 0 | <div style='text-align: right'>94,660</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 0 | <div style='text-align: right'>1,727,340</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 0 | <div style='text-align: right'>40,425</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 0 | <div style='text-align: right'>176,064</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 0 | <div style='text-align: right'>32,055</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 0 | <div style='text-align: right'>131,072</div>  |
| leaf_aggregation | ProgramChip | true | 1 | <div style='text-align: right'>104,932</div>  |
| leaf_aggregation | VmConnectorAir | true | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | Boundary | true | 1 | <div style='text-align: right'>439,205</div>  |
| leaf_aggregation | AccessAdapter<2> | true | 1 | <div style='text-align: right'>427,456</div>  |
| leaf_aggregation | AccessAdapter<4> | true | 1 | <div style='text-align: right'>213,938</div>  |
| leaf_aggregation | AccessAdapter<8> | true | 1 | <div style='text-align: right'>61,250</div>  |
| leaf_aggregation | PhantomAir | true | 1 | <div style='text-align: right'>237,623</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | 1 | <div style='text-align: right'>1,212,820</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | 1 | <div style='text-align: right'>761,017</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | 1 | <div style='text-align: right'>87,860</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | 1 | <div style='text-align: right'>1,571,753</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | 1 | <div style='text-align: right'>38,182</div>  |
| leaf_aggregation | FriMatOpeningAir | true | 1 | <div style='text-align: right'>170,520</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | 1 | <div style='text-align: right'>29,448</div>  |
| leaf_aggregation | VariableRangeCheckerAir | true | 1 | <div style='text-align: right'>131,072</div>  |

| group | collect_metrics | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- | --- |
| leaf_aggregation | true |  | JAL | 0 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 0 | <div style='text-align: right'>15,147</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>190</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>570</div>  |
| leaf_aggregation | true | AddEFI | ADD | 0 | <div style='text-align: right'>376</div>  |
| leaf_aggregation | true | AddEI | ADD | 0 | <div style='text-align: right'>33,348</div>  |
| leaf_aggregation | true | AddFI | ADD | 0 | <div style='text-align: right'>79,470</div>  |
| leaf_aggregation | true | AddV | ADD | 0 | <div style='text-align: right'>17,699</div>  |
| leaf_aggregation | true | AddVI | ADD | 0 | <div style='text-align: right'>439,162</div>  |
| leaf_aggregation | true | Alloc | ADD | 0 | <div style='text-align: right'>62,825</div>  |
| leaf_aggregation | true | Alloc | LOADW | 0 | <div style='text-align: right'>62,825</div>  |
| leaf_aggregation | true | Alloc | MUL | 0 | <div style='text-align: right'>37,048</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 0 | <div style='text-align: right'>260</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 0 | <div style='text-align: right'>10,123</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 0 | <div style='text-align: right'>2,523</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>270</div>  |
| leaf_aggregation | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>7,896</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>3,192</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>1,848</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>183,288</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>12,180</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>1,848</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>5,040</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>5,040</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>84</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>7,060</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>63</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 0 | <div style='text-align: right'>252</div>  |
| leaf_aggregation | true | DivFIN | DIV | 0 | <div style='text-align: right'>149</div>  |
| leaf_aggregation | true | For | ADD | 0 | <div style='text-align: right'>520,752</div>  |
| leaf_aggregation | true | For | BNE | 0 | <div style='text-align: right'>569,452</div>  |
| leaf_aggregation | true | For | JAL | 0 | <div style='text-align: right'>48,700</div>  |
| leaf_aggregation | true | For | LOADW | 0 | <div style='text-align: right'>2,562</div>  |
| leaf_aggregation | true | For | STOREW | 0 | <div style='text-align: right'>46,138</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>6,090</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>43</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>25,777</div>  |
| leaf_aggregation | true | IfEq | BNE | 0 | <div style='text-align: right'>20,560</div>  |
| leaf_aggregation | true | IfEqI | BNE | 0 | <div style='text-align: right'>199,348</div>  |
| leaf_aggregation | true | IfEqI | JAL | 0 | <div style='text-align: right'>45,956</div>  |
| leaf_aggregation | true | IfNe | BEQ | 0 | <div style='text-align: right'>17,959</div>  |
| leaf_aggregation | true | IfNe | JAL | 0 | <div style='text-align: right'>3</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 0 | <div style='text-align: right'>2,636</div>  |
| leaf_aggregation | true | ImmE | STOREW | 0 | <div style='text-align: right'>4,196</div>  |
| leaf_aggregation | true | ImmF | STOREW | 0 | <div style='text-align: right'>41,046</div>  |
| leaf_aggregation | true | ImmV | STOREW | 0 | <div style='text-align: right'>33,291</div>  |
| leaf_aggregation | true | LoadE | LOADW | 0 | <div style='text-align: right'>28,052</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>74,516</div>  |
| leaf_aggregation | true | LoadF | LOADW | 0 | <div style='text-align: right'>28,737</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>112,735</div>  |
| leaf_aggregation | true | LoadV | LOADW | 0 | <div style='text-align: right'>30,381</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>246,544</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>12,020</div>  |
| leaf_aggregation | true | MulEF | MUL | 0 | <div style='text-align: right'>4,200</div>  |
| leaf_aggregation | true | MulEFI | MUL | 0 | <div style='text-align: right'>644</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>2,199</div>  |
| leaf_aggregation | true | MulEI | STOREW | 0 | <div style='text-align: right'>8,796</div>  |
| leaf_aggregation | true | MulF | MUL | 0 | <div style='text-align: right'>153,092</div>  |
| leaf_aggregation | true | MulFI | MUL | 0 | <div style='text-align: right'>23</div>  |
| leaf_aggregation | true | MulV | MUL | 0 | <div style='text-align: right'>1,333</div>  |
| leaf_aggregation | true | MulVI | MUL | 0 | <div style='text-align: right'>22,849</div>  |
| leaf_aggregation | true | NegE | MUL | 0 | <div style='text-align: right'>252</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>19,068</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>12,987</div>  |
| leaf_aggregation | true | StoreE | STOREW | 0 | <div style='text-align: right'>25,828</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>39,064</div>  |
| leaf_aggregation | true | StoreF | STOREW | 0 | <div style='text-align: right'>33,054</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>99,765</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>257,797</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>284,907</div>  |
| leaf_aggregation | true | StoreV | STOREW | 0 | <div style='text-align: right'>3,315</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>70,717</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 0 | <div style='text-align: right'>3,936</div>  |
| leaf_aggregation | true | SubEF | LOADW | 0 | <div style='text-align: right'>18,468</div>  |
| leaf_aggregation | true | SubEF | SUB | 0 | <div style='text-align: right'>6,156</div>  |
| leaf_aggregation | true | SubEFI | ADD | 0 | <div style='text-align: right'>396</div>  |
| leaf_aggregation | true | SubEI | ADD | 0 | <div style='text-align: right'>504</div>  |
| leaf_aggregation | true | SubV | SUB | 0 | <div style='text-align: right'>85,913</div>  |
| leaf_aggregation | true | SubVI | SUB | 0 | <div style='text-align: right'>2,428</div>  |
| leaf_aggregation | true | SubVIN | SUB | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | true |  | JAL | 1 | <div style='text-align: right'>1</div>  |
| leaf_aggregation | true |  | STOREW | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | AddE | FE4ADD | 1 | <div style='text-align: right'>14,486</div>  |
| leaf_aggregation | true | AddEFFI | LOADW | 1 | <div style='text-align: right'>180</div>  |
| leaf_aggregation | true | AddEFFI | STOREW | 1 | <div style='text-align: right'>540</div>  |
| leaf_aggregation | true | AddEFI | ADD | 1 | <div style='text-align: right'>352</div>  |
| leaf_aggregation | true | AddEI | ADD | 1 | <div style='text-align: right'>31,644</div>  |
| leaf_aggregation | true | AddFI | ADD | 1 | <div style='text-align: right'>68,588</div>  |
| leaf_aggregation | true | AddV | ADD | 1 | <div style='text-align: right'>15,426</div>  |
| leaf_aggregation | true | AddVI | ADD | 1 | <div style='text-align: right'>405,615</div>  |
| leaf_aggregation | true | Alloc | ADD | 1 | <div style='text-align: right'>56,972</div>  |
| leaf_aggregation | true | Alloc | LOADW | 1 | <div style='text-align: right'>56,972</div>  |
| leaf_aggregation | true | Alloc | MUL | 1 | <div style='text-align: right'>33,778</div>  |
| leaf_aggregation | true | AssertEqE | BNE | 1 | <div style='text-align: right'>256</div>  |
| leaf_aggregation | true | AssertEqEI | BNE | 1 | <div style='text-align: right'>4</div>  |
| leaf_aggregation | true | AssertEqF | BNE | 1 | <div style='text-align: right'>9,451</div>  |
| leaf_aggregation | true | AssertEqV | BNE | 1 | <div style='text-align: right'>2,428</div>  |
| leaf_aggregation | true | AssertEqVI | BNE | 1 | <div style='text-align: right'>259</div>  |
| leaf_aggregation | true | CT-InitializePcsConst | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-ReadingProofFromInput | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-VerifierProgram | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-compute-reduced-opening | PHANTOM | 1 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-exp-reverse-bits-len | PHANTOM | 1 | <div style='text-align: right'>7,560</div>  |
| leaf_aggregation | true | CT-poseidon2-hash | PHANTOM | 1 | <div style='text-align: right'>3,192</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-ext | PHANTOM | 1 | <div style='text-align: right'>1,680</div>  |
| leaf_aggregation | true | CT-poseidon2-hash-setup | PHANTOM | 1 | <div style='text-align: right'>177,408</div>  |
| leaf_aggregation | true | CT-single-mat-reduced-opening | PHANTOM | 1 | <div style='text-align: right'>11,676</div>  |
| leaf_aggregation | true | CT-stage-c-build-rounds | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-2-fri-fold | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-3-verify-challenges | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-d-verify-pcs | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-stage-e-verify-constraints | PHANTOM | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | CT-verify-batch | PHANTOM | 1 | <div style='text-align: right'>672</div>  |
| leaf_aggregation | true | CT-verify-batch-ext | PHANTOM | 1 | <div style='text-align: right'>1,680</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast | PHANTOM | 1 | <div style='text-align: right'>4,872</div>  |
| leaf_aggregation | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 1 | <div style='text-align: right'>4,872</div>  |
| leaf_aggregation | true | CT-verify-query | PHANTOM | 1 | <div style='text-align: right'>84</div>  |
| leaf_aggregation | true | DivE | BBE4DIV | 1 | <div style='text-align: right'>6,722</div>  |
| leaf_aggregation | true | DivEIN | BBE4DIV | 1 | <div style='text-align: right'>60</div>  |
| leaf_aggregation | true | DivEIN | STOREW | 1 | <div style='text-align: right'>240</div>  |
| leaf_aggregation | true | DivFIN | DIV | 1 | <div style='text-align: right'>142</div>  |
| leaf_aggregation | true | For | ADD | 1 | <div style='text-align: right'>480,161</div>  |
| leaf_aggregation | true | For | BNE | 1 | <div style='text-align: right'>525,253</div>  |
| leaf_aggregation | true | For | JAL | 1 | <div style='text-align: right'>45,092</div>  |
| leaf_aggregation | true | For | LOADW | 1 | <div style='text-align: right'>2,478</div>  |
| leaf_aggregation | true | For | STOREW | 1 | <div style='text-align: right'>42,614</div>  |
| leaf_aggregation | true | FriMatOpening | FRI_FOLD | 1 | <div style='text-align: right'>5,838</div>  |
| leaf_aggregation | true | HintBitsF | PHANTOM | 1 | <div style='text-align: right'>43</div>  |
| leaf_aggregation | true | HintInputVec | PHANTOM | 1 | <div style='text-align: right'>23,194</div>  |
| leaf_aggregation | true | IfEq | BNE | 1 | <div style='text-align: right'>22,527</div>  |
| leaf_aggregation | true | IfEqI | BNE | 1 | <div style='text-align: right'>182,483</div>  |
| leaf_aggregation | true | IfEqI | JAL | 1 | <div style='text-align: right'>42,765</div>  |
| leaf_aggregation | true | IfNe | BEQ | 1 | <div style='text-align: right'>15,809</div>  |
| leaf_aggregation | true | IfNe | JAL | 1 | <div style='text-align: right'>2</div>  |
| leaf_aggregation | true | IfNeI | BEQ | 1 | <div style='text-align: right'>2,547</div>  |
| leaf_aggregation | true | ImmE | STOREW | 1 | <div style='text-align: right'>4,048</div>  |
| leaf_aggregation | true | ImmF | STOREW | 1 | <div style='text-align: right'>40,036</div>  |
| leaf_aggregation | true | ImmV | STOREW | 1 | <div style='text-align: right'>31,932</div>  |
| leaf_aggregation | true | LoadE | LOADW | 1 | <div style='text-align: right'>26,508</div>  |
| leaf_aggregation | true | LoadE | LOADW2 | 1 | <div style='text-align: right'>70,480</div>  |
| leaf_aggregation | true | LoadF | LOADW | 1 | <div style='text-align: right'>26,621</div>  |
| leaf_aggregation | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>108,927</div>  |
| leaf_aggregation | true | LoadV | LOADW | 1 | <div style='text-align: right'>27,609</div>  |
| leaf_aggregation | true | LoadV | LOADW2 | 1 | <div style='text-align: right'>228,328</div>  |
| leaf_aggregation | true | MulE | BBE4MUL | 1 | <div style='text-align: right'>11,144</div>  |
| leaf_aggregation | true | MulEF | MUL | 1 | <div style='text-align: right'>3,840</div>  |
| leaf_aggregation | true | MulEFI | MUL | 1 | <div style='text-align: right'>608</div>  |
| leaf_aggregation | true | MulEI | BBE4MUL | 1 | <div style='text-align: right'>2,108</div>  |
| leaf_aggregation | true | MulEI | STOREW | 1 | <div style='text-align: right'>8,432</div>  |
| leaf_aggregation | true | MulF | MUL | 1 | <div style='text-align: right'>131,410</div>  |
| leaf_aggregation | true | MulFI | MUL | 1 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | true | MulV | MUL | 1 | <div style='text-align: right'>1,333</div>  |
| leaf_aggregation | true | MulVI | MUL | 1 | <div style='text-align: right'>20,448</div>  |
| leaf_aggregation | true | NegE | MUL | 1 | <div style='text-align: right'>240</div>  |
| leaf_aggregation | true | Poseidon2CompressBabyBear | COMP_POS2 | 1 | <div style='text-align: right'>16,926</div>  |
| leaf_aggregation | true | Poseidon2PermuteBabyBear | PERM_POS2 | 1 | <div style='text-align: right'>12,522</div>  |
| leaf_aggregation | true | StoreE | STOREW | 1 | <div style='text-align: right'>24,472</div>  |
| leaf_aggregation | true | StoreE | STOREW2 | 1 | <div style='text-align: right'>37,040</div>  |
| leaf_aggregation | true | StoreF | STOREW | 1 | <div style='text-align: right'>32,382</div>  |
| leaf_aggregation | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>96,117</div>  |
| leaf_aggregation | true | StoreHintWord | ADD | 1 | <div style='text-align: right'>236,657</div>  |
| leaf_aggregation | true | StoreHintWord | SHINTW | 1 | <div style='text-align: right'>261,184</div>  |
| leaf_aggregation | true | StoreV | STOREW | 1 | <div style='text-align: right'>3,048</div>  |
| leaf_aggregation | true | StoreV | STOREW2 | 1 | <div style='text-align: right'>64,924</div>  |
| leaf_aggregation | true | SubE | FE4SUB | 1 | <div style='text-align: right'>3,662</div>  |
| leaf_aggregation | true | SubEF | LOADW | 1 | <div style='text-align: right'>17,706</div>  |
| leaf_aggregation | true | SubEF | SUB | 1 | <div style='text-align: right'>5,902</div>  |
| leaf_aggregation | true | SubEFI | ADD | 1 | <div style='text-align: right'>388</div>  |
| leaf_aggregation | true | SubEI | ADD | 1 | <div style='text-align: right'>480</div>  |
| leaf_aggregation | true | SubV | SUB | 1 | <div style='text-align: right'>74,572</div>  |
| leaf_aggregation | true | SubVI | SUB | 1 | <div style='text-align: right'>2,335</div>  |
| leaf_aggregation | true | SubVIN | SUB | 1 | <div style='text-align: right'>840</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 0 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 0 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>605,880</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>267,630</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 0 | <div style='text-align: right'>158,145</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 0 | <div style='text-align: right'>114,576</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>7,790</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,243</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>1,469</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 0 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>23,370</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>1,243</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 0 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 0 | <div style='text-align: right'>11,280</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,848</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,092</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 0 | <div style='text-align: right'>1,000,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 0 | <div style='text-align: right'>211,354</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 0 | <div style='text-align: right'>124,891</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 0 | <div style='text-align: right'>135,564</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 0 | <div style='text-align: right'>2,384,100</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 0 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 0 | <div style='text-align: right'>530,970</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 0 | <div style='text-align: right'>13,174,860</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 0 | <div style='text-align: right'>15,917</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 0 | <div style='text-align: right'>1,884,750</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 0 | <div style='text-align: right'>2,575,825</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 0 | <div style='text-align: right'>1,122</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 0 | <div style='text-align: right'>1,111,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 0 | <div style='text-align: right'>39</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>5,980</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>1,430</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 0 | <div style='text-align: right'>845</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 0 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 0 | <div style='text-align: right'>232,829</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 0 | <div style='text-align: right'>58,029</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 0 | <div style='text-align: right'>6,210</div>  |
| leaf_aggregation | PhantomAir | true | CT-InitializePcsConst | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-ReadingProofFromInput | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 0 | <div style='text-align: right'>47,376</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 0 | <div style='text-align: right'>19,152</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 0 | <div style='text-align: right'>11,088</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 0 | <div style='text-align: right'>1,099,728</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 0 | <div style='text-align: right'>73,080</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 0 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 0 | <div style='text-align: right'>11,088</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 0 | <div style='text-align: right'>30,240</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 0 | <div style='text-align: right'>30,240</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 0 | <div style='text-align: right'>504</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>282,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>135,520</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 0 | <div style='text-align: right'>80,080</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,520</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>2,684</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>1,586</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 0 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>10,332</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>913</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 0 | <div style='text-align: right'>260</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 0 | <div style='text-align: right'>4,470</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 0 | <div style='text-align: right'>15,622,560</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 0 | <div style='text-align: right'>13,097,396</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 0 | <div style='text-align: right'>487,000</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 0 | <div style='text-align: right'>473</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 0 | <div style='text-align: right'>559</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 0 | <div style='text-align: right'>105,042</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 0 | <div style='text-align: right'>473</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 0 | <div style='text-align: right'>1,891,658</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 0 | <div style='text-align: right'>803</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>179,080</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>105,820</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 0 | <div style='text-align: right'>11,268,096</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 0 | <div style='text-align: right'>258</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 0 | <div style='text-align: right'>154,662</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 0 | <div style='text-align: right'>472,880</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 0 | <div style='text-align: right'>4,585,004</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 0 | <div style='text-align: right'>459,560</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 0 | <div style='text-align: right'>413,057</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 0 | <div style='text-align: right'>30</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 0 | <div style='text-align: right'>60,628</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 0 | <div style='text-align: right'>172,036</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 0 | <div style='text-align: right'>10,604</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 0 | <div style='text-align: right'>6,266</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 0 | <div style='text-align: right'>13,376</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,682,886</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 0 | <div style='text-align: right'>1,573</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 0 | <div style='text-align: right'>1,364,931</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 0 | <div style='text-align: right'>16,346</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 0 | <div style='text-align: right'>1,150,132</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 0 | <div style='text-align: right'>203,412</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 0 | <div style='text-align: right'>120,198</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 0 | <div style='text-align: right'>3,740</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>3,055,156</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>61,952</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>36,608</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 0 | <div style='text-align: right'>1,178,217</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 0 | <div style='text-align: right'>55,440</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 0 | <div style='text-align: right'>32,760</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 0 | <div style='text-align: right'>21,420</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 0 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>4,622,135</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>836</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>494</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>527</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 0 | <div style='text-align: right'>539</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 0 | <div style='text-align: right'>1,245,621</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 0 | <div style='text-align: right'>15,114</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>10,108,304</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 0 | <div style='text-align: right'>979</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>480,800</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>238,744</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>141,076</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 0 | <div style='text-align: right'>135,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 0 | <div style='text-align: right'>126,000</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 0 | <div style='text-align: right'>21,230</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 0 | <div style='text-align: right'>12,545</div>  |
| leaf_aggregation | Boundary | true | MulEF | MUL | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEFI | MUL | 0 | <div style='text-align: right'>19,320</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEFI | MUL | 0 | <div style='text-align: right'>3,366</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,989</div>  |
| leaf_aggregation | Boundary | true | MulEFI | MUL | 0 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>87,960</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>100,760</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>59,540</div>  |
| leaf_aggregation | Boundary | true | MulEI | BBE4MUL | 0 | <div style='text-align: right'>4,312</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | 0 | <div style='text-align: right'>360,636</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 0 | <div style='text-align: right'>48,323</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 0 | <div style='text-align: right'>28,535</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 0 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 0 | <div style='text-align: right'>4,592,760</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 0 | <div style='text-align: right'>690</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 0 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 0 | <div style='text-align: right'>39,990</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 0 | <div style='text-align: right'>14,641</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 0 | <div style='text-align: right'>685,470</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 0 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 0 | <div style='text-align: right'>7,560</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 0 | <div style='text-align: right'>2,046</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 0 | <div style='text-align: right'>1,209</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 0 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>783,552</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>463,008</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>302,736</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | 0 | <div style='text-align: right'>10,659,012</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>669,834</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>396,630</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>263,636</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 0 | <div style='text-align: right'>7,259,733</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 0 | <div style='text-align: right'>1,058,948</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 0 | <div style='text-align: right'>20,372</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 0 | <div style='text-align: right'>12,038</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 0 | <div style='text-align: right'>284,108</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>1,601,624</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>173,712</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>102,648</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 0 | <div style='text-align: right'>41,624</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 0 | <div style='text-align: right'>1,355,214</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 0 | <div style='text-align: right'>363,594</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>4,090,365</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>447,172</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>265,057</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>177,412</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 0 | <div style='text-align: right'>85,404</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 0 | <div style='text-align: right'>7,733,910</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>11,681,187</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 0 | <div style='text-align: right'>3,133,977</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 0 | <div style='text-align: right'>135,915</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 0 | <div style='text-align: right'>36,465</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>2,899,397</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 0 | <div style='text-align: right'>717,200</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>157,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>136,642</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 0 | <div style='text-align: right'>80,743</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 0 | <div style='text-align: right'>27,896</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 0 | <div style='text-align: right'>757,188</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 0 | <div style='text-align: right'>67,573</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 0 | <div style='text-align: right'>184,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 0 | <div style='text-align: right'>67,573</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 0 | <div style='text-align: right'>79,859</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 0 | <div style='text-align: right'>11,880</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,870</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 0 | <div style='text-align: right'>1,105</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 0 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 0 | <div style='text-align: right'>15,120</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 0 | <div style='text-align: right'>4,180</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 0 | <div style='text-align: right'>2,470</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 0 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 0 | <div style='text-align: right'>2,577,390</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 0 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 0 | <div style='text-align: right'>72,840</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 0 | <div style='text-align: right'>15,147</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 0 | <div style='text-align: right'>27,720</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true |  | JAL | 1 | <div style='text-align: right'>10</div>  |
| leaf_aggregation | Boundary | true |  | JAL | 1 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true |  | STOREW | 1 | <div style='text-align: right'>82</div>  |
| leaf_aggregation | Boundary | true |  | STOREW | 1 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | AddE | FE4ADD | 1 | <div style='text-align: right'>579,440</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddE | FE4ADD | 1 | <div style='text-align: right'>257,334</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddE | FE4ADD | 1 | <div style='text-align: right'>152,061</div>  |
| leaf_aggregation | Boundary | true | AddE | FE4ADD | 1 | <div style='text-align: right'>114,576</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | LOADW | 1 | <div style='text-align: right'>7,380</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | LOADW | 1 | <div style='text-align: right'>1,133</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFFI | LOADW | 1 | <div style='text-align: right'>1,339</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | LOADW | 1 | <div style='text-align: right'>308</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | AddEFFI | STOREW | 1 | <div style='text-align: right'>22,140</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFFI | STOREW | 1 | <div style='text-align: right'>1,133</div>  |
| leaf_aggregation | Boundary | true | AddEFFI | STOREW | 1 | <div style='text-align: right'>924</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEFI | ADD | 1 | <div style='text-align: right'>10,560</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEFI | ADD | 1 | <div style='text-align: right'>1,694</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEFI | ADD | 1 | <div style='text-align: right'>1,001</div>  |
| leaf_aggregation | Boundary | true | AddEFI | ADD | 1 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddEI | ADD | 1 | <div style='text-align: right'>949,320</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AddEI | ADD | 1 | <div style='text-align: right'>200,354</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AddEI | ADD | 1 | <div style='text-align: right'>118,391</div>  |
| leaf_aggregation | Boundary | true | AddEI | ADD | 1 | <div style='text-align: right'>135,564</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddFI | ADD | 1 | <div style='text-align: right'>2,057,640</div>  |
| leaf_aggregation | Boundary | true | AddFI | ADD | 1 | <div style='text-align: right'>253</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddV | ADD | 1 | <div style='text-align: right'>462,780</div>  |
| leaf_aggregation | Boundary | true | AddV | ADD | 1 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | AddVI | ADD | 1 | <div style='text-align: right'>12,168,450</div>  |
| leaf_aggregation | Boundary | true | AddVI | ADD | 1 | <div style='text-align: right'>15,906</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | ADD | 1 | <div style='text-align: right'>1,709,160</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | Alloc | LOADW | 1 | <div style='text-align: right'>2,335,852</div>  |
| leaf_aggregation | Boundary | true | Alloc | LOADW | 1 | <div style='text-align: right'>1,133</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | Alloc | MUL | 1 | <div style='text-align: right'>1,013,340</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Alloc | MUL | 1 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Alloc | MUL | 1 | <div style='text-align: right'>39</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqE | BNE | 1 | <div style='text-align: right'>5,888</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqE | BNE | 1 | <div style='text-align: right'>1,408</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqE | BNE | 1 | <div style='text-align: right'>832</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqEI | BNE | 1 | <div style='text-align: right'>92</div>  |
| leaf_aggregation | AccessAdapter<2> | true | AssertEqEI | BNE | 1 | <div style='text-align: right'>22</div>  |
| leaf_aggregation | AccessAdapter<4> | true | AssertEqEI | BNE | 1 | <div style='text-align: right'>13</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqF | BNE | 1 | <div style='text-align: right'>217,373</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqV | BNE | 1 | <div style='text-align: right'>55,844</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | AssertEqVI | BNE | 1 | <div style='text-align: right'>5,957</div>  |
| leaf_aggregation | PhantomAir | true | CT-InitializePcsConst | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-ReadingProofFromInput | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-VerifierProgram | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-compute-reduced-opening | PHANTOM | 1 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-exp-reverse-bits-len | PHANTOM | 1 | <div style='text-align: right'>45,360</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash | PHANTOM | 1 | <div style='text-align: right'>19,152</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-ext | PHANTOM | 1 | <div style='text-align: right'>10,080</div>  |
| leaf_aggregation | PhantomAir | true | CT-poseidon2-hash-setup | PHANTOM | 1 | <div style='text-align: right'>1,064,448</div>  |
| leaf_aggregation | PhantomAir | true | CT-single-mat-reduced-opening | PHANTOM | 1 | <div style='text-align: right'>70,056</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-c-build-rounds | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-2-fri-fold | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-3-verify-challenges | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-d-verify-pcs | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-stage-e-verify-constraints | PHANTOM | 1 | <div style='text-align: right'>12</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch | PHANTOM | 1 | <div style='text-align: right'>4,032</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-ext | PHANTOM | 1 | <div style='text-align: right'>10,080</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast | PHANTOM | 1 | <div style='text-align: right'>29,232</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-batch-reduce-fast-setup | PHANTOM | 1 | <div style='text-align: right'>29,232</div>  |
| leaf_aggregation | PhantomAir | true | CT-verify-query | PHANTOM | 1 | <div style='text-align: right'>504</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivE | BBE4DIV | 1 | <div style='text-align: right'>268,880</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivE | BBE4DIV | 1 | <div style='text-align: right'>129,910</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivE | BBE4DIV | 1 | <div style='text-align: right'>76,765</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | DivEIN | BBE4DIV | 1 | <div style='text-align: right'>2,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | BBE4DIV | 1 | <div style='text-align: right'>2,574</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | BBE4DIV | 1 | <div style='text-align: right'>1,521</div>  |
| leaf_aggregation | Boundary | true | DivEIN | BBE4DIV | 1 | <div style='text-align: right'>528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | DivEIN | STOREW | 1 | <div style='text-align: right'>9,840</div>  |
| leaf_aggregation | AccessAdapter<2> | true | DivEIN | STOREW | 1 | <div style='text-align: right'>869</div>  |
| leaf_aggregation | AccessAdapter<4> | true | DivEIN | STOREW | 1 | <div style='text-align: right'>247</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | DivFIN | DIV | 1 | <div style='text-align: right'>4,260</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | For | ADD | 1 | <div style='text-align: right'>14,404,830</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | For | BNE | 1 | <div style='text-align: right'>12,080,819</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | For | JAL | 1 | <div style='text-align: right'>450,920</div>  |
| leaf_aggregation | AccessAdapter<2> | true | For | JAL | 1 | <div style='text-align: right'>451</div>  |
| leaf_aggregation | AccessAdapter<4> | true | For | JAL | 1 | <div style='text-align: right'>533</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | LOADW | 1 | <div style='text-align: right'>101,598</div>  |
| leaf_aggregation | Boundary | true | For | LOADW | 1 | <div style='text-align: right'>473</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | For | STOREW | 1 | <div style='text-align: right'>1,747,174</div>  |
| leaf_aggregation | Boundary | true | For | STOREW | 1 | <div style='text-align: right'>781</div>  |
| leaf_aggregation | AccessAdapter<2> | true | FriMatOpening | FRI_FOLD | 1 | <div style='text-align: right'>172,480</div>  |
| leaf_aggregation | AccessAdapter<4> | true | FriMatOpening | FRI_FOLD | 1 | <div style='text-align: right'>101,920</div>  |
| leaf_aggregation | FriMatOpeningAir | true | FriMatOpening | FRI_FOLD | 1 | <div style='text-align: right'>10,913,280</div>  |
| leaf_aggregation | PhantomAir | true | HintBitsF | PHANTOM | 1 | <div style='text-align: right'>258</div>  |
| leaf_aggregation | PhantomAir | true | HintInputVec | PHANTOM | 1 | <div style='text-align: right'>139,164</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEq | BNE | 1 | <div style='text-align: right'>518,121</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfEqI | BNE | 1 | <div style='text-align: right'>4,197,109</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfEqI | JAL | 1 | <div style='text-align: right'>427,650</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNe | BEQ | 1 | <div style='text-align: right'>363,607</div>  |
| leaf_aggregation | <JalNativeAdapterAir,JalCoreAir> | true | IfNe | JAL | 1 | <div style='text-align: right'>20</div>  |
| leaf_aggregation | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | true | IfNeI | BEQ | 1 | <div style='text-align: right'>58,581</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmE | STOREW | 1 | <div style='text-align: right'>165,968</div>  |
| leaf_aggregation | AccessAdapter<2> | true | ImmE | STOREW | 1 | <div style='text-align: right'>9,900</div>  |
| leaf_aggregation | AccessAdapter<4> | true | ImmE | STOREW | 1 | <div style='text-align: right'>5,850</div>  |
| leaf_aggregation | Boundary | true | ImmE | STOREW | 1 | <div style='text-align: right'>13,376</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmF | STOREW | 1 | <div style='text-align: right'>1,641,476</div>  |
| leaf_aggregation | Boundary | true | ImmF | STOREW | 1 | <div style='text-align: right'>1,573</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | ImmV | STOREW | 1 | <div style='text-align: right'>1,309,212</div>  |
| leaf_aggregation | Boundary | true | ImmV | STOREW | 1 | <div style='text-align: right'>16,346</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW | 1 | <div style='text-align: right'>1,086,828</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW | 1 | <div style='text-align: right'>196,372</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW | 1 | <div style='text-align: right'>116,038</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW | 1 | <div style='text-align: right'>3,740</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadE | LOADW2 | 1 | <div style='text-align: right'>2,889,680</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadE | LOADW2 | 1 | <div style='text-align: right'>56,408</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadE | LOADW2 | 1 | <div style='text-align: right'>33,332</div>  |
| leaf_aggregation | Boundary | true | LoadE | LOADW2 | 1 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW | 1 | <div style='text-align: right'>1,091,461</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW | 1 | <div style='text-align: right'>51,744</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW | 1 | <div style='text-align: right'>30,576</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW | 1 | <div style='text-align: right'>19,992</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW | 1 | <div style='text-align: right'>286</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>4,466,007</div>  |
| leaf_aggregation | AccessAdapter<2> | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<4> | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>468</div>  |
| leaf_aggregation | AccessAdapter<8> | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>493</div>  |
| leaf_aggregation | Boundary | true | LoadF | LOADW2 | 1 | <div style='text-align: right'>539</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW | 1 | <div style='text-align: right'>1,131,969</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW | 1 | <div style='text-align: right'>15,136</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | LoadV | LOADW2 | 1 | <div style='text-align: right'>9,361,448</div>  |
| leaf_aggregation | Boundary | true | LoadV | LOADW2 | 1 | <div style='text-align: right'>979</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulE | BBE4MUL | 1 | <div style='text-align: right'>445,760</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulE | BBE4MUL | 1 | <div style='text-align: right'>230,670</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulE | BBE4MUL | 1 | <div style='text-align: right'>136,305</div>  |
| leaf_aggregation | Boundary | true | MulE | BBE4MUL | 1 | <div style='text-align: right'>135,916</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEF | MUL | 1 | <div style='text-align: right'>115,200</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEF | MUL | 1 | <div style='text-align: right'>19,294</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEF | MUL | 1 | <div style='text-align: right'>11,401</div>  |
| leaf_aggregation | Boundary | true | MulEF | MUL | 1 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulEFI | MUL | 1 | <div style='text-align: right'>18,240</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEFI | MUL | 1 | <div style='text-align: right'>3,146</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEFI | MUL | 1 | <div style='text-align: right'>1,859</div>  |
| leaf_aggregation | Boundary | true | MulEFI | MUL | 1 | <div style='text-align: right'>1,364</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | MulEI | BBE4MUL | 1 | <div style='text-align: right'>84,320</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | BBE4MUL | 1 | <div style='text-align: right'>96,954</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | BBE4MUL | 1 | <div style='text-align: right'>57,291</div>  |
| leaf_aggregation | Boundary | true | MulEI | BBE4MUL | 1 | <div style='text-align: right'>4,312</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | MulEI | STOREW | 1 | <div style='text-align: right'>345,712</div>  |
| leaf_aggregation | AccessAdapter<2> | true | MulEI | STOREW | 1 | <div style='text-align: right'>46,321</div>  |
| leaf_aggregation | AccessAdapter<4> | true | MulEI | STOREW | 1 | <div style='text-align: right'>27,352</div>  |
| leaf_aggregation | Boundary | true | MulEI | STOREW | 1 | <div style='text-align: right'>33</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulF | MUL | 1 | <div style='text-align: right'>3,942,300</div>  |
| leaf_aggregation | Boundary | true | MulF | MUL | 1 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulFI | MUL | 1 | <div style='text-align: right'>660</div>  |
| leaf_aggregation | Boundary | true | MulFI | MUL | 1 | <div style='text-align: right'>11</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulV | MUL | 1 | <div style='text-align: right'>39,990</div>  |
| leaf_aggregation | Boundary | true | MulV | MUL | 1 | <div style='text-align: right'>14,641</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | MulVI | MUL | 1 | <div style='text-align: right'>613,440</div>  |
| leaf_aggregation | Boundary | true | MulVI | MUL | 1 | <div style='text-align: right'>77</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | NegE | MUL | 1 | <div style='text-align: right'>7,200</div>  |
| leaf_aggregation | AccessAdapter<2> | true | NegE | MUL | 1 | <div style='text-align: right'>1,914</div>  |
| leaf_aggregation | AccessAdapter<4> | true | NegE | MUL | 1 | <div style='text-align: right'>1,131</div>  |
| leaf_aggregation | Boundary | true | NegE | MUL | 1 | <div style='text-align: right'>792</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2CompressBabyBear | COMP_POS2 | 1 | <div style='text-align: right'>689,304</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2CompressBabyBear | COMP_POS2 | 1 | <div style='text-align: right'>407,316</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2CompressBabyBear | COMP_POS2 | 1 | <div style='text-align: right'>266,322</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2CompressBabyBear | COMP_POS2 | 1 | <div style='text-align: right'>9,461,634</div>  |
| leaf_aggregation | AccessAdapter<2> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 1 | <div style='text-align: right'>644,776</div>  |
| leaf_aggregation | AccessAdapter<4> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 1 | <div style='text-align: right'>382,369</div>  |
| leaf_aggregation | AccessAdapter<8> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 1 | <div style='text-align: right'>254,303</div>  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | true | Poseidon2PermuteBabyBear | PERM_POS2 | 1 | <div style='text-align: right'>6,999,798</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW | 1 | <div style='text-align: right'>1,003,352</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW | 1 | <div style='text-align: right'>18,524</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW | 1 | <div style='text-align: right'>10,946</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW | 1 | <div style='text-align: right'>269,192</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreE | STOREW2 | 1 | <div style='text-align: right'>1,518,640</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreE | STOREW2 | 1 | <div style='text-align: right'>166,320</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreE | STOREW2 | 1 | <div style='text-align: right'>98,280</div>  |
| leaf_aggregation | Boundary | true | StoreE | STOREW2 | 1 | <div style='text-align: right'>37,840</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW | 1 | <div style='text-align: right'>1,327,662</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW | 1 | <div style='text-align: right'>356,202</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>3,940,797</div>  |
| leaf_aggregation | AccessAdapter<2> | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>429,550</div>  |
| leaf_aggregation | AccessAdapter<4> | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>255,190</div>  |
| leaf_aggregation | AccessAdapter<8> | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>170,969</div>  |
| leaf_aggregation | Boundary | true | StoreF | STOREW2 | 1 | <div style='text-align: right'>77,836</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | StoreHintWord | ADD | 1 | <div style='text-align: right'>7,099,710</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreHintWord | SHINTW | 1 | <div style='text-align: right'>10,708,544</div>  |
| leaf_aggregation | Boundary | true | StoreHintWord | SHINTW | 1 | <div style='text-align: right'>2,873,024</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW | 1 | <div style='text-align: right'>124,968</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW | 1 | <div style='text-align: right'>33,528</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | StoreV | STOREW2 | 1 | <div style='text-align: right'>2,661,884</div>  |
| leaf_aggregation | Boundary | true | StoreV | STOREW2 | 1 | <div style='text-align: right'>657,173</div>  |
| leaf_aggregation | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | true | SubE | FE4SUB | 1 | <div style='text-align: right'>146,480</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubE | FE4SUB | 1 | <div style='text-align: right'>127,006</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubE | FE4SUB | 1 | <div style='text-align: right'>75,049</div>  |
| leaf_aggregation | Boundary | true | SubE | FE4SUB | 1 | <div style='text-align: right'>27,896</div>  |
| leaf_aggregation | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | true | SubEF | LOADW | 1 | <div style='text-align: right'>725,946</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | LOADW | 1 | <div style='text-align: right'>64,779</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEF | SUB | 1 | <div style='text-align: right'>177,060</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEF | SUB | 1 | <div style='text-align: right'>64,779</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEF | SUB | 1 | <div style='text-align: right'>76,557</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEFI | ADD | 1 | <div style='text-align: right'>11,640</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEFI | ADD | 1 | <div style='text-align: right'>1,826</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEFI | ADD | 1 | <div style='text-align: right'>1,079</div>  |
| leaf_aggregation | Boundary | true | SubEFI | ADD | 1 | <div style='text-align: right'>220</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubEI | ADD | 1 | <div style='text-align: right'>14,400</div>  |
| leaf_aggregation | AccessAdapter<2> | true | SubEI | ADD | 1 | <div style='text-align: right'>3,960</div>  |
| leaf_aggregation | AccessAdapter<4> | true | SubEI | ADD | 1 | <div style='text-align: right'>2,340</div>  |
| leaf_aggregation | Boundary | true | SubEI | ADD | 1 | <div style='text-align: right'>1,056</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubV | SUB | 1 | <div style='text-align: right'>2,237,160</div>  |
| leaf_aggregation | Boundary | true | SubV | SUB | 1 | <div style='text-align: right'>44</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVI | SUB | 1 | <div style='text-align: right'>70,050</div>  |
| leaf_aggregation | Boundary | true | SubVI | SUB | 1 | <div style='text-align: right'>15,147</div>  |
| leaf_aggregation | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | true | SubVIN | SUB | 1 | <div style='text-align: right'>25,200</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/33be8678e9fdd6c2cc3fb04562271a656b2850fc/bincode-2-2-64cpu-linux-arm64-mimalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/33be8678e9fdd6c2cc3fb04562271a656b2850fc

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11917092664)
