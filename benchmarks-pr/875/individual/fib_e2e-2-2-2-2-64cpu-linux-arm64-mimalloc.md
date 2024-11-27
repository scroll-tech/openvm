| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>987,166,567</div>  | <div style='text-align: right'>24,120,854</div>  | <div style='text-align: right'>95,307.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,024,798</div>  | <div style='text-align: right'>3,635,834</div>  | <div style='text-align: right'>73,299.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>572,100,061</div>  | <div style='text-align: right'>14,508,349</div>  | <div style='text-align: right'>55,896.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>286,742,205</div>  | <div style='text-align: right'>7,270,238</div>  | <div style='text-align: right'>28,000.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group_name | execute_time_ms | fri.log_blowup | num_segments |
| --- | --- | --- | --- |
| fibonacci_continuations_program | <div style='text-align: right'>7,774.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |

| air_name | group_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ProgramAir | fibonacci_continuations_program | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| PhantomAir | fibonacci_continuations_program | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>420</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | fibonacci_continuations_program | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | fibonacci_continuations_program | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | fibonacci_continuations_program | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | fibonacci_continuations_program | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 1 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 1 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 1 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 1 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 1 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 2 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 2 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 2 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 2 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 2 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 2 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 2 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 2 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 2 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 3 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 3 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 3 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 3 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 3 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 3 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 3 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 3 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 3 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 4 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 4 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 4 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 4 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 4 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 4 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 4 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 4 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 4 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 5 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 5 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 5 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 5 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 5 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 5 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 5 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 5 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 5 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 6 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 6 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| PhantomAir | fibonacci_continuations_program | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 6 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| ProgramAir | fibonacci_continuations_program | 7 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| VmConnectorAir | fibonacci_continuations_program | 7 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| MemoryMerkleAir<8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| AccessAdapterAir<8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| PhantomAir | fibonacci_continuations_program | 7 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>10,092,544</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>131,072</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | fibonacci_continuations_program | 7 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | fibonacci_continuations_program | 7 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | fibonacci_continuations_program | 7 | <div style='text-align: right'>2,031,616</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>32,768</div>  |
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | fibonacci_continuations_program | 7 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| Poseidon2VmAir<BabyBearParameters> | fibonacci_continuations_program | 7 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| BitwiseOperationLookupAir<8> | fibonacci_continuations_program | 7 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| RangeTupleCheckerAir<2> | fibonacci_continuations_program | 7 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| VariableRangeCheckerAir | fibonacci_continuations_program | 7 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group_name | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| fibonacci_continuations_program | 0 | <div style='text-align: right'>263.0</div>  | <div style='text-align: right'>4,979.0</div>  | <div style='text-align: right'>196,581,332</div>  |
| fibonacci_continuations_program | 1 | <div style='text-align: right'>297.0</div>  | <div style='text-align: right'>4,585.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 2 | <div style='text-align: right'>254.0</div>  | <div style='text-align: right'>4,585.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 3 | <div style='text-align: right'>298.0</div>  | <div style='text-align: right'>4,637.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 4 | <div style='text-align: right'>268.0</div>  | <div style='text-align: right'>4,614.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 5 | <div style='text-align: right'>282.0</div>  | <div style='text-align: right'>4,620.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 6 | <div style='text-align: right'>264.0</div>  | <div style='text-align: right'>4,682.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuations_program | 7 | <div style='text-align: right'>88.0</div>  | <div style='text-align: right'>2,035.0</div>  | <div style='text-align: right'>54,260,754</div>  |

