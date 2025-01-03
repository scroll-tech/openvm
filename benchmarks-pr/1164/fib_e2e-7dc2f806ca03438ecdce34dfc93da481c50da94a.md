| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  869,251 |  610,875 |
| fib_e2e |  50,778 |  7,565 |
| leaf |  107,076 |  18,193 |
| internal.0 |  126,508 |  36,283 |
| internal.1 |  72,980 |  36,925 |
| internal.2 |  36,670 |  36,670 |
| root |  79,905 |  79,905 |
| halo2_outer |  317,628 |  317,628 |
| halo2_wrapper |  77,706 |  77,706 |


| fib_e2e |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  7,254 |  50,778 |  7,565 |  6,892 |
| `main_cells_used     ` |  58,688,632.86 |  410,820,430 |  59,826,835 |  52,001,236 |
| `total_cycles        ` |  1,714,305.29 |  12,000,137 |  1,747,603 |  1,515,024 |
| `execute_time_ms     ` |  1,936.14 |  13,553 |  2,030 |  1,712 |
| `trace_gen_time_ms   ` |  283.71 |  1,986 |  300 |  265 |
| `stark_prove_excluding_trace_time_ms` |  5,034.14 |  35,239 |  5,366 |  4,651 |
| `main_trace_commit_time_ms` |  582.14 |  4,075 |  826 |  534 |
| `generate_perm_trace_time_ms` |  149.43 |  1,046 |  198 |  119 |
| `perm_trace_commit_time_ms` |  1,699 |  11,893 |  1,864 |  1,522 |
| `quotient_poly_compute_time_ms` |  847.43 |  5,932 |  921 |  760 |
| `quotient_poly_commit_time_ms` |  414.71 |  2,903 |  488 |  360 |
| `pcs_opening_time_ms ` |  1,339.29 |  9,375 |  1,403 |  1,291 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  15,296.57 |  107,076 |  18,193 |  14,594 |
| `main_cells_used     ` |  110,073,054.43 |  770,511,381 |  127,873,181 |  105,810,785 |
| `total_cycles        ` |  2,722,063 |  19,054,441 |  3,145,619 |  2,620,161 |
| `execute_time_ms     ` |  2,877.43 |  20,142 |  3,313 |  2,732 |
| `trace_gen_time_ms   ` |  725.86 |  5,081 |  804 |  658 |
| `stark_prove_excluding_trace_time_ms` |  11,693.29 |  81,853 |  14,076 |  11,131 |
| `main_trace_commit_time_ms` |  2,329.57 |  16,307 |  2,634 |  2,224 |
| `generate_perm_trace_time_ms` |  241.29 |  1,689 |  318 |  217 |
| `perm_trace_commit_time_ms` |  1,773.57 |  12,415 |  2,171 |  1,683 |
| `quotient_poly_compute_time_ms` |  2,619.57 |  18,337 |  3,296 |  2,457 |
| `quotient_poly_commit_time_ms` |  1,772.14 |  12,405 |  2,079 |  1,694 |
| `pcs_opening_time_ms ` |  2,954.57 |  20,682 |  3,575 |  2,804 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,627 |  126,508 |  36,283 |  18,799 |
| `main_cells_used     ` |  221,221,875 |  884,887,500 |  252,610,169 |  127,222,687 |
| `total_cycles        ` |  5,674,233.75 |  22,696,935 |  6,486,227 |  3,244,723 |
| `execute_time_ms     ` |  6,336 |  25,344 |  7,390 |  3,632 |
| `trace_gen_time_ms   ` |  1,268.50 |  5,074 |  1,470 |  683 |
| `stark_prove_excluding_trace_time_ms` |  24,022.50 |  96,090 |  27,439 |  14,484 |
| `main_trace_commit_time_ms` |  4,733.25 |  18,933 |  5,651 |  2,824 |
| `generate_perm_trace_time_ms` |  612.75 |  2,451 |  622 |  600 |
| `perm_trace_commit_time_ms` |  3,929.50 |  15,718 |  4,361 |  2,742 |
| `quotient_poly_compute_time_ms` |  5,070 |  20,280 |  5,818 |  2,887 |
| `quotient_poly_commit_time_ms` |  3,650.75 |  14,603 |  4,184 |  2,093 |
| `pcs_opening_time_ms ` |  6,023 |  24,092 |  7,057 |  3,324 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  36,490 |  72,980 |  36,925 |  36,055 |
| `main_cells_used     ` |  259,616,788 |  519,233,576 |  261,341,037 |  257,892,539 |
| `total_cycles        ` |  6,666,941 |  13,333,882 |  6,725,846 |  6,608,036 |
| `execute_time_ms     ` |  7,597.50 |  15,195 |  7,645 |  7,550 |
| `trace_gen_time_ms   ` |  1,510.50 |  3,021 |  1,532 |  1,489 |
| `stark_prove_excluding_trace_time_ms` |  27,382 |  54,764 |  27,748 |  27,016 |
| `main_trace_commit_time_ms` |  5,369.50 |  10,739 |  5,689 |  5,050 |
| `generate_perm_trace_time_ms` |  608.50 |  1,217 |  613 |  604 |
| `perm_trace_commit_time_ms` |  4,300.50 |  8,601 |  4,330 |  4,271 |
| `quotient_poly_compute_time_ms` |  5,833 |  11,666 |  5,838 |  5,828 |
| `quotient_poly_commit_time_ms` |  4,180.50 |  8,361 |  4,189 |  4,172 |
| `pcs_opening_time_ms ` |  7,087 |  14,174 |  7,088 |  7,086 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  36,670 |  36,670 |  36,670 |  36,670 |
| `main_cells_used     ` |  261,350,327 |  261,350,327 |  261,350,327 |  261,350,327 |
| `total_cycles        ` |  6,726,733 |  6,726,733 |  6,726,733 |  6,726,733 |
| `execute_time_ms     ` |  7,659 |  7,659 |  7,659 |  7,659 |
| `trace_gen_time_ms   ` |  1,554 |  1,554 |  1,554 |  1,554 |
| `stark_prove_excluding_trace_time_ms` |  27,457 |  27,457 |  27,457 |  27,457 |
| `main_trace_commit_time_ms` |  5,697 |  5,697 |  5,697 |  5,697 |
| `generate_perm_trace_time_ms` |  613 |  613 |  613 |  613 |
| `perm_trace_commit_time_ms` |  4,283 |  4,283 |  4,283 |  4,283 |
| `quotient_poly_compute_time_ms` |  5,811 |  5,811 |  5,811 |  5,811 |
| `quotient_poly_commit_time_ms` |  4,187 |  4,187 |  4,187 |  4,187 |
| `pcs_opening_time_ms ` |  6,865 |  6,865 |  6,865 |  6,865 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  79,905 |  79,905 |  79,905 |  79,905 |
| `main_cells_used     ` |  131,601,613 |  131,601,613 |  131,601,613 |  131,601,613 |
| `total_cycles        ` |  3,361,973 |  3,361,973 |  3,361,973 |  3,361,973 |
| `execute_time_ms     ` |  3,953 |  3,953 |  3,953 |  3,953 |
| `trace_gen_time_ms   ` |  1,235 |  1,235 |  1,235 |  1,235 |
| `stark_prove_excluding_trace_time_ms` |  74,717 |  74,717 |  74,717 |  74,717 |
| `main_trace_commit_time_ms` |  26,710 |  26,710 |  26,710 |  26,710 |
| `generate_perm_trace_time_ms` |  318 |  318 |  318 |  318 |
| `perm_trace_commit_time_ms` |  20,478 |  20,478 |  20,478 |  20,478 |
| `quotient_poly_compute_time_ms` |  3,060 |  3,060 |  3,060 |  3,060 |
| `quotient_poly_commit_time_ms` |  15,862 |  15,862 |  15,862 |  15,862 |
| `pcs_opening_time_ms ` |  8,286 |  8,286 |  8,286 |  8,286 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  317,628 |  317,628 |  317,628 |  317,628 |
| `main_cells_used     ` |  278,025,708 |  278,025,708 |  278,025,708 |  278,025,708 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  77,706 |  77,706 |  77,706 |  77,706 |



