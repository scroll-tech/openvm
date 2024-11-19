| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| bincode_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  | <div style='text-align: right'>90,560.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| bincode_program | true | <div style='text-align: right'>66,346.0</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  |

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
| bincode_program | <div style='text-align: right'>8.0</div>  | <div style='text-align: right'>26,864.0</div>  | <div style='text-align: right'>16,737.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>178.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>99,472,773</div>  | <div style='text-align: right'>12,445,036</div>  | <div style='text-align: right'>90,560.0</div>  |

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

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bincode_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| bincode_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>16,777,216</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>27,262,976</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>21,495,808</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | PhantomAir | 0 | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>486,539,264</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>20,185,088</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>469,762,048</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>58,195,968</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>77,594,624</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>92,274,688</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>32,505,856</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>33,554,432</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>12,845,056</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>4,096</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>142,336</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>1,024</div>  |
| bincode_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>657,457,152</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| bincode_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| bincode_program | ProgramAir | 1 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| bincode_program | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| bincode_program | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>3,407,872</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | AccessAdapterAir<8> | 1 | <div style='text-align: right'>2,686,976</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | PhantomAir | 1 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 1 | <div style='text-align: right'>3,440,640</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 1 | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 1 | <div style='text-align: right'>14,548,992</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| bincode_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>11,534,336</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>4,063,232</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 1 | <div style='text-align: right'>4,194,304</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| bincode_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 1 | <div style='text-align: right'>1,605,632</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 1 | <div style='text-align: right'>56,832</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>512</div>  |
| bincode_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 1 | <div style='text-align: right'>35,584</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>256</div>  |
| bincode_program | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>82,182,144</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>131,072</div>  |
| bincode_program | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| bincode_program | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| bincode_program | VariableRangeCheckerAir | 1 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| bincode_program | 0 | <div style='text-align: right'>8,901.0</div>  | <div style='text-align: right'>45,419.0</div>  | <div style='text-align: right'>2,036,374,560</div>  |
| bincode_program | 1 | <div style='text-align: right'>1,223.0</div>  | <div style='text-align: right'>8,153.0</div>  | <div style='text-align: right'>341,575,968</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/67bc0c0be453e2683f5f51f5991d951085ca85b8/bincode-2-2-64cpu-linux-arm64-mimalloc-bincode_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/67bc0c0be453e2683f5f51f5991d951085ca85b8

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11906265034)