| group | fri.log_blowup | halo2_keygen_time_ms | halo2_proof_time_ms | halo2_total_cells | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | <div style='text-align: right'>2</div>  |  |  |  |  |  | <div style='text-align: right'>987,166,567</div>  | <div style='text-align: right'>24,120,854</div>  | <div style='text-align: right'>95,307.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  |  |  |  | <div style='text-align: right'>73,299.0</div>  | <div style='text-align: right'>382,765,848</div>  | <div style='text-align: right'>144,024,798</div>  | <div style='text-align: right'>3,635,834</div>  | <div style='text-align: right'>73,299.0</div>  |
| static_verifier |  | <div style='text-align: right'>590,309.0</div>  | <div style='text-align: right'>395,496.0</div>  | <div style='text-align: right'>369,429,493.0</div>  |  |  |  |  |  |
| internal_verifier_height_0 |  |  |  |  |  |  | <div style='text-align: right'>572,100,061</div>  | <div style='text-align: right'>14,508,349</div>  | <div style='text-align: right'>55,896.0</div>  |
| internal_verifier_height_1 |  |  |  |  |  |  | <div style='text-align: right'>286,742,205</div>  | <div style='text-align: right'>7,270,238</div>  | <div style='text-align: right'>28,000.0</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>6,856.0</div>  | <div style='text-align: right'>262,785,800</div>  | <div style='text-align: right'>6,432,894</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,555.0</div>  | <div style='text-align: right'>240,020,683</div>  | <div style='text-align: right'>5,858,377</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,580.0</div>  | <div style='text-align: right'>240,051,023</div>  | <div style='text-align: right'>5,861,208</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>6,613.0</div>  | <div style='text-align: right'>244,309,061</div>  | <div style='text-align: right'>5,968,375</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>107,029</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>759,297</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>721,616</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>361,228</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,562</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,373</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,068,253</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,231,964</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>137,631</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,502,348</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>62,206</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>52,927</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>107,029</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>723,759</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>664,224</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>332,448</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,676</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,888,209</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,114,364</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>128,023</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,298,562</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,452</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>49,984</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>107,029</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>723,759</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>664,340</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>332,506</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,676</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,888,209</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,114,364</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>130,854</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,298,562</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,452</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>49,984</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>107,029</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>723,648</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>675,388</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>338,072</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>104,742</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>334,247</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,927,311</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>1,135,694</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>130,768</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>2,325,716</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>56,502</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>220,248</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>49,953</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 0 | FE4ADD | <div style='text-align: right'>22,848</div>  |
| leaf_verifier | AddEFFI | 0 | LOADW | <div style='text-align: right'>340</div>  |
| leaf_verifier | AddEFFI | 0 | STOREW | <div style='text-align: right'>1,020</div>  |
| leaf_verifier | AddEFI | 0 | ADD | <div style='text-align: right'>484</div>  |
| leaf_verifier | AddEI | 0 | ADD | <div style='text-align: right'>56,440</div>  |
| leaf_verifier | AddF | 0 | ADD | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | AddFI | 0 | ADD | <div style='text-align: right'>79,683</div>  |
| leaf_verifier | AddV | 0 | ADD | <div style='text-align: right'>28,494</div>  |
| leaf_verifier | AddVI | 0 | ADD | <div style='text-align: right'>644,581</div>  |
| leaf_verifier | Alloc | 0 | ADD | <div style='text-align: right'>110,013</div>  |
| leaf_verifier | Alloc | 0 | LOADW | <div style='text-align: right'>110,013</div>  |
| leaf_verifier | Alloc | 0 | MUL | <div style='text-align: right'>65,578</div>  |
| leaf_verifier | AssertEqE | 0 | BNE | <div style='text-align: right'>472</div>  |
| leaf_verifier | AssertEqEI | 0 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 0 | BNE | <div style='text-align: right'>21,585</div>  |
| leaf_verifier | AssertEqFI | 0 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 0 | BNE | <div style='text-align: right'>2,080</div>  |
| leaf_verifier | AssertEqVI | 0 | BNE | <div style='text-align: right'>408</div>  |
| leaf_verifier | AssertNeVI | 0 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 0 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 0 | PHANTOM | <div style='text-align: right'>11,760</div>  |
| leaf_verifier | CT-poseidon2-hash | 0 | PHANTOM | <div style='text-align: right'>6,636</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 0 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 0 | PHANTOM | <div style='text-align: right'>257,544</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 0 | PHANTOM | <div style='text-align: right'>18,312</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 0 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 0 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 0 | PHANTOM | <div style='text-align: right'>9,996</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | <div style='text-align: right'>9,996</div>  |
| leaf_verifier | CT-verify-query | 0 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 0 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 0 | BBE4DIV | <div style='text-align: right'>10,904</div>  |
| leaf_verifier | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>90</div>  |
| leaf_verifier | DivEIN | 0 | STOREW | <div style='text-align: right'>360</div>  |
| leaf_verifier | DivFIN | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier | For | 0 | ADD | <div style='text-align: right'>785,937</div>  |
| leaf_verifier | For | 0 | BNE | <div style='text-align: right'>869,212</div>  |
| leaf_verifier | For | 0 | JAL | <div style='text-align: right'>83,275</div>  |
| leaf_verifier | For | 0 | LOADW | <div style='text-align: right'>5,082</div>  |
| leaf_verifier | For | 0 | STOREW | <div style='text-align: right'>78,193</div>  |
| leaf_verifier | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier | HintBitsF | 0 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 0 | PHANTOM | <div style='text-align: right'>44,435</div>  |
| leaf_verifier | IfEq | 0 | BNE | <div style='text-align: right'>44,088</div>  |
| leaf_verifier | IfEqI | 0 | BNE | <div style='text-align: right'>257,405</div>  |
| leaf_verifier | IfEqI | 0 | JAL | <div style='text-align: right'>54,351</div>  |
| leaf_verifier | IfNe | 0 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 0 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 0 | BEQ | <div style='text-align: right'>5,170</div>  |
| leaf_verifier | ImmE | 0 | STOREW | <div style='text-align: right'>6,416</div>  |
| leaf_verifier | ImmF | 0 | STOREW | <div style='text-align: right'>81,290</div>  |
| leaf_verifier | ImmV | 0 | STOREW | <div style='text-align: right'>55,149</div>  |
| leaf_verifier | LoadE | 0 | LOADW | <div style='text-align: right'>43,992</div>  |
| leaf_verifier | LoadE | 0 | LOADW2 | <div style='text-align: right'>117,408</div>  |
| leaf_verifier | LoadF | 0 | LOADW | <div style='text-align: right'>55,964</div>  |
| leaf_verifier | LoadF | 0 | LOADW2 | <div style='text-align: right'>160,620</div>  |
| leaf_verifier | LoadV | 0 | LOADW | <div style='text-align: right'>50,418</div>  |
| leaf_verifier | LoadV | 0 | LOADW2 | <div style='text-align: right'>358,874</div>  |
| leaf_verifier | MulE | 0 | BBE4MUL | <div style='text-align: right'>18,844</div>  |
| leaf_verifier | MulEF | 0 | MUL | <div style='text-align: right'>7,440</div>  |
| leaf_verifier | MulEFI | 0 | MUL | <div style='text-align: right'>796</div>  |
| leaf_verifier | MulEI | 0 | BBE4MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulEI | 0 | STOREW | <div style='text-align: right'>10,664</div>  |
| leaf_verifier | MulF | 0 | MUL | <div style='text-align: right'>154,842</div>  |
| leaf_verifier | MulFI | 0 | MUL | <div style='text-align: right'>2,700</div>  |
| leaf_verifier | MulVI | 0 | MUL | <div style='text-align: right'>39,062</div>  |
| leaf_verifier | NegE | 0 | MUL | <div style='text-align: right'>332</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>33,894</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>19,033</div>  |
| leaf_verifier | Publish | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 0 | STOREW | <div style='text-align: right'>48,824</div>  |
| leaf_verifier | StoreE | 0 | STOREW2 | <div style='text-align: right'>60,648</div>  |
| leaf_verifier | StoreF | 0 | STOREW | <div style='text-align: right'>66,696</div>  |
| leaf_verifier | StoreF | 0 | STOREW2 | <div style='text-align: right'>143,400</div>  |
| leaf_verifier | StoreHintWord | 0 | ADD | <div style='text-align: right'>414,903</div>  |
| leaf_verifier | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>462,004</div>  |
| leaf_verifier | StoreV | 0 | STOREW | <div style='text-align: right'>5,890</div>  |
| leaf_verifier | StoreV | 0 | STOREW2 | <div style='text-align: right'>117,194</div>  |
| leaf_verifier | SubE | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier | SubEF | 0 | LOADW | <div style='text-align: right'>27,792</div>  |
| leaf_verifier | SubEF | 0 | SUB | <div style='text-align: right'>9,264</div>  |
| leaf_verifier | SubEFI | 0 | ADD | <div style='text-align: right'>516</div>  |
| leaf_verifier | SubEI | 0 | ADD | <div style='text-align: right'>720</div>  |
| leaf_verifier | SubFI | 0 | SUB | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | SubV | 0 | SUB | <div style='text-align: right'>91,342</div>  |
| leaf_verifier | SubVI | 0 | SUB | <div style='text-align: right'>1,994</div>  |
| leaf_verifier | SubVIN | 0 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 1 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier | AddEFFI | 1 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 1 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 1 | ADD | <div style='text-align: right'>344</div>  |
| leaf_verifier | AddEI | 1 | ADD | <div style='text-align: right'>51,424</div>  |
| leaf_verifier | AddF | 1 | ADD | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | AddFI | 1 | ADD | <div style='text-align: right'>71,583</div>  |
| leaf_verifier | AddV | 1 | ADD | <div style='text-align: right'>27,384</div>  |
| leaf_verifier | AddVI | 1 | ADD | <div style='text-align: right'>585,073</div>  |
| leaf_verifier | Alloc | 1 | ADD | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 1 | LOADW | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 1 | MUL | <div style='text-align: right'>64,240</div>  |
| leaf_verifier | AssertEqE | 1 | BNE | <div style='text-align: right'>448</div>  |
| leaf_verifier | AssertEqEI | 1 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 1 | BNE | <div style='text-align: right'>21,585</div>  |
| leaf_verifier | AssertEqFI | 1 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 1 | BNE | <div style='text-align: right'>2,014</div>  |
| leaf_verifier | AssertEqVI | 1 | BNE | <div style='text-align: right'>342</div>  |
| leaf_verifier | AssertNeVI | 1 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 1 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-poseidon2-hash | 1 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 1 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 1 | PHANTOM | <div style='text-align: right'>213,192</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 1 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 1 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 1 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-query | 1 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 1 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 1 | BBE4DIV | <div style='text-align: right'>9,380</div>  |
| leaf_verifier | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>72</div>  |
| leaf_verifier | DivEIN | 1 | STOREW | <div style='text-align: right'>288</div>  |
| leaf_verifier | DivFIN | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier | For | 1 | ADD | <div style='text-align: right'>713,889</div>  |
| leaf_verifier | For | 1 | BNE | <div style='text-align: right'>792,664</div>  |
| leaf_verifier | For | 1 | JAL | <div style='text-align: right'>78,775</div>  |
| leaf_verifier | For | 1 | LOADW | <div style='text-align: right'>4,956</div>  |
| leaf_verifier | For | 1 | STOREW | <div style='text-align: right'>73,819</div>  |
| leaf_verifier | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier | HintBitsF | 1 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 1 | PHANTOM | <div style='text-align: right'>43,313</div>  |
| leaf_verifier | IfEq | 1 | BNE | <div style='text-align: right'>33,630</div>  |
| leaf_verifier | IfEqI | 1 | BNE | <div style='text-align: right'>227,123</div>  |
| leaf_verifier | IfEqI | 1 | JAL | <div style='text-align: right'>49,243</div>  |
| leaf_verifier | IfNe | 1 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 1 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 1 | BEQ | <div style='text-align: right'>5,014</div>  |
| leaf_verifier | ImmE | 1 | STOREW | <div style='text-align: right'>5,168</div>  |
| leaf_verifier | ImmF | 1 | STOREW | <div style='text-align: right'>77,246</div>  |
| leaf_verifier | ImmV | 1 | STOREW | <div style='text-align: right'>51,705</div>  |
| leaf_verifier | LoadE | 1 | LOADW | <div style='text-align: right'>39,912</div>  |
| leaf_verifier | LoadE | 1 | LOADW2 | <div style='text-align: right'>103,272</div>  |
| leaf_verifier | LoadF | 1 | LOADW | <div style='text-align: right'>55,964</div>  |
| leaf_verifier | LoadF | 1 | LOADW2 | <div style='text-align: right'>133,284</div>  |
| leaf_verifier | LoadV | 1 | LOADW | <div style='text-align: right'>49,170</div>  |
| leaf_verifier | LoadV | 1 | LOADW2 | <div style='text-align: right'>316,004</div>  |
| leaf_verifier | MulE | 1 | BBE4MUL | <div style='text-align: right'>16,918</div>  |
| leaf_verifier | MulEF | 1 | MUL | <div style='text-align: right'>7,296</div>  |
| leaf_verifier | MulEFI | 1 | MUL | <div style='text-align: right'>480</div>  |
| leaf_verifier | MulEI | 1 | BBE4MUL | <div style='text-align: right'>2,040</div>  |
| leaf_verifier | MulEI | 1 | STOREW | <div style='text-align: right'>8,160</div>  |
| leaf_verifier | MulF | 1 | MUL | <div style='text-align: right'>139,662</div>  |
| leaf_verifier | MulFI | 1 | MUL | <div style='text-align: right'>2,694</div>  |
| leaf_verifier | MulVI | 1 | MUL | <div style='text-align: right'>38,012</div>  |
| leaf_verifier | NegE | 1 | MUL | <div style='text-align: right'>256</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier | Publish | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 1 | STOREW | <div style='text-align: right'>48,752</div>  |
| leaf_verifier | StoreE | 1 | STOREW2 | <div style='text-align: right'>52,584</div>  |
| leaf_verifier | StoreF | 1 | STOREW | <div style='text-align: right'>64,680</div>  |
| leaf_verifier | StoreF | 1 | STOREW2 | <div style='text-align: right'>121,104</div>  |
| leaf_verifier | StoreHintWord | 1 | ADD | <div style='text-align: right'>388,599</div>  |
| leaf_verifier | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier | StoreV | 1 | STOREW | <div style='text-align: right'>5,800</div>  |
| leaf_verifier | StoreV | 1 | STOREW2 | <div style='text-align: right'>109,676</div>  |
| leaf_verifier | SubE | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier | SubEF | 1 | LOADW | <div style='text-align: right'>23,220</div>  |
| leaf_verifier | SubEF | 1 | SUB | <div style='text-align: right'>7,740</div>  |
| leaf_verifier | SubEFI | 1 | ADD | <div style='text-align: right'>320</div>  |
| leaf_verifier | SubEI | 1 | ADD | <div style='text-align: right'>576</div>  |
| leaf_verifier | SubFI | 1 | SUB | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | SubV | 1 | SUB | <div style='text-align: right'>82,264</div>  |
| leaf_verifier | SubVI | 1 | SUB | <div style='text-align: right'>1,988</div>  |
| leaf_verifier | SubVIN | 1 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 2 | FE4ADD | <div style='text-align: right'>19,448</div>  |
| leaf_verifier | AddEFFI | 2 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 2 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 2 | ADD | <div style='text-align: right'>344</div>  |
| leaf_verifier | AddEI | 2 | ADD | <div style='text-align: right'>51,424</div>  |
| leaf_verifier | AddF | 2 | ADD | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | AddFI | 2 | ADD | <div style='text-align: right'>71,583</div>  |
| leaf_verifier | AddV | 2 | ADD | <div style='text-align: right'>27,384</div>  |
| leaf_verifier | AddVI | 2 | ADD | <div style='text-align: right'>585,073</div>  |
| leaf_verifier | Alloc | 2 | ADD | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 2 | LOADW | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 2 | MUL | <div style='text-align: right'>64,240</div>  |
| leaf_verifier | AssertEqE | 2 | BNE | <div style='text-align: right'>448</div>  |
| leaf_verifier | AssertEqEI | 2 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 2 | BNE | <div style='text-align: right'>21,585</div>  |
| leaf_verifier | AssertEqFI | 2 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 2 | BNE | <div style='text-align: right'>2,014</div>  |
| leaf_verifier | AssertEqVI | 2 | BNE | <div style='text-align: right'>342</div>  |
| leaf_verifier | AssertNeVI | 2 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 2 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-poseidon2-hash | 2 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 2 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 2 | PHANTOM | <div style='text-align: right'>213,192</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 2 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 2 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 2 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-query | 2 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 2 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 2 | BBE4DIV | <div style='text-align: right'>9,380</div>  |
| leaf_verifier | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>72</div>  |
| leaf_verifier | DivEIN | 2 | STOREW | <div style='text-align: right'>288</div>  |
| leaf_verifier | DivFIN | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier | For | 2 | ADD | <div style='text-align: right'>713,889</div>  |
| leaf_verifier | For | 2 | BNE | <div style='text-align: right'>792,664</div>  |
| leaf_verifier | For | 2 | JAL | <div style='text-align: right'>78,775</div>  |
| leaf_verifier | For | 2 | LOADW | <div style='text-align: right'>4,956</div>  |
| leaf_verifier | For | 2 | STOREW | <div style='text-align: right'>73,819</div>  |
| leaf_verifier | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier | HintBitsF | 2 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 2 | PHANTOM | <div style='text-align: right'>43,313</div>  |
| leaf_verifier | IfEq | 2 | BNE | <div style='text-align: right'>33,630</div>  |
| leaf_verifier | IfEqI | 2 | BNE | <div style='text-align: right'>227,123</div>  |
| leaf_verifier | IfEqI | 2 | JAL | <div style='text-align: right'>52,074</div>  |
| leaf_verifier | IfNe | 2 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 2 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 2 | BEQ | <div style='text-align: right'>5,014</div>  |
| leaf_verifier | ImmE | 2 | STOREW | <div style='text-align: right'>5,168</div>  |
| leaf_verifier | ImmF | 2 | STOREW | <div style='text-align: right'>77,246</div>  |
| leaf_verifier | ImmV | 2 | STOREW | <div style='text-align: right'>51,705</div>  |
| leaf_verifier | LoadE | 2 | LOADW | <div style='text-align: right'>39,912</div>  |
| leaf_verifier | LoadE | 2 | LOADW2 | <div style='text-align: right'>103,272</div>  |
| leaf_verifier | LoadF | 2 | LOADW | <div style='text-align: right'>55,964</div>  |
| leaf_verifier | LoadF | 2 | LOADW2 | <div style='text-align: right'>133,284</div>  |
| leaf_verifier | LoadV | 2 | LOADW | <div style='text-align: right'>49,170</div>  |
| leaf_verifier | LoadV | 2 | LOADW2 | <div style='text-align: right'>316,004</div>  |
| leaf_verifier | MulE | 2 | BBE4MUL | <div style='text-align: right'>16,918</div>  |
| leaf_verifier | MulEF | 2 | MUL | <div style='text-align: right'>7,296</div>  |
| leaf_verifier | MulEFI | 2 | MUL | <div style='text-align: right'>480</div>  |
| leaf_verifier | MulEI | 2 | BBE4MUL | <div style='text-align: right'>2,040</div>  |
| leaf_verifier | MulEI | 2 | STOREW | <div style='text-align: right'>8,160</div>  |
| leaf_verifier | MulF | 2 | MUL | <div style='text-align: right'>139,662</div>  |
| leaf_verifier | MulFI | 2 | MUL | <div style='text-align: right'>2,694</div>  |
| leaf_verifier | MulVI | 2 | MUL | <div style='text-align: right'>38,012</div>  |
| leaf_verifier | NegE | 2 | MUL | <div style='text-align: right'>256</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier | Publish | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 2 | STOREW | <div style='text-align: right'>48,752</div>  |
| leaf_verifier | StoreE | 2 | STOREW2 | <div style='text-align: right'>52,584</div>  |
| leaf_verifier | StoreF | 2 | STOREW | <div style='text-align: right'>64,680</div>  |
| leaf_verifier | StoreF | 2 | STOREW2 | <div style='text-align: right'>121,104</div>  |
| leaf_verifier | StoreHintWord | 2 | ADD | <div style='text-align: right'>388,599</div>  |
| leaf_verifier | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier | StoreV | 2 | STOREW | <div style='text-align: right'>5,800</div>  |
| leaf_verifier | StoreV | 2 | STOREW2 | <div style='text-align: right'>109,676</div>  |
| leaf_verifier | SubE | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier | SubEF | 2 | LOADW | <div style='text-align: right'>23,220</div>  |
| leaf_verifier | SubEF | 2 | SUB | <div style='text-align: right'>7,740</div>  |
| leaf_verifier | SubEFI | 2 | ADD | <div style='text-align: right'>320</div>  |
| leaf_verifier | SubEI | 2 | ADD | <div style='text-align: right'>576</div>  |
| leaf_verifier | SubFI | 2 | SUB | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | SubV | 2 | SUB | <div style='text-align: right'>82,264</div>  |
| leaf_verifier | SubVI | 2 | SUB | <div style='text-align: right'>1,988</div>  |
| leaf_verifier | SubVIN | 2 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 3 | FE4ADD | <div style='text-align: right'>20,492</div>  |
| leaf_verifier | AddEFFI | 3 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 3 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 3 | ADD | <div style='text-align: right'>400</div>  |
| leaf_verifier | AddEI | 3 | ADD | <div style='text-align: right'>52,624</div>  |
| leaf_verifier | AddF | 3 | ADD | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | AddFI | 3 | ADD | <div style='text-align: right'>72,267</div>  |
| leaf_verifier | AddV | 3 | ADD | <div style='text-align: right'>26,806</div>  |
| leaf_verifier | AddVI | 3 | ADD | <div style='text-align: right'>597,004</div>  |
| leaf_verifier | Alloc | 3 | ADD | <div style='text-align: right'>106,595</div>  |
| leaf_verifier | Alloc | 3 | LOADW | <div style='text-align: right'>106,595</div>  |
| leaf_verifier | Alloc | 3 | MUL | <div style='text-align: right'>63,610</div>  |
| leaf_verifier | AssertEqE | 3 | BNE | <div style='text-align: right'>456</div>  |
| leaf_verifier | AssertEqEI | 3 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 3 | BNE | <div style='text-align: right'>21,257</div>  |
| leaf_verifier | AssertEqFI | 3 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 3 | BNE | <div style='text-align: right'>1,994</div>  |
| leaf_verifier | AssertEqVI | 3 | BNE | <div style='text-align: right'>365</div>  |
| leaf_verifier | AssertNeVI | 3 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 3 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 3 | PHANTOM | <div style='text-align: right'>10,416</div>  |
| leaf_verifier | CT-poseidon2-hash | 3 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 3 | PHANTOM | <div style='text-align: right'>3,276</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 3 | PHANTOM | <div style='text-align: right'>229,320</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 3 | PHANTOM | <div style='text-align: right'>16,296</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 3 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 3 | PHANTOM | <div style='text-align: right'>3,276</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 3 | PHANTOM | <div style='text-align: right'>9,660</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 3 | PHANTOM | <div style='text-align: right'>9,660</div>  |
| leaf_verifier | CT-verify-query | 3 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 3 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 3 | BBE4DIV | <div style='text-align: right'>9,846</div>  |
| leaf_verifier | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>78</div>  |
| leaf_verifier | DivEIN | 3 | STOREW | <div style='text-align: right'>312</div>  |
| leaf_verifier | DivFIN | 3 | DIV | <div style='text-align: right'>186</div>  |
| leaf_verifier | For | 3 | ADD | <div style='text-align: right'>725,867</div>  |
| leaf_verifier | For | 3 | BNE | <div style='text-align: right'>805,105</div>  |
| leaf_verifier | For | 3 | JAL | <div style='text-align: right'>79,238</div>  |
| leaf_verifier | For | 3 | LOADW | <div style='text-align: right'>4,914</div>  |
| leaf_verifier | For | 3 | STOREW | <div style='text-align: right'>74,324</div>  |
| leaf_verifier | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>8,148</div>  |
| leaf_verifier | HintBitsF | 3 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 3 | PHANTOM | <div style='text-align: right'>42,985</div>  |
| leaf_verifier | IfEq | 3 | BNE | <div style='text-align: right'>36,566</div>  |
| leaf_verifier | IfEqI | 3 | BNE | <div style='text-align: right'>234,437</div>  |
| leaf_verifier | IfEqI | 3 | JAL | <div style='text-align: right'>51,525</div>  |
| leaf_verifier | IfNe | 3 | BEQ | <div style='text-align: right'>30,522</div>  |
| leaf_verifier | IfNe | 3 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 3 | BEQ | <div style='text-align: right'>4,982</div>  |
| leaf_verifier | ImmE | 3 | STOREW | <div style='text-align: right'>5,560</div>  |
| leaf_verifier | ImmF | 3 | STOREW | <div style='text-align: right'>77,586</div>  |
| leaf_verifier | ImmV | 3 | STOREW | <div style='text-align: right'>52,338</div>  |
| leaf_verifier | LoadE | 3 | LOADW | <div style='text-align: right'>40,896</div>  |
| leaf_verifier | LoadE | 3 | LOADW2 | <div style='text-align: right'>107,144</div>  |
| leaf_verifier | LoadF | 3 | LOADW | <div style='text-align: right'>56,186</div>  |
| leaf_verifier | LoadF | 3 | LOADW2 | <div style='text-align: right'>143,064</div>  |
| leaf_verifier | LoadV | 3 | LOADW | <div style='text-align: right'>48,801</div>  |
| leaf_verifier | LoadV | 3 | LOADW2 | <div style='text-align: right'>325,252</div>  |
| leaf_verifier | MulE | 3 | BBE4MUL | <div style='text-align: right'>17,271</div>  |
| leaf_verifier | MulEF | 3 | MUL | <div style='text-align: right'>7,176</div>  |
| leaf_verifier | MulEFI | 3 | MUL | <div style='text-align: right'>608</div>  |
| leaf_verifier | MulEI | 3 | BBE4MUL | <div style='text-align: right'>2,274</div>  |
| leaf_verifier | MulEI | 3 | STOREW | <div style='text-align: right'>9,096</div>  |
| leaf_verifier | MulF | 3 | MUL | <div style='text-align: right'>140,690</div>  |
| leaf_verifier | MulFI | 3 | MUL | <div style='text-align: right'>2,696</div>  |
| leaf_verifier | MulVI | 3 | MUL | <div style='text-align: right'>37,312</div>  |
| leaf_verifier | NegE | 3 | MUL | <div style='text-align: right'>284</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>32,813</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>17,140</div>  |
| leaf_verifier | Publish | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 3 | STOREW | <div style='text-align: right'>48,104</div>  |
| leaf_verifier | StoreE | 3 | STOREW2 | <div style='text-align: right'>54,932</div>  |
| leaf_verifier | StoreF | 3 | STOREW | <div style='text-align: right'>65,192</div>  |
| leaf_verifier | StoreF | 3 | STOREW2 | <div style='text-align: right'>129,288</div>  |
| leaf_verifier | StoreHintWord | 3 | ADD | <div style='text-align: right'>390,276</div>  |
| leaf_verifier | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>435,927</div>  |
| leaf_verifier | StoreV | 3 | STOREW | <div style='text-align: right'>5,704</div>  |
| leaf_verifier | StoreV | 3 | STOREW2 | <div style='text-align: right'>110,038</div>  |
| leaf_verifier | SubE | 3 | FE4SUB | <div style='text-align: right'>6,541</div>  |
| leaf_verifier | SubEF | 3 | LOADW | <div style='text-align: right'>24,744</div>  |
| leaf_verifier | SubEF | 3 | SUB | <div style='text-align: right'>8,248</div>  |
| leaf_verifier | SubEFI | 3 | ADD | <div style='text-align: right'>380</div>  |
| leaf_verifier | SubEI | 3 | ADD | <div style='text-align: right'>624</div>  |
| leaf_verifier | SubFI | 3 | SUB | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | SubV | 3 | SUB | <div style='text-align: right'>83,148</div>  |
| leaf_verifier | SubVI | 3 | SUB | <div style='text-align: right'>1,944</div>  |
| leaf_verifier | SubVIN | 3 | SUB | <div style='text-align: right'>1,638</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | <div style='text-align: right'>913,920</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 0 | FE4ADD | <div style='text-align: right'>382,360</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 0 | FE4ADD | <div style='text-align: right'>225,940</div>  |
| leaf_verifier | Boundary | AddE | 0 | FE4ADD | <div style='text-align: right'>112,508</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | LOADW | <div style='text-align: right'>13,940</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 0 | LOADW | <div style='text-align: right'>2,343</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 0 | LOADW | <div style='text-align: right'>2,769</div>  |
| leaf_verifier | Boundary | AddEFFI | 0 | LOADW | <div style='text-align: right'>308</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | STOREW | <div style='text-align: right'>41,820</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 0 | STOREW | <div style='text-align: right'>2,343</div>  |
| leaf_verifier | Boundary | AddEFFI | 0 | STOREW | <div style='text-align: right'>924</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | ADD | <div style='text-align: right'>14,520</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 0 | ADD | <div style='text-align: right'>2,266</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 0 | ADD | <div style='text-align: right'>1,339</div>  |
| leaf_verifier | Boundary | AddEFI | 0 | ADD | <div style='text-align: right'>1,364</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | <div style='text-align: right'>1,693,200</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 0 | ADD | <div style='text-align: right'>353,540</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 0 | ADD | <div style='text-align: right'>208,910</div>  |
| leaf_verifier | Boundary | AddEI | 0 | ADD | <div style='text-align: right'>136,664</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 0 | ADD | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | <div style='text-align: right'>2,390,490</div>  |
| leaf_verifier | Boundary | AddFI | 0 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | <div style='text-align: right'>854,820</div>  |
| leaf_verifier | Boundary | AddV | 0 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | <div style='text-align: right'>19,337,430</div>  |
| leaf_verifier | Boundary | AddVI | 0 | ADD | <div style='text-align: right'>671</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | <div style='text-align: right'>3,300,390</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | LOADW | <div style='text-align: right'>4,510,533</div>  |
| leaf_verifier | Boundary | Alloc | 0 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | <div style='text-align: right'>1,967,340</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 0 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 0 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | BNE | <div style='text-align: right'>10,856</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 0 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 0 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | BNE | <div style='text-align: right'>496,455</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | BNE | <div style='text-align: right'>47,840</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | BNE | <div style='text-align: right'>9,384</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 0 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 0 | PHANTOM | <div style='text-align: right'>70,560</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 0 | PHANTOM | <div style='text-align: right'>39,816</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 0 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 0 | PHANTOM | <div style='text-align: right'>1,545,264</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 0 | PHANTOM | <div style='text-align: right'>109,872</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 0 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 0 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 0 | PHANTOM | <div style='text-align: right'>59,976</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | <div style='text-align: right'>59,976</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | BBE4DIV | <div style='text-align: right'>436,160</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 0 | BBE4DIV | <div style='text-align: right'>203,720</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 0 | BBE4DIV | <div style='text-align: right'>120,380</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>3,600</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>3,872</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>2,288</div>  |
| leaf_verifier | Boundary | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>528</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | STOREW | <div style='text-align: right'>14,760</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 0 | STOREW | <div style='text-align: right'>1,298</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 0 | STOREW | <div style='text-align: right'>364</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | <div style='text-align: right'>23,578,110</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | BNE | <div style='text-align: right'>19,991,876</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | <div style='text-align: right'>832,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 0 | JAL | <div style='text-align: right'>682</div>  |
| leaf_verifier | AccessAdapter<4> | For | 0 | JAL | <div style='text-align: right'>806</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | LOADW | <div style='text-align: right'>208,362</div>  |
| leaf_verifier | Boundary | For | 0 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | STOREW | <div style='text-align: right'>3,205,913</div>  |
| leaf_verifier | Boundary | For | 0 | STOREW | <div style='text-align: right'>715</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 0 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 0 | PHANTOM | <div style='text-align: right'>266,610</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | <div style='text-align: right'>1,014,024</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | <div style='text-align: right'>5,920,315</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | JAL | <div style='text-align: right'>543,510</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 0 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | <div style='text-align: right'>118,910</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | STOREW | <div style='text-align: right'>263,056</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 0 | STOREW | <div style='text-align: right'>17,512</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 0 | STOREW | <div style='text-align: right'>10,348</div>  |
| leaf_verifier | Boundary | ImmE | 0 | STOREW | <div style='text-align: right'>13,772</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | STOREW | <div style='text-align: right'>3,332,890</div>  |
| leaf_verifier | Boundary | ImmF | 0 | STOREW | <div style='text-align: right'>16,291</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | <div style='text-align: right'>2,261,109</div>  |
| leaf_verifier | Boundary | ImmV | 0 | STOREW | <div style='text-align: right'>1,716</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | <div style='text-align: right'>1,803,672</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 0 | LOADW | <div style='text-align: right'>303,072</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 0 | LOADW | <div style='text-align: right'>179,088</div>  |
| leaf_verifier | Boundary | LoadE | 0 | LOADW | <div style='text-align: right'>3,740</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | LOADW2 | <div style='text-align: right'>4,813,728</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 0 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 0 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 0 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | <div style='text-align: right'>2,294,524</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 0 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 0 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 0 | LOADW | <div style='text-align: right'>14,905</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | LOADW2 | <div style='text-align: right'>6,585,420</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 0 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 0 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 0 | LOADW2 | <div style='text-align: right'>649</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | <div style='text-align: right'>2,067,138</div>  |
| leaf_verifier | Boundary | LoadV | 0 | LOADW | <div style='text-align: right'>462</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | LOADW2 | <div style='text-align: right'>14,713,834</div>  |
| leaf_verifier | Boundary | LoadV | 0 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | BBE4MUL | <div style='text-align: right'>753,760</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 0 | BBE4MUL | <div style='text-align: right'>361,328</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 0 | BBE4MUL | <div style='text-align: right'>213,512</div>  |
| leaf_verifier | Boundary | MulE | 0 | BBE4MUL | <div style='text-align: right'>136,752</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | <div style='text-align: right'>223,200</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 0 | MUL | <div style='text-align: right'>38,258</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 0 | MUL | <div style='text-align: right'>22,607</div>  |
| leaf_verifier | Boundary | MulEF | 0 | MUL | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | MUL | <div style='text-align: right'>23,880</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 0 | MUL | <div style='text-align: right'>4,620</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 0 | MUL | <div style='text-align: right'>2,730</div>  |
| leaf_verifier | Boundary | MulEFI | 0 | MUL | <div style='text-align: right'>1,496</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>106,640</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>119,834</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>70,811</div>  |
| leaf_verifier | Boundary | MulEI | 0 | BBE4MUL | <div style='text-align: right'>4,312</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | <div style='text-align: right'>437,224</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 0 | STOREW | <div style='text-align: right'>58,553</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 0 | STOREW | <div style='text-align: right'>34,554</div>  |
| leaf_verifier | Boundary | MulEI | 0 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | <div style='text-align: right'>4,645,260</div>  |
| leaf_verifier | Boundary | MulF | 0 | MUL | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | <div style='text-align: right'>81,000</div>  |
| leaf_verifier | Boundary | MulFI | 0 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | <div style='text-align: right'>1,171,860</div>  |
| leaf_verifier | Boundary | MulVI | 0 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | MUL | <div style='text-align: right'>9,960</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 0 | MUL | <div style='text-align: right'>2,662</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 0 | MUL | <div style='text-align: right'>1,573</div>  |
| leaf_verifier | Boundary | NegE | 0 | MUL | <div style='text-align: right'>792</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>18,946,746</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>1,029,688</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>611,182</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>408,561</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>10,639,447</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | <div style='text-align: right'>2,001,784</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 0 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 0 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 0 | STOREW | <div style='text-align: right'>537,064</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | STOREW2 | <div style='text-align: right'>2,486,568</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 0 | STOREW2 | <div style='text-align: right'>258,720</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 0 | STOREW2 | <div style='text-align: right'>152,880</div>  |
| leaf_verifier | Boundary | StoreE | 0 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | <div style='text-align: right'>2,734,536</div>  |
| leaf_verifier | Boundary | StoreF | 0 | STOREW | <div style='text-align: right'>733,656</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | STOREW2 | <div style='text-align: right'>5,879,400</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 0 | STOREW2 | <div style='text-align: right'>588,148</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 0 | STOREW2 | <div style='text-align: right'>350,272</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 0 | STOREW2 | <div style='text-align: right'>237,609</div>  |
| leaf_verifier | Boundary | StoreF | 0 | STOREW2 | <div style='text-align: right'>153,934</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | <div style='text-align: right'>12,447,090</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>18,942,164</div>  |
| leaf_verifier | Boundary | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>5,082,044</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | <div style='text-align: right'>241,490</div>  |
| leaf_verifier | Boundary | StoreV | 0 | STOREW | <div style='text-align: right'>64,790</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | STOREW2 | <div style='text-align: right'>4,804,954</div>  |
| leaf_verifier | Boundary | StoreV | 0 | STOREW2 | <div style='text-align: right'>1,178,848</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 0 | FE4SUB | <div style='text-align: right'>234,366</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 0 | FE4SUB | <div style='text-align: right'>138,489</div>  |
| leaf_verifier | Boundary | SubE | 0 | FE4SUB | <div style='text-align: right'>27,896</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | LOADW | <div style='text-align: right'>1,139,472</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 0 | LOADW | <div style='text-align: right'>101,662</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | SUB | <div style='text-align: right'>277,920</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 0 | SUB | <div style='text-align: right'>101,662</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 0 | SUB | <div style='text-align: right'>120,146</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | ADD | <div style='text-align: right'>15,480</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 0 | ADD | <div style='text-align: right'>2,640</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 0 | ADD | <div style='text-align: right'>1,560</div>  |
| leaf_verifier | Boundary | SubEFI | 0 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | ADD | <div style='text-align: right'>21,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 0 | ADD | <div style='text-align: right'>5,874</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 0 | ADD | <div style='text-align: right'>3,471</div>  |
| leaf_verifier | Boundary | SubEI | 0 | ADD | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 0 | SUB | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | SubFI | 0 | SUB | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | <div style='text-align: right'>2,740,260</div>  |
| leaf_verifier | Boundary | SubV | 0 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | <div style='text-align: right'>59,820</div>  |
| leaf_verifier | Boundary | SubVI | 0 | SUB | <div style='text-align: right'>506</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 1 | FE4ADD | <div style='text-align: right'>777,920</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 1 | FE4ADD | <div style='text-align: right'>339,152</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 1 | FE4ADD | <div style='text-align: right'>200,408</div>  |
| leaf_verifier | Boundary | AddE | 1 | FE4ADD | <div style='text-align: right'>118,492</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 1 | LOADW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 1 | LOADW | <div style='text-align: right'>2,691</div>  |
| leaf_verifier | Boundary | AddEFFI | 1 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 1 | STOREW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | Boundary | AddEFFI | 1 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 1 | ADD | <div style='text-align: right'>10,320</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 1 | ADD | <div style='text-align: right'>1,540</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 1 | ADD | <div style='text-align: right'>910</div>  |
| leaf_verifier | Boundary | AddEFI | 1 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 1 | ADD | <div style='text-align: right'>1,542,720</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 1 | ADD | <div style='text-align: right'>307,494</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 1 | ADD | <div style='text-align: right'>181,701</div>  |
| leaf_verifier | Boundary | AddEI | 1 | ADD | <div style='text-align: right'>129,360</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 1 | ADD | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 1 | ADD | <div style='text-align: right'>2,147,490</div>  |
| leaf_verifier | Boundary | AddFI | 1 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 1 | ADD | <div style='text-align: right'>821,520</div>  |
| leaf_verifier | Boundary | AddV | 1 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 1 | ADD | <div style='text-align: right'>17,552,190</div>  |
| leaf_verifier | Boundary | AddVI | 1 | ADD | <div style='text-align: right'>671</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | ADD | <div style='text-align: right'>3,226,590</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 1 | LOADW | <div style='text-align: right'>4,409,673</div>  |
| leaf_verifier | Boundary | Alloc | 1 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | MUL | <div style='text-align: right'>1,927,200</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 1 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 1 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 1 | BNE | <div style='text-align: right'>10,304</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 1 | BNE | <div style='text-align: right'>2,464</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 1 | BNE | <div style='text-align: right'>1,456</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 1 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 1 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 1 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 1 | BNE | <div style='text-align: right'>496,455</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 1 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 1 | BNE | <div style='text-align: right'>46,322</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 1 | BNE | <div style='text-align: right'>7,866</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 1 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 1 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 1 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 1 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 1 | PHANTOM | <div style='text-align: right'>1,279,152</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 1 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 1 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 1 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 1 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 1 | BBE4DIV | <div style='text-align: right'>375,200</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 1 | BBE4DIV | <div style='text-align: right'>170,060</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 1 | BBE4DIV | <div style='text-align: right'>100,490</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>2,880</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>3,102</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>1,833</div>  |
| leaf_verifier | Boundary | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 1 | STOREW | <div style='text-align: right'>11,808</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 1 | STOREW | <div style='text-align: right'>1,034</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 1 | STOREW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 1 | ADD | <div style='text-align: right'>21,416,670</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 1 | BNE | <div style='text-align: right'>18,231,272</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 1 | JAL | <div style='text-align: right'>787,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 1 | JAL | <div style='text-align: right'>550</div>  |
| leaf_verifier | AccessAdapter<4> | For | 1 | JAL | <div style='text-align: right'>650</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | LOADW | <div style='text-align: right'>203,196</div>  |
| leaf_verifier | Boundary | For | 1 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | STOREW | <div style='text-align: right'>3,026,579</div>  |
| leaf_verifier | Boundary | For | 1 | STOREW | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 1 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 1 | PHANTOM | <div style='text-align: right'>259,878</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 1 | BNE | <div style='text-align: right'>773,490</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 1 | BNE | <div style='text-align: right'>5,223,829</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 1 | JAL | <div style='text-align: right'>492,430</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 1 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 1 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 1 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 1 | BEQ | <div style='text-align: right'>115,322</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 1 | STOREW | <div style='text-align: right'>211,888</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 1 | STOREW | <div style='text-align: right'>12,342</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 1 | STOREW | <div style='text-align: right'>7,293</div>  |
| leaf_verifier | Boundary | ImmE | 1 | STOREW | <div style='text-align: right'>12,408</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 1 | STOREW | <div style='text-align: right'>3,167,086</div>  |
| leaf_verifier | Boundary | ImmF | 1 | STOREW | <div style='text-align: right'>16,291</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 1 | STOREW | <div style='text-align: right'>2,119,905</div>  |
| leaf_verifier | Boundary | ImmV | 1 | STOREW | <div style='text-align: right'>1,716</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | LOADW | <div style='text-align: right'>1,636,392</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 1 | LOADW | <div style='text-align: right'>264,264</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 1 | LOADW | <div style='text-align: right'>156,156</div>  |
| leaf_verifier | Boundary | LoadE | 1 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | LOADW2 | <div style='text-align: right'>4,234,152</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 1 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 1 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 1 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | LOADW | <div style='text-align: right'>2,294,524</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 1 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 1 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 1 | LOADW | <div style='text-align: right'>14,905</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | LOADW2 | <div style='text-align: right'>5,464,644</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 1 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 1 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 1 | LOADW2 | <div style='text-align: right'>649</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | LOADW | <div style='text-align: right'>2,015,970</div>  |
| leaf_verifier | Boundary | LoadV | 1 | LOADW | <div style='text-align: right'>528</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | LOADW2 | <div style='text-align: right'>12,956,164</div>  |
| leaf_verifier | Boundary | LoadV | 1 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 1 | BBE4MUL | <div style='text-align: right'>676,720</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 1 | BBE4MUL | <div style='text-align: right'>326,150</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 1 | BBE4MUL | <div style='text-align: right'>192,725</div>  |
| leaf_verifier | Boundary | MulE | 1 | BBE4MUL | <div style='text-align: right'>139,832</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 1 | MUL | <div style='text-align: right'>218,880</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 1 | MUL | <div style='text-align: right'>38,082</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 1 | MUL | <div style='text-align: right'>22,503</div>  |
| leaf_verifier | Boundary | MulEF | 1 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 1 | MUL | <div style='text-align: right'>14,400</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 1 | MUL | <div style='text-align: right'>2,684</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 1 | MUL | <div style='text-align: right'>1,586</div>  |
| leaf_verifier | Boundary | MulEFI | 1 | MUL | <div style='text-align: right'>1,232</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>81,600</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>93,566</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>55,289</div>  |
| leaf_verifier | Boundary | MulEI | 1 | BBE4MUL | <div style='text-align: right'>5,984</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 1 | STOREW | <div style='text-align: right'>334,560</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 1 | STOREW | <div style='text-align: right'>44,781</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 1 | STOREW | <div style='text-align: right'>26,416</div>  |
| leaf_verifier | Boundary | MulEI | 1 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 1 | MUL | <div style='text-align: right'>4,189,860</div>  |
| leaf_verifier | Boundary | MulF | 1 | MUL | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 1 | MUL | <div style='text-align: right'>80,820</div>  |
| leaf_verifier | Boundary | MulFI | 1 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 1 | MUL | <div style='text-align: right'>1,140,360</div>  |
| leaf_verifier | Boundary | MulVI | 1 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 1 | MUL | <div style='text-align: right'>7,680</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 1 | MUL | <div style='text-align: right'>2,002</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 1 | MUL | <div style='text-align: right'>1,183</div>  |
| leaf_verifier | Boundary | NegE | 1 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 1 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | STOREW | <div style='text-align: right'>1,998,832</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 1 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 1 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 1 | STOREW | <div style='text-align: right'>536,272</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | STOREW2 | <div style='text-align: right'>2,155,944</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 1 | STOREW2 | <div style='text-align: right'>214,368</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 1 | STOREW2 | <div style='text-align: right'>126,672</div>  |
| leaf_verifier | Boundary | StoreE | 1 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | STOREW | <div style='text-align: right'>2,651,880</div>  |
| leaf_verifier | Boundary | StoreF | 1 | STOREW | <div style='text-align: right'>711,480</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | STOREW2 | <div style='text-align: right'>4,965,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 1 | STOREW2 | <div style='text-align: right'>471,592</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 1 | STOREW2 | <div style='text-align: right'>280,852</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 1 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary | StoreF | 1 | STOREW2 | <div style='text-align: right'>152,878</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 1 | ADD | <div style='text-align: right'>11,657,970</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>4,780,358</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | STOREW | <div style='text-align: right'>237,800</div>  |
| leaf_verifier | Boundary | StoreV | 1 | STOREW | <div style='text-align: right'>63,800</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | STOREW2 | <div style='text-align: right'>4,496,716</div>  |
| leaf_verifier | Boundary | StoreV | 1 | STOREW2 | <div style='text-align: right'>1,114,630</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 1 | FE4SUB | <div style='text-align: right'>229,790</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 1 | FE4SUB | <div style='text-align: right'>135,785</div>  |
| leaf_verifier | Boundary | SubE | 1 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 1 | LOADW | <div style='text-align: right'>952,020</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 1 | LOADW | <div style='text-align: right'>84,898</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 1 | SUB | <div style='text-align: right'>232,200</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 1 | SUB | <div style='text-align: right'>84,898</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 1 | SUB | <div style='text-align: right'>100,334</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 1 | ADD | <div style='text-align: right'>9,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 1 | ADD | <div style='text-align: right'>1,760</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 1 | ADD | <div style='text-align: right'>1,040</div>  |
| leaf_verifier | Boundary | SubEFI | 1 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 1 | ADD | <div style='text-align: right'>17,280</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 1 | ADD | <div style='text-align: right'>4,576</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 1 | ADD | <div style='text-align: right'>2,704</div>  |
| leaf_verifier | Boundary | SubEI | 1 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 1 | SUB | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | SubFI | 1 | SUB | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 1 | SUB | <div style='text-align: right'>2,467,920</div>  |
| leaf_verifier | Boundary | SubV | 1 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 1 | SUB | <div style='text-align: right'>59,640</div>  |
| leaf_verifier | Boundary | SubVI | 1 | SUB | <div style='text-align: right'>506</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 1 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 2 | FE4ADD | <div style='text-align: right'>777,920</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 2 | FE4ADD | <div style='text-align: right'>339,152</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 2 | FE4ADD | <div style='text-align: right'>200,408</div>  |
| leaf_verifier | Boundary | AddE | 2 | FE4ADD | <div style='text-align: right'>118,492</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 2 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 2 | LOADW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 2 | LOADW | <div style='text-align: right'>2,691</div>  |
| leaf_verifier | Boundary | AddEFFI | 2 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 2 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 2 | STOREW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | Boundary | AddEFFI | 2 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 2 | ADD | <div style='text-align: right'>10,320</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 2 | ADD | <div style='text-align: right'>1,540</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 2 | ADD | <div style='text-align: right'>910</div>  |
| leaf_verifier | Boundary | AddEFI | 2 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 2 | ADD | <div style='text-align: right'>1,542,720</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 2 | ADD | <div style='text-align: right'>308,132</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 2 | ADD | <div style='text-align: right'>182,078</div>  |
| leaf_verifier | Boundary | AddEI | 2 | ADD | <div style='text-align: right'>129,360</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 2 | ADD | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 2 | ADD | <div style='text-align: right'>2,147,490</div>  |
| leaf_verifier | Boundary | AddFI | 2 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 2 | ADD | <div style='text-align: right'>821,520</div>  |
| leaf_verifier | Boundary | AddV | 2 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 2 | ADD | <div style='text-align: right'>17,552,190</div>  |
| leaf_verifier | Boundary | AddVI | 2 | ADD | <div style='text-align: right'>671</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 2 | ADD | <div style='text-align: right'>3,226,590</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 2 | LOADW | <div style='text-align: right'>4,409,673</div>  |
| leaf_verifier | Boundary | Alloc | 2 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 2 | MUL | <div style='text-align: right'>1,927,200</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 2 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 2 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 2 | BNE | <div style='text-align: right'>10,304</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 2 | BNE | <div style='text-align: right'>2,464</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 2 | BNE | <div style='text-align: right'>1,456</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 2 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 2 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 2 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 2 | BNE | <div style='text-align: right'>496,455</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 2 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 2 | BNE | <div style='text-align: right'>46,322</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 2 | BNE | <div style='text-align: right'>7,866</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 2 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 2 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 2 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 2 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 2 | PHANTOM | <div style='text-align: right'>1,279,152</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 2 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 2 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 2 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 2 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 2 | BBE4DIV | <div style='text-align: right'>375,200</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 2 | BBE4DIV | <div style='text-align: right'>170,060</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 2 | BBE4DIV | <div style='text-align: right'>100,490</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>2,880</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>3,102</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>1,833</div>  |
| leaf_verifier | Boundary | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 2 | STOREW | <div style='text-align: right'>11,808</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 2 | STOREW | <div style='text-align: right'>1,034</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 2 | STOREW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 2 | ADD | <div style='text-align: right'>21,416,670</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 2 | BNE | <div style='text-align: right'>18,231,272</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 2 | JAL | <div style='text-align: right'>787,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 2 | JAL | <div style='text-align: right'>550</div>  |
| leaf_verifier | AccessAdapter<4> | For | 2 | JAL | <div style='text-align: right'>650</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 2 | LOADW | <div style='text-align: right'>203,196</div>  |
| leaf_verifier | Boundary | For | 2 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 2 | STOREW | <div style='text-align: right'>3,026,579</div>  |
| leaf_verifier | Boundary | For | 2 | STOREW | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 2 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 2 | PHANTOM | <div style='text-align: right'>259,878</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 2 | BNE | <div style='text-align: right'>773,490</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 2 | BNE | <div style='text-align: right'>5,223,829</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 2 | JAL | <div style='text-align: right'>520,740</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 2 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 2 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 2 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 2 | BEQ | <div style='text-align: right'>115,322</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 2 | STOREW | <div style='text-align: right'>211,888</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 2 | STOREW | <div style='text-align: right'>12,342</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 2 | STOREW | <div style='text-align: right'>7,293</div>  |
| leaf_verifier | Boundary | ImmE | 2 | STOREW | <div style='text-align: right'>12,408</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 2 | STOREW | <div style='text-align: right'>3,167,086</div>  |
| leaf_verifier | Boundary | ImmF | 2 | STOREW | <div style='text-align: right'>16,291</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 2 | STOREW | <div style='text-align: right'>2,119,905</div>  |
| leaf_verifier | Boundary | ImmV | 2 | STOREW | <div style='text-align: right'>1,716</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 2 | LOADW | <div style='text-align: right'>1,636,392</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 2 | LOADW | <div style='text-align: right'>264,264</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 2 | LOADW | <div style='text-align: right'>156,156</div>  |
| leaf_verifier | Boundary | LoadE | 2 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 2 | LOADW2 | <div style='text-align: right'>4,234,152</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 2 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 2 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 2 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 2 | LOADW | <div style='text-align: right'>2,294,524</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 2 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 2 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 2 | LOADW | <div style='text-align: right'>14,905</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 2 | LOADW2 | <div style='text-align: right'>5,464,644</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 2 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 2 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 2 | LOADW2 | <div style='text-align: right'>649</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 2 | LOADW | <div style='text-align: right'>2,015,970</div>  |
| leaf_verifier | Boundary | LoadV | 2 | LOADW | <div style='text-align: right'>528</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 2 | LOADW2 | <div style='text-align: right'>12,956,164</div>  |
| leaf_verifier | Boundary | LoadV | 2 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 2 | BBE4MUL | <div style='text-align: right'>676,720</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 2 | BBE4MUL | <div style='text-align: right'>326,788</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 2 | BBE4MUL | <div style='text-align: right'>193,102</div>  |
| leaf_verifier | Boundary | MulE | 2 | BBE4MUL | <div style='text-align: right'>139,832</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 2 | MUL | <div style='text-align: right'>218,880</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 2 | MUL | <div style='text-align: right'>38,082</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 2 | MUL | <div style='text-align: right'>22,503</div>  |
| leaf_verifier | Boundary | MulEF | 2 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 2 | MUL | <div style='text-align: right'>14,400</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 2 | MUL | <div style='text-align: right'>2,684</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 2 | MUL | <div style='text-align: right'>1,586</div>  |
| leaf_verifier | Boundary | MulEFI | 2 | MUL | <div style='text-align: right'>1,232</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>81,600</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>93,566</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>55,289</div>  |
| leaf_verifier | Boundary | MulEI | 2 | BBE4MUL | <div style='text-align: right'>5,984</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 2 | STOREW | <div style='text-align: right'>334,560</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 2 | STOREW | <div style='text-align: right'>44,781</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 2 | STOREW | <div style='text-align: right'>26,416</div>  |
| leaf_verifier | Boundary | MulEI | 2 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 2 | MUL | <div style='text-align: right'>4,189,860</div>  |
| leaf_verifier | Boundary | MulF | 2 | MUL | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 2 | MUL | <div style='text-align: right'>80,820</div>  |
| leaf_verifier | Boundary | MulFI | 2 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 2 | MUL | <div style='text-align: right'>1,140,360</div>  |
| leaf_verifier | Boundary | MulVI | 2 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 2 | MUL | <div style='text-align: right'>7,680</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 2 | MUL | <div style='text-align: right'>2,002</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 2 | MUL | <div style='text-align: right'>1,183</div>  |
| leaf_verifier | Boundary | NegE | 2 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 2 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 2 | STOREW | <div style='text-align: right'>1,998,832</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 2 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 2 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 2 | STOREW | <div style='text-align: right'>536,272</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 2 | STOREW2 | <div style='text-align: right'>2,155,944</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 2 | STOREW2 | <div style='text-align: right'>214,368</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 2 | STOREW2 | <div style='text-align: right'>126,672</div>  |
| leaf_verifier | Boundary | StoreE | 2 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 2 | STOREW | <div style='text-align: right'>2,651,880</div>  |
| leaf_verifier | Boundary | StoreF | 2 | STOREW | <div style='text-align: right'>711,480</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 2 | STOREW2 | <div style='text-align: right'>4,965,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 2 | STOREW2 | <div style='text-align: right'>471,592</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 2 | STOREW2 | <div style='text-align: right'>280,852</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 2 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary | StoreF | 2 | STOREW2 | <div style='text-align: right'>152,878</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 2 | ADD | <div style='text-align: right'>11,657,970</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>4,780,358</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 2 | STOREW | <div style='text-align: right'>237,800</div>  |
| leaf_verifier | Boundary | StoreV | 2 | STOREW | <div style='text-align: right'>63,800</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 2 | STOREW2 | <div style='text-align: right'>4,496,716</div>  |
| leaf_verifier | Boundary | StoreV | 2 | STOREW2 | <div style='text-align: right'>1,114,630</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 2 | FE4SUB | <div style='text-align: right'>229,790</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 2 | FE4SUB | <div style='text-align: right'>135,785</div>  |
| leaf_verifier | Boundary | SubE | 2 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 2 | LOADW | <div style='text-align: right'>952,020</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 2 | LOADW | <div style='text-align: right'>84,898</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 2 | SUB | <div style='text-align: right'>232,200</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 2 | SUB | <div style='text-align: right'>84,898</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 2 | SUB | <div style='text-align: right'>100,334</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 2 | ADD | <div style='text-align: right'>9,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 2 | ADD | <div style='text-align: right'>1,760</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 2 | ADD | <div style='text-align: right'>1,040</div>  |
| leaf_verifier | Boundary | SubEFI | 2 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 2 | ADD | <div style='text-align: right'>17,280</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 2 | ADD | <div style='text-align: right'>4,576</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 2 | ADD | <div style='text-align: right'>2,704</div>  |
| leaf_verifier | Boundary | SubEI | 2 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 2 | SUB | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | SubFI | 2 | SUB | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 2 | SUB | <div style='text-align: right'>2,467,920</div>  |
| leaf_verifier | Boundary | SubV | 2 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 2 | SUB | <div style='text-align: right'>59,640</div>  |
| leaf_verifier | Boundary | SubVI | 2 | SUB | <div style='text-align: right'>506</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 2 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 3 | FE4ADD | <div style='text-align: right'>819,680</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 3 | FE4ADD | <div style='text-align: right'>352,946</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 3 | FE4ADD | <div style='text-align: right'>208,559</div>  |
| leaf_verifier | Boundary | AddE | 3 | FE4ADD | <div style='text-align: right'>118,492</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 3 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 3 | LOADW | <div style='text-align: right'>2,266</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 3 | LOADW | <div style='text-align: right'>2,678</div>  |
| leaf_verifier | Boundary | AddEFFI | 3 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 3 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 3 | STOREW | <div style='text-align: right'>2,266</div>  |
| leaf_verifier | Boundary | AddEFFI | 3 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 3 | ADD | <div style='text-align: right'>12,000</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 3 | ADD | <div style='text-align: right'>1,826</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 3 | ADD | <div style='text-align: right'>1,079</div>  |
| leaf_verifier | Boundary | AddEFI | 3 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 3 | ADD | <div style='text-align: right'>1,578,720</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 3 | ADD | <div style='text-align: right'>322,146</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 3 | ADD | <div style='text-align: right'>190,359</div>  |
| leaf_verifier | Boundary | AddEI | 3 | ADD | <div style='text-align: right'>129,360</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 3 | ADD | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 3 | ADD | <div style='text-align: right'>2,168,010</div>  |
| leaf_verifier | Boundary | AddFI | 3 | ADD | <div style='text-align: right'>627</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 3 | ADD | <div style='text-align: right'>804,180</div>  |
| leaf_verifier | Boundary | AddV | 3 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 3 | ADD | <div style='text-align: right'>17,910,120</div>  |
| leaf_verifier | Boundary | AddVI | 3 | ADD | <div style='text-align: right'>671</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 3 | ADD | <div style='text-align: right'>3,197,850</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 3 | LOADW | <div style='text-align: right'>4,370,395</div>  |
| leaf_verifier | Boundary | Alloc | 3 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 3 | MUL | <div style='text-align: right'>1,908,300</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 3 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 3 | MUL | <div style='text-align: right'>91</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 3 | BNE | <div style='text-align: right'>10,488</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 3 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 3 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 3 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 3 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 3 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 3 | BNE | <div style='text-align: right'>488,911</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 3 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 3 | BNE | <div style='text-align: right'>45,862</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 3 | BNE | <div style='text-align: right'>8,395</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 3 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 3 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 3 | PHANTOM | <div style='text-align: right'>62,496</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 3 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 3 | PHANTOM | <div style='text-align: right'>19,656</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 3 | PHANTOM | <div style='text-align: right'>1,375,920</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 3 | PHANTOM | <div style='text-align: right'>97,776</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 3 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 3 | PHANTOM | <div style='text-align: right'>19,656</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 3 | PHANTOM | <div style='text-align: right'>57,960</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 3 | PHANTOM | <div style='text-align: right'>57,960</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 3 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 3 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 3 | BBE4DIV | <div style='text-align: right'>393,840</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 3 | BBE4DIV | <div style='text-align: right'>181,280</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 3 | BBE4DIV | <div style='text-align: right'>107,120</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>3,120</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>3,344</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>1,976</div>  |
| leaf_verifier | Boundary | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 3 | STOREW | <div style='text-align: right'>12,792</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 3 | STOREW | <div style='text-align: right'>1,122</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 3 | STOREW | <div style='text-align: right'>312</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 3 | DIV | <div style='text-align: right'>5,580</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 3 | ADD | <div style='text-align: right'>21,776,010</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 3 | BNE | <div style='text-align: right'>18,517,415</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 3 | JAL | <div style='text-align: right'>792,380</div>  |
| leaf_verifier | AccessAdapter<2> | For | 3 | JAL | <div style='text-align: right'>594</div>  |
| leaf_verifier | AccessAdapter<4> | For | 3 | JAL | <div style='text-align: right'>702</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 3 | LOADW | <div style='text-align: right'>201,474</div>  |
| leaf_verifier | Boundary | For | 3 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 3 | STOREW | <div style='text-align: right'>3,047,284</div>  |
| leaf_verifier | Boundary | For | 3 | STOREW | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>229,944</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>135,876</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,095,872</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 3 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 3 | PHANTOM | <div style='text-align: right'>257,910</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 3 | BNE | <div style='text-align: right'>841,018</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 3 | BNE | <div style='text-align: right'>5,392,051</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 3 | JAL | <div style='text-align: right'>515,250</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 3 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 3 | BEQ | <div style='text-align: right'>702,006</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 3 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 3 | BEQ | <div style='text-align: right'>114,586</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 3 | STOREW | <div style='text-align: right'>227,960</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 3 | STOREW | <div style='text-align: right'>14,146</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 3 | STOREW | <div style='text-align: right'>8,359</div>  |
| leaf_verifier | Boundary | ImmE | 3 | STOREW | <div style='text-align: right'>12,408</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 3 | STOREW | <div style='text-align: right'>3,181,026</div>  |
| leaf_verifier | Boundary | ImmF | 3 | STOREW | <div style='text-align: right'>16,291</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 3 | STOREW | <div style='text-align: right'>2,145,858</div>  |
| leaf_verifier | Boundary | ImmV | 3 | STOREW | <div style='text-align: right'>1,716</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 3 | LOADW | <div style='text-align: right'>1,676,736</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 3 | LOADW | <div style='text-align: right'>276,650</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 3 | LOADW | <div style='text-align: right'>163,475</div>  |
| leaf_verifier | Boundary | LoadE | 3 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 3 | LOADW2 | <div style='text-align: right'>4,392,904</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 3 | LOADW2 | <div style='text-align: right'>110,066</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 3 | LOADW2 | <div style='text-align: right'>65,039</div>  |
| leaf_verifier | Boundary | LoadE | 3 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 3 | LOADW | <div style='text-align: right'>2,303,626</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 3 | LOADW | <div style='text-align: right'>103,972</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 3 | LOADW | <div style='text-align: right'>61,438</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 3 | LOADW | <div style='text-align: right'>40,171</div>  |
| leaf_verifier | Boundary | LoadF | 3 | LOADW | <div style='text-align: right'>14,905</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 3 | LOADW2 | <div style='text-align: right'>5,865,624</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 3 | LOADW2 | <div style='text-align: right'>1,562</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 3 | LOADW2 | <div style='text-align: right'>923</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 3 | LOADW2 | <div style='text-align: right'>969</div>  |
| leaf_verifier | Boundary | LoadF | 3 | LOADW2 | <div style='text-align: right'>649</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 3 | LOADW | <div style='text-align: right'>2,000,841</div>  |
| leaf_verifier | Boundary | LoadV | 3 | LOADW | <div style='text-align: right'>528</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 3 | LOADW2 | <div style='text-align: right'>13,335,332</div>  |
| leaf_verifier | Boundary | LoadV | 3 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 3 | BBE4MUL | <div style='text-align: right'>690,840</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 3 | BBE4MUL | <div style='text-align: right'>336,204</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 3 | BBE4MUL | <div style='text-align: right'>198,666</div>  |
| leaf_verifier | Boundary | MulE | 3 | BBE4MUL | <div style='text-align: right'>139,832</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 3 | MUL | <div style='text-align: right'>215,280</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 3 | MUL | <div style='text-align: right'>37,246</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 3 | MUL | <div style='text-align: right'>22,009</div>  |
| leaf_verifier | Boundary | MulEF | 3 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 3 | MUL | <div style='text-align: right'>18,240</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 3 | MUL | <div style='text-align: right'>3,432</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 3 | MUL | <div style='text-align: right'>2,028</div>  |
| leaf_verifier | Boundary | MulEFI | 3 | MUL | <div style='text-align: right'>1,232</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>90,960</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>103,818</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>61,347</div>  |
| leaf_verifier | Boundary | MulEI | 3 | BBE4MUL | <div style='text-align: right'>5,984</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 3 | STOREW | <div style='text-align: right'>372,936</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 3 | STOREW | <div style='text-align: right'>49,929</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 3 | STOREW | <div style='text-align: right'>29,458</div>  |
| leaf_verifier | Boundary | MulEI | 3 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 3 | MUL | <div style='text-align: right'>4,220,700</div>  |
| leaf_verifier | Boundary | MulF | 3 | MUL | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 3 | MUL | <div style='text-align: right'>80,880</div>  |
| leaf_verifier | Boundary | MulFI | 3 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 3 | MUL | <div style='text-align: right'>1,119,360</div>  |
| leaf_verifier | Boundary | MulVI | 3 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 3 | MUL | <div style='text-align: right'>8,520</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 3 | MUL | <div style='text-align: right'>2,332</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 3 | MUL | <div style='text-align: right'>1,378</div>  |
| leaf_verifier | Boundary | NegE | 3 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>1,337,556</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>790,374</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>516,783</div>  |
| leaf_verifier | Boundary | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>18,342,467</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>943,162</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>559,780</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>373,524</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>9,581,260</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 3 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 3 | STOREW | <div style='text-align: right'>1,972,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 3 | STOREW | <div style='text-align: right'>36,124</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 3 | STOREW | <div style='text-align: right'>21,346</div>  |
| leaf_verifier | Boundary | StoreE | 3 | STOREW | <div style='text-align: right'>529,144</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 3 | STOREW2 | <div style='text-align: right'>2,252,212</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 3 | STOREW2 | <div style='text-align: right'>229,152</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 3 | STOREW2 | <div style='text-align: right'>135,408</div>  |
| leaf_verifier | Boundary | StoreE | 3 | STOREW2 | <div style='text-align: right'>73,876</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 3 | STOREW | <div style='text-align: right'>2,672,872</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 3 | STOREW | <div style='text-align: right'>4,576</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 3 | STOREW | <div style='text-align: right'>2,704</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary | StoreF | 3 | STOREW | <div style='text-align: right'>707,960</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 3 | STOREW2 | <div style='text-align: right'>5,300,808</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 3 | STOREW2 | <div style='text-align: right'>516,428</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 3 | STOREW2 | <div style='text-align: right'>307,619</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 3 | STOREW2 | <div style='text-align: right'>208,301</div>  |
| leaf_verifier | Boundary | StoreF | 3 | STOREW2 | <div style='text-align: right'>154,198</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 3 | ADD | <div style='text-align: right'>11,708,280</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>17,873,007</div>  |
| leaf_verifier | Boundary | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>4,795,197</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 3 | STOREW | <div style='text-align: right'>233,864</div>  |
| leaf_verifier | Boundary | StoreV | 3 | STOREW | <div style='text-align: right'>62,744</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 3 | STOREW2 | <div style='text-align: right'>4,511,558</div>  |
| leaf_verifier | Boundary | StoreV | 3 | STOREW2 | <div style='text-align: right'>1,110,758</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 3 | FE4SUB | <div style='text-align: right'>261,640</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 3 | FE4SUB | <div style='text-align: right'>226,380</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 3 | FE4SUB | <div style='text-align: right'>133,770</div>  |
| leaf_verifier | Boundary | SubE | 3 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 3 | LOADW | <div style='text-align: right'>1,014,504</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 3 | LOADW | <div style='text-align: right'>90,486</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 3 | SUB | <div style='text-align: right'>247,440</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 3 | SUB | <div style='text-align: right'>90,486</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 3 | SUB | <div style='text-align: right'>106,938</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 3 | ADD | <div style='text-align: right'>11,400</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 3 | ADD | <div style='text-align: right'>2,090</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 3 | ADD | <div style='text-align: right'>1,235</div>  |
| leaf_verifier | Boundary | SubEFI | 3 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 3 | ADD | <div style='text-align: right'>18,720</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 3 | ADD | <div style='text-align: right'>4,950</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 3 | ADD | <div style='text-align: right'>2,925</div>  |
| leaf_verifier | Boundary | SubEI | 3 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 3 | SUB | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | SubFI | 3 | SUB | <div style='text-align: right'>14,630</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 3 | SUB | <div style='text-align: right'>2,494,440</div>  |
| leaf_verifier | Boundary | SubV | 3 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 3 | SUB | <div style='text-align: right'>58,320</div>  |
| leaf_verifier | Boundary | SubVI | 3 | SUB | <div style='text-align: right'>506</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 3 | SUB | <div style='text-align: right'>49,140</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,466.0</div>  | <div style='text-align: right'>22,480.0</div>  | <div style='text-align: right'>617,024,216</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,405.0</div>  | <div style='text-align: right'>22,335.0</div>  | <div style='text-align: right'>614,140,632</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,429.0</div>  | <div style='text-align: right'>22,348.0</div>  | <div style='text-align: right'>614,140,632</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>1,410.0</div>  | <div style='text-align: right'>22,434.0</div>  | <div style='text-align: right'>614,140,632</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,900.0</div>  | <div style='text-align: right'>286,062,644</div>  | <div style='text-align: right'>7,254,243</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>8,102.0</div>  | <div style='text-align: right'>286,037,417</div>  | <div style='text-align: right'>7,254,106</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>8,148.0</div>  | <div style='text-align: right'>286,742,205</div>  | <div style='text-align: right'>7,270,238</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>153,197</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>771,084</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>751,680</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>376,176</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,720</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,989</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,019</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,384,716</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>181,475</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,965,449</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>77,953</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,736</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,006</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>153,197</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>771,420</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>751,716</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>376,194</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>108,720</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>353,989</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,227,977</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,384,689</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>182,721</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,964,160</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,928</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,736</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>52,006</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>153,197</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>771,996</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>753,068</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>376,786</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,232</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>354,241</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,045</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,497</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>181,233</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,977,072</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,252</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,904</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,262</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>24,262</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>688</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>2,064</div>  |
| internal_verifier_height_0 | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_0 | AddEI | 0 | 0 | ADD | <div style='text-align: right'>65,424</div>  |
| internal_verifier_height_0 | AddF | 0 | 0 | ADD | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | AddFI | 0 | 0 | ADD | <div style='text-align: right'>149,511</div>  |
| internal_verifier_height_0 | AddV | 0 | 0 | ADD | <div style='text-align: right'>34,464</div>  |
| internal_verifier_height_0 | AddVI | 0 | 0 | ADD | <div style='text-align: right'>712,704</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | ADD | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | MUL | <div style='text-align: right'>68,640</div>  |
| internal_verifier_height_0 | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_0 | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_0 | AssertEqF | 0 | 0 | BNE | <div style='text-align: right'>21,593</div>  |
| internal_verifier_height_0 | AssertEqFI | 0 | 0 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_0 | AssertEqV | 0 | 0 | BNE | <div style='text-align: right'>2,272</div>  |
| internal_verifier_height_0 | AssertEqVI | 0 | 0 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_0 | AssertNeVI | 0 | 0 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 | CT-InitializePcsConst | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-ReadProofsFromInput | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-VerifyProofs | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-compute-reduced-opening | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-exp-reverse-bits-len | 0 | 0 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash | 0 | 0 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>237,552</div>  |
| internal_verifier_height_0 | CT-single-reduced-opening-eval | 0 | 0 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_0 | CT-stage-c-build-rounds | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-2-fri-fold | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-3-verify-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-verify-pcs | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-e-verify-constraints | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-verify-batch | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-verify-batch-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast | 0 | 0 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-query | 0 | 0 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_0 | CastFV | 0 | 0 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_0 | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 | For | 0 | 0 | ADD | <div style='text-align: right'>867,660</div>  |
| internal_verifier_height_0 | For | 0 | 0 | BNE | <div style='text-align: right'>958,635</div>  |
| internal_verifier_height_0 | For | 0 | 0 | JAL | <div style='text-align: right'>90,975</div>  |
| internal_verifier_height_0 | For | 0 | 0 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_0 | For | 0 | 0 | STOREW | <div style='text-align: right'>86,523</div>  |
| internal_verifier_height_0 | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 | HintBitsF | 0 | 0 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_0 | HintInputVec | 0 | 0 | PHANTOM | <div style='text-align: right'>47,777</div>  |
| internal_verifier_height_0 | IfEq | 0 | 0 | BNE | <div style='text-align: right'>43,896</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 0 | BNE | <div style='text-align: right'>319,960</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>90,497</div>  |
| internal_verifier_height_0 | IfNe | 0 | 0 | BEQ | <div style='text-align: right'>32,894</div>  |
| internal_verifier_height_0 | IfNe | 0 | 0 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | IfNeI | 0 | 0 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_0 | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>6,408</div>  |
| internal_verifier_height_0 | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>73,442</div>  |
| internal_verifier_height_0 | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>52,709</div>  |
| internal_verifier_height_0 | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>43,288</div>  |
| internal_verifier_height_0 | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_0 | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>57,564</div>  |
| internal_verifier_height_0 | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>159,224</div>  |
| internal_verifier_height_0 | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>53,856</div>  |
| internal_verifier_height_0 | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>468,104</div>  |
| internal_verifier_height_0 | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>31,495</div>  |
| internal_verifier_height_0 | MulEF | 0 | 0 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_0 | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>856</div>  |
| internal_verifier_height_0 | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_0 | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_0 | MulF | 0 | 0 | MUL | <div style='text-align: right'>292,770</div>  |
| internal_verifier_height_0 | MulFI | 0 | 0 | MUL | <div style='text-align: right'>2,698</div>  |
| internal_verifier_height_0 | MulVI | 0 | 0 | MUL | <div style='text-align: right'>42,370</div>  |
| internal_verifier_height_0 | NegE | 0 | 0 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_0 | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,608</div>  |
| internal_verifier_height_0 | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,398</div>  |
| internal_verifier_height_0 | Publish | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_0 | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_0 | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_0 | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>134,964</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 0 | ADD | <div style='text-align: right'>413,721</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>464,164</div>  |
| internal_verifier_height_0 | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_0 | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>132,470</div>  |
| internal_verifier_height_0 | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>7,268</div>  |
| internal_verifier_height_0 | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_0 | SubEF | 0 | 0 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_0 | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>448</div>  |
| internal_verifier_height_0 | SubEI | 0 | 0 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_0 | SubF | 0 | 0 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | SubFI | 0 | 0 | SUB | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | SubV | 0 | 0 | SUB | <div style='text-align: right'>162,992</div>  |
| internal_verifier_height_0 | SubVI | 0 | 0 | SUB | <div style='text-align: right'>2,178</div>  |
| internal_verifier_height_0 | SubVIN | 0 | 0 | SUB | <div style='text-align: right'>1,848</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>24,262</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>688</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>2,064</div>  |
| internal_verifier_height_0 | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_0 | AddEI | 0 | 1 | ADD | <div style='text-align: right'>65,424</div>  |
| internal_verifier_height_0 | AddF | 0 | 1 | ADD | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | AddFI | 0 | 1 | ADD | <div style='text-align: right'>149,259</div>  |
| internal_verifier_height_0 | AddV | 0 | 1 | ADD | <div style='text-align: right'>34,462</div>  |
| internal_verifier_height_0 | AddVI | 0 | 1 | ADD | <div style='text-align: right'>712,452</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | ADD | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | MUL | <div style='text-align: right'>68,640</div>  |
| internal_verifier_height_0 | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_0 | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_0 | AssertEqF | 0 | 1 | BNE | <div style='text-align: right'>21,593</div>  |
| internal_verifier_height_0 | AssertEqFI | 0 | 1 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_0 | AssertEqV | 0 | 1 | BNE | <div style='text-align: right'>2,272</div>  |
| internal_verifier_height_0 | AssertEqVI | 0 | 1 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_0 | AssertNeVI | 0 | 1 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 | CT-InitializePcsConst | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-ReadProofsFromInput | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-VerifyProofs | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-compute-reduced-opening | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-exp-reverse-bits-len | 0 | 1 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash | 0 | 1 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>237,552</div>  |
| internal_verifier_height_0 | CT-single-reduced-opening-eval | 0 | 1 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_0 | CT-stage-c-build-rounds | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-2-fri-fold | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-3-verify-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-verify-pcs | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-e-verify-constraints | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-verify-batch | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-verify-batch-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast | 0 | 1 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-query | 0 | 1 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_0 | CastFV | 0 | 1 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_0 | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 | For | 0 | 1 | ADD | <div style='text-align: right'>867,633</div>  |
| internal_verifier_height_0 | For | 0 | 1 | BNE | <div style='text-align: right'>958,608</div>  |
| internal_verifier_height_0 | For | 0 | 1 | JAL | <div style='text-align: right'>90,975</div>  |
| internal_verifier_height_0 | For | 0 | 1 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_0 | For | 0 | 1 | STOREW | <div style='text-align: right'>86,523</div>  |
| internal_verifier_height_0 | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 | HintBitsF | 0 | 1 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_0 | HintInputVec | 0 | 1 | PHANTOM | <div style='text-align: right'>47,777</div>  |
| internal_verifier_height_0 | IfEq | 0 | 1 | BNE | <div style='text-align: right'>44,148</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 1 | BNE | <div style='text-align: right'>319,708</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>91,743</div>  |
| internal_verifier_height_0 | IfNe | 0 | 1 | BEQ | <div style='text-align: right'>32,894</div>  |
| internal_verifier_height_0 | IfNe | 0 | 1 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | IfNeI | 0 | 1 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_0 | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>6,408</div>  |
| internal_verifier_height_0 | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>73,442</div>  |
| internal_verifier_height_0 | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>52,667</div>  |
| internal_verifier_height_0 | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>43,288</div>  |
| internal_verifier_height_0 | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_0 | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>57,564</div>  |
| internal_verifier_height_0 | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>159,224</div>  |
| internal_verifier_height_0 | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>53,856</div>  |
| internal_verifier_height_0 | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>468,104</div>  |
| internal_verifier_height_0 | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>31,470</div>  |
| internal_verifier_height_0 | MulEF | 0 | 1 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_0 | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>856</div>  |
| internal_verifier_height_0 | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_0 | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_0 | MulF | 0 | 1 | MUL | <div style='text-align: right'>292,266</div>  |
| internal_verifier_height_0 | MulFI | 0 | 1 | MUL | <div style='text-align: right'>2,698</div>  |
| internal_verifier_height_0 | MulVI | 0 | 1 | MUL | <div style='text-align: right'>42,370</div>  |
| internal_verifier_height_0 | NegE | 0 | 1 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_0 | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>34,608</div>  |
| internal_verifier_height_0 | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,398</div>  |
| internal_verifier_height_0 | Publish | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_0 | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_0 | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_0 | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>134,964</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 1 | ADD | <div style='text-align: right'>413,721</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>464,164</div>  |
| internal_verifier_height_0 | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_0 | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>132,470</div>  |
| internal_verifier_height_0 | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>7,268</div>  |
| internal_verifier_height_0 | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_0 | SubEF | 0 | 1 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_0 | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>448</div>  |
| internal_verifier_height_0 | SubEI | 0 | 1 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_0 | SubF | 0 | 1 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | SubFI | 0 | 1 | SUB | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | SubV | 0 | 1 | SUB | <div style='text-align: right'>162,740</div>  |
| internal_verifier_height_0 | SubVI | 0 | 1 | SUB | <div style='text-align: right'>2,178</div>  |
| internal_verifier_height_0 | SubVIN | 0 | 1 | SUB | <div style='text-align: right'>1,848</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>24,324</div>  |
| internal_verifier_height_1 | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>720</div>  |
| internal_verifier_height_1 | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>2,160</div>  |
| internal_verifier_height_1 | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_1 | AddEI | 1 | 2 | ADD | <div style='text-align: right'>65,792</div>  |
| internal_verifier_height_1 | AddF | 1 | 2 | ADD | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_1 | AddFI | 1 | 2 | ADD | <div style='text-align: right'>151,155</div>  |
| internal_verifier_height_1 | AddV | 1 | 2 | ADD | <div style='text-align: right'>34,478</div>  |
| internal_verifier_height_1 | AddVI | 1 | 2 | ADD | <div style='text-align: right'>715,036</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | ADD | <div style='text-align: right'>116,585</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>116,585</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | MUL | <div style='text-align: right'>68,724</div>  |
| internal_verifier_height_1 | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_1 | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_1 | AssertEqF | 1 | 2 | BNE | <div style='text-align: right'>21,609</div>  |
| internal_verifier_height_1 | AssertEqFI | 1 | 2 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_1 | AssertEqV | 1 | 2 | BNE | <div style='text-align: right'>2,272</div>  |
| internal_verifier_height_1 | AssertEqVI | 1 | 2 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_1 | AssertNeVI | 1 | 2 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_1 | CT-InitializePcsConst | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-ReadProofsFromInput | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-VerifyProofs | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-compute-reduced-opening | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | CT-exp-reverse-bits-len | 1 | 2 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash | 1 | 2 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>237,720</div>  |
| internal_verifier_height_1 | CT-single-reduced-opening-eval | 1 | 2 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_1 | CT-stage-c-build-rounds | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-2-fri-fold | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-3-verify-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-verify-pcs | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-e-verify-constraints | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-verify-batch | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | CT-verify-batch-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_1 | CT-verify-batch-reduce-fast | 1 | 2 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_1 | CT-verify-batch-reduce-fast-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_1 | CT-verify-query | 1 | 2 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_1 | CastFV | 1 | 2 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_1 | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_1 | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_1 | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_1 | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 | For | 1 | 2 | ADD | <div style='text-align: right'>868,973</div>  |
| internal_verifier_height_1 | For | 1 | 2 | BNE | <div style='text-align: right'>960,032</div>  |
| internal_verifier_height_1 | For | 1 | 2 | JAL | <div style='text-align: right'>91,059</div>  |
| internal_verifier_height_1 | For | 1 | 2 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_1 | For | 1 | 2 | STOREW | <div style='text-align: right'>86,607</div>  |
| internal_verifier_height_1 | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 | HintBitsF | 1 | 2 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_1 | HintInputVec | 1 | 2 | PHANTOM | <div style='text-align: right'>47,861</div>  |
| internal_verifier_height_1 | IfEq | 1 | 2 | BNE | <div style='text-align: right'>42,384</div>  |
| internal_verifier_height_1 | IfEqI | 1 | 2 | BNE | <div style='text-align: right'>321,756</div>  |
| internal_verifier_height_1 | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>90,171</div>  |
| internal_verifier_height_1 | IfNe | 1 | 2 | BEQ | <div style='text-align: right'>32,978</div>  |
| internal_verifier_height_1 | IfNe | 1 | 2 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | IfNeI | 1 | 2 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_1 | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>6,536</div>  |
| internal_verifier_height_1 | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>73,442</div>  |
| internal_verifier_height_1 | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>52,523</div>  |
| internal_verifier_height_1 | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>43,304</div>  |
| internal_verifier_height_1 | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_1 | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>57,628</div>  |
| internal_verifier_height_1 | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>159,340</div>  |
| internal_verifier_height_1 | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>53,940</div>  |
| internal_verifier_height_1 | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>468,356</div>  |
| internal_verifier_height_1 | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>31,732</div>  |
| internal_verifier_height_1 | MulEF | 1 | 2 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_1 | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>856</div>  |
| internal_verifier_height_1 | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_1 | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_1 | MulF | 1 | 2 | MUL | <div style='text-align: right'>295,962</div>  |
| internal_verifier_height_1 | MulFI | 1 | 2 | MUL | <div style='text-align: right'>2,698</div>  |
| internal_verifier_height_1 | MulVI | 1 | 2 | MUL | <div style='text-align: right'>42,454</div>  |
| internal_verifier_height_1 | NegE | 1 | 2 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_1 | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_1 | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,570</div>  |
| internal_verifier_height_1 | Publish | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_1 | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_1 | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_1 | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>135,080</div>  |
| internal_verifier_height_1 | StoreHintWord | 1 | 2 | ADD | <div style='text-align: right'>414,525</div>  |
| internal_verifier_height_1 | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>465,052</div>  |
| internal_verifier_height_1 | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_1 | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>132,638</div>  |
| internal_verifier_height_1 | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>7,268</div>  |
| internal_verifier_height_1 | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_1 | SubEF | 1 | 2 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_1 | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>472</div>  |
| internal_verifier_height_1 | SubEI | 1 | 2 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_1 | SubF | 1 | 2 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_1 | SubFI | 1 | 2 | SUB | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_1 | SubV | 1 | 2 | SUB | <div style='text-align: right'>164,588</div>  |
| internal_verifier_height_1 | SubVI | 1 | 2 | SUB | <div style='text-align: right'>2,178</div>  |
| internal_verifier_height_1 | SubVIN | 1 | 2 | SUB | <div style='text-align: right'>1,848</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>970,480</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>424,468</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>250,822</div>  |
| internal_verifier_height_0 | Boundary | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>95,480</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>28,208</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>4,147</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>4,901</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>1,100</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>84,624</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>4,147</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>3,300</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>3,938</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>2,327</div>  |
| internal_verifier_height_0 | Boundary | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>1,540</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>1,962,720</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>413,116</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>244,114</div>  |
| internal_verifier_height_0 | Boundary | AddEI | 0 | 0 | ADD | <div style='text-align: right'>160,908</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 0 | 0 | ADD | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | 0 | ADD | <div style='text-align: right'>4,485,330</div>  |
| internal_verifier_height_0 | Boundary | AddFI | 0 | 0 | ADD | <div style='text-align: right'>1,221</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | 0 | ADD | <div style='text-align: right'>1,033,920</div>  |
| internal_verifier_height_0 | Boundary | AddV | 0 | 0 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | 0 | ADD | <div style='text-align: right'>21,381,120</div>  |
| internal_verifier_height_0 | Boundary | AddVI | 0 | 0 | ADD | <div style='text-align: right'>1,144</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 0 | ADD | <div style='text-align: right'>3,492,510</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>4,773,097</div>  |
| internal_verifier_height_0 | Boundary | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>2,059,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | 0 | BNE | <div style='text-align: right'>496,639</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | 0 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | 0 | BNE | <div style='text-align: right'>52,256</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | 0 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | 0 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_0 | PhantomAir | CT-InitializePcsConst | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-ReadProofsFromInput | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-VerifyProofs | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-compute-reduced-opening | 0 | 0 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-exp-reverse-bits-len | 0 | 0 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash | 0 | 0 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>1,425,312</div>  |
| internal_verifier_height_0 | PhantomAir | CT-single-reduced-opening-eval | 0 | 0 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-c-build-rounds | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-2-fri-fold | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-3-verify-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-verify-pcs | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-e-verify-constraints | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch | 0 | 0 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast | 0 | 0 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-query | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | 0 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | CastFV | 0 | 0 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>17,204</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>10,166</div>  |
| internal_verifier_height_0 | Boundary | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | 0 | ADD | <div style='text-align: right'>26,029,800</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | 0 | BNE | <div style='text-align: right'>22,048,605</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | For | 0 | 0 | JAL | <div style='text-align: right'>909,750</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | For | 0 | 0 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | For | 0 | 0 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 0 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 0 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 0 | STOREW | <div style='text-align: right'>3,547,443</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 0 | STOREW | <div style='text-align: right'>880</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,076</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,999,104</div>  |
| internal_verifier_height_0 | PhantomAir | HintBitsF | 0 | 0 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_0 | PhantomAir | HintInputVec | 0 | 0 | PHANTOM | <div style='text-align: right'>286,662</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | 0 | BNE | <div style='text-align: right'>1,009,608</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | 0 | BNE | <div style='text-align: right'>7,359,080</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>904,970</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>13</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | 0 | BEQ | <div style='text-align: right'>756,562</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | 0 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | 0 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>262,728</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>17,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>10,244</div>  |
| internal_verifier_height_0 | Boundary | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>13,288</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>3,011,122</div>  |
| internal_verifier_height_0 | Boundary | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>17,193</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>2,161,069</div>  |
| internal_verifier_height_0 | Boundary | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>1,309</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>1,774,808</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>284,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>168,194</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>2,360,124</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>15,136</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>6,528,184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>649</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>2,208,096</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>572</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>19,192,264</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,259,800</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>457,930</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>270,595</div>  |
| internal_verifier_height_0 | Boundary | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>149,380</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>49,852</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>29,458</div>  |
| internal_verifier_height_0 | Boundary | MulEF | 0 | 0 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>25,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>4,818</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>2,847</div>  |
| internal_verifier_height_0 | Boundary | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>90,904</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>53,716</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | 0 | MUL | <div style='text-align: right'>8,783,100</div>  |
| internal_verifier_height_0 | Boundary | MulF | 0 | 0 | MUL | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | 0 | MUL | <div style='text-align: right'>80,940</div>  |
| internal_verifier_height_0 | Boundary | MulFI | 0 | 0 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | 0 | MUL | <div style='text-align: right'>1,271,100</div>  |
| internal_verifier_height_0 | Boundary | MulVI | 0 | 0 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | 0 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | NegE | 0 | 0 | MUL | <div style='text-align: right'>2,464</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | NegE | 0 | 0 | MUL | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | Boundary | NegE | 0 | 0 | MUL | <div style='text-align: right'>924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,434,048</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>847,392</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>554,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,345,872</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>941,160</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>558,324</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,056</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,725,482</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | 0 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>5,533,524</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>554,972</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>330,122</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>220,456</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | 0 | ADD | <div style='text-align: right'>12,411,630</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>19,030,724</div>  |
| internal_verifier_height_0 | Boundary | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>5,105,804</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>5,431,270</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>1,317,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>290,720</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>250,778</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>148,187</div>  |
| internal_verifier_height_0 | Boundary | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>25,344</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>13,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>1,980</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>1,170</div>  |
| internal_verifier_height_0 | Boundary | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>24,354</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>14,391</div>  |
| internal_verifier_height_0 | Boundary | SubEI | 0 | 0 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 0 | 0 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | SubF | 0 | 0 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 0 | 0 | SUB | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | Boundary | SubFI | 0 | 0 | SUB | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | 0 | SUB | <div style='text-align: right'>4,889,760</div>  |
| internal_verifier_height_0 | Boundary | SubV | 0 | 0 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | 0 | SUB | <div style='text-align: right'>65,340</div>  |
| internal_verifier_height_0 | Boundary | SubVI | 0 | 0 | SUB | <div style='text-align: right'>506</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | 0 | SUB | <div style='text-align: right'>55,440</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>970,480</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>424,468</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>250,822</div>  |
| internal_verifier_height_0 | Boundary | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>95,480</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>28,208</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>4,147</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>4,901</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>1,100</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>84,624</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>4,147</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>3,300</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>3,938</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>2,327</div>  |
| internal_verifier_height_0 | Boundary | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>1,540</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>1,962,720</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>413,314</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>244,231</div>  |
| internal_verifier_height_0 | Boundary | AddEI | 0 | 1 | ADD | <div style='text-align: right'>160,908</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 0 | 1 | ADD | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | 1 | ADD | <div style='text-align: right'>4,477,770</div>  |
| internal_verifier_height_0 | Boundary | AddFI | 0 | 1 | ADD | <div style='text-align: right'>1,221</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | 1 | ADD | <div style='text-align: right'>1,033,860</div>  |
| internal_verifier_height_0 | Boundary | AddV | 0 | 1 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | 1 | ADD | <div style='text-align: right'>21,373,560</div>  |
| internal_verifier_height_0 | Boundary | AddVI | 0 | 1 | ADD | <div style='text-align: right'>1,144</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 1 | ADD | <div style='text-align: right'>3,492,510</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>4,773,097</div>  |
| internal_verifier_height_0 | Boundary | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>2,059,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | 1 | BNE | <div style='text-align: right'>496,639</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | 1 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | 1 | BNE | <div style='text-align: right'>52,256</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | 1 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | 1 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_0 | PhantomAir | CT-InitializePcsConst | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-ReadProofsFromInput | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-VerifyProofs | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-compute-reduced-opening | 0 | 1 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-exp-reverse-bits-len | 0 | 1 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash | 0 | 1 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>1,425,312</div>  |
| internal_verifier_height_0 | PhantomAir | CT-single-reduced-opening-eval | 0 | 1 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-c-build-rounds | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-2-fri-fold | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-3-verify-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-verify-pcs | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-e-verify-constraints | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch | 0 | 1 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast | 0 | 1 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-query | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | 1 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | CastFV | 0 | 1 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>17,204</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>10,166</div>  |
| internal_verifier_height_0 | Boundary | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | 1 | ADD | <div style='text-align: right'>26,028,990</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | 1 | BNE | <div style='text-align: right'>22,047,984</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | For | 0 | 1 | JAL | <div style='text-align: right'>909,750</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | For | 0 | 1 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | For | 0 | 1 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 1 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 1 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 1 | STOREW | <div style='text-align: right'>3,547,443</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 1 | STOREW | <div style='text-align: right'>880</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,076</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,999,104</div>  |
| internal_verifier_height_0 | PhantomAir | HintBitsF | 0 | 1 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_0 | PhantomAir | HintInputVec | 0 | 1 | PHANTOM | <div style='text-align: right'>286,662</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | 1 | BNE | <div style='text-align: right'>1,015,404</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | 1 | BNE | <div style='text-align: right'>7,353,284</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>917,430</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>13</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | 1 | BEQ | <div style='text-align: right'>756,562</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | 1 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | 1 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>262,728</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>17,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>10,244</div>  |
| internal_verifier_height_0 | Boundary | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>13,288</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>3,011,122</div>  |
| internal_verifier_height_0 | Boundary | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>17,193</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>2,159,347</div>  |
| internal_verifier_height_0 | Boundary | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>1,309</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>1,774,808</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>284,636</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>168,194</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>2,360,124</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>15,136</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>6,528,184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>649</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>2,208,096</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>572</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>19,192,264</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,258,800</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>458,128</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>270,712</div>  |
| internal_verifier_height_0 | Boundary | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>149,380</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>49,852</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>29,458</div>  |
| internal_verifier_height_0 | Boundary | MulEF | 0 | 1 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>25,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>4,818</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>2,847</div>  |
| internal_verifier_height_0 | Boundary | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>90,904</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>53,716</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | 1 | MUL | <div style='text-align: right'>8,767,980</div>  |
| internal_verifier_height_0 | Boundary | MulF | 0 | 1 | MUL | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | 1 | MUL | <div style='text-align: right'>80,940</div>  |
| internal_verifier_height_0 | Boundary | MulFI | 0 | 1 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | 1 | MUL | <div style='text-align: right'>1,271,100</div>  |
| internal_verifier_height_0 | Boundary | MulVI | 0 | 1 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | 1 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | NegE | 0 | 1 | MUL | <div style='text-align: right'>2,464</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | NegE | 0 | 1 | MUL | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | Boundary | NegE | 0 | 1 | MUL | <div style='text-align: right'>924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,434,048</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>847,392</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>554,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>19,345,872</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>941,160</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>558,324</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>370,056</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,725,482</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | 1 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>5,533,524</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>554,972</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>330,122</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>220,456</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | 1 | ADD | <div style='text-align: right'>12,411,630</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>19,030,724</div>  |
| internal_verifier_height_0 | Boundary | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>5,105,804</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>5,431,270</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>1,321,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>290,720</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>250,778</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>148,187</div>  |
| internal_verifier_height_0 | Boundary | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>25,344</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>13,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>1,980</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>1,170</div>  |
| internal_verifier_height_0 | Boundary | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>24,354</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>14,391</div>  |
| internal_verifier_height_0 | Boundary | SubEI | 0 | 1 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 0 | 1 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | SubF | 0 | 1 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 0 | 1 | SUB | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | Boundary | SubFI | 0 | 1 | SUB | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | 1 | SUB | <div style='text-align: right'>4,882,200</div>  |
| internal_verifier_height_0 | Boundary | SubV | 0 | 1 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | 1 | SUB | <div style='text-align: right'>65,340</div>  |
| internal_verifier_height_0 | Boundary | SubVI | 0 | 1 | SUB | <div style='text-align: right'>506</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | 1 | SUB | <div style='text-align: right'>55,440</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>972,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>424,160</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>250,640</div>  |
| internal_verifier_height_1 | Boundary | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>94,468</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>29,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>4,180</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>4,940</div>  |
| internal_verifier_height_1 | Boundary | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>1,276</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>88,560</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>4,180</div>  |
| internal_verifier_height_1 | Boundary | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>3,828</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>3,894</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>2,301</div>  |
| internal_verifier_height_1 | Boundary | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>1,496</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>1,973,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>415,492</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>245,518</div>  |
| internal_verifier_height_1 | Boundary | AddEI | 1 | 2 | ADD | <div style='text-align: right'>161,348</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 1 | 2 | ADD | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 1 | 2 | ADD | <div style='text-align: right'>4,534,650</div>  |
| internal_verifier_height_1 | Boundary | AddFI | 1 | 2 | ADD | <div style='text-align: right'>1,309</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 1 | 2 | ADD | <div style='text-align: right'>1,034,340</div>  |
| internal_verifier_height_1 | Boundary | AddV | 1 | 2 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 1 | 2 | ADD | <div style='text-align: right'>21,451,080</div>  |
| internal_verifier_height_1 | Boundary | AddVI | 1 | 2 | ADD | <div style='text-align: right'>1,144</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | 2 | ADD | <div style='text-align: right'>3,497,550</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>4,779,985</div>  |
| internal_verifier_height_1 | Boundary | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>2,061,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 1 | 2 | BNE | <div style='text-align: right'>497,007</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 1 | 2 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 1 | 2 | BNE | <div style='text-align: right'>52,256</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 1 | 2 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 1 | 2 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_1 | PhantomAir | CT-InitializePcsConst | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-ReadProofsFromInput | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-VerifyProofs | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-compute-reduced-opening | 1 | 2 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_1 | PhantomAir | CT-exp-reverse-bits-len | 1 | 2 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash | 1 | 2 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>1,426,320</div>  |
| internal_verifier_height_1 | PhantomAir | CT-single-reduced-opening-eval | 1 | 2 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-c-build-rounds | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-2-fri-fold | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-3-verify-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-verify-pcs | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-e-verify-constraints | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch | 1 | 2 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-reduce-fast | 1 | 2 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-reduce-fast-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-query | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 1 | 2 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_1 | Boundary | CastFV | 1 | 2 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>17,050</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>10,075</div>  |
| internal_verifier_height_1 | Boundary | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 1 | 2 | ADD | <div style='text-align: right'>26,069,190</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 1 | 2 | BNE | <div style='text-align: right'>22,080,736</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | For | 1 | 2 | JAL | <div style='text-align: right'>910,590</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | For | 1 | 2 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | For | 1 | 2 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | 2 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_1 | Boundary | For | 1 | 2 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | 2 | STOREW | <div style='text-align: right'>3,550,887</div>  |
| internal_verifier_height_1 | Boundary | For | 1 | 2 | STOREW | <div style='text-align: right'>880</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,832</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,128</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,009,856</div>  |
| internal_verifier_height_1 | PhantomAir | HintBitsF | 1 | 2 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_1 | PhantomAir | HintInputVec | 1 | 2 | PHANTOM | <div style='text-align: right'>287,166</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 1 | 2 | BNE | <div style='text-align: right'>974,832</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 1 | 2 | BNE | <div style='text-align: right'>7,400,388</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>901,710</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>26</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 1 | 2 | BEQ | <div style='text-align: right'>758,494</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 1 | 2 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 1 | 2 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>267,976</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>17,754</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>10,491</div>  |
| internal_verifier_height_1 | Boundary | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>13,640</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>3,011,122</div>  |
| internal_verifier_height_1 | Boundary | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>17,193</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>2,153,443</div>  |
| internal_verifier_height_1 | Boundary | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>1,309</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>1,775,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>284,856</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>168,324</div>  |
| internal_verifier_height_1 | Boundary | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_1 | Boundary | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>2,362,748</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>15,312</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>6,532,940</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>649</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>2,211,540</div>  |
| internal_verifier_height_1 | Boundary | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>572</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>19,202,596</div>  |
| internal_verifier_height_1 | Boundary | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,269,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>461,516</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>272,714</div>  |
| internal_verifier_height_1 | Boundary | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>149,160</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>49,522</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>29,263</div>  |
| internal_verifier_height_1 | Boundary | MulEF | 1 | 2 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>25,680</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>4,686</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>2,769</div>  |
| internal_verifier_height_1 | Boundary | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>90,728</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>53,612</div>  |
| internal_verifier_height_1 | Boundary | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_1 | Boundary | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 1 | 2 | MUL | <div style='text-align: right'>8,878,860</div>  |
| internal_verifier_height_1 | Boundary | MulF | 1 | 2 | MUL | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 1 | 2 | MUL | <div style='text-align: right'>80,940</div>  |
| internal_verifier_height_1 | Boundary | MulFI | 1 | 2 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 1 | 2 | MUL | <div style='text-align: right'>1,273,620</div>  |
| internal_verifier_height_1 | Boundary | MulVI | 1 | 2 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 1 | 2 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | NegE | 1 | 2 | MUL | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | NegE | 1 | 2 | MUL | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_1 | Boundary | NegE | 1 | 2 | MUL | <div style='text-align: right'>836</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>942,260</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>558,428</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>372,980</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,821,630</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 1 | 2 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_1 | Boundary | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_1 | Boundary | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_1 | Boundary | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>5,538,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>556,072</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>330,226</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>223,380</div>  |
| internal_verifier_height_1 | Boundary | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 1 | 2 | ADD | <div style='text-align: right'>12,435,750</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>19,067,132</div>  |
| internal_verifier_height_1 | Boundary | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>5,115,572</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_1 | Boundary | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>5,438,158</div>  |
| internal_verifier_height_1 | Boundary | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>1,317,316</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>290,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>250,580</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>148,070</div>  |
| internal_verifier_height_1 | Boundary | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>25,212</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>14,160</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>2,068</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>1,222</div>  |
| internal_verifier_height_1 | Boundary | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>24,398</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>14,417</div>  |
| internal_verifier_height_1 | Boundary | SubEI | 1 | 2 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 1 | 2 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_1 | Boundary | SubF | 1 | 2 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 1 | 2 | SUB | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_1 | Boundary | SubFI | 1 | 2 | SUB | <div style='text-align: right'>14,630</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 1 | 2 | SUB | <div style='text-align: right'>4,937,640</div>  |
| internal_verifier_height_1 | Boundary | SubV | 1 | 2 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 1 | 2 | SUB | <div style='text-align: right'>65,340</div>  |
| internal_verifier_height_1 | Boundary | SubVI | 1 | 2 | SUB | <div style='text-align: right'>506</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 1 | 2 | SUB | <div style='text-align: right'>55,440</div>  |

