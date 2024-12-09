| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>39,724.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>989,328,588</div>  | <div style='text-align: right'>24,160,031</div>  | <div style='text-align: right'>92,382.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>573,088,718</div>  | <div style='text-align: right'>14,521,305</div>  | <div style='text-align: right'>55,751.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>287,041,547</div>  | <div style='text-align: right'>7,273,463</div>  | <div style='text-align: right'>28,105.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,263,666</div>  | <div style='text-align: right'>3,638,868</div>  | <div style='text-align: right'>73,495.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- |
| <div style='text-align: right'>37,580.0</div>  | <div style='text-align: right'>12,291,525</div>  | <div style='text-align: right'>12,000,219</div>  |

| chip_name | rows_used |
| --- | --- |
| ProgramChip | <div style='text-align: right'>5,874</div>  |
| VmConnectorAir | <div style='text-align: right'>2</div>  |
| Boundary | <div style='text-align: right'>54</div>  |
| Merkle | <div style='text-align: right'>256</div>  |
| AccessAdapter<8> | <div style='text-align: right'>54</div>  |
| <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>3</div>  |
| RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>11</div>  |
| <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>16</div>  |
| <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>111,118</div>  |
| <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>222,227</div>  |
| <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>51</div>  |
| <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>4</div>  |
| <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>333,341</div>  |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>1,000,039</div>  |
| BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| PhantomAir | <div style='text-align: right'>3</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>310</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| dsl_ir | opcode | frequency |
| --- | --- | --- |
|  | ADD | <div style='text-align: right'>1,000,022</div>  |
|  | AND | <div style='text-align: right'>5</div>  |
|  | AUIPC | <div style='text-align: right'>11</div>  |
|  | BEQ | <div style='text-align: right'>111,114</div>  |
|  | BGEU | <div style='text-align: right'>3</div>  |
|  | BLT | <div style='text-align: right'>1</div>  |
|  | BLTU | <div style='text-align: right'>7</div>  |
|  | BNE | <div style='text-align: right'>111,114</div>  |
|  | HINT_STOREW | <div style='text-align: right'>3</div>  |
|  | JAL | <div style='text-align: right'>111,114</div>  |
|  | JALR | <div style='text-align: right'>16</div>  |
|  | LOADBU | <div style='text-align: right'>6</div>  |
|  | LOADW | <div style='text-align: right'>18</div>  |
|  | LUI | <div style='text-align: right'>10</div>  |
|  | OR | <div style='text-align: right'>4</div>  |
|  | PHANTOM | <div style='text-align: right'>3</div>  |
|  | SLL | <div style='text-align: right'>3</div>  |
|  | SLTU | <div style='text-align: right'>333,341</div>  |
|  | SRL | <div style='text-align: right'>1</div>  |
|  | STOREB | <div style='text-align: right'>1</div>  |
|  | STOREW | <div style='text-align: right'>26</div>  |
|  | SUB | <div style='text-align: right'>4</div>  |
|  | XOR | <div style='text-align: right'>4</div>  |