<details>
<summary>Detailed Metrics</summary>

|  | total_cycles | main_cells_used | execute_time_ms |
| --- | --- | --- |
|  | 3,361,973 | 131,601,613 | 2,235 | 

| group | total_proof_time_ms | num_segments | num_children | main_cells_used | fri.log_blowup |
| --- | --- | --- | --- | --- | --- |
| fib_e2e |  | 7 |  |  | 2 | 
| halo2_outer | 317,628 |  |  | 278,025,708 |  | 
| halo2_wrapper | 77,706 |  |  |  |  | 
| internal.0 |  |  | 2 |  | 2 | 
| internal.1 |  |  | 2 |  | 2 | 
| internal.2 |  |  | 2 |  | 2 | 
| leaf |  |  | 1 |  | 2 | 

| group | air_name | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 0 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.0 | AccessAdapterAir<2> | 1 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.0 | AccessAdapterAir<2> | 2 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.0 | AccessAdapterAir<2> | 3 | 524,288 |  | 16 | 11 | 14,155,776 | 
| internal.0 | AccessAdapterAir<4> | 0 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.0 | AccessAdapterAir<4> | 1 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.0 | AccessAdapterAir<4> | 2 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.0 | AccessAdapterAir<4> | 3 | 262,144 |  | 16 | 13 | 7,602,176 | 
| internal.0 | AccessAdapterAir<8> | 0 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.0 | AccessAdapterAir<8> | 1 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.0 | AccessAdapterAir<8> | 2 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.0 | AccessAdapterAir<8> | 3 | 65,536 |  | 16 | 17 | 2,162,688 | 
| internal.0 | FriReducedOpeningAir | 0 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.0 | FriReducedOpeningAir | 1 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.0 | FriReducedOpeningAir | 2 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.0 | FriReducedOpeningAir | 3 | 131,072 |  | 76 | 64 | 18,350,080 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 1 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 2 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 3 | 32,768 |  | 36 | 348 | 12,582,912 | 
| internal.0 | PhantomAir | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 3 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 2 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 3 | 2,097,152 |  | 24 | 41 | 136,314,880 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 65,536 |  | 20 | 40 | 3,932,160 | 
| internal.0 | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 1 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 2 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 3 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VolatileBoundaryAir | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 1 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 3 | 524,288 |  | 8 | 11 | 9,961,472 | 
| internal.1 | AccessAdapterAir<2> | 4 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.1 | AccessAdapterAir<2> | 5 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.1 | AccessAdapterAir<4> | 4 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.1 | AccessAdapterAir<4> | 5 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.1 | AccessAdapterAir<8> | 4 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.1 | AccessAdapterAir<8> | 5 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.1 | FriReducedOpeningAir | 4 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.1 | FriReducedOpeningAir | 5 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 4 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 5 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.1 | PhantomAir | 4 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.1 | PhantomAir | 5 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.1 | ProgramAir | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 4 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 5 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 4 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 5 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 4 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 5 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 5 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 5 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.1 | VmConnectorAir | 4 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 5 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 4 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.1 | VolatileBoundaryAir | 5 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.2 | AccessAdapterAir<2> | 6 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| internal.2 | AccessAdapterAir<4> | 6 | 524,288 |  | 16 | 13 | 15,204,352 | 
| internal.2 | AccessAdapterAir<8> | 6 | 131,072 |  | 16 | 17 | 4,325,376 | 
| internal.2 | FriReducedOpeningAir | 6 | 262,144 |  | 76 | 64 | 36,700,160 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 6 | 65,536 |  | 36 | 348 | 25,165,824 | 
| internal.2 | PhantomAir | 6 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.2 | ProgramAir | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 6 | 2,097,152 |  | 28 | 23 | 106,954,752 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 6 | 262,144 |  | 12 | 10 | 5,767,168 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 6 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 6 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 6 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.2 | VmConnectorAir | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 6 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | AccessAdapterAir<2> | 0 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 1 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 2 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 3 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 4 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 5 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 6 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<4> | 0 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 1 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 2 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 3 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 4 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 5 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 6 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<8> | 0 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 1 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 2 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 3 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 4 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 5 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | AccessAdapterAir<8> | 6 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | FriReducedOpeningAir | 0 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 1 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 2 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 3 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 4 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 5 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | FriReducedOpeningAir | 6 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 1 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 2 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 3 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 4 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 5 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 6 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 1 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 2 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 3 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 4 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 5 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 6 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | ProgramAir | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 4 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 5 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 6 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 4 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 5 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 6 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 4 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 5 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 6 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 4 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 5 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 6 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 24 | 41 | 136,314,880 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 2 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 3 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 5 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 6 | 1,048,576 |  | 24 | 41 | 68,157,440 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 5 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 6 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 1 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 2 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 3 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 4 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 5 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 6 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 0 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 1 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 2 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 3 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 4 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 5 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 6 | 524,288 |  | 8 | 11 | 9,961,472 | 
| root | AccessAdapterAir<2> | 0 | 524,288 |  | 16 | 11 | 14,155,776 | 
| root | AccessAdapterAir<4> | 0 | 262,144 |  | 16 | 13 | 7,602,176 | 
| root | AccessAdapterAir<8> | 0 | 65,536 |  | 16 | 17 | 2,162,688 | 
| root | FriReducedOpeningAir | 0 | 131,072 |  | 76 | 64 | 18,350,080 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 32,768 |  | 36 | 348 | 12,582,912 | 
| root | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| root | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| root | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| root | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| root | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 24 | 41 | 136,314,880 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 65,536 |  | 20 | 40 | 3,932,160 | 
| root | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| root | VolatileBoundaryAir | 0 | 524,288 |  | 8 | 11 | 9,961,472 | 

