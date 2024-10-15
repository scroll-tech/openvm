| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-26.0 [-0.1%])</span> <div style='text-align: right'>17,498.0</div>  | <span style="color: green">(-2,048 [-0.0%])</span> <div style='text-align: right'>22,478,356</div>  | <span style="color: green">(-1,256 [-0.0%])</span> <div style='text-align: right'>2,663,633</div>  | <span style="color: green">(-26.0 [-0.1%])</span> <div style='text-align: right'>17,601.0</div>  | <div style='text-align: right'>103.0</div>  |  |
| inner_verifier | <span style="color: green">(-17.0 [-0.0%])</span> <div style='text-align: right'>40,863.0</div>  | <span style="color: green">(-1,048,576 [-0.3%])</span> <div style='text-align: right'>340,525,076</div>  | <span style="color: green">(-855,637 [-0.5%])</span> <div style='text-align: right'>177,332,780</div>  | <span style="color: red">(+452.0 [+0.8%])</span> <div style='text-align: right'>54,135.0</div>  | <span style="color: red">(+469.0 [+3.7%])</span> <div style='text-align: right'>13,272.0</div>  | <span style="color: red">(+80.0 [+19.3%])</span> <div style='text-align: right'>494.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | <div style='text-align: right'>288</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>1,296</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>1,256</div>  |
| bench_program_inner | Memory AccessAdapter<16> | <div style='text-align: right'>584</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>4,672</div>  |
| bench_program_inner | Memory AccessAdapter<32> | <div style='text-align: right'>292</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>2,336</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>1,168</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>9,607</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>370</div>  |
| bench_program_inner | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |
| bench_program_inner | Shift256 | <div style='text-align: right'>96</div>  |
| inner_verifier | ByteXor | <div style='text-align: right'>65,536</div>  |
| inner_verifier | Core | <span style="color: red">(+541 [+0.0%])</span> <div style='text-align: right'>1,725,016</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-151 [-0.0%])</span> <div style='text-align: right'>875,559</div>  |
| inner_verifier | FieldExtension | <span style="color: green">(-172 [-0.1%])</span> <div style='text-align: right'>270,558</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-200 [-0.0%])</span> <div style='text-align: right'>665,123</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-100 [-0.0%])</span> <div style='text-align: right'>332,711</div>  |
| inner_verifier | Memory AccessAdapter<8> | <div style='text-align: right'>19,258</div>  |
| inner_verifier | Memory Boundary | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>309,720</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>14,607</div>  |
| inner_verifier | ProgramChip | <span style="color: red">(+64 [+0.1%])</span> <div style='text-align: right'>90,340</div>  |
| inner_verifier | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |

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
| bench_program_inner | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
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
| inner_verifier | AddE | FE4ADD | <span style="color: green">(-19 [-0.0%])</span> <div style='text-align: right'>69,575</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>147</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>441</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+16 [+10.3%])</span> <div style='text-align: right'>172</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+60 [+0.2%])</span> <div style='text-align: right'>27,488</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+24 [+0.1%])</span> <div style='text-align: right'>22,128</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>8,513</div>  |
| inner_verifier | AddVI | ADD | <span style="color: green">(-42 [-0.0%])</span> <div style='text-align: right'>167,881</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>31,675</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>31,675</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>18,862</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>144</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,894</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,300</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>181</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: green">(-21 [-0.1%])</span> <div style='text-align: right'>38,395</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: green">(-21 [-0.1%])</span> <div style='text-align: right'>38,395</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: green">(-42 [-0.1%])</span> <div style='text-align: right'>59,817</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>37</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>148</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>89</div>  |
| inner_verifier | For | ADD | <span style="color: green">(-92 [-0.0%])</span> <div style='text-align: right'>275,797</div>  |
| inner_verifier | For | BNE | <span style="color: green">(-92 [-0.0%])</span> <div style='text-align: right'>299,855</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>24,058</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,197</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>22,861</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>12,813</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>8,354</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: green">(-21 [-0.0%])</span> <div style='text-align: right'>67,102</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+1,233 [+8.3%])</span> <div style='text-align: right'>16,163</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>9,618</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>25</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,252</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+4 [+0.1%])</span> <div style='text-align: right'>7,220</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>18,067</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>15,218</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: green">(-8 [-0.0%])</span> <div style='text-align: right'>16,728</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: green">(-168 [-0.1%])</span> <div style='text-align: right'>264,576</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>13,948</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: green">(-63 [-0.1%])</span> <div style='text-align: right'>97,596</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>15,189</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>88,949</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: green">(-109 [-0.1%])</span> <div style='text-align: right'>135,223</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>2,060</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>536</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>1,669</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>6,676</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>41,097</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>15</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>11,326</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>140</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>10,059</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>4,548</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>12,624</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>13,860</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>15,414</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: green">(-21 [-0.1%])</span> <div style='text-align: right'>35,155</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>121,496</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: green">(-29 [-0.0%])</span> <div style='text-align: right'>134,991</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,702</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>31,889</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>4,237</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: green">(-252 [-0.1%])</span> <div style='text-align: right'>356,166</div>  |
| inner_verifier | SubEF | SUB | <span style="color: green">(-84 [-0.1%])</span> <div style='text-align: right'>118,722</div>  |
| inner_verifier | SubEFI | ADD | <span style="color: green">(-4 [-0.7%])</span> <div style='text-align: right'>592</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>296</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>24,165</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,386</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>441</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | AccessAdapter<16> | Add256 | ADD<32,8> | <div style='text-align: right'>3,300</div>  |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD<32,8> | <div style='text-align: right'>11,616</div>  |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD<32,8> | <div style='text-align: right'>2,706</div>  |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD<32,8> | <div style='text-align: right'>6,864</div>  |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD<32,8> | <div style='text-align: right'>4,488</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Add256 | ADD<32,8> | <div style='text-align: right'>11,008</div>  |
| bench_program_inner | Boundary | Add256 | ADD<32,8> | <div style='text-align: right'>38,912</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>13,440</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>285</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>24,056</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | AccessAdapter<16> | And256 | AND<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | And256 | AND<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | And256 | AND<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | And256 | AND<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | And256 | AND<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | And256 | AND<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | And256 | AND<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | EqualTo256 | EQ<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | EqualTo256 | EQ<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | <NativeAdapterAir,NewFieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>960</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>2,046</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>7,936</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>2,717</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>32,054</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanI256 | SLT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanI256 | SLT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanU256 | LT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanU256 | LT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | Boundary | LoadV | LOADW | <div style='text-align: right'>57</div>  |
| bench_program_inner | CoreAir | LoadV | LOADW | <div style='text-align: right'>5,952</div>  |
| bench_program_inner | AccessAdapter<16> | Or256 | OR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Or256 | OR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Or256 | OR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Or256 | OR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Or256 | OR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Or256 | OR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Or256 | OR<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Boundary | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Boundary | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | Boundary | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | Boundary | StoreV | STOREW | <div style='text-align: right'>2,432</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW | <div style='text-align: right'>7,936</div>  |
| bench_program_inner | AccessAdapter<16> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | Sub256 | SUB<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | Sub256 | SUB<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Sub256 | SUB<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | AccessAdapter<16> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Xor256 | XOR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Xor256 | XOR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | Xor256 | XOR<32,8> | <div style='text-align: right'>19,456</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+198 [+0.1%])</span> <div style='text-align: right'>216,722</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+117 [+0.1%])</span> <div style='text-align: right'>128,063</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>412,984</div>  |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | <span style="color: green">(-779 [-0.0%])</span> <div style='text-align: right'>2,852,575</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: green">(-11 [-1.0%])</span> <div style='text-align: right'>1,100</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: green">(-13 [-1.0%])</span> <div style='text-align: right'>1,300</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>9,702</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: green">(-11 [-1.0%])</span> <div style='text-align: right'>1,100</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,254</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>29,106</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>5,160</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: red">(+88 [+17.4%])</span> <div style='text-align: right'>594</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: red">(+52 [+17.4%])</span> <div style='text-align: right'>351</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,280</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>824,640</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+880 [+0.6%])</span> <div style='text-align: right'>160,050</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+520 [+0.6%])</span> <div style='text-align: right'>94,575</div>  |
| inner_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>350,208</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>663,840</div>  |
| inner_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>255,390</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>5,036,430</div>  |
| inner_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>14,953</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>950,250</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>2,090,550</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>565,860</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>792</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>468</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <div style='text-align: right'>9,504</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <div style='text-align: right'>323,004</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <div style='text-align: right'>85,800</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <div style='text-align: right'>11,946</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-1,386 [-0.1%])</span> <div style='text-align: right'>2,534,070</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-1,386 [-0.1%])</span> <div style='text-align: right'>2,534,070</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: green">(-1,848 [-0.1%])</span> <div style='text-align: right'>2,612,236</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: green">(-1,092 [-0.1%])</span> <div style='text-align: right'>1,543,594</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | <span style="color: green">(-1,722 [-0.1%])</span> <div style='text-align: right'>2,452,497</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+22 [+1.5%])</span> <div style='text-align: right'>1,518</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+13 [+1.5%])</span> <div style='text-align: right'>897</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>456</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <div style='text-align: right'>1,517</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>143</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>9,768</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,670</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>8,273,910</div>  |
| inner_verifier | CoreAir | For | BNE | <span style="color: green">(-6,072 [-0.0%])</span> <div style='text-align: right'>19,790,430</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>624</div>  |
| inner_verifier | CoreAir | For | JAL | <div style='text-align: right'>1,587,828</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>79,002</div>  |
| inner_verifier | Boundary | For | STOREW | <div style='text-align: right'>1,045</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,508,826</div>  |
| inner_verifier | CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,452</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>845,658</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <div style='text-align: right'>551,364</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <span style="color: green">(-1,386 [-0.0%])</span> <div style='text-align: right'>4,428,732</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: red">(+81,378 [+8.3%])</span> <div style='text-align: right'>1,066,758</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>634,788</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,650</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>82,632</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: green">(-22 [-0.7%])</span> <div style='text-align: right'>3,344</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: green">(-13 [-0.7%])</span> <div style='text-align: right'>1,976</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>116,280</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <span style="color: red">(+264 [+0.1%])</span> <div style='text-align: right'>476,520</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>1,192,422</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>1,004,388</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: green">(-66 [-0.1%])</span> <div style='text-align: right'>66,264</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: green">(-39 [-0.1%])</span> <div style='text-align: right'>39,156</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>8,816</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <span style="color: green">(-528 [-0.0%])</span> <div style='text-align: right'>1,104,048</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>29,634</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>17,511</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <span style="color: green">(-11,088 [-0.1%])</span> <div style='text-align: right'>17,462,016</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>26,796</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>15,834</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>10,353</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>475</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>920,568</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>693</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>416</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>459</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>551</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <span style="color: green">(-4,158 [-0.1%])</span> <div style='text-align: right'>6,441,336</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,813</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>1,002,474</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>5,870,634</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+462 [+0.1%])</span> <div style='text-align: right'>427,790</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+273 [+0.1%])</span> <div style='text-align: right'>252,785</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>824,752</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | <span style="color: green">(-4,469 [-0.1%])</span> <div style='text-align: right'>5,544,143</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>61,800</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: green">(-22 [-0.2%])</span> <div style='text-align: right'>10,318</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: green">(-13 [-0.2%])</span> <div style='text-align: right'>6,097</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>16,080</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>2,068</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,222</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>7,676</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+88 [+0.1%])</span> <div style='text-align: right'>82,456</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+52 [+0.1%])</span> <div style='text-align: right'>48,724</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>18,088</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | <div style='text-align: right'>68,429</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>36,432</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>21,372</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>440,616</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>1,232,910</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>450</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>339,780</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>4,200</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: green">(-44 [-5.3%])</span> <div style='text-align: right'>792</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: green">(-26 [-5.3%])</span> <div style='text-align: right'>468</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>1,596</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>417,648</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>246,792</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>161,364</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>4,204,662</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>248,721</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>147,940</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>97,835</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>1,901,064</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>9,746</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>5,759</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>239,856</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>833,184</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>56,364</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>33,306</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>35,112</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>914,760</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>292,866</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>1,017,324</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>144,485</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>86,346</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>57,375</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>72,200</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <span style="color: green">(-1,386 [-0.1%])</span> <div style='text-align: right'>2,320,230</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>3,644,880</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: green">(-551 [-0.0%])</span> <div style='text-align: right'>2,564,829</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <span style="color: green">(-1,914 [-0.0%])</span> <div style='text-align: right'>8,909,406</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>32,338</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>112,332</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>603,212</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>2,104,674</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-22 [-0.0%])</span> <div style='text-align: right'>142,670</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-13 [-0.0%])</span> <div style='text-align: right'>84,305</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>209,000</div>  |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | <span style="color: green">(-82 [-0.0%])</span> <div style='text-align: right'>173,717</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: green">(-924 [-0.1%])</span> <div style='text-align: right'>1,305,942</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <span style="color: green">(-16,632 [-0.1%])</span> <div style='text-align: right'>23,506,956</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>3,561,660</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: green">(-924 [-0.1%])</span> <div style='text-align: right'>1,305,942</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: green">(-1,092 [-0.1%])</span> <div style='text-align: right'>1,543,386</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>17,760</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: green">(-44 [-7.7%])</span> <div style='text-align: right'>528</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: green">(-26 [-7.7%])</span> <div style='text-align: right'>312</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>9,576</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>8,880</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,508</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,482</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>912</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>724,950</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>41,580</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir,NewFieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>13,230</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>217,088</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir, NewFieldArithmeticCoreAir> | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | <div style='text-align: right'>223,232</div>  | <div style='text-align: right'>187</div>  | <div style='text-align: right'>65</div>  | <div style='text-align: right'>172</div>  | <div style='text-align: right'>264</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | <div style='text-align: right'>54,784</div>  | <div style='text-align: right'>3,193</div>  | <div style='text-align: right'>93</div>  | <div style='text-align: right'>236</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>303,104</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,192</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>167,936</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4,096</div>  |
| bench_program_inner | AccessAdapterAir<16> | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | AccessAdapterAir<32> | <div style='text-align: right'>66,560</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,024</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>180,355,072</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir, NewFieldArithmeticCoreAir> | <div style='text-align: right'>48,234,496</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | FieldExtensionArithmeticAir | <div style='text-align: right'>29,884,416</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>7,307,264</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>16,384</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>950,272</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6b543cb0a3bba565cd1dddf0904a54dcc3f97169/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6b543cb0a3bba565cd1dddf0904a54dcc3f97169
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11354735087)
