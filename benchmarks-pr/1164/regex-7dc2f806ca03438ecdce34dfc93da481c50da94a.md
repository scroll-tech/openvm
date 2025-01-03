| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-511 [-0.9%])</span> 57,982 | <span style='color: green'>(-511 [-0.9%])</span> 57,982 |
| regex_program | <span style='color: red'>(+56 [+0.3%])</span> 22,299 | <span style='color: red'>(+56 [+0.3%])</span> 22,299 |
| leaf | <span style='color: green'>(-567 [-1.6%])</span> 35,683 | <span style='color: green'>(-567 [-1.6%])</span> 35,683 |


| regex_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+56 [+0.3%])</span> 22,299 | <span style='color: red'>(+56 [+0.3%])</span> 22,299 | <span style='color: red'>(+56 [+0.3%])</span> 22,299 | <span style='color: red'>(+56 [+0.3%])</span> 22,299 |
| `main_cells_used     ` |  165,028,173 |  165,028,173 |  165,028,173 |  165,028,173 |
| `total_cycles        ` |  4,190,904 |  4,190,904 |  4,190,904 |  4,190,904 |
| `execute_time_ms     ` | <span style='color: red'>(+56 [+1.0%])</span> 5,795 | <span style='color: red'>(+56 [+1.0%])</span> 5,795 | <span style='color: red'>(+56 [+1.0%])</span> 5,795 | <span style='color: red'>(+56 [+1.0%])</span> 5,795 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+42 [+5.9%])</span> 755 | <span style='color: red'>(+42 [+5.9%])</span> 755 | <span style='color: red'>(+42 [+5.9%])</span> 755 | <span style='color: red'>(+42 [+5.9%])</span> 755 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-42 [-0.3%])</span> 15,749 | <span style='color: green'>(-42 [-0.3%])</span> 15,749 | <span style='color: green'>(-42 [-0.3%])</span> 15,749 | <span style='color: green'>(-42 [-0.3%])</span> 15,749 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+12 [+0.5%])</span> 2,465 | <span style='color: red'>(+12 [+0.5%])</span> 2,465 | <span style='color: red'>(+12 [+0.5%])</span> 2,465 | <span style='color: red'>(+12 [+0.5%])</span> 2,465 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+12 [+2.4%])</span> 512 | <span style='color: red'>(+12 [+2.4%])</span> 512 | <span style='color: red'>(+12 [+2.4%])</span> 512 | <span style='color: red'>(+12 [+2.4%])</span> 512 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-39 [-0.7%])</span> 5,427 | <span style='color: green'>(-39 [-0.7%])</span> 5,427 | <span style='color: green'>(-39 [-0.7%])</span> 5,427 | <span style='color: green'>(-39 [-0.7%])</span> 5,427 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-22 [-0.8%])</span> 2,616 | <span style='color: green'>(-22 [-0.8%])</span> 2,616 | <span style='color: green'>(-22 [-0.8%])</span> 2,616 | <span style='color: green'>(-22 [-0.8%])</span> 2,616 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-20 [-1.6%])</span> 1,258 | <span style='color: green'>(-20 [-1.6%])</span> 1,258 | <span style='color: green'>(-20 [-1.6%])</span> 1,258 | <span style='color: green'>(-20 [-1.6%])</span> 1,258 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+15 [+0.4%])</span> 3,468 | <span style='color: red'>(+15 [+0.4%])</span> 3,468 | <span style='color: red'>(+15 [+0.4%])</span> 3,468 | <span style='color: red'>(+15 [+0.4%])</span> 3,468 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-567 [-1.6%])</span> 35,683 | <span style='color: green'>(-567 [-1.6%])</span> 35,683 | <span style='color: green'>(-567 [-1.6%])</span> 35,683 | <span style='color: green'>(-567 [-1.6%])</span> 35,683 |
| `main_cells_used     ` |  291,291,355 |  291,291,355 |  291,291,355 |  291,291,355 |
| `total_cycles        ` |  6,523,215 |  6,523,215 |  6,523,215 |  6,523,215 |
| `execute_time_ms     ` | <span style='color: red'>(+24 [+0.3%])</span> 7,626 | <span style='color: red'>(+24 [+0.3%])</span> 7,626 | <span style='color: red'>(+24 [+0.3%])</span> 7,626 | <span style='color: red'>(+24 [+0.3%])</span> 7,626 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-29 [-2.0%])</span> 1,406 | <span style='color: green'>(-29 [-2.0%])</span> 1,406 | <span style='color: green'>(-29 [-2.0%])</span> 1,406 | <span style='color: green'>(-29 [-2.0%])</span> 1,406 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-562 [-2.1%])</span> 26,651 | <span style='color: green'>(-562 [-2.1%])</span> 26,651 | <span style='color: green'>(-562 [-2.1%])</span> 26,651 | <span style='color: green'>(-562 [-2.1%])</span> 26,651 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-34 [-0.7%])</span> 4,804 | <span style='color: green'>(-34 [-0.7%])</span> 4,804 | <span style='color: green'>(-34 [-0.7%])</span> 4,804 | <span style='color: green'>(-34 [-0.7%])</span> 4,804 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+4 [+0.7%])</span> 588 | <span style='color: red'>(+4 [+0.7%])</span> 588 | <span style='color: red'>(+4 [+0.7%])</span> 588 | <span style='color: red'>(+4 [+0.7%])</span> 588 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+17 [+0.4%])</span> 4,850 | <span style='color: red'>(+17 [+0.4%])</span> 4,850 | <span style='color: red'>(+17 [+0.4%])</span> 4,850 | <span style='color: red'>(+17 [+0.4%])</span> 4,850 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-728 [-11.8%])</span> 5,416 | <span style='color: green'>(-728 [-11.8%])</span> 5,416 | <span style='color: green'>(-728 [-11.8%])</span> 5,416 | <span style='color: green'>(-728 [-11.8%])</span> 5,416 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-13 [-0.3%])</span> 4,074 | <span style='color: green'>(-13 [-0.3%])</span> 4,074 | <span style='color: green'>(-13 [-0.3%])</span> 4,074 | <span style='color: green'>(-13 [-0.3%])</span> 4,074 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+190 [+2.8%])</span> 6,916 | <span style='color: red'>(+190 [+2.8%])</span> 6,916 | <span style='color: red'>(+190 [+2.8%])</span> 6,916 | <span style='color: red'>(+190 [+2.8%])</span> 6,916 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | num_children | keygen_time_ms | fri.log_blowup | commit_exe_time_ms |
| --- | --- | --- | --- | --- | --- |
| leaf |  | 1 |  | 2 |  | 
| regex_program | 1 |  | 726 | 2 | 44 | 

