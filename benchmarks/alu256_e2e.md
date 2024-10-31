| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| bench_program_inner | <div style='text-align: right'>656,394</div>  | <div style='text-align: right'>2,935</div>  | <span style="color: green">(-10.0 [-0.6%])</span> <div style='text-align: right'>1,775.0</div>  |
| inner_verifier | <span style="color: red">(+8,150 [+0.0%])</span> <div style='text-align: right'>114,355,011</div>  | <span style="color: red">(+736 [+0.0%])</span> <div style='text-align: right'>2,789,333</div>  | <span style="color: green">(-388.0 [-0.9%])</span> <div style='text-align: right'>41,084.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_and_trace_gen_time_ms | execute_time_ms | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-1.0 [-3.1%])</span> <div style='text-align: right'>31.0</div>  | <span style="color: green">(-1.0 [-3.3%])</span> <div style='text-align: right'>29.0</div>  | <span style="color: green">(-9.0 [-0.5%])</span> <div style='text-align: right'>1,744.0</div>  | <div style='text-align: right'>4,037,980</div>  | <div style='text-align: right'>656,394</div>  | <div style='text-align: right'>2,935</div>  | <span style="color: green">(-10.0 [-0.6%])</span> <div style='text-align: right'>1,775.0</div>  |  |
| inner_verifier | <span style="color: green">(-58.0 [-1.4%])</span> <div style='text-align: right'>3,981.0</div>  | <span style="color: green">(-48.0 [-1.4%])</span> <div style='text-align: right'>3,338.0</div>  | <span style="color: green">(-330.0 [-0.9%])</span> <div style='text-align: right'>37,103.0</div>  | <div style='text-align: right'>290,324,504</div>  | <span style="color: red">(+8,150 [+0.0%])</span> <div style='text-align: right'>114,355,011</div>  | <span style="color: red">(+736 [+0.0%])</span> <div style='text-align: right'>2,789,333</div>  | <span style="color: green">(-388.0 [-0.9%])</span> <div style='text-align: right'>41,084.0</div>  | <div style='text-align: right'>411.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | <div style='text-align: right'>288</div>  |
| bench_program_inner | BitwiseOperationLookup | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | BranchEqual | <div style='text-align: right'>161</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>1,256</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | LoadStore | <div style='text-align: right'>1,132</div>  |
| bench_program_inner | Memory AccessAdapter<16> | <div style='text-align: right'>584</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>4,672</div>  |
| bench_program_inner | Memory AccessAdapter<32> | <div style='text-align: right'>292</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>2,336</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>1,168</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>9,607</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>370</div>  |
| bench_program_inner | Shift256 | <div style='text-align: right'>96</div>  |
| inner_verifier | BranchEqual | <div style='text-align: right'>365,391</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+22 [+0.0%])</span> <div style='text-align: right'>811,402</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>282,300</div>  |
| inner_verifier | Jal | <span style="color: red">(+714 [+2.0%])</span> <div style='text-align: right'>35,698</div>  |
| inner_verifier | LoadStore | <div style='text-align: right'>1,192,295</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+20 [+0.0%])</span> <div style='text-align: right'>678,723</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+10 [+0.0%])</span> <div style='text-align: right'>339,510</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>16,699</div>  |
| inner_verifier | Memory Boundary | <div style='text-align: right'>279,981</div>  |
| inner_verifier | Phantom | <div style='text-align: right'>90,123</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>12,124</div>  |
| inner_verifier | ProgramChip | <div style='text-align: right'>92,976</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | Add256 | ADD<32,8> | <div style='text-align: right'>64</div>  |
| bench_program_inner | AddVI | ADD | <div style='text-align: right'>448</div>  |
| bench_program_inner | Alloc | ADD | <div style='text-align: right'>388</div>  |
| bench_program_inner | Alloc | LOADW | <div style='text-align: right'>388</div>  |
| bench_program_inner | Alloc | MUL | <div style='text-align: right'>388</div>  |
| bench_program_inner | And256 | AND<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | EqualTo256 | EQ<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | For | ADD | <div style='text-align: right'>32</div>  |
| bench_program_inner | For | BNE | <div style='text-align: right'>33</div>  |
| bench_program_inner | For | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner | For | STOREW | <div style='text-align: right'>1</div>  |
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>128</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>517</div>  |
| bench_program_inner | LessThanI256 | SLT<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | LessThanU256 | LT<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | LoadV | LOADW | <div style='text-align: right'>96</div>  |
| bench_program_inner | Or256 | OR<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | StoreV | STOREW | <div style='text-align: right'>128</div>  |
| bench_program_inner | Sub256 | SUB<32,8> | <div style='text-align: right'>32</div>  |
| bench_program_inner | Xor256 | XOR<32,8> | <div style='text-align: right'>32</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>72,627</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>137</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>411</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>212</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>27,072</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+22 [+0.1%])</span> <div style='text-align: right'>21,995</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>6,612</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>153,181</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>25,377</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>25,377</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>15,384</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>148</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,227</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>194</div>  |
| inner_verifier | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>294</div>  |
| inner_verifier | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>2,772</div>  |
| inner_verifier | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>1,554</div>  |
| inner_verifier | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>714</div>  |
| inner_verifier | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>64,890</div>  |
| inner_verifier | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>4,284</div>  |
| inner_verifier | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| inner_verifier | CT-verify-batch | PHANTOM | <div style='text-align: right'>294</div>  |
| inner_verifier | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>714</div>  |
| inner_verifier | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>2,268</div>  |
| inner_verifier | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>2,268</div>  |
| inner_verifier | CT-verify-query | PHANTOM | <div style='text-align: right'>42</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>62,759</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>44</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>176</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>104</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>255,487</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>276,661</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>21,174</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,155</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>20,019</div>  |
| inner_verifier | HintBitsF | PHANTOM | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | PHANTOM | <div style='text-align: right'>9,993</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>9,749</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>65,183</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+714 [+5.2%])</span> <div style='text-align: right'>14,502</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,956</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>21</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,215</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>7,316</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>18,271</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>14,528</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>16,304</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>276,340</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,474</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>102,757</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>12,180</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>86,277</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>141,085</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,780</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>512</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>1,796</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>7,184</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>40,888</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>16</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>8,708</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>160</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,476</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>4,648</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>11,292</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>14,012</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>15,404</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>36,119</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>102,024</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>112,699</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,456</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>27,077</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>3,989</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>374,328</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>124,776</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>536</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>352</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>23,914</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,273</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| bench_program_inner | AccessAdapter<16> | Add256 | ADD<32,8> | <div style='text-align: right'>3,300</div>  |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD<32,8> | <div style='text-align: right'>11,616</div>  |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD<32,8> | <div style='text-align: right'>2,706</div>  |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD<32,8> | <div style='text-align: right'>6,864</div>  |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD<32,8> | <div style='text-align: right'>4,488</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Add256 | ADD<32,8> | <div style='text-align: right'>11,008</div>  |
| bench_program_inner | Boundary | Add256 | ADD<32,8> | <div style='text-align: right'>22,528</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>13,440</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>15,908</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>165</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | AccessAdapter<16> | And256 | AND<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | And256 | AND<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | And256 | AND<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | And256 | AND<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | And256 | AND<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | And256 | AND<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | And256 | AND<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | EqualTo256 | EQ<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | EqualTo256 | EQ<32,8> | <div style='text-align: right'>352</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>960</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>759</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>41</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>11</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>2,944</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>21,197</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>1,573</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanI256 | SLT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanI256 | SLT<32,8> | <div style='text-align: right'>352</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanU256 | LT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanU256 | LT<32,8> | <div style='text-align: right'>352</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>3,936</div>  |
| bench_program_inner | Boundary | LoadV | LOADW | <div style='text-align: right'>33</div>  |
| bench_program_inner | AccessAdapter<16> | Or256 | OR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Or256 | OR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Or256 | OR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Or256 | OR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Or256 | OR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Or256 | OR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Or256 | OR<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Boundary | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Boundary | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | Boundary | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>5,248</div>  |
| bench_program_inner | Boundary | StoreV | STOREW | <div style='text-align: right'>1,408</div>  |
| bench_program_inner | AccessAdapter<16> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | Sub256 | SUB<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | Sub256 | SUB<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Sub256 | SUB<32,8> | <div style='text-align: right'>11,264</div>  |
| bench_program_inner | AccessAdapter<16> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Xor256 | XOR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Xor256 | XOR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Xor256 | XOR<32,8> | <div style='text-align: right'>11,264</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>2,905,080</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>219,824</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>129,896</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>237,864</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>5,617</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>913</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,079</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>330</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>16,851</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>913</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>990</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>6,360</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>792</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>468</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,364</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>812,160</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+110 [+0.1%])</span> <div style='text-align: right'>159,918</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+65 [+0.1%])</span> <div style='text-align: right'>94,497</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>205,744</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+660 [+0.1%])</span> <div style='text-align: right'>659,850</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>253</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>198,360</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>4,595,430</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>8,690</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>761,310</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>1,040,457</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>461,520</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>3,404</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>814</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>481</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>28,221</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>4,462</div>  |
| inner_verifier | PhantomAir | CT-VerifierProgram | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>1,764</div>  |
| inner_verifier | PhantomAir | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>16,632</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>9,324</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>4,284</div>  |
| inner_verifier | PhantomAir | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>389,340</div>  |
| inner_verifier | PhantomAir | CT-sp1-fri-fold | PHANTOM | <div style='text-align: right'>25,704</div>  |
| inner_verifier | PhantomAir | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| inner_verifier | PhantomAir | CT-verify-batch | PHANTOM | <div style='text-align: right'>1,764</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>4,284</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>13,608</div>  |
| inner_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>13,608</div>  |
| inner_verifier | PhantomAir | CT-verify-query | PHANTOM | <div style='text-align: right'>252</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>2,510,360</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>2,745,358</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>1,622,257</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,760</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,958</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>1,157</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>660</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>7,216</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>638</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>182</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>3,120</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>7,664,610</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>6,363,203</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>211,740</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>624</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>47,355</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>231</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>820,779</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>627</div>  |
| inner_verifier | PhantomAir | HintBitsF | PHANTOM | <div style='text-align: right'>132</div>  |
| inner_verifier | PhantomAir | HintInputVec | PHANTOM | <div style='text-align: right'>59,958</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>224,227</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>1,499,209</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: red">(+7,140 [+5.2%])</span> <div style='text-align: right'>145,020</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>159,988</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>210</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>27,945</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>299,956</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>4,114</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>2,431</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>66,616</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>749,111</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>1,353</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>595,648</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>668,464</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>65,758</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>38,857</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>4,092</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>11,329,940</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>470,434</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>286</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>4,213,037</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>308</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>499,380</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>7,975</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>3,537,357</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>5,643,400</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+110 [+0.0%])</span> <div style='text-align: right'>424,380</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+65 [+0.0%])</span> <div style='text-align: right'>250,770</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>470,096</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>53,400</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,382</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>4,953</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>15,360</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>2,200</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,300</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>3,696</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>71,840</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>90,200</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>53,300</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>19,096</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>294,544</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>39,270</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>23,075</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>1,226,640</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>480</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>261,240</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>4,800</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,012</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>598</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>968</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>301,224</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>177,996</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>116,382</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,124,968</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>250,756</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>149,136</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>98,753</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>1,942,864</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>462,972</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>124,212</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>574,492</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>60,984</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>36,036</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>16,456</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>631,564</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>169,444</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>1,480,879</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>150,304</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>89,778</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>59,789</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>34,672</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>3,060,720</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>4,620,659</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>1,239,689</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>59,696</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>16,016</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>1,110,157</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>295,372</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>159,560</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>132,572</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>78,338</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>119,592</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>15,347,448</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>1,372,525</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>3,743,280</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>1,372,525</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>1,622,075</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>16,080</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>594</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>351</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>4,576</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>10,560</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,684</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,586</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>717,420</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>38,190</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>376,832</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>303,104</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,192</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>167,936</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4,096</div>  |
| bench_program_inner | AccessAdapterAir<16> | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | AccessAdapterAir<32> | <div style='text-align: right'>66,560</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,024</div>  |
| bench_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>174,080</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>18,176</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>256</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | <div style='text-align: right'>223,232</div>  | <div style='text-align: right'>187</div>  | <div style='text-align: right'>65</div>  | <div style='text-align: right'>172</div>  | <div style='text-align: right'>264</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | <div style='text-align: right'>54,784</div>  | <div style='text-align: right'>3,193</div>  | <div style='text-align: right'>93</div>  | <div style='text-align: right'>236</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | BitwiseOperationLookupAir<8> | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>950,272</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | PhantomAir | <div style='text-align: right'>1,835,008</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>127,926,272</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>22,544,384</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>1,441,792</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>48,234,496</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>29,360,128</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>7,307,264</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>16,384</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-bench_program_inner.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/cd0471026f5beb78d39b28fa994bb8205832349c/alu256_e2e-inner_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/cd0471026f5beb78d39b28fa994bb8205832349c
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11620072025)