| air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>36,000,792</div>  |
| AccessAdapter<8> |  | ADD | <div style='text-align: right'>68</div>  |
| Boundary |  | ADD | <div style='text-align: right'>160</div>  |
| Merkle |  | ADD | <div style='text-align: right'>3,712</div>  |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180</div>  |
| <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>231</div>  |
| AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>2,888,964</div>  |
| <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>96</div>  |
| <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>32</div>  |
| <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>224</div>  |
| <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>2,888,964</div>  |
| <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>34</div>  |
| Boundary |  | HINT_STOREW | <div style='text-align: right'>80</div>  |
| Merkle |  | HINT_STOREW | <div style='text-align: right'>128</div>  |
| <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>2,000,052</div>  |
| <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>448</div>  |
| <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>240</div>  |
| <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>720</div>  |
| AccessAdapter<8> |  | LOADW | <div style='text-align: right'>68</div>  |
| Boundary |  | LOADW | <div style='text-align: right'>160</div>  |
| Merkle |  | LOADW | <div style='text-align: right'>2,432</div>  |
| <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>180</div>  |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>144</div>  |
| PhantomAir |  | PHANTOM | <div style='text-align: right'>18</div>  |
| <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>159</div>  |
| <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>12,333,617</div>  |
| AccessAdapter<8> |  | SLTU | <div style='text-align: right'>51</div>  |
| Boundary |  | SLTU | <div style='text-align: right'>120</div>  |
| Merkle |  | SLTU | <div style='text-align: right'>3,648</div>  |
| <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>53</div>  |
| <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>40</div>  |
| AccessAdapter<8> |  | STOREB | <div style='text-align: right'>17</div>  |
| Boundary |  | STOREB | <div style='text-align: right'>40</div>  |
| <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>1,040</div>  |
| AccessAdapter<8> |  | STOREW | <div style='text-align: right'>255</div>  |
| Boundary |  | STOREW | <div style='text-align: right'>600</div>  |
| Merkle |  | STOREW | <div style='text-align: right'>1,984</div>  |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>144</div>  |
| <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>144</div>  |
| AccessAdapter<8> |  | BEQ | <div style='text-align: right'>34</div>  |
| Boundary |  | BEQ | <div style='text-align: right'>80</div>  |
| Merkle |  | BEQ | <div style='text-align: right'>192</div>  |
| AccessAdapter<8> |  | BNE | <div style='text-align: right'>17</div>  |
| Boundary |  | BNE | <div style='text-align: right'>40</div>  |
| Merkle |  | BNE | <div style='text-align: right'>64</div>  |