| group | air_name | height | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramAir | 0 | 0 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | ProgramAir | 0 | 1 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | ProgramAir | 1 | 2 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_1 | VolatileBoundaryAir | 1 | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<2> | 1 | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<4> | 1 | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | AccessAdapterAir<8> | 1 | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | height | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,709.0</div>  | <div style='text-align: right'>26,200.0</div>  | <div style='text-align: right'>759,630,616</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,696.0</div>  | <div style='text-align: right'>26,291.0</div>  | <div style='text-align: right'>759,630,616</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,711.0</div>  | <div style='text-align: right'>26,289.0</div>  | <div style='text-align: right'>759,630,616</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| root_verifier | ProgramChip | <div style='text-align: right'>153,470</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>411,630</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>387,480</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>193,866</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,682</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>177,123</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,667</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,274</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>90,514</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,488,647</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,126</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,452</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,143</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| root_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| root_verifier | AddE | FE4ADD | <div style='text-align: right'>12,162</div>  |
| root_verifier | AddEFFI | LOADW | <div style='text-align: right'>360</div>  |
| root_verifier | AddEFFI | STOREW | <div style='text-align: right'>1,080</div>  |
| root_verifier | AddEFI | ADD | <div style='text-align: right'>332</div>  |
| root_verifier | AddEI | ADD | <div style='text-align: right'>32,896</div>  |
| root_verifier | AddF | ADD | <div style='text-align: right'>1,333</div>  |
| root_verifier | AddFI | ADD | <div style='text-align: right'>75,590</div>  |
| root_verifier | AddV | ADD | <div style='text-align: right'>17,239</div>  |
| root_verifier | AddVI | ADD | <div style='text-align: right'>357,538</div>  |
| root_verifier | Alloc | ADD | <div style='text-align: right'>58,302</div>  |
| root_verifier | Alloc | LOADW | <div style='text-align: right'>58,302</div>  |
| root_verifier | Alloc | MUL | <div style='text-align: right'>34,374</div>  |
| root_verifier | AssertEqE | BNE | <div style='text-align: right'>232</div>  |
| root_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| root_verifier | AssertEqF | BNE | <div style='text-align: right'>10,800</div>  |
| root_verifier | AssertEqFI | BNE | <div style='text-align: right'>5</div>  |
| root_verifier | AssertEqV | BNE | <div style='text-align: right'>1,136</div>  |
| root_verifier | AssertEqVI | BNE | <div style='text-align: right'>221</div>  |
| root_verifier | AssertNeVI | BEQ | <div style='text-align: right'>1</div>  |
| root_verifier | CT-ExtractPublicValues | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-ReadProofsFromInput | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-VerifyProofs | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>7,644</div>  |
| root_verifier | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>2,520</div>  |
| root_verifier | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>1,848</div>  |
| root_verifier | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>118,860</div>  |
| root_verifier | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>10,584</div>  |
| root_verifier | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-verify-batch | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>1,848</div>  |
| root_verifier | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>4,368</div>  |
| root_verifier | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>4,368</div>  |
| root_verifier | CT-verify-query | PHANTOM | <div style='text-align: right'>84</div>  |
| root_verifier | CastFV | ADD | <div style='text-align: right'>8</div>  |
| root_verifier | DivE | BBE4DIV | <div style='text-align: right'>6,248</div>  |
| root_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>174</div>  |
| root_verifier | DivEIN | STOREW | <div style='text-align: right'>696</div>  |
| root_verifier | DivFIN | DIV | <div style='text-align: right'>364</div>  |
| root_verifier | For | ADD | <div style='text-align: right'>434,515</div>  |
| root_verifier | For | BNE | <div style='text-align: right'>480,043</div>  |
| root_verifier | For | JAL | <div style='text-align: right'>45,528</div>  |
| root_verifier | For | LOADW | <div style='text-align: right'>2,226</div>  |
| root_verifier | For | STOREW | <div style='text-align: right'>43,302</div>  |
| root_verifier | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier | HintBitsF | PHANTOM | <div style='text-align: right'>43</div>  |
| root_verifier | HintInputVec | PHANTOM | <div style='text-align: right'>23,928</div>  |
| root_verifier | IfEq | BNE | <div style='text-align: right'>21,192</div>  |
| root_verifier | IfEqI | BNE | <div style='text-align: right'>160,878</div>  |
| root_verifier | IfEqI | JAL | <div style='text-align: right'>44,984</div>  |
| root_verifier | IfNe | BEQ | <div style='text-align: right'>16,489</div>  |
| root_verifier | IfNe | JAL | <div style='text-align: right'>1</div>  |
| root_verifier | IfNeI | BEQ | <div style='text-align: right'>2,273</div>  |
| root_verifier | ImmE | STOREW | <div style='text-align: right'>3,268</div>  |
| root_verifier | ImmF | STOREW | <div style='text-align: right'>36,814</div>  |
| root_verifier | ImmV | STOREW | <div style='text-align: right'>26,332</div>  |
| root_verifier | LoadE | LOADW | <div style='text-align: right'>21,652</div>  |
| root_verifier | LoadE | LOADW2 | <div style='text-align: right'>70,292</div>  |
| root_verifier | LoadF | LOADW | <div style='text-align: right'>28,946</div>  |
| root_verifier | LoadF | LOADW2 | <div style='text-align: right'>79,670</div>  |
| root_verifier | LoadV | LOADW | <div style='text-align: right'>26,968</div>  |
| root_verifier | LoadV | LOADW2 | <div style='text-align: right'>234,178</div>  |
| root_verifier | MulE | BBE4MUL | <div style='text-align: right'>15,866</div>  |
| root_verifier | MulEF | MUL | <div style='text-align: right'>5,088</div>  |
| root_verifier | MulEFI | MUL | <div style='text-align: right'>428</div>  |
| root_verifier | MulEI | BBE4MUL | <div style='text-align: right'>1,042</div>  |
| root_verifier | MulEI | STOREW | <div style='text-align: right'>4,168</div>  |
| root_verifier | MulF | MUL | <div style='text-align: right'>147,981</div>  |
| root_verifier | MulFI | MUL | <div style='text-align: right'>1,349</div>  |
| root_verifier | MulVI | MUL | <div style='text-align: right'>21,227</div>  |
| root_verifier | NegE | MUL | <div style='text-align: right'>188</div>  |
| root_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>17,358</div>  |
| root_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>8,785</div>  |
| root_verifier | Publish | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier | StoreE | STOREW | <div style='text-align: right'>25,744</div>  |
| root_verifier | StoreE | STOREW2 | <div style='text-align: right'>38,060</div>  |
| root_verifier | StoreF | STOREW | <div style='text-align: right'>27,918</div>  |
| root_verifier | StoreF | STOREW2 | <div style='text-align: right'>67,540</div>  |
| root_verifier | StoreHintWord | ADD | <div style='text-align: right'>207,291</div>  |
| root_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>232,552</div>  |
| root_verifier | StoreV | STOREW | <div style='text-align: right'>3,294</div>  |
| root_verifier | StoreV | STOREW2 | <div style='text-align: right'>66,319</div>  |
| root_verifier | SubE | FE4SUB | <div style='text-align: right'>3,634</div>  |
| root_verifier | SubEF | LOADW | <div style='text-align: right'>15,984</div>  |
| root_verifier | SubEF | SUB | <div style='text-align: right'>5,328</div>  |
| root_verifier | SubEFI | ADD | <div style='text-align: right'>236</div>  |
| root_verifier | SubEI | ADD | <div style='text-align: right'>1,392</div>  |
| root_verifier | SubF | SUB | <div style='text-align: right'>8</div>  |
| root_verifier | SubFI | SUB | <div style='text-align: right'>1,333</div>  |
| root_verifier | SubV | SUB | <div style='text-align: right'>82,294</div>  |
| root_verifier | SubVI | SUB | <div style='text-align: right'>1,089</div>  |
| root_verifier | SubVIN | SUB | <div style='text-align: right'>924</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>486,480</div>  |
| root_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>230,824</div>  |
| root_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>136,396</div>  |
| root_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>94,468</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>14,760</div>  |
| root_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>1,738</div>  |
| root_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>2,054</div>  |
| root_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>1,276</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>44,280</div>  |
| root_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>1,738</div>  |
| root_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>3,828</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>9,960</div>  |
| root_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>1,672</div>  |
| root_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>988</div>  |
| root_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,496</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>986,880</div>  |
| root_verifier | AccessAdapter<2> | AddEI | ADD | <div style='text-align: right'>194,656</div>  |
| root_verifier | AccessAdapter<4> | AddEI | ADD | <div style='text-align: right'>115,024</div>  |
| root_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>161,348</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>39,990</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>2,267,700</div>  |
| root_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>1,309</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>517,170</div>  |
| root_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>10,726,140</div>  |
| root_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>1,056</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>1,749,060</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>2,390,382</div>  |
| root_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,144</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>1,031,220</div>  |
| root_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>33</div>  |
| root_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>39</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>5,336</div>  |
| root_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>1,276</div>  |
| root_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>754</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| root_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| root_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>248,400</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | BNE | <div style='text-align: right'>115</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>26,128</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>5,083</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | BEQ | <div style='text-align: right'>23</div>  |
| root_verifier | PhantomAir | CT-ExtractPublicValues | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-ReadProofsFromInput | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-VerifyProofs | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>3,024</div>  |
| root_verifier | PhantomAir | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>45,864</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>15,120</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>11,088</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>713,160</div>  |
| root_verifier | PhantomAir | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>63,504</div>  |
| root_verifier | PhantomAir | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-verify-batch | PHANTOM | <div style='text-align: right'>3,024</div>  |
| root_verifier | PhantomAir | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>11,088</div>  |
| root_verifier | PhantomAir | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>26,208</div>  |
| root_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>26,208</div>  |
| root_verifier | PhantomAir | CT-verify-query | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | ADD | <div style='text-align: right'>240</div>  |
| root_verifier | Boundary | CastFV | ADD | <div style='text-align: right'>88</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>249,920</div>  |
| root_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>117,502</div>  |
| root_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>69,433</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>6,960</div>  |
| root_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>8,954</div>  |
| root_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>5,291</div>  |
| root_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>28,536</div>  |
| root_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>3,498</div>  |
| root_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>1,872</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary | DivFIN | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>13,035,450</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>11,040,989</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>455,280</div>  |
| root_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>330</div>  |
| root_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>390</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>91,266</div>  |
| root_verifier | Boundary | For | LOADW | <div style='text-align: right'>473</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>1,775,382</div>  |
| root_verifier | Boundary | For | STOREW | <div style='text-align: right'>792</div>  |
| root_verifier | AccessAdapter<2> | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>141,416</div>  |
| root_verifier | AccessAdapter<4> | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>83,564</div>  |
| root_verifier | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>7,004,928</div>  |
| root_verifier | PhantomAir | HintBitsF | PHANTOM | <div style='text-align: right'>258</div>  |
| root_verifier | PhantomAir | HintInputVec | PHANTOM | <div style='text-align: right'>143,568</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>487,416</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>3,700,194</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <div style='text-align: right'>449,840</div>  |
| root_verifier | AccessAdapter<2> | IfEqI | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | AccessAdapter<4> | IfEqI | JAL | <div style='text-align: right'>13</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>379,247</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>10</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>52,279</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>133,988</div>  |
| root_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>6,050</div>  |
| root_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>3,575</div>  |
| root_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>13,640</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>1,509,374</div>  |
| root_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>17,281</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>1,079,612</div>  |
| root_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>1,309</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>887,732</div>  |
| root_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>141,482</div>  |
| root_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>83,603</div>  |
| root_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>4,444</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>2,881,972</div>  |
| root_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>61,952</div>  |
| root_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>36,608</div>  |
| root_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>1,186,786</div>  |
| root_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>52,272</div>  |
| root_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>30,888</div>  |
| root_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>15,224</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>3,266,470</div>  |
| root_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>836</div>  |
| root_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>494</div>  |
| root_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>649</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>1,105,688</div>  |
| root_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>484</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>9,601,298</div>  |
| root_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,111</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>634,640</div>  |
| root_verifier | AccessAdapter<2> | MulE | BBE4MUL | <div style='text-align: right'>263,054</div>  |
| root_verifier | AccessAdapter<4> | MulE | BBE4MUL | <div style='text-align: right'>155,441</div>  |
| root_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>149,160</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>152,640</div>  |
| root_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>24,244</div>  |
| root_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>14,326</div>  |
| root_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>4,224</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>12,840</div>  |
| root_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>2,112</div>  |
| root_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,248</div>  |
| root_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>1,012</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>41,680</div>  |
| root_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>46,508</div>  |
| root_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>27,482</div>  |
| root_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>170,888</div>  |
| root_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>22,869</div>  |
| root_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>13,494</div>  |
| root_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>4,439,430</div>  |
| root_verifier | Boundary | MulF | MUL | <div style='text-align: right'>14,630</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>40,470</div>  |
| root_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>14,641</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>636,810</div>  |
| root_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>5,640</div>  |
| root_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,100</div>  |
| root_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>650</div>  |
| root_verifier | Boundary | NegE | MUL | <div style='text-align: right'>836</div>  |
| root_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>720,324</div>  |
| root_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>425,646</div>  |
| root_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>278,307</div>  |
| root_verifier | Boundary | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>9,703,122</div>  |
| root_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>471,130</div>  |
| root_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>279,214</div>  |
| root_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>186,490</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>4,910,815</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | PUBLISH | <div style='text-align: right'>1,344</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>1,055,504</div>  |
| root_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>20,372</div>  |
| root_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>12,038</div>  |
| root_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>283,184</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>1,560,460</div>  |
| root_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>168,168</div>  |
| root_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>99,372</div>  |
| root_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>41,668</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>1,144,638</div>  |
| root_verifier | AccessAdapter<2> | StoreF | STOREW | <div style='text-align: right'>484</div>  |
| root_verifier | AccessAdapter<4> | StoreF | STOREW | <div style='text-align: right'>286</div>  |
| root_verifier | AccessAdapter<8> | StoreF | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>306,130</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>2,769,140</div>  |
| root_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>278,036</div>  |
| root_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>165,113</div>  |
| root_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>111,690</div>  |
| root_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>85,371</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>6,218,730</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>9,534,632</div>  |
| root_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>2,558,072</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>135,054</div>  |
| root_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>36,234</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>2,719,079</div>  |
| root_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>658,658</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>145,360</div>  |
| root_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>131,428</div>  |
| root_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>77,662</div>  |
| root_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>25,212</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>655,344</div>  |
| root_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>58,619</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>159,840</div>  |
| root_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>58,619</div>  |
| root_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>69,277</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>7,080</div>  |
| root_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>1,012</div>  |
| root_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>598</div>  |
| root_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>132</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>41,760</div>  |
| root_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>11,440</div>  |
| root_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>6,760</div>  |
| root_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>4,224</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | SUB | <div style='text-align: right'>240</div>  |
| root_verifier | Boundary | SubF | SUB | <div style='text-align: right'>88</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | SUB | <div style='text-align: right'>39,990</div>  |
| root_verifier | Boundary | SubFI | SUB | <div style='text-align: right'>14,630</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>2,468,820</div>  |
| root_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>32,670</div>  |
| root_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>506</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>27,720</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| root_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<2> | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | AccessAdapterAir<4> | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| root_verifier | AccessAdapterAir<8> | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |

| group | cell_tracker_span | fixed_cells | lookup_advice_cells | simple_advice_cells |
| --- | --- | --- | --- | --- |
| static_verifier | VerifierProgram | <div style='text-align: right'>264,441</div>  | <div style='text-align: right'>441,678</div>  | <div style='text-align: right'>1,658,528</div>  |
| static_verifier | VerifierProgram;PoseidonCell | <div style='text-align: right'>5,800</div>  |  | <div style='text-align: right'>40,240</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds | <div style='text-align: right'>94,741</div>  | <div style='text-align: right'>1,320</div>  | <div style='text-align: right'>661,792</div>  |
| static_verifier | VerifierProgram;stage-c-build-rounds;PoseidonCell | <div style='text-align: right'>11,600</div>  |  | <div style='text-align: right'>80,480</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges | <div style='text-align: right'>161,671</div>  | <div style='text-align: right'>3,028</div>  | <div style='text-align: right'>1,138,770</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-1-verify-shape-and-sample-challenges;PoseidonCell | <div style='text-align: right'>18,850</div>  |  | <div style='text-align: right'>130,780</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold | <div style='text-align: right'>10,752</div>  |  | <div style='text-align: right'>21,504</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch | <div style='text-align: right'>27,972</div>  |  | <div style='text-align: right'>242,424</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>13,381,788</div>  | <div style='text-align: right'>713,160</div>  | <div style='text-align: right'>85,849,764</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;verify-batch;PoseidonCell | <div style='text-align: right'>4,139,100</div>  |  | <div style='text-align: right'>28,644,840</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening | <div style='text-align: right'>61,110</div>  | <div style='text-align: right'>88,704</div>  | <div style='text-align: right'>348,432</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;exp-reverse-bits-len | <div style='text-align: right'>2,744,784</div>  | <div style='text-align: right'>3,882,984</div>  | <div style='text-align: right'>16,221,156</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-2-fri-fold;compute-reduced-opening;single-reduced-opening-eval | <div style='text-align: right'>69,397,062</div>  | <div style='text-align: right'>83,647,704</div>  | <div style='text-align: right'>403,922,232</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges | <div style='text-align: right'>4,788</div>  | <div style='text-align: right'>6,552</div>  | <div style='text-align: right'>25,536</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query | <div style='text-align: right'>2,182,278</div>  | <div style='text-align: right'>2,972,760</div>  | <div style='text-align: right'>12,582,864</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext | <div style='text-align: right'>63,504</div>  |  | <div style='text-align: right'>550,368</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | <div style='text-align: right'>1,958,166</div>  | <div style='text-align: right'>263,088</div>  | <div style='text-align: right'>12,682,152</div>  |
| static_verifier | VerifierProgram;stage-d-verify-pcs;stage-d-3-verify-challenges;verify-query;verify-batch-ext;PoseidonCell | <div style='text-align: right'>7,726,320</div>  |  | <div style='text-align: right'>53,470,368</div>  |
| static_verifier | VerifierProgram;stage-e-verify-constraints | <div style='text-align: right'>4,062,252</div>  | <div style='text-align: right'>5,308,728</div>  | <div style='text-align: right'>23,257,046</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cab96411c5bffb639175eee39f65365a67a3fcd9/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-static_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/cab96411c5bffb639175eee39f65365a67a3fcd9

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12053971928)
