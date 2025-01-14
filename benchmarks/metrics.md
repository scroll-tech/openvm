| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(+0 [NaN%])</span> 0 | <span style='color: green'>(+0 [NaN%])</span> 0 |




<details>
<summary>Detailed Metrics</summary>

|  | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 1,862 | 45,688 | 4,191,033 | 632,452,480 | 43,422 | 8,332 | 2,632 | 14,262 | 8,183 | 1 | 6,815 | 165,014,123 | 845 | 3,196 | 404 | 79 | 

| air_name | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| AccessAdapterAir<16> |  | 2 |  |  |  | 5 | 14 |  | 
| AccessAdapterAir<2> | 64 | 2 |  | 24 | 11 | 5 | 14 | 2,240 | 
| AccessAdapterAir<32> |  | 2 |  |  |  | 5 | 14 |  | 
| AccessAdapterAir<4> | 32 | 2 |  | 24 | 13 | 5 | 14 | 1,184 | 
| AccessAdapterAir<64> |  | 2 |  |  |  | 5 | 14 |  | 
| AccessAdapterAir<8> | 131,072 | 2 |  | 24 | 17 | 5 | 14 | 5,373,952 | 
| BitwiseOperationLookupAir<8> | 65,536 | 2 | 3 | 8 | 2 | 2 | 4 | 655,360 | 
| KeccakVmAir | 32 | 2 |  | 1,288 | 3,164 | 321 | 4,571 | 142,464 | 
| MemoryMerkleAir<8> | 131,072 | 2 |  | 20 | 32 | 4 | 40 | 6,815,744 | 
| PersistentBoundaryAir<8> | 131,072 | 2 |  | 12 | 20 | 3 | 6 | 4,194,304 | 
| PhantomAir | 512 | 2 |  | 12 | 6 | 3 | 5 | 9,216 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 16,384 | 2 |  | 8 | 300 | 1 | 286 | 5,046,272 | 
| ProgramAir | 131,072 | 1 |  | 8 | 10 | 1 | 4 | 2,359,296 | 
| RangeTupleCheckerAir<2> | 524,288 | 1 | 2 | 8 | 1 | 1 | 4 | 4,718,592 | 
| VariableRangeCheckerAir | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2,097,152 | 2 |  | 80 | 36 | 19 | 43 | 243,269,632 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 65,536 | 2 |  | 40 | 37 | 17 | 39 | 5,046,272 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 262,144 | 2 |  | 52 | 53 | 23 | 90 | 27,525,120 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 524,288 | 2 |  | 48 | 26 | 11 | 25 | 38,797,312 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 262,144 | 2 |  | 56 | 32 | 13 | 41 | 23,068,672 | 
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 131,072 | 2 |  | 44 | 18 | 10 | 22 | 8,126,464 | 
| VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 16,384 | 2 |  | 36 | 26 | 15 | 17 | 1,015,808 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 131,072 | 2 |  | 36 | 28 | 16 | 20 | 8,388,608 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 1,024 | 2 |  | 76 | 35 | 18 | 33 | 113,664 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2,097,152 | 2 |  | 72 | 40 | 17 | 38 | 234,881,024 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 128 | 2 |  | 104 | 57 | 25 | 88 | 20,608 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 256 | 2 |  | 100 | 39 | 24 | 38 | 35,584 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 65,536 | 2 |  | 80 | 31 | 19 | 26 | 7,274,496 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 65,536 | 2 |  | 28 | 21 | 11 | 15 | 3,211,264 | 
| VmConnectorAir | 2 | 2 | 1 | 12 | 4 | 3 | 9 | 32 | 

</details>