| group | execute_time_ms | fri.log_blowup | halo2_proof_time_ms | halo2_total_cells | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>7,586.0</div>  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>8</div>  |  |  |  |  | <div style='text-align: right'>39,724.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  |  |  |  |  | <div style='text-align: right'>989,328,588</div>  | <div style='text-align: right'>24,160,031</div>  | <div style='text-align: right'>92,382.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  | <div style='text-align: right'>573,088,718</div>  | <div style='text-align: right'>14,521,305</div>  | <div style='text-align: right'>55,751.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>2</div>  |  |  |  |  |  | <div style='text-align: right'>287,041,547</div>  | <div style='text-align: right'>7,273,463</div>  | <div style='text-align: right'>28,105.0</div>  |
| root_verifier |  | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>73,495.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>144,263,666</div>  | <div style='text-align: right'>3,638,868</div>  | <div style='text-align: right'>73,495.0</div>  |
| halo2_verifier |  |  | <div style='text-align: right'>378,432.0</div>  | <div style='text-align: right'>318,500,970.0</div>  |  |  |  |  |  |  |
| halo2_wrapper |  |  | <div style='text-align: right'>86,811.0</div>  |  |  |  |  |  |  |  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>420</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 1 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 1 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 1 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 1 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 2 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 2 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 2 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 2 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 2 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 2 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 2 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 2 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 2 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 3 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 3 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 3 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 3 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 3 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 3 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 3 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 3 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 3 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 4 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 4 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 4 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 4 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 4 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 4 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 4 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 4 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 4 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 4 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 5 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 5 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 5 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 5 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 5 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 5 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 5 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 5 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 5 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 5 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 6 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 6 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 6 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | MemoryMerkleAir<8> | 6 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | AccessAdapterAir<8> | 6 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 6 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| app_proof | ProgramAir | 7 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmConnectorAir | 7 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 7 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| app_proof | MemoryMerkleAir<8> | 7 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | AccessAdapterAir<8> | 7 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| app_proof | RangeTupleCheckerAir<2> | 7 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 7 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 7 | <div style='text-align: right'>2,031,616</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 7 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 7 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 7 | <div style='text-align: right'>10,092,544</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 7 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 7 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 7 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 7 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| app_proof | VariableRangeCheckerAir | 7 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| app_proof | 0 | <div style='text-align: right'>255.0</div>  | <div style='text-align: right'>5,185.0</div>  | <div style='text-align: right'>197,760,980</div>  |
| app_proof | 1 | <div style='text-align: right'>257.0</div>  | <div style='text-align: right'>4,993.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 2 | <div style='text-align: right'>279.0</div>  | <div style='text-align: right'>5,138.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 3 | <div style='text-align: right'>283.0</div>  | <div style='text-align: right'>5,051.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 4 | <div style='text-align: right'>285.0</div>  | <div style='text-align: right'>4,977.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 5 | <div style='text-align: right'>288.0</div>  | <div style='text-align: right'>5,088.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 6 | <div style='text-align: right'>285.0</div>  | <div style='text-align: right'>5,097.0</div>  | <div style='text-align: right'>197,579,202</div>  |
| app_proof | 7 | <div style='text-align: right'>82.0</div>  | <div style='text-align: right'>2,181.0</div>  | <div style='text-align: right'>55,440,402</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>6,664.0</div>  | <div style='text-align: right'>263,366,659</div>  | <div style='text-align: right'>6,444,472</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,576.0</div>  | <div style='text-align: right'>240,580,907</div>  | <div style='text-align: right'>5,870,923</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,643.0</div>  | <div style='text-align: right'>240,546,287</div>  | <div style='text-align: right'>5,867,874</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>6,749.0</div>  | <div style='text-align: right'>244,834,735</div>  | <div style='text-align: right'>5,976,762</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>760,650</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>725,286</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>363,064</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,738</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>53,015</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>62,362</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,508,488</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>139,552</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,233,307</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,070,099</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,457</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>725,112</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>667,100</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>333,888</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,544</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,304,084</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>131,671</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>725,112</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>666,864</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>333,770</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,852</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>50,072</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,544</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,304,084</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>128,622</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,115,694</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,889,991</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,187</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>108,926</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>725,001</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>678,330</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>339,586</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>104,918</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>50,041</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>220,248</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>56,618</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>2,331,436</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>130,011</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>1,137,028</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,929,113</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>334,331</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,129,402</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>10,994</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>21,204</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>36,789</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,196,518</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>33,978</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>23,310</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>139,552</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>293,889</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>637,358</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>271,590</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>19,037</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>368,457</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>462,760</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>354,648</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>321,444</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>107,282</div>  |
| leaf_verifier |  | 1 | ADD | <div style='text-align: right'>1,953,758</div>  |
| leaf_verifier |  | 1 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 1 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 1 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 1 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 1 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 1 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>131,671</div>  |
| leaf_verifier |  | 1 | LOADW | <div style='text-align: right'>281,391</div>  |
| leaf_verifier |  | 1 | LOADW2 | <div style='text-align: right'>552,980</div>  |
| leaf_verifier |  | 1 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 1 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 1 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 1 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>336,726</div>  |
| leaf_verifier |  | 1 | STOREW2 | <div style='text-align: right'>283,560</div>  |
| leaf_verifier |  | 1 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 2 | ADD | <div style='text-align: right'>1,953,758</div>  |
| leaf_verifier |  | 2 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 2 | BBE4MUL | <div style='text-align: right'>18,736</div>  |
| leaf_verifier |  | 2 | BEQ | <div style='text-align: right'>36,633</div>  |
| leaf_verifier |  | 2 | BNE | <div style='text-align: right'>1,079,061</div>  |
| leaf_verifier |  | 2 | COMP_POS2 | <div style='text-align: right'>33,852</div>  |
| leaf_verifier |  | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 2 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>128,622</div>  |
| leaf_verifier |  | 2 | LOADW | <div style='text-align: right'>281,391</div>  |
| leaf_verifier |  | 2 | LOADW2 | <div style='text-align: right'>552,980</div>  |
| leaf_verifier |  | 2 | MUL | <div style='text-align: right'>253,480</div>  |
| leaf_verifier |  | 2 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 2 | PHANTOM | <div style='text-align: right'>317,187</div>  |
| leaf_verifier |  | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 2 | SHINTW | <div style='text-align: right'>435,334</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>336,726</div>  |
| leaf_verifier |  | 2 | STOREW2 | <div style='text-align: right'>283,560</div>  |
| leaf_verifier |  | 2 | SUB | <div style='text-align: right'>96,674</div>  |
| leaf_verifier |  | 3 | ADD | <div style='text-align: right'>1,980,054</div>  |
| leaf_verifier |  | 3 | BBE4DIV | <div style='text-align: right'>9,924</div>  |
| leaf_verifier |  | 3 | BBE4MUL | <div style='text-align: right'>19,300</div>  |
| leaf_verifier |  | 3 | BEQ | <div style='text-align: right'>35,589</div>  |
| leaf_verifier |  | 3 | BNE | <div style='text-align: right'>1,101,439</div>  |
| leaf_verifier |  | 3 | COMP_POS2 | <div style='text-align: right'>32,897</div>  |
| leaf_verifier |  | 3 | DIV | <div style='text-align: right'>186</div>  |
| leaf_verifier |  | 3 | FE4ADD | <div style='text-align: right'>20,853</div>  |
| leaf_verifier |  | 3 | FE4SUB | <div style='text-align: right'>6,541</div>  |
| leaf_verifier |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>8,148</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>130,011</div>  |
| leaf_verifier |  | 3 | LOADW | <div style='text-align: right'>282,752</div>  |
| leaf_verifier |  | 3 | LOADW2 | <div style='text-align: right'>575,892</div>  |
| leaf_verifier |  | 3 | MUL | <div style='text-align: right'>253,216</div>  |
| leaf_verifier |  | 3 | PERM_POS2 | <div style='text-align: right'>17,144</div>  |
| leaf_verifier |  | 3 | PHANTOM | <div style='text-align: right'>334,331</div>  |
| leaf_verifier |  | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 3 | SHINTW | <div style='text-align: right'>436,683</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>339,330</div>  |
| leaf_verifier |  | 3 | STOREW2 | <div style='text-align: right'>294,456</div>  |
| leaf_verifier |  | 3 | SUB | <div style='text-align: right'>97,980</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>63,882,060</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>378,312</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>223,548</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,047</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>439,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>207,460</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>122,590</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>848,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>484,990</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>286,585</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>846,147</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>27,519,914</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>2,640</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>1,560</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>18,993,702</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>932,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>395,296</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>233,584</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>234,036</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>138,294</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>1,395,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>693</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>819</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>12,049,449</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>512,105</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>243,919</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>26,131,678</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>8,147,700</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>45,826</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>27,118</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,029,875</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>611,299</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>408,629</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>10,641,683</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>2,210,742</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>18,973,160</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,090,360</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>14,540,568</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>116,600</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>67,041</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>1,368,543</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>13,179,204</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>847,055</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>503,269</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>237,677</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>1,410,398</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>3,218,460</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>101,706</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>120,198</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | ADD | <div style='text-align: right'>58,612,740</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | ADD | <div style='text-align: right'>325,402</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | ADD | <div style='text-align: right'>192,283</div>  |
| leaf_verifier | Boundary |  | 1 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4MUL | <div style='text-align: right'>424,138</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4MUL | <div style='text-align: right'>250,627</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BNE | <div style='text-align: right'>24,818,403</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 1 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 1 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>1,316,710</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 1 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 1 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 1 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW | <div style='text-align: right'>97,559</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW | <div style='text-align: right'>55,926</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>1,342,759</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 1 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | ADD | <div style='text-align: right'>58,612,740</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | ADD | <div style='text-align: right'>324,104</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | ADD | <div style='text-align: right'>191,516</div>  |
| leaf_verifier | Boundary |  | 2 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4MUL | <div style='text-align: right'>749,440</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4MUL | <div style='text-align: right'>422,840</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4MUL | <div style='text-align: right'>249,860</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BEQ | <div style='text-align: right'>842,559</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BNE | <div style='text-align: right'>24,818,403</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | COMP_POS2 | <div style='text-align: right'>1,378,608</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | COMP_POS2 | <div style='text-align: right'>814,632</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | COMP_POS2 | <div style='text-align: right'>532,644</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | COMP_POS2 | <div style='text-align: right'>18,923,268</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 2 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 2 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>1,286,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW | <div style='text-align: right'>11,537,031</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW2 | <div style='text-align: right'>22,672,180</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | MUL | <div style='text-align: right'>7,604,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 2 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 2 | PHANTOM | <div style='text-align: right'>1,903,122</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 2 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | SHINTW | <div style='text-align: right'>17,848,694</div>  |
| leaf_verifier | Boundary |  | 2 | SHINTW | <div style='text-align: right'>4,788,674</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>13,805,766</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW | <div style='text-align: right'>97,559</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW | <div style='text-align: right'>55,926</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>1,342,759</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW2 | <div style='text-align: right'>11,625,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW2 | <div style='text-align: right'>686,114</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW2 | <div style='text-align: right'>407,628</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW2 | <div style='text-align: right'>191,930</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW2 | <div style='text-align: right'>1,345,124</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | SUB | <div style='text-align: right'>2,900,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 2 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | ADD | <div style='text-align: right'>59,401,620</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | ADD | <div style='text-align: right'>340,758</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | ADD | <div style='text-align: right'>201,357</div>  |
| leaf_verifier | Boundary |  | 3 | ADD | <div style='text-align: right'>136,983</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4DIV | <div style='text-align: right'>396,960</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4DIV | <div style='text-align: right'>184,514</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4DIV | <div style='text-align: right'>109,031</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4MUL | <div style='text-align: right'>772,000</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4MUL | <div style='text-align: right'>442,486</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4MUL | <div style='text-align: right'>261,469</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BEQ | <div style='text-align: right'>818,547</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BNE | <div style='text-align: right'>25,333,097</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BNE | <div style='text-align: right'>2,552</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BNE | <div style='text-align: right'>1,508</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | COMP_POS2 | <div style='text-align: right'>1,341,252</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | COMP_POS2 | <div style='text-align: right'>792,558</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | COMP_POS2 | <div style='text-align: right'>518,211</div>  |
| leaf_verifier | Boundary |  | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | COMP_POS2 | <div style='text-align: right'>18,389,423</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | DIV | <div style='text-align: right'>5,580</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4ADD | <div style='text-align: right'>834,120</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4ADD | <div style='text-align: right'>362,912</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4ADD | <div style='text-align: right'>214,448</div>  |
| leaf_verifier | Boundary |  | 3 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4SUB | <div style='text-align: right'>261,640</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4SUB | <div style='text-align: right'>225,918</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4SUB | <div style='text-align: right'>133,497</div>  |
| leaf_verifier | Boundary |  | 3 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>229,944</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>135,876</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,095,872</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>1,300,110</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | JAL | <div style='text-align: right'>605</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | JAL | <div style='text-align: right'>715</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW | <div style='text-align: right'>11,592,832</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW | <div style='text-align: right'>474,584</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW | <div style='text-align: right'>228,280</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW | <div style='text-align: right'>40,171</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW2 | <div style='text-align: right'>23,611,572</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW2 | <div style='text-align: right'>111,628</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW2 | <div style='text-align: right'>65,962</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW2 | <div style='text-align: right'>969</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW2 | <div style='text-align: right'>1,386</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | MUL | <div style='text-align: right'>7,596,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | MUL | <div style='text-align: right'>43,175</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | MUL | <div style='text-align: right'>25,558</div>  |
| leaf_verifier | Boundary |  | 3 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | PERM_POS2 | <div style='text-align: right'>943,789</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | PERM_POS2 | <div style='text-align: right'>560,430</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | PERM_POS2 | <div style='text-align: right'>373,592</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | PERM_POS2 | <div style='text-align: right'>9,583,496</div>  |
| leaf_verifier | PhantomAir |  | 3 | PHANTOM | <div style='text-align: right'>2,005,986</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 3 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | SHINTW | <div style='text-align: right'>17,904,003</div>  |
| leaf_verifier | Boundary |  | 3 | SHINTW | <div style='text-align: right'>4,803,513</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>13,912,530</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW | <div style='text-align: right'>108,273</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW | <div style='text-align: right'>62,244</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>1,331,055</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW2 | <div style='text-align: right'>12,072,696</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW2 | <div style='text-align: right'>746,207</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW2 | <div style='text-align: right'>443,677</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW2 | <div style='text-align: right'>208,369</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW2 | <div style='text-align: right'>1,340,680</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | SUB | <div style='text-align: right'>2,939,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | SUB | <div style='text-align: right'>90,530</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | SUB | <div style='text-align: right'>106,990</div>  |
| leaf_verifier | Boundary |  | 3 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,502.0</div>  | <div style='text-align: right'>21,714.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,484.0</div>  | <div style='text-align: right'>21,698.0</div>  | <div style='text-align: right'>618,203,608</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,500.0</div>  | <div style='text-align: right'>21,221.0</div>  | <div style='text-align: right'>615,320,024</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>1,510.0</div>  | <div style='text-align: right'>21,753.0</div>  | <div style='text-align: right'>615,320,024</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,640.0</div>  | <div style='text-align: right'>286,574,441</div>  | <div style='text-align: right'>7,263,151</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>7,591.0</div>  | <div style='text-align: right'>286,514,277</div>  | <div style='text-align: right'>7,258,154</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>7,784.0</div>  | <div style='text-align: right'>287,041,547</div>  | <div style='text-align: right'>7,273,463</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>771,879</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>756,512</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>378,592</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,896</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,094</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>78,518</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,972,624</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>182,210</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,385,009</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,659</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>771,879</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>756,352</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>378,512</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>109,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>52,178</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>78,468</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,970,046</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>179,895</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,384,955</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,228,575</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>157,050</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>773,363</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>757,784</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>379,144</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,182</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,716</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,982,242</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>179,180</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,495</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,527</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | ADD | <div style='text-align: right'>2,371,722</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>33,458</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BNE | <div style='text-align: right'>1,347,484</div>  |
| internal_verifier_height_0 |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4ADD | <div style='text-align: right'>24,936</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>182,210</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW | <div style='text-align: right'>308,457</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW2 | <div style='text-align: right'>768,020</div>  |
| internal_verifier_height_0 |  | 0 | 0 | MUL | <div style='text-align: right'>419,230</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,402</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>344,244</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW2 | <div style='text-align: right'>343,418</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SUB | <div style='text-align: right'>180,944</div>  |
| internal_verifier_height_0 |  | 0 | 1 | ADD | <div style='text-align: right'>2,370,656</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>33,408</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BNE | <div style='text-align: right'>1,347,430</div>  |
| internal_verifier_height_0 |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4ADD | <div style='text-align: right'>24,936</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>179,895</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW | <div style='text-align: right'>308,457</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW2 | <div style='text-align: right'>768,020</div>  |
| internal_verifier_height_0 |  | 0 | 1 | MUL | <div style='text-align: right'>418,222</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,486</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>344,160</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW2 | <div style='text-align: right'>343,418</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SUB | <div style='text-align: right'>180,440</div>  |
| internal_verifier_height_1 |  | 1 | 2 | ADD | <div style='text-align: right'>2,377,140</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>33,624</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BEQ | <div style='text-align: right'>37,609</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BNE | <div style='text-align: right'>1,348,886</div>  |
| internal_verifier_height_1 |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,776</div>  |
| internal_verifier_height_1 |  | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4ADD | <div style='text-align: right'>24,968</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>179,180</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW | <div style='text-align: right'>308,805</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW2 | <div style='text-align: right'>768,304</div>  |
| internal_verifier_height_1 |  | 1 | 2 | MUL | <div style='text-align: right'>422,086</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,406</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PHANTOM | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SHINTW | <div style='text-align: right'>465,308</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>344,492</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW2 | <div style='text-align: right'>343,618</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SUB | <div style='text-align: right'>182,288</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | ADD | <div style='text-align: right'>71,151,660</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | ADD | <div style='text-align: right'>478,258</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | ADD | <div style='text-align: right'>282,607</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | ADD | <div style='text-align: right'>166,287</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,338,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>553,586</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>327,119</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BNE | <div style='text-align: right'>30,992,132</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>997,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>443,212</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>261,898</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>252,450</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>149,175</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>1,822,100</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW | <div style='text-align: right'>499,191</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW | <div style='text-align: right'>228,254</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | MUL | <div style='text-align: right'>12,576,900</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | MUL | <div style='text-align: right'>56,826</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | MUL | <div style='text-align: right'>33,618</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,124</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,727,718</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 0 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 0 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>14,114,004</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW | <div style='text-align: right'>115,588</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW | <div style='text-align: right'>65,312</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>1,280,653</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>889,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>220,524</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | SUB | <div style='text-align: right'>5,428,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | ADD | <div style='text-align: right'>71,119,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | ADD | <div style='text-align: right'>477,378</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | ADD | <div style='text-align: right'>282,087</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | ADD | <div style='text-align: right'>166,287</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,336,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>552,706</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>326,599</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BNE | <div style='text-align: right'>30,990,890</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>997,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>443,212</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>261,898</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>252,450</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>149,175</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>1,798,950</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW | <div style='text-align: right'>499,191</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW | <div style='text-align: right'>228,254</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | MUL | <div style='text-align: right'>12,546,660</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | MUL | <div style='text-align: right'>56,826</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | MUL | <div style='text-align: right'>33,618</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>371,552</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,774,674</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 1 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 1 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SHINTW | <div style='text-align: right'>5,109,720</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>14,110,560</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW | <div style='text-align: right'>115,588</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW | <div style='text-align: right'>65,312</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>1,280,653</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>889,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>527,878</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>221,952</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW2 | <div style='text-align: right'>1,571,394</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | SUB | <div style='text-align: right'>5,413,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | ADD | <div style='text-align: right'>71,314,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | ADD | <div style='text-align: right'>478,302</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | ADD | <div style='text-align: right'>282,633</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | ADD | <div style='text-align: right'>167,343</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>251,658</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>148,707</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,344,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>556,116</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>328,614</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BEQ | <div style='text-align: right'>865,007</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BNE | <div style='text-align: right'>31,024,378</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,441,440</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>851,760</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>556,920</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,439,784</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>998,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>443,322</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>261,963</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4ADD | <div style='text-align: right'>106,172</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>252,296</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>149,084</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4SUB | <div style='text-align: right'>24,860</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>1,791,800</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | JAL | <div style='text-align: right'>682</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | JAL | <div style='text-align: right'>806</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW | <div style='text-align: right'>12,661,005</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW | <div style='text-align: right'>500,214</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW | <div style='text-align: right'>228,995</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW | <div style='text-align: right'>22,759</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>31,500,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | MUL | <div style='text-align: right'>12,662,580</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | MUL | <div style='text-align: right'>57,090</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | MUL | <div style='text-align: right'>33,774</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>940,588</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>557,440</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>370,192</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,729,954</div>  |
| internal_verifier_height_1 | PhantomAir |  | 1 | 2 | PHANTOM | <div style='text-align: right'>2,120,910</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | 2 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | SHINTW | <div style='text-align: right'>19,077,628</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SHINTW | <div style='text-align: right'>5,118,388</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>14,124,172</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW | <div style='text-align: right'>116,457</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW | <div style='text-align: right'>65,689</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>1,281,533</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>14,088,338</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>890,736</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>527,982</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>220,592</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW2 | <div style='text-align: right'>1,578,786</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | SUB | <div style='text-align: right'>5,468,640</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | height | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramAir | 0 | 0 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramAir | 0 | 1 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramAir | 1 | 2 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_1 | VolatileBoundaryAir | 1 | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<2> | 1 | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<4> | 1 | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | AccessAdapterAir<8> | 1 | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | height | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,755.0</div>  | <div style='text-align: right'>26,121.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,708.0</div>  | <div style='text-align: right'>26,167.0</div>  | <div style='text-align: right'>760,809,944</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,745.0</div>  | <div style='text-align: right'>26,360.0</div>  | <div style='text-align: right'>760,809,944</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| root_verifier | ProgramChip | <div style='text-align: right'>157,323</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>412,533</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>390,136</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>195,194</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,602</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,103</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,358</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,491,232</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>90,909</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,273</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,908</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>176,745</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| root_verifier |  | ADD | <div style='text-align: right'>1,188,669</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,422</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,812</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>18,805</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>674,468</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>17,400</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,484</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,640</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>90,909</div>  |
| root_verifier |  | LOADW | <div style='text-align: right'>154,542</div>  |
| root_verifier |  | LOADW2 | <div style='text-align: right'>384,152</div>  |
| root_verifier |  | MUL | <div style='text-align: right'>211,055</div>  |
| root_verifier |  | PERM_POS2 | <div style='text-align: right'>8,703</div>  |
| root_verifier |  | PHANTOM | <div style='text-align: right'>176,745</div>  |
| root_verifier |  | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier |  | SHINTW | <div style='text-align: right'>232,680</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>172,725</div>  |
| root_verifier |  | STOREW2 | <div style='text-align: right'>171,809</div>  |
| root_verifier |  | SUB | <div style='text-align: right'>91,144</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>35,660,070</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>226,050</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>133,575</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>167,255</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>256,880</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,170</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,555</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>672,480</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>311,344</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>183,976</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>432,515</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>15,512,764</div>  |
| root_verifier | AccessAdapter<2> |  | BNE | <div style='text-align: right'>1,298</div>  |
| root_verifier | AccessAdapter<4> |  | BNE | <div style='text-align: right'>767</div>  |
| root_verifier | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>722,172</div>  |
| root_verifier | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>426,738</div>  |
| root_verifier | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>279,021</div>  |
| root_verifier | Boundary |  | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>9,726,600</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary |  | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>499,360</div>  |
| root_verifier | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>242,418</div>  |
| root_verifier | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>143,247</div>  |
| root_verifier | Boundary |  | FE4ADD | <div style='text-align: right'>106,172</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>145,600</div>  |
| root_verifier | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>132,154</div>  |
| root_verifier | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>78,091</div>  |
| root_verifier | Boundary |  | FE4SUB | <div style='text-align: right'>24,860</div>  |
| root_verifier | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>141,196</div>  |
| root_verifier | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>83,434</div>  |
| root_verifier | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>6,978,048</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>909,090</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,336,222</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>249,557</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>113,958</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,407</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,750,232</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>62,788</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>37,102</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,815</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,331,650</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>27,929</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>16,523</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>470,294</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>278,720</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,096</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,864,977</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,060,470</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,539,880</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,559,480</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>7,081,725</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>55,693</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>31,564</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>661,001</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>7,044,169</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>445,368</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>263,991</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>110,296</div>  |
| root_verifier | Boundary |  | STOREW2 | <div style='text-align: right'>789,393</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>2,734,320</div>  |
| root_verifier | AccessAdapter<2> |  | SUB | <div style='text-align: right'>58,619</div>  |
| root_verifier | AccessAdapter<4> |  | SUB | <div style='text-align: right'>69,277</div>  |
| root_verifier | Boundary |  | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| root_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<2> | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | AccessAdapterAir<4> | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | AccessAdapterAir<8> | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/81be44698ac2db804d35fdb9122dfd1f9cae66e9/fib_e2e-2-2-2-2-1000000-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/81be44698ac2db804d35fdb9122dfd1f9cae66e9

Max Segment Length: 1000000

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12245942363)
