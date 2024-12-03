| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,425,827</div>  | <div style='text-align: right'>198,497</div>  | <span style="color: green">(-44.0 [-2.6%])</span> <div style='text-align: right'>1,631.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| air_name | cells | constraints | main_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- |
| FibonacciAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>16</div>  |

| stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- |
| <div style='text-align: right'>12.0</div>  | <div style='text-align: right'>32</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | <div style='text-align: right'>9.0</div>  | <span style="color: green">(-5.0 [-2.1%])</span> <div style='text-align: right'>232.0</div>  | <span style="color: green">(-14.0 [-7.5%])</span> <div style='text-align: right'>172.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+9.0 [+16.7%])</span> <div style='text-align: right'>63.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8,425,827</div>  | <div style='text-align: right'>198,497</div>  | <span style="color: green">(-44.0 [-2.6%])</span> <div style='text-align: right'>1,631.0</div>  | <div style='text-align: right'>14.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VolatileBoundaryAir | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<2> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<4> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | AccessAdapterAir<8> | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | ProgramChip | <div style='text-align: right'>16,295</div>  |
| verify_fibair | VmConnectorAir | <div style='text-align: right'>2</div>  |
| verify_fibair | Boundary | <div style='text-align: right'>44,590</div>  |
| verify_fibair | AccessAdapter<2> | <div style='text-align: right'>21,996</div>  |
| verify_fibair | AccessAdapter<4> | <div style='text-align: right'>10,998</div>  |
| verify_fibair | AccessAdapter<8> | <div style='text-align: right'>3,220</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>1,357</div>  |
| verify_fibair | FriReducedOpeningAir | <div style='text-align: right'>336</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>2,186</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>68,137</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>5,038</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>30,555</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>85,882</div>  |
| verify_fibair | PhantomAir | <div style='text-align: right'>5,216</div>  |
| verify_fibair | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | ADD | <div style='text-align: right'>54,977</div>  |
| verify_fibair |  | BBE4DIV | <div style='text-align: right'>297</div>  |
| verify_fibair |  | BBE4MUL | <div style='text-align: right'>891</div>  |
| verify_fibair |  | BEQ | <div style='text-align: right'>1,418</div>  |
| verify_fibair |  | BNE | <div style='text-align: right'>29,137</div>  |
| verify_fibair |  | COMP_POS2 | <div style='text-align: right'>1,092</div>  |
| verify_fibair |  | DIV | <div style='text-align: right'>3</div>  |
| verify_fibair |  | FE4ADD | <div style='text-align: right'>492</div>  |
| verify_fibair |  | FE4SUB | <div style='text-align: right'>506</div>  |
| verify_fibair |  | FRI_REDUCED_OPENING | <div style='text-align: right'>126</div>  |
| verify_fibair |  | JAL | <div style='text-align: right'>5,038</div>  |
| verify_fibair |  | LOADW | <div style='text-align: right'>18,438</div>  |
| verify_fibair |  | LOADW2 | <div style='text-align: right'>14,563</div>  |
| verify_fibair |  | MUL | <div style='text-align: right'>9,857</div>  |
| verify_fibair |  | PERM_POS2 | <div style='text-align: right'>265</div>  |
| verify_fibair |  | PHANTOM | <div style='text-align: right'>5,216</div>  |
| verify_fibair |  | SHINTW | <div style='text-align: right'>13,651</div>  |
| verify_fibair |  | STOREW | <div style='text-align: right'>30,345</div>  |
| verify_fibair |  | STOREW2 | <div style='text-align: right'>8,885</div>  |
| verify_fibair |  | SUB | <div style='text-align: right'>3,300</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>1,649,310</div>  |
| verify_fibair | AccessAdapter<2> |  | ADD | <div style='text-align: right'>11,902</div>  |
| verify_fibair | AccessAdapter<4> |  | ADD | <div style='text-align: right'>7,033</div>  |
| verify_fibair | Boundary |  | ADD | <div style='text-align: right'>1,727</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>11,880</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>2,904</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>1,716</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>35,640</div>  |
| verify_fibair | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>15,730</div>  |
| verify_fibair | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>9,295</div>  |
| verify_fibair | Boundary |  | BBE4MUL | <div style='text-align: right'>1,496</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>32,614</div>  |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>670,151</div>  |
| verify_fibair | AccessAdapter<2> |  | BNE | <div style='text-align: right'>946</div>  |
| verify_fibair | AccessAdapter<4> |  | BNE | <div style='text-align: right'>559</div>  |
| verify_fibair | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>48,048</div>  |
| verify_fibair | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>28,392</div>  |
| verify_fibair | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>18,564</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>610,428</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>90</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>19,680</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>10,846</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>6,409</div>  |
| verify_fibair | Boundary |  | FE4ADD | <div style='text-align: right'>792</div>  |
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>20,240</div>  |
| verify_fibair | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>18,656</div>  |
| verify_fibair | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>11,024</div>  |
| verify_fibair | Boundary |  | FE4SUB | <div style='text-align: right'>220</div>  |
| verify_fibair | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>2,024</div>  |
| verify_fibair | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>1,196</div>  |
| verify_fibair | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>21,504</div>  |
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>50,380</div>  |
| verify_fibair | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>755,958</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>20,295</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>11,232</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>4,284</div>  |
| verify_fibair | Boundary |  | LOADW | <div style='text-align: right'>17,424</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>597,083</div>  |
| verify_fibair | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>12,452</div>  |
| verify_fibair | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>7,358</div>  |
| verify_fibair | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>204</div>  |
| verify_fibair | Boundary |  | LOADW2 | <div style='text-align: right'>1,650</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>295,710</div>  |
| verify_fibair | AccessAdapter<2> |  | MUL | <div style='text-align: right'>3,751</div>  |
| verify_fibair | AccessAdapter<4> |  | MUL | <div style='text-align: right'>2,236</div>  |
| verify_fibair | Boundary |  | MUL | <div style='text-align: right'>29,348</div>  |
| verify_fibair | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>22,770</div>  |
| verify_fibair | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>13,455</div>  |
| verify_fibair | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>8,806</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>148,135</div>  |
| verify_fibair | PhantomAir |  | PHANTOM | <div style='text-align: right'>31,296</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>559,691</div>  |
| verify_fibair | Boundary |  | SHINTW | <div style='text-align: right'>150,161</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>1,244,145</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>5,445</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>3,120</div>  |
| verify_fibair | Boundary |  | STOREW | <div style='text-align: right'>204,556</div>  |
| verify_fibair | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>364,285</div>  |
| verify_fibair | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>3,828</div>  |
| verify_fibair | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>2,262</div>  |
| verify_fibair | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>17</div>  |
| verify_fibair | Boundary |  | STOREW2 | <div style='text-align: right'>67,848</div>  |
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>99,000</div>  |
| verify_fibair | AccessAdapter<2> |  | SUB | <div style='text-align: right'>1,419</div>  |
| verify_fibair | AccessAdapter<4> |  | SUB | <div style='text-align: right'>1,677</div>  |
| verify_fibair | Boundary |  | SUB | <div style='text-align: right'>15,257</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| verify_fibair | VolatileBoundaryAir | 0 | <div style='text-align: right'>1,245,184</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>65,536</div>  |
| verify_fibair | AccessAdapterAir<2> | 0 | <div style='text-align: right'>884,736</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | AccessAdapterAir<4> | 0 | <div style='text-align: right'>475,136</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>16,384</div>  |
| verify_fibair | AccessAdapterAir<8> | 0 | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>1,218,560</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| verify_fibair | FriReducedOpeningAir | 0 | <div style='text-align: right'>71,680</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>512</div>  |
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>245,760</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,096</div>  |
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>6,553,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>180,224</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>1,671,168</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>32,768</div>  |
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>8,519,680</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| verify_fibair | PhantomAir | 0 | <div style='text-align: right'>114,688</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| verify_fibair | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| verify_fibair | 0 | <span style="color: red">(+3.0 [+6.0%])</span> <div style='text-align: right'>53.0</div>  | <span style="color: green">(-42.0 [-3.0%])</span> <div style='text-align: right'>1,346.0</div>  | <div style='text-align: right'>23,969,816</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672/verify_fibair-2-2-64cpu-linux-arm64-mimalloc-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/0fd4a7ed88b5ea013b6f03b00c93de9f7fc3c672

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12130528540)
