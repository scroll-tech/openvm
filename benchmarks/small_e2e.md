| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: red">(+15.0 [+1.1%])</span> <div style='text-align: right'>1,434.0</div>  | <div style='text-align: right'>1,980,240</div>  | <div style='text-align: right'>341,581</div>  | <span style="color: red">(+15.0 [+1.0%])</span> <div style='text-align: right'>1,453.0</div>  | <div style='text-align: right'>19.0</div>  |  |
| inner_verifier | <span style="color: red">(+751.0 [+0.9%])</span> <div style='text-align: right'>82,332.0</div>  | <div style='text-align: right'>651,755,544</div>  | <span style="color: green">(-1,660 [-0.0%])</span> <div style='text-align: right'>270,745,001</div>  | <span style="color: red">(+640.0 [+0.7%])</span> <div style='text-align: right'>91,705.0</div>  | <span style="color: green">(-111.0 [-1.2%])</span> <div style='text-align: right'>9,373.0</div>  | <span style="color: red">(+537.0 [+1.3%])</span> <div style='text-align: right'>43,121.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | BitwiseOperationLookup | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | BranchEqual | <div style='text-align: right'>5</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | LoadStore | <div style='text-align: right'>20</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| inner_verifier | BranchEqual | <div style='text-align: right'>706,303</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>1,595,224</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>857,974</div>  |
| inner_verifier | Jal | <span style="color: green">(-115 [-0.4%])</span> <div style='text-align: right'>29,666</div>  |
| inner_verifier | LoadStore | <div style='text-align: right'>2,928,722</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-24 [-0.0%])</span> <div style='text-align: right'>1,976,499</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-12 [-0.0%])</span> <div style='text-align: right'>988,293</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>33,560</div>  |
| inner_verifier | Memory Boundary | <div style='text-align: right'>598,936</div>  |
| inner_verifier | Phantom | <div style='text-align: right'>223,666</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>20,397</div>  |
| inner_verifier | ProgramChip | <div style='text-align: right'>202,954</div>  |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | AddE | FE4ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddF | ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddVI | ADD | <div style='text-align: right'>6</div>  |
| bench_program_inner | Alloc | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | LOADW | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | MUL | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | BNE | <div style='text-align: right'>3</div>  |
| bench_program_inner | For | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner | For | STOREW | <div style='text-align: right'>1</div>  |
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmE | STOREW | <div style='text-align: right'>8</div>  |
| bench_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>3</div>  |
| bench_program_inner | Keccak256 | KECCAK256 | <div style='text-align: right'>1</div>  |
| bench_program_inner | StoreV | STOREW2 | <div style='text-align: right'>2</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>227,660</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>133</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>399</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>224</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>67,996</div>  |
| inner_verifier | AddFI | ADD | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>14,055</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>6,262</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>278,789</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>24,557</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>24,557</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>14,833</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>140</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,205</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>172</div>  |
| inner_verifier | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>294</div>  |
| inner_verifier | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>2,436</div>  |
| inner_verifier | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>1,302</div>  |
| inner_verifier | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>714</div>  |
| inner_verifier | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>200,298</div>  |
| inner_verifier | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>3,780</div>  |
| inner_verifier | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-verify-batch | PHANTOM | <div style='text-align: right'>294</div>  |
| inner_verifier | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>714</div>  |
| inner_verifier | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>2,016</div>  |
| inner_verifier | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>2,016</div>  |
| inner_verifier | CT-verify-query | PHANTOM | <div style='text-align: right'>42</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>198,499</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>38</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>152</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>90</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>540,355</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>560,217</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>19,862</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,029</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>18,833</div>  |
| inner_verifier | HintBitsF | PHANTOM | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | PHANTOM | <div style='text-align: right'>9,724</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>7,629</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>124,910</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: green">(-115 [-1.2%])</span> <div style='text-align: right'>9,783</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>20</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,079</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>12,496</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,915</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>22,284</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>42,144</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>816,956</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,474</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>305,317</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>11,869</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>71,878</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>415,206</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,732</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,408</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>2,726</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>10,904</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>25,244</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>14</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>8,463</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>208</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,287</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>13,110</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>11,268</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,668</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,388</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>103,775</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>195,048</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>205,454</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,426</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>25,621</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>13,845</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>1,188,780</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>396,260</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,224</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>304</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>15,848</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,271</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>40</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <div style='text-align: right'>44</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>69</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>41</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>11</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>46</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>328</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <div style='text-align: right'>88</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>123</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <div style='text-align: right'>418</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <div style='text-align: right'>22</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>9,106,400</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>1,116,368</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>659,672</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>1,200,540</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>5,453</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,118</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>16,359</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>946</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>792</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>6,720</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>858</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>507</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>2,039,880</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: green">(-132 [-0.0%])</span> <div style='text-align: right'>381,524</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: green">(-78 [-0.0%])</span> <div style='text-align: right'>225,446</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>660,220</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: green">(-90 [-0.0%])</span> <div style='text-align: right'>421,650</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>187,860</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>8,363,670</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>8,668</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>736,710</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>1,006,837</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>444,990</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>3,220</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>455</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>27,715</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>3,956</div>  |
| inner_verifier | PhantomAir | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>1,764</div>  |
| inner_verifier | PhantomAir | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>14,616</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>7,812</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>4,284</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>1,201,788</div>  |
| inner_verifier | PhantomAir | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>22,680</div>  |
| inner_verifier | PhantomAir | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-verify-batch | PHANTOM | <div style='text-align: right'>1,764</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>4,284</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>12,096</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>12,096</div>  |
| inner_verifier | PhantomAir | CT-verify-query | PHANTOM | <div style='text-align: right'>252</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>7,939,960</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>8,717,962</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>5,151,523</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,520</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>936</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>396</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>6,232</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>550</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>156</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,700</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>16,210,650</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>12,884,991</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>198,620</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>484</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>572</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>42,189</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>231</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>772,153</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>583</div>  |
| inner_verifier | PhantomAir | HintBitsF | PHANTOM | <div style='text-align: right'>132</div>  |
| inner_verifier | PhantomAir | HintInputVec | PHANTOM | <div style='text-align: right'>58,344</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>175,467</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>2,872,930</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: green">(-1,150 [-1.2%])</span> <div style='text-align: right'>97,830</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>158,539</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>200</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>24,817</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>512,336</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>3,806</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>2,249</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>124,168</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>652,515</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>1,353</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>913,644</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>1,727,904</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>58,256</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>34,424</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>309,760</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>33,495,196</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>470,434</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>286</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>12,517,997</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>297</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>486,629</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>7,975</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>2,946,998</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>16,608,240</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: green">(-132 [-0.0%])</span> <div style='text-align: right'>495,880</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: green">(-78 [-0.0%])</span> <div style='text-align: right'>293,020</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>698,060</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>51,960</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,514</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>5,031</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>42,240</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,782</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,053</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>13,640</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>109,040</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>162,404</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>95,966</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>87,252</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>447,064</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>59,730</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>35,165</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>757,320</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>420</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>253,890</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>6,240</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,320</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>780</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>1,408</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,045,966</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>617,309</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>365,053</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>240,465</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>5,479,980</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>461,988</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>123,948</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>519,388</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>53,592</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>31,668</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>16,456</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>548,908</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>147,268</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>4,254,775</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>527,945</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>312,247</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>205,785</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>34,320</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>5,851,440</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>8,423,614</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>2,259,994</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>58,466</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>15,686</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>1,050,461</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>279,356</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>553,800</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>455,048</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>268,892</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>554,180</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>48,739,980</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>4,358,849</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>11,887,800</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>4,358,849</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>5,151,367</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>36,720</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>338</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>12,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>9,120</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,376</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,404</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>704</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>475,440</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>38,130</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>2,944</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>2,720</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>568</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | BitwiseOperationLookupAir<8> | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>255,852,544</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/97953e8d76ec6706c728dccc7eed175bd7221e8f/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/97953e8d76ec6706c728dccc7eed175bd7221e8f
AWS Instance Type: [64cpu-linux-arm64](https://instances.vantage.sh/aws/ec2/64cpu-linux-arm64)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11601308344)