| group | air_name | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| leaf | AccessAdapterAir<2> | 4 | 5 | 12 | 
| leaf | AccessAdapterAir<4> | 4 | 5 | 12 | 
| leaf | AccessAdapterAir<8> | 4 | 5 | 12 | 
| leaf | FriReducedOpeningAir | 4 | 35 | 59 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 4 | 31 | 302 | 
| leaf | PhantomAir | 4 | 3 | 4 | 
| leaf | ProgramAir | 1 | 1 | 4 | 
| leaf | VariableRangeCheckerAir | 1 | 1 | 4 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 11 | 23 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 4 | 7 | 6 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 4 | 11 | 23 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 4 | 15 | 23 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 19 | 31 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4 | 15 | 23 | 
| leaf | VmConnectorAir | 4 | 3 | 8 | 
| leaf | VolatileBoundaryAir | 4 | 4 | 16 | 
| regex_program | AccessAdapterAir<16> | 2 | 5 | 14 | 
| regex_program | AccessAdapterAir<2> | 2 | 5 | 14 | 
| regex_program | AccessAdapterAir<32> | 2 | 5 | 14 | 
| regex_program | AccessAdapterAir<4> | 2 | 5 | 14 | 
| regex_program | AccessAdapterAir<64> | 2 | 5 | 14 | 
| regex_program | AccessAdapterAir<8> | 2 | 5 | 14 | 
| regex_program | BitwiseOperationLookupAir<8> | 2 | 2 | 4 | 
| regex_program | KeccakVmAir | 2 | 321 | 4,571 | 
| regex_program | MemoryMerkleAir<8> | 2 | 4 | 40 | 
| regex_program | PersistentBoundaryAir<8> | 2 | 3 | 6 | 
| regex_program | PhantomAir | 2 | 3 | 5 | 
| regex_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 1 | 286 | 
| regex_program | ProgramAir | 1 | 1 | 4 | 
| regex_program | RangeTupleCheckerAir<2> | 1 | 1 | 4 | 
| regex_program | VariableRangeCheckerAir | 1 | 1 | 4 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 19 | 43 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 17 | 39 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 23 | 90 | 
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 11 | 25 | 
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 13 | 41 | 
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 10 | 22 | 
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 2 | 15 | 17 | 
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 16 | 20 | 
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 18 | 33 | 
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 17 | 38 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 2 | 25 | 88 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 24 | 38 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 19 | 26 | 
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 11 | 15 | 
| regex_program | VmConnectorAir | 2 | 3 | 9 | 