| group | air_name | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | AccessAdapterAir<8> | 0 | 64 |  | 24 | 17 | 2,624 | 
| fib_e2e | AccessAdapterAir<8> | 1 | 16 |  | 24 | 17 | 656 | 
| fib_e2e | AccessAdapterAir<8> | 2 | 16 |  | 24 | 17 | 656 | 
| fib_e2e | AccessAdapterAir<8> | 3 | 16 |  | 24 | 17 | 656 | 
| fib_e2e | AccessAdapterAir<8> | 4 | 16 |  | 24 | 17 | 656 | 
| fib_e2e | AccessAdapterAir<8> | 5 | 16 |  | 24 | 17 | 656 | 
| fib_e2e | AccessAdapterAir<8> | 6 | 32 |  | 24 | 17 | 1,312 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | BitwiseOperationLookupAir<8> | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fib_e2e | MemoryMerkleAir<8> | 0 | 256 |  | 20 | 32 | 13,312 | 
| fib_e2e | MemoryMerkleAir<8> | 1 | 128 |  | 20 | 32 | 6,656 | 
| fib_e2e | MemoryMerkleAir<8> | 2 | 128 |  | 20 | 32 | 6,656 | 
| fib_e2e | MemoryMerkleAir<8> | 3 | 128 |  | 20 | 32 | 6,656 | 
| fib_e2e | MemoryMerkleAir<8> | 4 | 128 |  | 20 | 32 | 6,656 | 
| fib_e2e | MemoryMerkleAir<8> | 5 | 128 |  | 20 | 32 | 6,656 | 
| fib_e2e | MemoryMerkleAir<8> | 6 | 256 |  | 20 | 32 | 13,312 | 
| fib_e2e | PersistentBoundaryAir<8> | 0 | 64 |  | 12 | 20 | 2,048 | 
| fib_e2e | PersistentBoundaryAir<8> | 1 | 16 |  | 12 | 20 | 512 | 
| fib_e2e | PersistentBoundaryAir<8> | 2 | 16 |  | 12 | 20 | 512 | 
| fib_e2e | PersistentBoundaryAir<8> | 3 | 16 |  | 12 | 20 | 512 | 
| fib_e2e | PersistentBoundaryAir<8> | 4 | 16 |  | 12 | 20 | 512 | 
| fib_e2e | PersistentBoundaryAir<8> | 5 | 16 |  | 12 | 20 | 512 | 
| fib_e2e | PersistentBoundaryAir<8> | 6 | 32 |  | 12 | 20 | 1,024 | 
| fib_e2e | PhantomAir | 0 | 2 |  | 12 | 6 | 36 | 
| fib_e2e | PhantomAir | 1 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | PhantomAir | 2 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | PhantomAir | 3 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | PhantomAir | 4 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | PhantomAir | 5 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | PhantomAir | 6 | 1 |  | 12 | 6 | 18 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 1 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 3 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 4 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 5 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 6 | 256 |  | 8 | 300 | 78,848 | 
| fib_e2e | ProgramAir | 0 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 1 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 2 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 3 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 4 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 5 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | ProgramAir | 6 | 4,096 |  | 8 | 10 | 73,728 | 
| fib_e2e | RangeTupleCheckerAir<2> | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 1 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 2 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 3 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 4 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 5 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | RangeTupleCheckerAir<2> | 6 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fib_e2e | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VariableRangeCheckerAir | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 2 |  | 52 | 53 | 210 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 8 |  | 56 | 32 | 704 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fib_e2e | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | 4 |  | 36 | 26 | 248 | 
| fib_e2e | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 16 |  | 36 | 28 | 1,024 | 
| fib_e2e | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 6 | 1 |  | 36 | 28 | 64 | 
| fib_e2e | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 32 |  | 72 | 40 | 3,584 | 
| fib_e2e | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 6 | 8 |  | 72 | 40 | 896 | 
| fib_e2e | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 16 |  | 28 | 21 | 784 | 
| fib_e2e | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 1 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 2 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 3 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 4 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 5 | 2 | 1 | 12 | 4 | 32 | 
| fib_e2e | VmConnectorAir | 6 | 2 | 1 | 12 | 4 | 32 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | fri.log_blowup | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 0 | 1,470 | 35,237 | 6,486,227 | 738,200,024 | 26,771 | 5,818 | 4,163 | 4,284 | 7,057 | 4,846 | 252,610,169 | 600 |  | 6,996 | 
| internal.0 | 1 | 1,467 | 36,189 | 6,482,249 | 738,200,024 | 27,396 | 5,782 | 4,184 | 4,331 | 6,865 | 5,612 | 252,518,767 | 619 |  | 7,326 | 
| internal.0 | 2 | 1,454 | 36,283 | 6,483,736 | 738,200,024 | 27,439 | 5,793 | 4,163 | 4,361 | 6,846 | 5,651 | 252,535,877 | 622 |  | 7,390 | 
| internal.0 | 3 | 683 | 18,799 | 3,244,723 | 371,460,568 | 14,484 | 2,887 | 2,093 | 2,742 | 3,324 | 2,824 | 127,222,687 | 610 |  | 3,632 | 
| internal.1 | 4 | 1,489 | 36,055 | 6,725,846 | 738,200,024 | 27,016 | 5,828 | 4,172 | 4,271 | 7,088 | 5,050 | 261,341,037 | 604 |  | 7,550 | 
| internal.1 | 5 | 1,532 | 36,925 | 6,608,036 | 738,200,024 | 27,748 | 5,838 | 4,189 | 4,330 | 7,086 | 5,689 | 257,892,539 | 613 |  | 7,645 | 
| internal.2 | 6 | 1,554 | 36,670 | 6,726,733 | 738,200,024 | 27,457 | 5,811 | 4,187 | 4,283 | 6,865 | 5,697 | 261,350,327 | 613 |  | 7,659 | 
| leaf | 0 | 804 | 18,193 | 3,145,619 | 369,494,488 | 14,076 | 3,296 | 2,079 | 2,171 | 3,575 | 2,634 | 127,873,181 | 318 |  | 3,313 | 
| leaf | 1 | 700 | 14,964 | 2,623,381 | 274,598,360 | 11,532 | 2,564 | 1,717 | 1,685 | 2,932 | 2,413 | 105,847,465 | 219 |  | 2,732 | 
| leaf | 2 | 711 | 14,643 | 2,621,245 | 273,156,568 | 11,137 | 2,457 | 1,719 | 1,687 | 2,822 | 2,232 | 105,822,745 | 217 |  | 2,795 | 
| leaf | 3 | 741 | 14,706 | 2,621,234 | 273,156,568 | 11,164 | 2,490 | 1,700 | 1,684 | 2,829 | 2,235 | 105,822,635 | 224 |  | 2,801 | 
| leaf | 4 | 730 | 14,594 | 2,620,632 | 273,156,568 | 11,131 | 2,487 | 1,694 | 1,692 | 2,812 | 2,224 | 105,816,405 | 219 |  | 2,733 | 
| leaf | 5 | 737 | 14,664 | 2,620,161 | 273,156,568 | 11,136 | 2,477 | 1,706 | 1,683 | 2,804 | 2,229 | 105,810,785 | 235 |  | 2,791 | 
| leaf | 6 | 658 | 15,312 | 2,802,169 | 301,337,048 | 11,677 | 2,566 | 1,790 | 1,813 | 2,908 | 2,340 | 113,518,165 | 257 |  | 2,977 | 
| root | 0 | 1,235 | 79,905 | 3,361,973 | 371,460,568 | 74,717 | 3,060 | 15,862 | 20,478 | 8,286 | 26,710 | 131,601,613 | 318 | 2 | 3,953 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | 0 | 265 | 7,565 | 1,747,603 | 197,440,542 | 5,366 | 827 | 488 | 1,630 | 1,394 | 826 | 59,826,835 | 198 | 1,934 | 
| fib_e2e | 1 | 287 | 7,283 | 1,747,502 | 197,423,810 | 4,966 | 795 | 367 | 1,798 | 1,340 | 534 | 59,798,170 | 130 | 2,030 | 
| fib_e2e | 2 | 300 | 7,544 | 1,747,502 | 197,423,810 | 5,235 | 854 | 408 | 1,864 | 1,403 | 536 | 59,798,161 | 168 | 2,009 | 
| fib_e2e | 3 | 295 | 7,307 | 1,747,502 | 197,423,810 | 5,048 | 881 | 451 | 1,750 | 1,294 | 535 | 59,798,480 | 135 | 1,964 | 
| fib_e2e | 4 | 286 | 6,892 | 1,747,502 | 197,423,810 | 4,651 | 760 | 366 | 1,571 | 1,291 | 539 | 59,798,779 | 122 | 1,955 | 
| fib_e2e | 5 | 287 | 7,285 | 1,747,502 | 197,423,810 | 5,049 | 921 | 360 | 1,758 | 1,318 | 571 | 59,798,769 | 119 | 1,949 | 
| fib_e2e | 6 | 266 | 6,902 | 1,515,024 | 197,432,594 | 4,924 | 894 | 463 | 1,522 | 1,335 | 534 | 52,001,236 | 174 | 1,712 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/7dc2f806ca03438ecdce34dfc93da481c50da94a

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12593651014)
