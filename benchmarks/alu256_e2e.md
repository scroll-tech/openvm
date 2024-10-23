| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: red">(+5.0 [+0.3%])</span> <div style='text-align: right'>1,710.0</div>  | <span style="color: red">(+7,996 [+0.2%])</span> <div style='text-align: right'>4,197,724</div>  | <span style="color: green">(-11,717 [-2.1%])</span> <div style='text-align: right'>552,112</div>  | <span style="color: red">(+5.0 [+0.3%])</span> <div style='text-align: right'>1,753.0</div>  | <div style='text-align: right'>43.0</div>  |  |
| inner_verifier | <span style="color: red">(+1,306.0 [+3.5%])</span> <div style='text-align: right'>38,908.0</div>  | <span style="color: red">(+13,500,416 [+4.4%])</span> <div style='text-align: right'>322,043,928</div>  | <span style="color: green">(-12,460,073 [-8.3%])</span> <div style='text-align: right'>138,539,886</div>  | <span style="color: red">(+1,099.0 [+2.5%])</span> <div style='text-align: right'>44,964.0</div>  | <span style="color: green">(-207.0 [-3.3%])</span> <div style='text-align: right'>6,056.0</div>  | <span style="color: red">(+25.0 [+6.0%])</span> <div style='text-align: right'>440.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | <div style='text-align: right'>288</div>  |
| bench_program_inner | BranchEqual | <div style='text-align: right'>161</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <span style="color: green">(-163 [-12.6%])</span> <div style='text-align: right'>1,132</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>1,256</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | Memory AccessAdapter<16> | <div style='text-align: right'>584</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>4,672</div>  |
| bench_program_inner | Memory AccessAdapter<32> | <div style='text-align: right'>292</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>2,336</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>1,168</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>9,607</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>370</div>  |
| bench_program_inner | Shift256 | <div style='text-align: right'>96</div>  |
| inner_verifier | BranchEqual | <div style='text-align: right'>366,627</div>  |
| inner_verifier | Core | <span style="color: green">(-296,047 [-18.6%])</span> <div style='text-align: right'>1,293,724</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+42,409 [+5.5%])</span> <div style='text-align: right'>813,747</div>  |
| inner_verifier | FieldExtension | <span style="color: red">(+18,167 [+6.8%])</span> <div style='text-align: right'>286,249</div>  |
| inner_verifier | Jal | <div style='text-align: right'>35,358</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: red">(+41,400 [+6.4%])</span> <div style='text-align: right'>687,265</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: red">(+20,742 [+6.4%])</span> <div style='text-align: right'>343,802</div>  |
| inner_verifier | Memory AccessAdapter<8> | <span style="color: red">(+548 [+3.4%])</span> <div style='text-align: right'>16,825</div>  |
| inner_verifier | Memory Boundary | <span style="color: red">(+5,852 [+2.1%])</span> <div style='text-align: right'>280,377</div>  |
| inner_verifier | Poseidon2 | <span style="color: red">(+337 [+2.8%])</span> <div style='text-align: right'>12,187</div>  |
| inner_verifier | ProgramChip | <span style="color: red">(+3,450 [+3.8%])</span> <div style='text-align: right'>93,350</div>  |

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
| inner_verifier | AddE | FE4ADD | <span style="color: red">(+4,634 [+6.7%])</span> <div style='text-align: right'>73,620</div>  |
| inner_verifier | AddEFFI | LOADW | <span style="color: red">(+4 [+3.0%])</span> <div style='text-align: right'>136</div>  |
| inner_verifier | AddEFFI | STOREW | <span style="color: red">(+12 [+3.0%])</span> <div style='text-align: right'>408</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+32 [+18.2%])</span> <div style='text-align: right'>208</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+636 [+2.4%])</span> <div style='text-align: right'>27,052</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+1,611 [+8.1%])</span> <div style='text-align: right'>21,532</div>  |
| inner_verifier | AddV | ADD | <span style="color: red">(+200 [+3.1%])</span> <div style='text-align: right'>6,589</div>  |
| inner_verifier | AddVI | ADD | <span style="color: red">(+7,637 [+5.2%])</span> <div style='text-align: right'>153,490</div>  |
| inner_verifier | Alloc | ADD | <span style="color: red">(+568 [+2.3%])</span> <div style='text-align: right'>25,330</div>  |
| inner_verifier | Alloc | LOADW | <span style="color: red">(+568 [+2.3%])</span> <div style='text-align: right'>25,330</div>  |
| inner_verifier | Alloc | MUL | <span style="color: red">(+362 [+2.4%])</span> <div style='text-align: right'>15,360</div>  |
| inner_verifier | AssertEqE | BNE | <span style="color: red">(+8 [+5.7%])</span> <div style='text-align: right'>148</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <span style="color: red">(+22 [+1.8%])</span> <div style='text-align: right'>1,226</div>  |
| inner_verifier | AssertEqVI | BNE | <span style="color: red">(+22 [+12.9%])</span> <div style='text-align: right'>193</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: red">(+2,961 [+7.9%])</span> <div style='text-align: right'>40,453</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: red">(+2,961 [+7.9%])</span> <div style='text-align: right'>40,453</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: red">(+4,372 [+7.4%])</span> <div style='text-align: right'>63,725</div>  |
| inner_verifier | DivEIN | BBE4DIV | <span style="color: red">(+6 [+16.7%])</span> <div style='text-align: right'>42</div>  |
| inner_verifier | DivEIN | STOREW | <span style="color: red">(+24 [+16.7%])</span> <div style='text-align: right'>168</div>  |
| inner_verifier | DivFIN | DIV | <span style="color: red">(+14 [+16.3%])</span> <div style='text-align: right'>100</div>  |
| inner_verifier | For | ADD | <span style="color: red">(+14,265 [+5.9%])</span> <div style='text-align: right'>256,879</div>  |
| inner_verifier | For | BNE | <span style="color: red">(+15,388 [+5.9%])</span> <div style='text-align: right'>277,942</div>  |
| inner_verifier | For | JAL | <span style="color: red">(+1,123 [+5.6%])</span> <div style='text-align: right'>21,063</div>  |
| inner_verifier | For | LOADW | <span style="color: red">(+63 [+5.8%])</span> <div style='text-align: right'>1,155</div>  |
| inner_verifier | For | STOREW | <span style="color: red">(+1,060 [+5.6%])</span> <div style='text-align: right'>19,908</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <span style="color: red">(+206 [+2.1%])</span> <div style='text-align: right'>9,970</div>  |
| inner_verifier | IfEq | BNE | <span style="color: red">(+1,847 [+23.5%])</span> <div style='text-align: right'>9,707</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: red">(+3,966 [+6.5%])</span> <div style='text-align: right'>65,182</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+811 [+6.0%])</span> <div style='text-align: right'>14,273</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,956</div>  |
| inner_verifier | IfNe | JAL | <span style="color: red">(+1 [+5.0%])</span> <div style='text-align: right'>21</div>  |
| inner_verifier | IfNeI | BEQ | <span style="color: red">(+73 [+6.4%])</span> <div style='text-align: right'>1,215</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+180 [+2.5%])</span> <div style='text-align: right'>7,396</div>  |
| inner_verifier | ImmF | STOREW | <span style="color: red">(+1,348 [+8.0%])</span> <div style='text-align: right'>18,229</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: red">(+644 [+4.6%])</span> <div style='text-align: right'>14,568</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: red">(+848 [+5.4%])</span> <div style='text-align: right'>16,488</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: red">(+19,832 [+7.6%])</span> <div style='text-align: right'>279,952</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: red">(+7,608 [+7.9%])</span> <div style='text-align: right'>104,055</div>  |
| inner_verifier | LoadV | LOADW | <span style="color: red">(+248 [+2.1%])</span> <div style='text-align: right'>12,154</div>  |
| inner_verifier | LoadV | LOADW2 | <span style="color: red">(+7,700 [+9.9%])</span> <div style='text-align: right'>85,328</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: red">(+8,958 [+6.7%])</span> <div style='text-align: right'>143,041</div>  |
| inner_verifier | MulEF | MUL | <span style="color: red">(+48 [+2.8%])</span> <div style='text-align: right'>1,764</div>  |
| inner_verifier | MulEFI | MUL | <span style="color: red">(+16 [+3.0%])</span> <div style='text-align: right'>544</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: red">(+158 [+9.6%])</span> <div style='text-align: right'>1,798</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: red">(+632 [+9.6%])</span> <div style='text-align: right'>7,192</div>  |
| inner_verifier | MulF | MUL | <span style="color: red">(+3,044 [+8.2%])</span> <div style='text-align: right'>40,021</div>  |
| inner_verifier | MulFI | MUL | <span style="color: red">(+2 [+14.3%])</span> <div style='text-align: right'>16</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <span style="color: red">(+182 [+2.1%])</span> <div style='text-align: right'>8,686</div>  |
| inner_verifier | NegE | MUL | <span style="color: red">(+20 [+14.7%])</span> <div style='text-align: right'>156</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+63 [+0.8%])</span> <div style='text-align: right'>7,476</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+274 [+6.2%])</span> <div style='text-align: right'>4,711</div>  |
| inner_verifier | StoreE | STOREW | <span style="color: red">(+24 [+0.2%])</span> <div style='text-align: right'>11,292</div>  |
| inner_verifier | StoreE | STOREW2 | <span style="color: red">(+1,344 [+10.8%])</span> <div style='text-align: right'>13,844</div>  |
| inner_verifier | StoreF | STOREW | <span style="color: red">(+1,008 [+7.0%])</span> <div style='text-align: right'>15,404</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: red">(+2,400 [+7.0%])</span> <div style='text-align: right'>36,554</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: red">(+3,216 [+3.2%])</span> <div style='text-align: right'>102,647</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: red">(+3,422 [+3.1%])</span> <div style='text-align: right'>113,299</div>  |
| inner_verifier | StoreV | STOREW | <span style="color: red">(+30 [+2.1%])</span> <div style='text-align: right'>1,454</div>  |
| inner_verifier | StoreV | STOREW2 | <span style="color: red">(+1,330 [+5.2%])</span> <div style='text-align: right'>26,919</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: red">(+39 [+1.0%])</span> <div style='text-align: right'>4,023</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: red">(+26,220 [+7.4%])</span> <div style='text-align: right'>380,118</div>  |
| inner_verifier | SubEF | SUB | <span style="color: red">(+8,740 [+7.4%])</span> <div style='text-align: right'>126,706</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>568</div>  |
| inner_verifier | SubEI | ADD | <span style="color: red">(+48 [+16.7%])</span> <div style='text-align: right'>336</div>  |
| inner_verifier | SubV | SUB | <span style="color: red">(+1,766 [+8.1%])</span> <div style='text-align: right'>23,452</div>  |
| inner_verifier | SubVI | SUB | <span style="color: red">(+2 [+0.2%])</span> <div style='text-align: right'>1,272</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <span style="color: green">(-10 [-8.3%])</span> <div style='text-align: right'>110</div>  |
| bench_program_inner | AccessAdapter<16> | Add256 | ADD<32,8> | <div style='text-align: right'>3,300</div>  |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD<32,8> | <div style='text-align: right'>11,616</div>  |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD<32,8> | <div style='text-align: right'>2,706</div>  |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD<32,8> | <div style='text-align: right'>6,864</div>  |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD<32,8> | <div style='text-align: right'>4,488</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | Add256 | ADD<32,8> | <div style='text-align: right'>11,008</div>  |
| bench_program_inner | Boundary | Add256 | ADD<32,8> | <div style='text-align: right'>38,912</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>13,440</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <div style='text-align: right'>285</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <span style="color: green">(-1,940 [-8.3%])</span> <div style='text-align: right'>21,340</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>11,640</div>  |
| bench_program_inner | AccessAdapter<16> | And256 | AND<32,8> | <div style='text-align: right'>1,600</div>  |
| bench_program_inner | AccessAdapter<2> | And256 | AND<32,8> | <div style='text-align: right'>5,632</div>  |
| bench_program_inner | AccessAdapter<32> | And256 | AND<32,8> | <div style='text-align: right'>1,312</div>  |
| bench_program_inner | AccessAdapter<4> | And256 | AND<32,8> | <div style='text-align: right'>3,328</div>  |
| bench_program_inner | AccessAdapter<8> | And256 | AND<32,8> | <div style='text-align: right'>2,176</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | And256 | AND<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | And256 | AND<32,8> | <div style='text-align: right'>19,456</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | EqualTo256 | EQ<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | EqualTo256 | EQ<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>960</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>759</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <span style="color: green">(-5 [-8.3%])</span> <div style='text-align: right'>55</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>2,944</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <div style='text-align: right'>2,717</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <span style="color: green">(-2,585 [-8.3%])</span> <div style='text-align: right'>28,435</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanI256 | SLT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanI256 | SLT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | LessThanU256 | LT<32,8> | <div style='text-align: right'>5,504</div>  |
| bench_program_inner | Boundary | LessThanU256 | LT<32,8> | <div style='text-align: right'>608</div>  |
| bench_program_inner | Boundary | LoadV | LOADW | <div style='text-align: right'>57</div>  |
| bench_program_inner | CoreAir | LoadV | LOADW | <span style="color: green">(-480 [-8.3%])</span> <div style='text-align: right'>5,280</div>  |
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
| bench_program_inner | CoreAir | StoreV | STOREW | <span style="color: green">(-640 [-8.3%])</span> <div style='text-align: right'>7,040</div>  |
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
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | Boundary |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <span style="color: green">(-10 [-8.3%])</span> <div style='text-align: right'>110</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <span style="color: red">(+185,360 [+6.7%])</span> <div style='text-align: right'>2,944,800</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+7,106 [+3.4%])</span> <div style='text-align: right'>218,064</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+4,199 [+3.4%])</span> <div style='text-align: right'>128,856</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <span style="color: green">(-228 [-0.1%])</span> <div style='text-align: right'>412,680</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: red">(+33 [+3.6%])</span> <div style='text-align: right'>957</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: red">(+39 [+3.6%])</span> <div style='text-align: right'>1,131</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>456</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <span style="color: green">(-440 [-5.6%])</span> <div style='text-align: right'>7,480</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: red">(+33 [+3.6%])</span> <div style='text-align: right'>957</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>1,368</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <span style="color: green">(-1,320 [-5.6%])</span> <div style='text-align: right'>22,440</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <span style="color: red">(+960 [+18.2%])</span> <div style='text-align: right'>6,240</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: red">(+198 [+34.6%])</span> <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: red">(+117 [+34.6%])</span> <div style='text-align: right'>455</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>2,356</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <span style="color: red">(+19,080 [+2.4%])</span> <div style='text-align: right'>811,560</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+6,204 [+4.0%])</span> <div style='text-align: right'>160,578</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+3,666 [+4.0%])</span> <div style='text-align: right'>94,887</div>  |
| inner_verifier | Boundary | AddEI | ADD | <span style="color: green">(-228 [-0.1%])</span> <div style='text-align: right'>350,740</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+48,330 [+8.1%])</span> <div style='text-align: right'>645,960</div>  |
| inner_verifier | Boundary | AddFI | ADD | <span style="color: green">(-19 [-4.2%])</span> <div style='text-align: right'>437</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <span style="color: red">(+6,000 [+3.1%])</span> <div style='text-align: right'>197,670</div>  |
| inner_verifier | Boundary | AddV | ADD | <div style='text-align: right'>38</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <span style="color: red">(+229,110 [+5.2%])</span> <div style='text-align: right'>4,604,700</div>  |
| inner_verifier | Boundary | AddVI | ADD | <span style="color: red">(+38 [+0.3%])</span> <div style='text-align: right'>14,953</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <span style="color: red">(+17,040 [+2.3%])</span> <div style='text-align: right'>759,900</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,653</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <span style="color: green">(-92,570 [-6.2%])</span> <div style='text-align: right'>1,393,150</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <span style="color: red">(+10,860 [+2.4%])</span> <div style='text-align: right'>460,800</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>3,404</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <span style="color: red">(+44 [+5.7%])</span> <div style='text-align: right'>814</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <span style="color: red">(+26 [+5.7%])</span> <div style='text-align: right'>481</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>28,198</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>4,439</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-24,605 [-1.1%])</span> <div style='text-align: right'>2,224,915</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-24,605 [-1.1%])</span> <div style='text-align: right'>2,224,915</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <span style="color: red">(+174,880 [+7.4%])</span> <div style='text-align: right'>2,549,000</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: red">(+192,324 [+7.4%])</span> <div style='text-align: right'>2,787,862</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: red">(+113,646 [+7.4%])</span> <div style='text-align: right'>1,647,373</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <span style="color: red">(+240 [+16.7%])</span> <div style='text-align: right'>1,680</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+264 [+17.4%])</span> <div style='text-align: right'>1,782</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+156 [+17.4%])</span> <div style='text-align: right'>1,053</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>608</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <span style="color: red">(+88 [+17.0%])</span> <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <span style="color: red">(+26 [+18.2%])</span> <div style='text-align: right'>169</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <span style="color: red">(+600 [+6.9%])</span> <div style='text-align: right'>9,240</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <span style="color: red">(+420 [+16.3%])</span> <div style='text-align: right'>3,000</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <span style="color: red">(+427,950 [+5.9%])</span> <div style='text-align: right'>7,706,370</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>6,392,666</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>210,630</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <span style="color: red">(+44 [+9.3%])</span> <div style='text-align: right'>517</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <span style="color: red">(+52 [+9.3%])</span> <div style='text-align: right'>611</div>  |
| inner_verifier | Boundary | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <span style="color: green">(-1,995 [-3.0%])</span> <div style='text-align: right'>63,525</div>  |
| inner_verifier | Boundary | For | STOREW | <span style="color: red">(+76 [+7.8%])</span> <div style='text-align: right'>1,045</div>  |
| inner_verifier | CoreAir | For | STOREW | <span style="color: green">(-35,940 [-3.2%])</span> <div style='text-align: right'>1,094,940</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <span style="color: green">(-110 [-8.3%])</span> <div style='text-align: right'>1,210</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <span style="color: green">(-37,490 [-6.4%])</span> <div style='text-align: right'>548,350</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>223,261</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>1,499,186</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <div style='text-align: right'>142,730</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>159,988</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>210</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>27,945</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: red">(+748 [+22.7%])</span> <div style='text-align: right'>4,048</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: red">(+442 [+22.7%])</span> <div style='text-align: right'>2,392</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <span style="color: green">(-152 [-0.1%])</span> <div style='text-align: right'>116,356</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <span style="color: green">(-26,180 [-6.0%])</span> <div style='text-align: right'>406,780</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <span style="color: green">(-10,265 [-1.0%])</span> <div style='text-align: right'>1,002,595</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,067</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <span style="color: green">(-34,200 [-4.1%])</span> <div style='text-align: right'>801,240</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: red">(+3,718 [+5.9%])</span> <div style='text-align: right'>66,814</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: red">(+2,197 [+5.9%])</span> <div style='text-align: right'>39,481</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>5,244</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <span style="color: green">(-31,560 [-3.4%])</span> <div style='text-align: right'>906,840</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <span style="color: green">(-209,840 [-1.3%])</span> <div style='text-align: right'>15,397,360</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <span style="color: green">(-57,365 [-8.3%])</span> <div style='text-align: right'>631,015</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <span style="color: red">(+19 [+3.7%])</span> <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <span style="color: green">(-63,795 [-1.1%])</span> <div style='text-align: right'>5,723,025</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>13,737</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <span style="color: green">(-45,890 [-6.4%])</span> <div style='text-align: right'>668,470</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <span style="color: red">(+35,360 [+0.8%])</span> <div style='text-align: right'>4,693,040</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <span style="color: red">(+358,320 [+6.7%])</span> <div style='text-align: right'>5,721,640</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+6,402 [+1.5%])</span> <div style='text-align: right'>427,086</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+3,783 [+1.5%])</span> <div style='text-align: right'>252,369</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <span style="color: green">(-4,104 [-0.5%])</span> <div style='text-align: right'>820,952</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <span style="color: red">(+1,440 [+2.8%])</span> <div style='text-align: right'>52,920</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: red">(+198 [+2.4%])</span> <div style='text-align: right'>8,536</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: red">(+117 [+2.4%])</span> <div style='text-align: right'>5,044</div>  |
| inner_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>1,216</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <span style="color: red">(+480 [+3.0%])</span> <div style='text-align: right'>16,320</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: red">(+66 [+3.2%])</span> <div style='text-align: right'>2,112</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: red">(+39 [+3.2%])</span> <div style='text-align: right'>1,248</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <span style="color: green">(-228 [-3.0%])</span> <div style='text-align: right'>7,296</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <span style="color: red">(+6,320 [+9.6%])</span> <div style='text-align: right'>71,920</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+8,558 [+10.5%])</span> <div style='text-align: right'>90,024</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+5,057 [+10.5%])</span> <div style='text-align: right'>53,196</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <span style="color: red">(+5,700 [+28.8%])</span> <div style='text-align: right'>25,460</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: red">(+3,476 [+9.7%])</span> <div style='text-align: right'>39,314</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: red">(+2,054 [+9.8%])</span> <div style='text-align: right'>23,101</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <span style="color: red">(+1,960 [+0.5%])</span> <div style='text-align: right'>395,560</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <span style="color: red">(+91,320 [+8.2%])</span> <div style='text-align: right'>1,200,630</div>  |
| inner_verifier | Boundary | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <span style="color: red">(+60 [+14.3%])</span> <div style='text-align: right'>480</div>  |
| inner_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <span style="color: red">(+5,460 [+2.1%])</span> <div style='text-align: right'>260,580</div>  |
| inner_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <span style="color: red">(+600 [+14.7%])</span> <div style='text-align: right'>4,680</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: red">(+176 [+22.2%])</span> <div style='text-align: right'>968</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: red">(+104 [+22.2%])</span> <div style='text-align: right'>572</div>  |
| inner_verifier | Boundary | NegE | MUL | <div style='text-align: right'>1,596</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>301,224</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>177,996</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>116,382</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+26,334 [+0.8%])</span> <div style='text-align: right'>3,124,968</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+15,290 [+6.4%])</span> <div style='text-align: right'>253,759</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+9,308 [+6.6%])</span> <div style='text-align: right'>151,047</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+5,729 [+6.1%])</span> <div style='text-align: right'>99,824</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+114,532 [+6.2%])</span> <div style='text-align: right'>1,969,198</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <span style="color: red">(+456 [+0.2%])</span> <div style='text-align: right'>214,548</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <span style="color: green">(-55,020 [-8.1%])</span> <div style='text-align: right'>621,060</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <span style="color: red">(+7,392 [+14.0%])</span> <div style='text-align: right'>60,060</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <span style="color: red">(+4,368 [+14.0%])</span> <div style='text-align: right'>35,490</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>28,424</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <span style="color: red">(+11,420 [+1.5%])</span> <div style='text-align: right'>761,420</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <span style="color: red">(+19,152 [+7.0%])</span> <div style='text-align: right'>292,676</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <span style="color: green">(-16,540 [-1.9%])</span> <div style='text-align: right'>847,220</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: red">(+9,746 [+6.8%])</span> <div style='text-align: right'>153,307</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: red">(+6,032 [+7.0%])</span> <div style='text-align: right'>91,689</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: red">(+3,587 [+6.3%])</span> <div style='text-align: right'>60,860</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <span style="color: red">(+608 [+1.0%])</span> <div style='text-align: right'>59,812</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <span style="color: green">(-38,770 [-1.9%])</span> <div style='text-align: right'>2,010,470</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <span style="color: red">(+96,480 [+3.2%])</span> <div style='text-align: right'>3,079,410</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: red">(+65,018 [+3.1%])</span> <div style='text-align: right'>2,152,681</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <span style="color: green">(-361,175 [-5.5%])</span> <div style='text-align: right'>6,231,445</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <span style="color: red">(+570 [+2.1%])</span> <div style='text-align: right'>27,626</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <span style="color: green">(-5,470 [-6.4%])</span> <div style='text-align: right'>79,970</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <span style="color: red">(+25,270 [+5.2%])</span> <div style='text-align: right'>507,186</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <span style="color: green">(-54,795 [-3.6%])</span> <div style='text-align: right'>1,480,545</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <span style="color: red">(+1,560 [+1.0%])</span> <div style='text-align: right'>160,920</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: red">(+528 [+0.4%])</span> <div style='text-align: right'>134,090</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: red">(+312 [+0.4%])</span> <div style='text-align: right'>79,235</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <span style="color: green">(-532 [-0.3%])</span> <div style='text-align: right'>208,772</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: red">(+96,140 [+7.4%])</span> <div style='text-align: right'>1,393,766</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <span style="color: green">(-327,390 [-1.5%])</span> <div style='text-align: right'>20,906,490</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <span style="color: red">(+262,200 [+7.4%])</span> <div style='text-align: right'>3,801,180</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: red">(+96,140 [+7.4%])</span> <div style='text-align: right'>1,393,766</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: red">(+113,620 [+7.4%])</span> <div style='text-align: right'>1,647,178</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>17,040</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <span style="color: red">(+44 [+9.5%])</span> <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <span style="color: red">(+26 [+9.5%])</span> <div style='text-align: right'>299</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <span style="color: green">(-228 [-2.5%])</span> <div style='text-align: right'>8,892</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <span style="color: red">(+1,440 [+16.7%])</span> <div style='text-align: right'>10,080</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: red">(+440 [+18.5%])</span> <div style='text-align: right'>2,816</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: red">(+260 [+18.5%])</span> <div style='text-align: right'>1,664</div>  |
| inner_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>1,216</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <span style="color: red">(+52,980 [+8.1%])</span> <div style='text-align: right'>703,560</div>  |
| inner_verifier | Boundary | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <span style="color: red">(+60 [+0.2%])</span> <div style='text-align: right'>38,160</div>  |
| inner_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>9,216</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | CoreAir | <span style="color: green">(-10,240 [-4.8%])</span> <div style='text-align: right'>202,752</div>  | <span style="color: green">(-21 [-20.2%])</span> <div style='text-align: right'>83</div>  | <div style='text-align: right'>19</div>  | <span style="color: green">(-5 [-8.3%])</span> <div style='text-align: right'>55</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>18,176</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>256</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>135,168</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | ArithmeticLogicCoreAir<32, 8> | <div style='text-align: right'>223,232</div>  | <div style='text-align: right'>187</div>  | <div style='text-align: right'>65</div>  | <div style='text-align: right'>172</div>  | <div style='text-align: right'>264</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>512</div>  |
| bench_program_inner | ShiftCoreAir<32, 8> | <div style='text-align: right'>54,784</div>  | <div style='text-align: right'>3,193</div>  | <div style='text-align: right'>93</div>  | <div style='text-align: right'>236</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VolatileBoundaryAir | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>573,440</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16,384</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>303,104</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8,192</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>167,936</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>4,096</div>  |
| bench_program_inner | AccessAdapterAir<16> | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2,048</div>  |
| bench_program_inner | AccessAdapterAir<32> | <div style='text-align: right'>66,560</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1,024</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | CoreAir | <span style="color: green">(-10,485,760 [-6.2%])</span> <div style='text-align: right'>157,286,400</div>  | <span style="color: green">(-21 [-21.4%])</span> <div style='text-align: right'>77</div>  | <div style='text-align: right'>19</div>  | <span style="color: green">(-5 [-8.3%])</span> <div style='text-align: right'>55</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>22,544,384</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>1,441,792</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>48,234,496</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>29,360,128</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>7,307,264</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>16,384</div>  |
| inner_verifier | VolatileBoundaryAir | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>24,117,248</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>13,107,200</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>524,288</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>950,272</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/c3a09de5cf8ef18a6b39fad0346d0a01ebca94f5
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11483951639)
