| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-6.0 [-0.4%])</span> <div style='text-align: right'>1,705.0</div>  | <span style="color: green">(-28,641 [-0.7%])</span> <div style='text-align: right'>3,972,475</div>  | <span style="color: green">(-15,848 [-2.6%])</span> <div style='text-align: right'>590,858</div>  | <span style="color: green">(-8.0 [-0.5%])</span> <div style='text-align: right'>1,736.0</div>  | <span style="color: green">(-2.0 [-6.1%])</span> <div style='text-align: right'>31.0</div>  |  |
| inner_verifier | <span style="color: green">(-534.0 [-1.4%])</span> <div style='text-align: right'>37,302.0</div>  | <span style="color: green">(-25,821,184 [-8.1%])</span> <div style='text-align: right'>292,028,440</div>  | <span style="color: green">(-18,160,683 [-13.4%])</span> <div style='text-align: right'>117,659,738</div>  | <span style="color: green">(-2,197.0 [-5.0%])</span> <div style='text-align: right'>41,332.0</div>  | <span style="color: green">(-1,663.0 [-29.2%])</span> <div style='text-align: right'>4,030.0</div>  | <span style="color: green">(-8.0 [-1.9%])</span> <div style='text-align: right'>415.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | <div style='text-align: right'>288</div>  |
| bench_program_inner | BranchEqual | <div style='text-align: right'>161</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
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
| inner_verifier | BranchEqual | <span style="color: red">(+6,698 [+1.8%])</span> <div style='text-align: right'>371,969</div>  |
| inner_verifier | Core | <span style="color: green">(-1,194,241 [-92.8%])</span> <div style='text-align: right'>92,345</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+11,340 [+1.4%])</span> <div style='text-align: right'>822,024</div>  |
| inner_verifier | FieldExtension | <span style="color: red">(+3,624 [+1.3%])</span> <div style='text-align: right'>287,771</div>  |
| inner_verifier | Jal | <span style="color: red">(+285 [+0.8%])</span> <div style='text-align: right'>35,458</div>  |
| inner_verifier | LoadStore | <div style='text-align: right'>1,218,431</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+8,946 [+1.3%])</span> <div style='text-align: right'>691,321</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+4,452 [+1.3%])</span> <div style='text-align: right'>345,809</div>  |
| inner_verifier | Memory AccessAdapter<8> | <span style="color: red">(+170 [+1.0%])</span> <div style='text-align: right'>16,953</div>  |
| inner_verifier | Memory Boundary | <span style="color: red">(+2,531 [+0.9%])</span> <div style='text-align: right'>282,560</div>  |
| inner_verifier | Poseidon2 | <span style="color: red">(+147 [+1.2%])</span> <div style='text-align: right'>12,313</div>  |
| inner_verifier | ProgramChip | <span style="color: red">(+798 [+0.9%])</span> <div style='text-align: right'>93,762</div>  |

