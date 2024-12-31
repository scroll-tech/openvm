| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <span style="color: green">(-6,386,998 [-44.0%])</span> <div style='text-align: right'>8,121,174</div>  | <span style="color: green">(-15 [-0.0%])</span> <div style='text-align: right'>195,357</div>  | <span style="color: green">(-39.0 [-2.6%])</span> <div style='text-align: right'>1,444.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | interactions | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |
| ProgramAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |
| VmConnectorAir |  | <span style="color: green">(-1 [-11.1%])</span> <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> |  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>11</div>  |  | <div style='text-align: right'>4</div>  |  |
| VolatileBoundaryAir |  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  |  | <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<2> |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<4> |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| AccessAdapterAir<8> |  | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| NativePoseidon2Air<BabyBearParameters>, 1> |  | <span style="color: green">(-8 [-2.6%])</span> <div style='text-align: right'>302</div>  | <div style='text-align: right'>31</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| FriReducedOpeningAir |  | <span style="color: green">(-17 [-22.4%])</span> <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> |  | <span style="color: green">(-4 [-14.8%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> |  | <span style="color: green">(-2 [-25.0%])</span> <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> |  | <span style="color: green">(-5 [-17.9%])</span> <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  | <div style='text-align: right'>2</div>  |  |
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | <span style="color: green">(-5 [-13.9%])</span> <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| PhantomAir |  | <span style="color: green">(-1 [-20.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  | <span style="color: red">(+2 [+100.0%])</span> <div style='text-align: right'>4</div>  |  |
| VariableRangeCheckerAir |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>1</div>  |  |

| main_trace_commit_time_ms | pcs_opening_time_ms | perm_trace_commit_time_ms | quotient_poly_commit_time_ms | quotient_poly_compute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <div style='text-align: right'>5.0</div>  | <div style='text-align: right'>4.0</div>  | <div style='text-align: right'>0.0</div>  | <div style='text-align: right'>1.0</div>  | <div style='text-align: right'>0.0</div>  | <div style='text-align: right'>11.0</div>  | <div style='text-align: right'>32</div>  | <span style="color: green">(-1.0 [-6.2%])</span> <div style='text-align: right'>15.0</div>  |

| group | fri.log_blowup | generate_perm_trace_time_ms | main_trace_commit_time_ms | pcs_opening_time_ms | perm_trace_commit_time_ms | quotient_poly_commit_time_ms | quotient_poly_compute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <div style='text-align: right'>24.0</div>  | <div style='text-align: right'>217.0</div>  | <div style='text-align: right'>456.0</div>  | <div style='text-align: right'>189.0</div>  | <div style='text-align: right'>297.0</div>  | <div style='text-align: right'>257.0</div>  | <div style='text-align: right'>1,444.0</div>  | <div style='text-align: right'>23,451,672</div>  | <span style="color: green">(-6,386,998 [-44.0%])</span> <div style='text-align: right'>8,121,174</div>  | <span style="color: green">(-15 [-0.0%])</span> <div style='text-align: right'>195,357</div>  | <span style="color: green">(-39.0 [-2.6%])</span> <div style='text-align: right'>1,444.0</div>  |

| group | segment | execute_time_ms | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: green">(-156.0 [-43.7%])</span> <div style='text-align: right'>201.0</div>  | <span style="color: green">(-6,386,998 [-44.0%])</span> <div style='text-align: right'>8,121,174</div>  | <span style="color: green">(-15 [-0.0%])</span> <div style='text-align: right'>195,357</div>  | <span style="color: green">(-14.0 [-22.6%])</span> <div style='text-align: right'>48.0</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | <div style='text-align: right'>1,245,184</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | <div style='text-align: right'>884,736</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<4> | <div style='text-align: right'>475,136</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | AccessAdapterAir<8> | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | <div style='text-align: right'>786,432</div>  | <div style='text-align: right'>348</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>512</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>28,672</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>2,048</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

</details>



Commit: https://github.com/openvm-org/openvm/commit/ebe83e9eeecf8c13c260199bbd90eb925ada0b85

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12555326338)
