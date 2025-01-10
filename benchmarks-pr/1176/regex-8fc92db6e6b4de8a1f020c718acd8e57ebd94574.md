| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  49.20 |  49.20 |
| regex_program | <span style='color: red'>(+1 [+4.1%])</span> 19.30 | <span style='color: red'>(+1 [+4.1%])</span> 19.30 |
| leaf | <span style='color: green'>(-1 [-2.5%])</span> 29.89 | <span style='color: green'>(-1 [-2.5%])</span> 29.89 |


| regex_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+767 [+4.1%])</span> 19,303 | <span style='color: red'>(+767 [+4.1%])</span> 19,303 | <span style='color: red'>(+767 [+4.1%])</span> 19,303 | <span style='color: red'>(+767 [+4.1%])</span> 19,303 |
| `main_cells_used     ` |  165,028,173 |  165,028,173 |  165,028,173 |  165,028,173 |
| `total_cycles        ` |  4,190,904 |  4,190,904 |  4,190,904 |  4,190,904 |
| `execute_time_ms     ` | <span style='color: red'>(+6 [+0.5%])</span> 1,163 | <span style='color: red'>(+6 [+0.5%])</span> 1,163 | <span style='color: red'>(+6 [+0.5%])</span> 1,163 | <span style='color: red'>(+6 [+0.5%])</span> 1,163 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+533 [+16.2%])</span> 3,818 | <span style='color: red'>(+533 [+16.2%])</span> 3,818 | <span style='color: red'>(+533 [+16.2%])</span> 3,818 | <span style='color: red'>(+533 [+16.2%])</span> 3,818 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+228 [+1.6%])</span> 14,322 | <span style='color: red'>(+228 [+1.6%])</span> 14,322 | <span style='color: red'>(+228 [+1.6%])</span> 14,322 | <span style='color: red'>(+228 [+1.6%])</span> 14,322 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+20 [+0.8%])</span> 2,451 | <span style='color: red'>(+20 [+0.8%])</span> 2,451 | <span style='color: red'>(+20 [+0.8%])</span> 2,451 | <span style='color: red'>(+20 [+0.8%])</span> 2,451 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-11 [-2.2%])</span> 486 | <span style='color: green'>(-11 [-2.2%])</span> 486 | <span style='color: green'>(-11 [-2.2%])</span> 486 | <span style='color: green'>(-11 [-2.2%])</span> 486 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-21 [-0.4%])</span> 5,274 | <span style='color: green'>(-21 [-0.4%])</span> 5,274 | <span style='color: green'>(-21 [-0.4%])</span> 5,274 | <span style='color: green'>(-21 [-0.4%])</span> 5,274 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+375 [+20.5%])</span> 2,203 | <span style='color: red'>(+375 [+20.5%])</span> 2,203 | <span style='color: red'>(+375 [+20.5%])</span> 2,203 | <span style='color: red'>(+375 [+20.5%])</span> 2,203 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+9 [+0.7%])</span> 1,223 | <span style='color: red'>(+9 [+0.7%])</span> 1,223 | <span style='color: red'>(+9 [+0.7%])</span> 1,223 | <span style='color: red'>(+9 [+0.7%])</span> 1,223 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-144 [-5.1%])</span> 2,681 | <span style='color: green'>(-144 [-5.1%])</span> 2,681 | <span style='color: green'>(-144 [-5.1%])</span> 2,681 | <span style='color: green'>(-144 [-5.1%])</span> 2,681 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-778 [-2.5%])</span> 29,895 | <span style='color: green'>(-778 [-2.5%])</span> 29,895 | <span style='color: green'>(-778 [-2.5%])</span> 29,895 | <span style='color: green'>(-778 [-2.5%])</span> 29,895 |
| `main_cells_used     ` | <span style='color: green'>(-32488823 [-11.2%])</span> 258,783,956 | <span style='color: green'>(-32488823 [-11.2%])</span> 258,783,956 | <span style='color: green'>(-32488823 [-11.2%])</span> 258,783,956 | <span style='color: green'>(-32488823 [-11.2%])</span> 258,783,956 |
| `total_cycles        ` | <span style='color: green'>(-407164 [-6.2%])</span> 6,113,242 | <span style='color: green'>(-407164 [-6.2%])</span> 6,113,242 | <span style='color: green'>(-407164 [-6.2%])</span> 6,113,242 | <span style='color: green'>(-407164 [-6.2%])</span> 6,113,242 |
| `execute_time_ms     ` | <span style='color: green'>(-139 [-7.5%])</span> 1,704 | <span style='color: green'>(-139 [-7.5%])</span> 1,704 | <span style='color: green'>(-139 [-7.5%])</span> 1,704 | <span style='color: green'>(-139 [-7.5%])</span> 1,704 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-318 [-7.0%])</span> 4,224 | <span style='color: green'>(-318 [-7.0%])</span> 4,224 | <span style='color: green'>(-318 [-7.0%])</span> 4,224 | <span style='color: green'>(-318 [-7.0%])</span> 4,224 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-321 [-1.3%])</span> 23,967 | <span style='color: green'>(-321 [-1.3%])</span> 23,967 | <span style='color: green'>(-321 [-1.3%])</span> 23,967 | <span style='color: green'>(-321 [-1.3%])</span> 23,967 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-166 [-3.4%])</span> 4,651 | <span style='color: green'>(-166 [-3.4%])</span> 4,651 | <span style='color: green'>(-166 [-3.4%])</span> 4,651 | <span style='color: green'>(-166 [-3.4%])</span> 4,651 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-67 [-11.6%])</span> 512 | <span style='color: green'>(-67 [-11.6%])</span> 512 | <span style='color: green'>(-67 [-11.6%])</span> 512 | <span style='color: green'>(-67 [-11.6%])</span> 512 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-137 [-2.8%])</span> 4,723 | <span style='color: green'>(-137 [-2.8%])</span> 4,723 | <span style='color: green'>(-137 [-2.8%])</span> 4,723 | <span style='color: green'>(-137 [-2.8%])</span> 4,723 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-59 [-1.2%])</span> 4,833 | <span style='color: green'>(-59 [-1.2%])</span> 4,833 | <span style='color: green'>(-59 [-1.2%])</span> 4,833 | <span style='color: green'>(-59 [-1.2%])</span> 4,833 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-25 [-0.6%])</span> 3,991 | <span style='color: green'>(-25 [-0.6%])</span> 3,991 | <span style='color: green'>(-25 [-0.6%])</span> 3,991 | <span style='color: green'>(-25 [-0.6%])</span> 3,991 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+132 [+2.6%])</span> 5,254 | <span style='color: red'>(+132 [+2.6%])</span> 5,254 | <span style='color: red'>(+132 [+2.6%])</span> 5,254 | <span style='color: red'>(+132 [+2.6%])</span> 5,254 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| regex_program | 1 | 719 | 41 | 

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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 15 | 24 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 20 | 31 | 106,954,752 | 
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
| leaf | 0 | 4,224 | 29,895 | 6,113,242 | 729,876,952 | 23,967 | 4,833 | 3,991 | 4,723 | 5,254 | 4,651 | 258,783,956 | 512 | 1,704 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| regex_program | 0 | 3,818 | 19,303 | 4,190,904 | 632,452,480 | 14,322 | 2,203 | 1,223 | 5,274 | 2,681 | 2,451 | 165,028,173 | 486 | 1,163 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/8fc92db6e6b4de8a1f020c718acd8e57ebd94574

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12706004017)