<details>
<summary>Click to expand</summary>

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
| inner_verifier | AddE | FE4ADD | <span style="color: red">(+927 [+1.3%])</span> <div style='text-align: right'>74,017</div>  |
| inner_verifier | AddEFFI | LOADW | <span style="color: red">(+2 [+1.5%])</span> <div style='text-align: right'>138</div>  |
| inner_verifier | AddEFFI | STOREW | <span style="color: red">(+6 [+1.5%])</span> <div style='text-align: right'>414</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+8 [+3.8%])</span> <div style='text-align: right'>220</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+128 [+0.5%])</span> <div style='text-align: right'>27,088</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+424 [+2.0%])</span> <div style='text-align: right'>21,961</div>  |
| inner_verifier | AddV | ADD | <span style="color: red">(+91 [+1.4%])</span> <div style='text-align: right'>6,680</div>  |
| inner_verifier | AddVI | ADD | <span style="color: red">(+2,233 [+1.5%])</span> <div style='text-align: right'>155,219</div>  |
| inner_verifier | Alloc | ADD | <span style="color: red">(+347 [+1.4%])</span> <div style='text-align: right'>25,677</div>  |
| inner_verifier | Alloc | LOADW | <span style="color: red">(+347 [+1.4%])</span> <div style='text-align: right'>25,677</div>  |
| inner_verifier | Alloc | MUL | <span style="color: red">(+244 [+1.6%])</span> <div style='text-align: right'>15,604</div>  |
| inner_verifier | AssertEqE | BNE | <span style="color: red">(+4 [+2.7%])</span> <div style='text-align: right'>152</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <span style="color: red">(+11 [+0.9%])</span> <div style='text-align: right'>1,237</div>  |
| inner_verifier | AssertEqVI | BNE | <span style="color: red">(+11 [+5.7%])</span> <div style='text-align: right'>204</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: red">(+924 [+2.3%])</span> <div style='text-align: right'>41,125</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: red">(+924 [+2.3%])</span> <div style='text-align: right'>41,125</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: red">(+884 [+1.4%])</span> <div style='text-align: right'>64,105</div>  |
| inner_verifier | DivEIN | BBE4DIV | <span style="color: red">(+3 [+7.1%])</span> <div style='text-align: right'>45</div>  |
| inner_verifier | DivEIN | STOREW | <span style="color: red">(+12 [+7.1%])</span> <div style='text-align: right'>180</div>  |
| inner_verifier | DivFIN | DIV | <span style="color: red">(+7 [+7.0%])</span> <div style='text-align: right'>107</div>  |
| inner_verifier | For | ADD | <span style="color: red">(+3,955 [+1.5%])</span> <div style='text-align: right'>259,730</div>  |
| inner_verifier | For | BNE | <span style="color: red">(+4,579 [+1.7%])</span> <div style='text-align: right'>281,417</div>  |
| inner_verifier | For | JAL | <span style="color: red">(+624 [+3.0%])</span> <div style='text-align: right'>21,687</div>  |
| inner_verifier | For | LOADW | <span style="color: red">(+63 [+5.5%])</span> <div style='text-align: right'>1,218</div>  |
| inner_verifier | For | STOREW | <span style="color: red">(+561 [+2.8%])</span> <div style='text-align: right'>20,469</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <span style="color: red">(+103 [+1.0%])</span> <div style='text-align: right'>10,073</div>  |
| inner_verifier | IfEq | BNE | <span style="color: red">(+1,071 [+11.0%])</span> <div style='text-align: right'>10,778</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: red">(+954 [+1.5%])</span> <div style='text-align: right'>65,884</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: green">(-339 [-2.4%])</span> <div style='text-align: right'>13,749</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,956</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>21</div>  |
| inner_verifier | IfNeI | BEQ | <span style="color: red">(+68 [+5.6%])</span> <div style='text-align: right'>1,283</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+16 [+0.2%])</span> <div style='text-align: right'>7,344</div>  |
| inner_verifier | ImmF | STOREW | <span style="color: red">(+1,178 [+6.5%])</span> <div style='text-align: right'>19,407</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: red">(+405 [+2.8%])</span> <div style='text-align: right'>14,931</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: red">(+176 [+1.1%])</span> <div style='text-align: right'>16,568</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: red">(+4,708 [+1.7%])</span> <div style='text-align: right'>282,644</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: red">(+1,851 [+1.8%])</span> <div style='text-align: right'>105,150</div>  |
| inner_verifier | LoadV | LOADW | <span style="color: red">(+124 [+1.0%])</span> <div style='text-align: right'>12,278</div>  |
| inner_verifier | LoadV | LOADW2 | <span style="color: red">(+3,619 [+4.2%])</span> <div style='text-align: right'>88,947</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: red">(+1,773 [+1.2%])</span> <div style='text-align: right'>143,776</div>  |
| inner_verifier | MulEF | MUL | <span style="color: red">(+24 [+1.4%])</span> <div style='text-align: right'>1,788</div>  |
| inner_verifier | MulEFI | MUL | <span style="color: red">(+12 [+2.3%])</span> <div style='text-align: right'>540</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: red">(+35 [+2.0%])</span> <div style='text-align: right'>1,825</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: red">(+140 [+2.0%])</span> <div style='text-align: right'>7,300</div>  |
| inner_verifier | MulF | MUL | <span style="color: red">(+766 [+1.9%])</span> <div style='text-align: right'>40,787</div>  |
| inner_verifier | MulFI | MUL | <span style="color: red">(+1 [+6.2%])</span> <div style='text-align: right'>17</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <span style="color: red">(+91 [+1.0%])</span> <div style='text-align: right'>8,777</div>  |
| inner_verifier | NegE | MUL | <span style="color: red">(+4 [+2.6%])</span> <div style='text-align: right'>160</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+63 [+0.8%])</span> <div style='text-align: right'>7,539</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+84 [+1.8%])</span> <div style='text-align: right'>4,774</div>  |
| inner_verifier | StoreE | STOREW | <span style="color: red">(+12 [+0.1%])</span> <div style='text-align: right'>11,304</div>  |
| inner_verifier | StoreE | STOREW2 | <span style="color: red">(+672 [+4.9%])</span> <div style='text-align: right'>14,516</div>  |
| inner_verifier | StoreF | STOREW | <span style="color: red">(+1,008 [+6.5%])</span> <div style='text-align: right'>16,412</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: red">(+549 [+1.5%])</span> <div style='text-align: right'>36,851</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: red">(+709 [+0.7%])</span> <div style='text-align: right'>103,008</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: red">(+812 [+0.7%])</span> <div style='text-align: right'>113,763</div>  |
| inner_verifier | StoreV | STOREW | <span style="color: red">(+15 [+1.0%])</span> <div style='text-align: right'>1,469</div>  |
| inner_verifier | StoreV | STOREW2 | <span style="color: red">(+665 [+2.5%])</span> <div style='text-align: right'>27,584</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>4,003</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: red">(+5,298 [+1.4%])</span> <div style='text-align: right'>382,392</div>  |
| inner_verifier | SubEF | SUB | <span style="color: red">(+1,766 [+1.4%])</span> <div style='text-align: right'>127,464</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>568</div>  |
| inner_verifier | SubEI | ADD | <span style="color: red">(+24 [+7.1%])</span> <div style='text-align: right'>360</div>  |
| inner_verifier | SubV | SUB | <span style="color: red">(+505 [+2.2%])</span> <div style='text-align: right'>23,957</div>  |
| inner_verifier | SubVI | SUB | <span style="color: red">(+1 [+0.1%])</span> <div style='text-align: right'>1,273</div>  |
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
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <span style="color: red">(+37,080 [+1.3%])</span> <div style='text-align: right'>2,960,680</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+3,894 [+1.8%])</span> <div style='text-align: right'>220,264</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+2,301 [+1.8%])</span> <div style='text-align: right'>130,156</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <span style="color: red">(+2,288 [+1.0%])</span> <div style='text-align: right'>237,776</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>5,658</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>924</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,092</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>330</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>16,974</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>924</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>990</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <span style="color: red">(+240 [+3.8%])</span> <div style='text-align: right'>6,600</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: red">(+88 [+11.1%])</span> <div style='text-align: right'>880</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: red">(+52 [+11.1%])</span> <div style='text-align: right'>520</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <span style="color: green">(-44 [-3.1%])</span> <div style='text-align: right'>1,364</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <span style="color: red">(+3,840 [+0.5%])</span> <div style='text-align: right'>812,640</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+1,364 [+0.9%])</span> <div style='text-align: right'>160,600</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+806 [+0.9%])</span> <div style='text-align: right'>94,900</div>  |
| inner_verifier | Boundary | AddEI | ADD | <span style="color: green">(-880 [-0.4%])</span> <div style='text-align: right'>204,116</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+12,720 [+2.0%])</span> <div style='text-align: right'>658,830</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>253</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <span style="color: red">(+2,730 [+1.4%])</span> <div style='text-align: right'>200,400</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <span style="color: red">(+66,990 [+1.5%])</span> <div style='text-align: right'>4,656,570</div>  |
| inner_verifier | Boundary | AddVI | ADD | <span style="color: red">(+11 [+0.1%])</span> <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <span style="color: red">(+10,410 [+1.4%])</span> <div style='text-align: right'>770,310</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>1,052,757</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>946</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <span style="color: red">(+7,320 [+1.6%])</span> <div style='text-align: right'>468,120</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <span style="color: red">(+92 [+2.7%])</span> <div style='text-align: right'>3,496</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <span style="color: red">(+22 [+2.7%])</span> <div style='text-align: right'>836</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <span style="color: red">(+13 [+2.7%])</span> <div style='text-align: right'>494</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <span style="color: red">(+253 [+0.9%])</span> <div style='text-align: right'>28,451</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <span style="color: red">(+253 [+5.7%])</span> <div style='text-align: right'>4,692</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-1,429,680 [-64.7%])</span> <div style='text-align: right'>781,375</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-1,429,680 [-64.7%])</span> <div style='text-align: right'>781,375</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <span style="color: red">(+35,360 [+1.4%])</span> <div style='text-align: right'>2,564,200</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: red">(+38,874 [+1.4%])</span> <div style='text-align: right'>2,804,560</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: red">(+22,971 [+1.4%])</span> <div style='text-align: right'>1,657,240</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <span style="color: red">(+120 [+7.1%])</span> <div style='text-align: right'>1,800</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+88 [+4.7%])</span> <div style='text-align: right'>1,958</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+52 [+4.7%])</span> <div style='text-align: right'>1,157</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>660</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>7,380</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <span style="color: red">(+44 [+7.3%])</span> <div style='text-align: right'>649</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <span style="color: red">(+13 [+7.7%])</span> <div style='text-align: right'>182</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <span style="color: red">(+210 [+7.0%])</span> <div style='text-align: right'>3,210</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <span style="color: red">(+118,650 [+1.5%])</span> <div style='text-align: right'>7,791,900</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <span style="color: red">(+105,317 [+1.7%])</span> <div style='text-align: right'>6,472,591</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <span style="color: red">(+6,240 [+3.0%])</span> <div style='text-align: right'>216,870</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <span style="color: red">(+22 [+4.3%])</span> <div style='text-align: right'>539</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <span style="color: red">(+26 [+4.3%])</span> <div style='text-align: right'>637</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>49,938</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>231</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>839,229</div>  |
| inner_verifier | Boundary | For | STOREW | <span style="color: red">(+22 [+3.8%])</span> <div style='text-align: right'>605</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <span style="color: green">(-792 [-65.5%])</span> <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <span style="color: green">(-356,963 [-65.1%])</span> <div style='text-align: right'>191,387</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <span style="color: red">(+24,633 [+11.0%])</span> <div style='text-align: right'>247,894</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <span style="color: red">(+21,942 [+1.5%])</span> <div style='text-align: right'>1,515,332</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: green">(-3,390 [-2.4%])</span> <div style='text-align: right'>137,490</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>159,988</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>210</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <span style="color: red">(+1,564 [+5.6%])</span> <div style='text-align: right'>29,509</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>301,104</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: red">(+330 [+8.3%])</span> <div style='text-align: right'>4,312</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: red">(+195 [+8.3%])</span> <div style='text-align: right'>2,548</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <span style="color: green">(-880 [-1.3%])</span> <div style='text-align: right'>66,616</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>795,687</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>1,353</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>612,171</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>8,701</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>679,288</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: red">(+924 [+1.4%])</span> <div style='text-align: right'>66,748</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: red">(+546 [+1.4%])</span> <div style='text-align: right'>39,442</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>4,092</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>11,588,404</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>470,393</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>286</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>4,311,150</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>308</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>503,398</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>7,975</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>3,646,827</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>935</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <span style="color: red">(+70,920 [+1.2%])</span> <div style='text-align: right'>5,751,040</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+1,144 [+0.3%])</span> <div style='text-align: right'>427,108</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+676 [+0.3%])</span> <div style='text-align: right'>252,382</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <span style="color: green">(-3,828 [-0.8%])</span> <div style='text-align: right'>471,416</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <span style="color: red">(+720 [+1.4%])</span> <div style='text-align: right'>53,640</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: green">(-22 [-0.3%])</span> <div style='text-align: right'>8,360</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: green">(-13 [-0.3%])</span> <div style='text-align: right'>4,940</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <span style="color: red">(+360 [+2.3%])</span> <div style='text-align: right'>16,200</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: red">(+110 [+4.6%])</span> <div style='text-align: right'>2,486</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: red">(+65 [+4.6%])</span> <div style='text-align: right'>1,469</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>3,696</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <span style="color: red">(+1,400 [+2.0%])</span> <div style='text-align: right'>73,000</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+2,970 [+3.3%])</span> <div style='text-align: right'>91,872</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+1,755 [+3.3%])</span> <div style='text-align: right'>54,288</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <span style="color: red">(+4,752 [+33.1%])</span> <div style='text-align: right'>19,096</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>299,300</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: red">(+770 [+2.0%])</span> <div style='text-align: right'>39,908</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: red">(+455 [+2.0%])</span> <div style='text-align: right'>23,452</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <span style="color: red">(+22,980 [+1.9%])</span> <div style='text-align: right'>1,223,610</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <span style="color: red">(+30 [+6.2%])</span> <div style='text-align: right'>510</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <span style="color: red">(+2,730 [+1.0%])</span> <div style='text-align: right'>263,310</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <span style="color: red">(+120 [+2.6%])</span> <div style='text-align: right'>4,800</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: green">(-22 [-2.1%])</span> <div style='text-align: right'>1,012</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: green">(-13 [-2.1%])</span> <div style='text-align: right'>598</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>968</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>301,224</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>177,996</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>116,382</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+26,334 [+0.8%])</span> <div style='text-align: right'>3,151,302</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+5,797 [+2.3%])</span> <div style='text-align: right'>258,170</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+3,289 [+2.2%])</span> <div style='text-align: right'>153,517</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+2,516 [+2.5%])</span> <div style='text-align: right'>101,983</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+35,112 [+1.8%])</span> <div style='text-align: right'>1,995,532</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>463,464</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <span style="color: red">(+132 [+0.1%])</span> <div style='text-align: right'>124,344</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>595,156</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <span style="color: red">(+3,696 [+6.2%])</span> <div style='text-align: right'>63,756</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <span style="color: red">(+2,184 [+6.2%])</span> <div style='text-align: right'>37,674</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>16,456</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>672,892</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <span style="color: red">(+11,088 [+6.5%])</span> <div style='text-align: right'>180,532</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>1,510,891</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: red">(+253 [+0.2%])</span> <div style='text-align: right'>152,174</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: red">(+13 [+0.0%])</span> <div style='text-align: right'>90,883</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: red">(+374 [+0.6%])</span> <div style='text-align: right'>60,877</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <span style="color: red">(+176 [+0.5%])</span> <div style='text-align: right'>34,804</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <span style="color: red">(+21,270 [+0.7%])</span> <div style='text-align: right'>3,090,240</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>4,664,283</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: red">(+8,932 [+0.7%])</span> <div style='text-align: right'>1,251,393</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>60,229</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <span style="color: red">(+165 [+1.0%])</span> <div style='text-align: right'>16,159</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>1,130,944</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <span style="color: red">(+7,315 [+2.5%])</span> <div style='text-align: right'>300,949</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <span style="color: red">(+80 [+0.0%])</span> <div style='text-align: right'>160,120</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-792 [-0.6%])</span> <div style='text-align: right'>132,836</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-468 [-0.6%])</span> <div style='text-align: right'>78,494</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <span style="color: green">(-1,408 [-1.2%])</span> <div style='text-align: right'>119,592</div>  |
| inner_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>15,678,072</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: red">(+19,426 [+1.4%])</span> <div style='text-align: right'>1,402,104</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <span style="color: red">(+52,980 [+1.4%])</span> <div style='text-align: right'>3,823,920</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: red">(+19,426 [+1.4%])</span> <div style='text-align: right'>1,402,104</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: red">(+22,958 [+1.4%])</span> <div style='text-align: right'>1,657,032</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>17,040</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: green">(-44 [-5.4%])</span> <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: green">(-26 [-5.4%])</span> <div style='text-align: right'>455</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>4,576</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <span style="color: red">(+720 [+7.1%])</span> <div style='text-align: right'>10,800</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: red">(+44 [+1.7%])</span> <div style='text-align: right'>2,640</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: red">(+26 [+1.7%])</span> <div style='text-align: right'>1,560</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <span style="color: red">(+15,150 [+2.2%])</span> <div style='text-align: right'>718,710</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <span style="color: red">(+30 [+0.1%])</span> <div style='text-align: right'>38,190</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>376,832</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>303,104</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,192</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>167,936</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4,096</div>  |
| bench_program_inner | AccessAdapterAir<16> | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | AccessAdapterAir<32> | <div style='text-align: right'>66,560</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,024</div>  |
| bench_program_inner | CoreAir | <span style="color: green">(-202,721 [-100.0%])</span> <div style='text-align: right'>31</div>  | <span style="color: green">(-67 [-80.7%])</span> <div style='text-align: right'>16</div>  | <span style="color: green">(-16 [-84.2%])</span> <div style='text-align: right'>3</div>  | <span style="color: green">(-36 [-65.5%])</span> <div style='text-align: right'>19</div>  | <span style="color: green">(-32 [-72.7%])</span> <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <span style="color: green">(-2,047 [-100.0%])</span> <div style='text-align: right'>1</div>  |
| bench_program_inner | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>174,080</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>18,176</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>256</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | <div style='text-align: right'>223,232</div>  | <div style='text-align: right'>187</div>  | <div style='text-align: right'>65</div>  | <div style='text-align: right'>172</div>  | <div style='text-align: right'>264</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | <div style='text-align: right'>54,784</div>  | <div style='text-align: right'>3,193</div>  | <div style='text-align: right'>93</div>  | <div style='text-align: right'>236</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>950,272</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | CoreAir | <span style="color: green">(-153,747,456 [-97.8%])</span> <div style='text-align: right'>3,538,944</div>  | <span style="color: green">(-62 [-80.5%])</span> <div style='text-align: right'>15</div>  | <span style="color: green">(-16 [-84.2%])</span> <div style='text-align: right'>3</div>  | <span style="color: green">(-36 [-65.5%])</span> <div style='text-align: right'>19</div>  | <span style="color: green">(-12 [-60.0%])</span> <div style='text-align: right'>8</div>  |  | <span style="color: green">(-4 [-50.0%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-1,966,080 [-93.8%])</span> <div style='text-align: right'>131,072</div>  |
| inner_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | <div style='text-align: right'>127,926,272</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>22,544,384</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>1,441,792</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>48,234,496</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>29,360,128</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>7,307,264</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>16,384</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/571a6e0a01dcf28d7a36f2c3ccafa2a9ff254af0
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11493051621)