| group | air_name | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | AccessAdapterAir<2> | 0 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<4> | 0 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<8> | 0 | 131,072 |  | 16 | 17 | 4,325,376 | 
| leaf | FriReducedOpeningAir | 0 | 1,048,576 |  | 76 | 64 | 146,800,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 65,536 |  | 36 | 348 | 25,165,824 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 524,288 |  | 8 | 10 | 9,437,184 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 24 | 41 | 136,314,880 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 131,072 |  | 20 | 40 | 7,864,320 | 
| leaf | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 

| group | air_name | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | AccessAdapterAir<2> | 0 | 64 |  | 24 | 11 | 2,240 | 
| regex_program | AccessAdapterAir<4> | 0 | 32 |  | 24 | 13 | 1,184 | 
| regex_program | AccessAdapterAir<8> | 0 | 131,072 |  | 24 | 17 | 5,373,952 | 
| regex_program | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| regex_program | KeccakVmAir | 0 | 32 |  | 1,288 | 3,164 | 142,464 | 
| regex_program | MemoryMerkleAir<8> | 0 | 131,072 |  | 20 | 32 | 6,815,744 | 
| regex_program | PersistentBoundaryAir<8> | 0 | 131,072 |  | 12 | 20 | 4,194,304 | 
| regex_program | PhantomAir | 0 | 512 |  | 12 | 6 | 9,216 | 
| regex_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 16,384 |  | 8 | 300 | 5,046,272 | 
| regex_program | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| regex_program | RangeTupleCheckerAir<2> | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| regex_program | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 2,097,152 |  | 80 | 36 | 243,269,632 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 65,536 |  | 40 | 37 | 5,046,272 | 
| regex_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 262,144 |  | 52 | 53 | 27,525,120 | 
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 524,288 |  | 48 | 26 | 38,797,312 | 
| regex_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 262,144 |  | 56 | 32 | 23,068,672 | 
| regex_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 131,072 |  | 44 | 18 | 8,126,464 | 
| regex_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | 16,384 |  | 36 | 26 | 1,015,808 | 
| regex_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 131,072 |  | 36 | 28 | 8,388,608 | 
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | 1,024 |  | 76 | 35 | 113,664 | 
| regex_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 2,097,152 |  | 72 | 40 | 234,881,024 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | 128 |  | 104 | 57 | 20,608 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | 256 |  | 100 | 39 | 35,584 | 
| regex_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | 65,536 |  | 80 | 31 | 7,274,496 | 
| regex_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 65,536 |  | 28 | 21 | 3,211,264 | 
| regex_program | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 1,406 | 35,683 | 6,523,215 | 759,237,080 | 26,651 | 5,416 | 4,074 | 4,850 | 6,916 | 4,804 | 291,291,355 | 588 | 7,626 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | 0 | 755 | 22,299 | 4,190,904 | 632,452,480 | 15,749 | 2,616 | 1,258 | 5,427 | 3,468 | 2,465 | 165,028,173 | 512 | 5,795 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/7dc2f806ca03438ecdce34dfc93da481c50da94a

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12593651014)
