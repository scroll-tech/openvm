| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-3.0 [-0.3%])</span> <div style='text-align: right'>1,038.0</div>  | <div style='text-align: right'>3,601,428</div>  | <div style='text-align: right'>633,643</div>  | <span style="color: green">(-3.0 [-0.3%])</span> <div style='text-align: right'>1,079.0</div>  | <div style='text-align: right'>41.0</div>  |  |
| inner_verifier | <span style="color: red">(+9.0 [+0.0%])</span> <div style='text-align: right'>38,396.0</div>  | <div style='text-align: right'>321,519,636</div>  | <span style="color: green">(-4,144 [-0.0%])</span> <div style='text-align: right'>158,527,135</div>  | <span style="color: green">(-19.0 [-0.0%])</span> <div style='text-align: right'>52,777.0</div>  | <span style="color: green">(-28.0 [-0.2%])</span> <div style='text-align: right'>14,381.0</div>  | <div style='text-align: right'>379.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | <div style='text-align: right'>288</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>1,296</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>1,256</div>  |
| bench_program_inner | Memory | <div style='text-align: right'>9,607</div>  |
| bench_program_inner | Memory 2 | <div style='text-align: right'>4,672</div>  |
| bench_program_inner | Memory 3 | <div style='text-align: right'>2,336</div>  |
| bench_program_inner | Memory 4 | <div style='text-align: right'>1,168</div>  |
| bench_program_inner | Memory 5 | <div style='text-align: right'>584</div>  |
| bench_program_inner | Memory 6 | <div style='text-align: right'>292</div>  |
| bench_program_inner | Program | <div style='text-align: right'>370</div>  |
| bench_program_inner | RangeChecker | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Shift256 | <div style='text-align: right'>96</div>  |
| inner_verifier | ByteXor | <div style='text-align: right'>65,536</div>  |
| inner_verifier | Core | <span style="color: green">(-63 [-0.0%])</span> <div style='text-align: right'>1,557,832</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+14 [+0.0%])</span> <div style='text-align: right'>751,373</div>  |
| inner_verifier | FieldExtension | <div style='text-align: right'>267,359</div>  |
| inner_verifier | Memory | <div style='text-align: right'>266,779</div>  |
| inner_verifier | Memory 2 | <span style="color: green">(-24 [-0.0%])</span> <div style='text-align: right'>641,071</div>  |
| inner_verifier | Memory 3 | <span style="color: green">(-12 [-0.0%])</span> <div style='text-align: right'>320,685</div>  |
| inner_verifier | Memory 4 | <div style='text-align: right'>15,580</div>  |
| inner_verifier | Poseidon2 | <div style='text-align: right'>11,198</div>  |
| inner_verifier | Program | <div style='text-align: right'>89,758</div>  |
| inner_verifier | RangeChecker | <div style='text-align: right'>65,536</div>  |

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
| inner_verifier | AddE | FE4ADD | <div style='text-align: right'>68,798</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>127</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>381</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>156</div>  |
| inner_verifier | AddEI | ADD | <div style='text-align: right'>26,084</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+14 [+0.1%])</span> <div style='text-align: right'>19,820</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>5,946</div>  |
| inner_verifier | AddVI | ADD | <div style='text-align: right'>141,696</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>23,328</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>23,328</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>14,134</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>140</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>3,886</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,181</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>140</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <div style='text-align: right'>37,156</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <div style='text-align: right'>37,156</div>  |
| inner_verifier | DivE | BBE4DIV | <div style='text-align: right'>59,206</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>36</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>144</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>86</div>  |
| inner_verifier | For | ADD | <div style='text-align: right'>235,455</div>  |
| inner_verifier | For | BNE | <div style='text-align: right'>254,571</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>19,116</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,008</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>18,108</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>9,194</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>6,723</div>  |
| inner_verifier | IfEqI | BNE | <div style='text-align: right'>60,346</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: green">(-63 [-0.5%])</span> <div style='text-align: right'>12,382</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,448</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,016</div>  |
| inner_verifier | ImmE | STOREW | <div style='text-align: right'>7,200</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,713</div>  |
| inner_verifier | ImmV | STOREW | <div style='text-align: right'>13,363</div>  |
| inner_verifier | LoadE | LOADW | <div style='text-align: right'>15,364</div>  |
| inner_verifier | LoadE | LOADW2 | <div style='text-align: right'>259,196</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>10,939</div>  |
| inner_verifier | LoadF | LOADW2 | <div style='text-align: right'>96,246</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>11,289</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>74,962</div>  |
| inner_verifier | MulE | BBE4MUL | <div style='text-align: right'>133,772</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,632</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>536</div>  |
| inner_verifier | MulEI | BBE4MUL | <div style='text-align: right'>1,628</div>  |
| inner_verifier | MulEI | STOREW | <div style='text-align: right'>6,512</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>36,809</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>14</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>7,979</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>136</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>6,846</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>4,352</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>10,932</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,328</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,228</div>  |
| inner_verifier | StoreF | STOREW2 | <div style='text-align: right'>33,905</div>  |
| inner_verifier | StoreHintWord | ADD | <div style='text-align: right'>95,168</div>  |
| inner_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>105,044</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,363</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>24,512</div>  |
| inner_verifier | SubE | FE4SUB | <div style='text-align: right'>3,919</div>  |
| inner_verifier | SubEF | LOADW | <div style='text-align: right'>353,136</div>  |
| inner_verifier | SubEF | SUB | <div style='text-align: right'>117,712</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>596</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>288</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>21,539</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,241</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>336</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | AccessAdapter<16> | Add256 | ADD<32,8> | <div style='text-align: right'>3,300</div>  |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD<32,8> | <div style='text-align: right'>11,616</div>  |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD<32,8> | <div style='text-align: right'>2,706</div>  |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD<32,8> | <div style='text-align: right'>6,864</div>  |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD<32,8> | <div style='text-align: right'>4,488</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Add256 | ADD<32,8> | <div style='text-align: right'>11,008</div>  |
| bench_program_inner | Audit | Add256 | ADD<32,8> | <div style='text-align: right'>38,912</div>  |
| bench_program_inner | Audit | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>13,888</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>12,028</div>  |
| bench_program_inner | Audit | Alloc | LOADW | <div style='text-align: right'>285</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>24,056</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>12,028</div>  |
| bench_program_inner | AccessAdapter<16> | And256 | AND<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | And256 | AND<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | And256 | AND<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | And256 | AND<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | And256 | AND<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | And256 | AND<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | And256 | AND<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | EqualTo256 | EQ<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | EqualTo256 | EQ<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | FieldArithmeticAir | For | ADD | <div style='text-align: right'>992</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>2,046</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>7,936</div>  |
| bench_program_inner | Audit | ImmV | STOREW | <div style='text-align: right'>2,717</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>32,054</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanI256 | SLT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | LessThanI256 | SLT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanU256 | LT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | LessThanU256 | LT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | Audit | LoadV | LOADW | <div style='text-align: right'>57</div>  |
| bench_program_inner | CoreAir | LoadV | LOADW | <div style='text-align: right'>5,952</div>  |
| bench_program_inner | AccessAdapter<16> | Or256 | OR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Or256 | OR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Or256 | OR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Or256 | OR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Or256 | OR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Or256 | OR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | Or256 | OR<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Audit | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftAir<32, 8> | ShiftLeft256 | SLL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | Audit | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightArith256 | SRA<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | Audit | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightLogic256 | SRL<32,8> | <div style='text-align: right'>7,552</div>  |
| bench_program_inner | Audit | StoreV | STOREW | <div style='text-align: right'>2,432</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW | <div style='text-align: right'>7,936</div>  |
| bench_program_inner | AccessAdapter<16> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,650</div>  |
| bench_program_inner | AccessAdapter<2> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,808</div>  |
| bench_program_inner | AccessAdapter<32> | Sub256 | SUB<32,8> | <div style='text-align: right'>1,353</div>  |
| bench_program_inner | AccessAdapter<4> | Sub256 | SUB<32,8> | <div style='text-align: right'>3,432</div>  |
| bench_program_inner | AccessAdapter<8> | Sub256 | SUB<32,8> | <div style='text-align: right'>2,244</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Sub256 | SUB<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | Sub256 | SUB<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | AccessAdapter<16> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | Xor256 | XOR<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | Xor256 | XOR<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | Xor256 | XOR<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Xor256 | XOR<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Audit | Xor256 | XOR<32,8> | <div style='text-align: right'>19,456</div>  |
| inner_verifier | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| inner_verifier | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>209,572</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>123,838</div>  |
| inner_verifier | Audit | AddE | FE4ADD | <div style='text-align: right'>412,984</div>  |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>2,820,718</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>891</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>1,053</div>  |
| inner_verifier | Audit | AddEFFI | LOADW | <div style='text-align: right'>418</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>8,382</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>891</div>  |
| inner_verifier | Audit | AddEFFI | STOREW | <div style='text-align: right'>1,254</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>25,146</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>299</div>  |
| inner_verifier | Audit | AddEFI | ADD | <div style='text-align: right'>2,280</div>  |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | <div style='text-align: right'>4,836</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: green">(-132 [-0.1%])</span> <div style='text-align: right'>152,570</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: green">(-78 [-0.1%])</span> <div style='text-align: right'>90,155</div>  |
| inner_verifier | Audit | AddEI | ADD | <div style='text-align: right'>350,208</div>  |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | <div style='text-align: right'>808,604</div>  |
| inner_verifier | Audit | AddFI | ADD | <div style='text-align: right'>456</div>  |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | <span style="color: red">(+434 [+0.1%])</span> <div style='text-align: right'>614,420</div>  |
| inner_verifier | Audit | AddV | ADD | <div style='text-align: right'>57</div>  |
| inner_verifier | FieldArithmeticAir | AddV | ADD | <div style='text-align: right'>184,326</div>  |
| inner_verifier | Audit | AddVI | ADD | <div style='text-align: right'>15,029</div>  |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>4,392,576</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>723,168</div>  |
| inner_verifier | Audit | Alloc | LOADW | <div style='text-align: right'>1,634</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>1,539,648</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>438,154</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>455</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <div style='text-align: right'>9,240</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <div style='text-align: right'>256,476</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <div style='text-align: right'>77,946</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <div style='text-align: right'>9,240</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <div style='text-align: right'>2,452,296</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <div style='text-align: right'>2,452,296</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>2,589,994</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>1,530,451</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | <div style='text-align: right'>2,427,446</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>1,474</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>871</div>  |
| inner_verifier | Audit | DivEIN | BBE4DIV | <div style='text-align: right'>456</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <div style='text-align: right'>1,476</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>517</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>143</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>9,504</div>  |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | <div style='text-align: right'>2,666</div>  |
| inner_verifier | FieldArithmeticAir | For | ADD | <div style='text-align: right'>7,299,105</div>  |
| inner_verifier | CoreAir | For | BNE | <div style='text-align: right'>16,801,686</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>462</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>546</div>  |
| inner_verifier | CoreAir | For | JAL | <div style='text-align: right'>1,261,656</div>  |
| inner_verifier | Audit | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>66,528</div>  |
| inner_verifier | Audit | For | STOREW | <div style='text-align: right'>988</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,195,128</div>  |
| inner_verifier | CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,452</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>606,804</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <div style='text-align: right'>443,718</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <div style='text-align: right'>3,982,836</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: green">(-4,158 [-0.5%])</span> <div style='text-align: right'>817,212</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <div style='text-align: right'>425,568</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <div style='text-align: right'>1,254</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <div style='text-align: right'>67,056</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>3,300</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>1,950</div>  |
| inner_verifier | Audit | ImmE | STOREW | <div style='text-align: right'>116,280</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <div style='text-align: right'>475,200</div>  |
| inner_verifier | Audit | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>1,037,058</div>  |
| inner_verifier | Audit | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <div style='text-align: right'>881,958</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>61,182</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>36,153</div>  |
| inner_verifier | Audit | LoadE | LOADW | <div style='text-align: right'>8,816</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <div style='text-align: right'>1,014,024</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>22,704</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>13,416</div>  |
| inner_verifier | Audit | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <div style='text-align: right'>17,106,936</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>21,252</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>12,558</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,211</div>  |
| inner_verifier | Audit | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>721,974</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>583</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>351</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>374</div>  |
| inner_verifier | Audit | LoadF | LOADW2 | <div style='text-align: right'>513</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <div style='text-align: right'>6,352,236</div>  |
| inner_verifier | Audit | LoadV | LOADW | <div style='text-align: right'>13,737</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>745,074</div>  |
| inner_verifier | Audit | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>4,947,492</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: green">(-132 [-0.0%])</span> <div style='text-align: right'>419,298</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: green">(-78 [-0.0%])</span> <div style='text-align: right'>247,767</div>  |
| inner_verifier | Audit | MulE | BBE4MUL | <div style='text-align: right'>824,752</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | <div style='text-align: right'>5,484,652</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>8,030</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>4,745</div>  |
| inner_verifier | Audit | MulEF | MUL | <div style='text-align: right'>912</div>  |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | <div style='text-align: right'>50,592</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>2,068</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,222</div>  |
| inner_verifier | Audit | MulEFI | MUL | <div style='text-align: right'>7,676</div>  |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | <div style='text-align: right'>16,616</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>80,432</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>47,528</div>  |
| inner_verifier | Audit | MulEI | BBE4MUL | <div style='text-align: right'>18,088</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | <div style='text-align: right'>66,748</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>35,585</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>20,904</div>  |
| inner_verifier | Audit | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <div style='text-align: right'>429,792</div>  |
| inner_verifier | Audit | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulF | MUL | <div style='text-align: right'>1,141,079</div>  |
| inner_verifier | Audit | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | <div style='text-align: right'>434</div>  |
| inner_verifier | Audit | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | FieldArithmeticAir | MulV | MUL | <div style='text-align: right'>21,142</div>  |
| inner_verifier | Audit | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | <div style='text-align: right'>247,349</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>814</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>481</div>  |
| inner_verifier | Audit | NegE | MUL | <div style='text-align: right'>1,596</div>  |
| inner_verifier | FieldArithmeticAir | NegE | MUL | <div style='text-align: right'>4,216</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>279,048</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>164,892</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>107,814</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>2,861,628</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>231,693</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>137,878</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>91,205</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>1,819,136</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,436</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,394</div>  |
| inner_verifier | Audit | StoreE | STOREW | <div style='text-align: right'>207,708</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>721,512</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>52,668</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>31,122</div>  |
| inner_verifier | Audit | StoreE | STOREW2 | <div style='text-align: right'>26,752</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>813,648</div>  |
| inner_verifier | Audit | StoreF | STOREW | <div style='text-align: right'>251,332</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>873,048</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>144,199</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>86,177</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>57,256</div>  |
| inner_verifier | Audit | StoreF | STOREW2 | <div style='text-align: right'>56,012</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <div style='text-align: right'>2,237,730</div>  |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | <div style='text-align: right'>2,950,208</div>  |
| inner_verifier | Audit | StoreHintWord | SHINTW | <div style='text-align: right'>1,995,836</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <div style='text-align: right'>6,932,904</div>  |
| inner_verifier | Audit | StoreV | STOREW | <div style='text-align: right'>25,897</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>89,958</div>  |
| inner_verifier | Audit | StoreV | STOREW2 | <div style='text-align: right'>461,054</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>1,617,792</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>131,120</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>77,480</div>  |
| inner_verifier | Audit | SubE | FE4SUB | <div style='text-align: right'>209,000</div>  |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | <div style='text-align: right'>160,679</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>1,294,832</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <div style='text-align: right'>23,306,976</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>1,294,832</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>1,530,256</div>  |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | <div style='text-align: right'>3,649,072</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>338</div>  |
| inner_verifier | Audit | SubEFI | ADD | <div style='text-align: right'>9,576</div>  |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | <div style='text-align: right'>18,476</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>2,442</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>1,443</div>  |
| inner_verifier | Audit | SubEI | ADD | <div style='text-align: right'>912</div>  |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | <div style='text-align: right'>8,928</div>  |
| inner_verifier | Audit | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | FieldArithmeticAir | SubV | SUB | <div style='text-align: right'>667,709</div>  |
| inner_verifier | Audit | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | <div style='text-align: right'>38,471</div>  |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | <div style='text-align: right'>10,416</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | <div style='text-align: right'>4,608</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>217,088</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | FieldArithmeticAir | <div style='text-align: right'>137,216</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | ArithmeticLogicAir<32, 8> | <div style='text-align: right'>223,232</div>  | <div style='text-align: right'>187</div>  | <div style='text-align: right'>65</div>  | <div style='text-align: right'>172</div>  | <div style='text-align: right'>264</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | ShiftAir<32, 8> | <div style='text-align: right'>54,784</div>  | <div style='text-align: right'>3,193</div>  | <div style='text-align: right'>93</div>  | <div style='text-align: right'>236</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | MemoryAuditAir | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>303,104</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,192</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>167,936</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4,096</div>  |
| bench_program_inner | AccessAdapterAir<16> | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | AccessAdapterAir<32> | <div style='text-align: right'>66,560</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,024</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | ProgramAir<BabyBear> | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>180,355,072</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | FieldArithmeticAir | <div style='text-align: right'>49,283,072</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | FieldExtensionArithmeticAir | <div style='text-align: right'>29,884,416</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>7,307,264</div>  | <div style='text-align: right'>373</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>16,384</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | MemoryAuditAir | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>950,272</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a08603668fbfc3bccfcdbb13254db1065baac038/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/a08603668fbfc3bccfcdbb13254db1065baac038
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11266992923)
