| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: red">(+16,525.0 [+2337.3%])</span> <div style='text-align: right'>17,232.0</div>  | <span style="color: red">(+18,874,368 [+1423.5%])</span> <div style='text-align: right'>20,200,257</div>  | <span style="color: red">(+2,097,152 [+990.1%])</span> <div style='text-align: right'>2,308,968</div>  | <span style="color: red">(+16,591.0 [+2340.1%])</span> <div style='text-align: right'>17,300.0</div>  | <span style="color: red">(+66.0 [+3300.0%])</span> <div style='text-align: right'>68.0</div>  |  |
| inner_verifier | <span style="color: red">(+2,722.0 [+3.2%])</span> <div style='text-align: right'>87,166.0</div>  | <span style="color: red">(+18,874,368 [+2.6%])</span> <div style='text-align: right'>735,182,868</div>  | <span style="color: red">(+19,761,493 [+5.1%])</span> <div style='text-align: right'>404,768,966</div>  | <span style="color: red">(+7,072.0 [+6.0%])</span> <div style='text-align: right'>124,124.0</div>  | <span style="color: red">(+4,350.0 [+13.3%])</span> <div style='text-align: right'>36,958.0</div>  | <span style="color: green">(-260.0 [-0.6%])</span> <div style='text-align: right'>45,757.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>28</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory | <div style='text-align: right'>65</div>  |
| bench_program_inner | Memory 2 | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory 3 | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory 4 | <div style='text-align: right'>5</div>  |
| bench_program_inner | Program | <div style='text-align: right'>37</div>  |
| bench_program_inner | RangeChecker | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | ByteXor | <div style='text-align: right'>65,536</div>  |
| inner_verifier | Core | <span style="color: red">(+165,932 [+4.4%])</span> <div style='text-align: right'>3,933,592</div>  |
| inner_verifier | FieldArithmetic | <span style="color: red">(+124,085 [+8.1%])</span> <div style='text-align: right'>1,662,243</div>  |
| inner_verifier | FieldExtension | <span style="color: red">(+3,371 [+0.4%])</span> <div style='text-align: right'>846,468</div>  |
| inner_verifier | Memory | <span style="color: red">(+42,965 [+7.3%])</span> <div style='text-align: right'>627,664</div>  |
| inner_verifier | Memory 2 | <span style="color: red">(+24,232 [+1.2%])</span> <div style='text-align: right'>1,964,313</div>  |
| inner_verifier | Memory 3 | <span style="color: red">(+12,116 [+1.2%])</span> <div style='text-align: right'>982,222</div>  |
| inner_verifier | Memory 4 | <span style="color: red">(+3,678 [+11.3%])</span> <div style='text-align: right'>36,161</div>  |
| inner_verifier | Poseidon2 | <span style="color: red">(+3,408 [+17.5%])</span> <div style='text-align: right'>22,901</div>  |
| inner_verifier | Program | <span style="color: red">(+518 [+0.3%])</span> <div style='text-align: right'>199,649</div>  |
| inner_verifier | RangeChecker | <div style='text-align: right'>65,536</div>  |
| inner_verifier | RangeTupleChecker | <div style='text-align: right'>2,097,152</div>  |

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
| bench_program_inner | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmE | STOREW | <div style='text-align: right'>8</div>  |
| bench_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>3</div>  |
| bench_program_inner | Keccak256 | KECCAK256 | <div style='text-align: right'>1</div>  |
| bench_program_inner | StoreV | STOREW2 | <div style='text-align: right'>2</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <span style="color: red">(+796 [+0.4%])</span> <div style='text-align: right'>224,538</div>  |
| inner_verifier | AddEFFI | LOADW | <span style="color: red">(+20 [+16.3%])</span> <div style='text-align: right'>143</div>  |
| inner_verifier | AddEFFI | STOREW | <span style="color: red">(+60 [+16.3%])</span> <div style='text-align: right'>429</div>  |
| inner_verifier | AddEFI | ADD | <div style='text-align: right'>168</div>  |
| inner_verifier | AddEI | ADD | <span style="color: red">(+1,344 [+2.0%])</span> <div style='text-align: right'>67,876</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+2,294 [+18.6%])</span> <div style='text-align: right'>14,621</div>  |
| inner_verifier | AddV | ADD | <span style="color: red">(+2,567 [+45.8%])</span> <div style='text-align: right'>8,173</div>  |
| inner_verifier | AddVI | ADD | <span style="color: red">(+26,161 [+9.8%])</span> <div style='text-align: right'>293,935</div>  |
| inner_verifier | Alloc | ADD | <span style="color: red">(+8,343 [+37.1%])</span> <div style='text-align: right'>30,851</div>  |
| inner_verifier | Alloc | LOADW | <span style="color: red">(+8,343 [+37.1%])</span> <div style='text-align: right'>30,851</div>  |
| inner_verifier | Alloc | MUL | <span style="color: red">(+4,726 [+34.8%])</span> <div style='text-align: right'>18,309</div>  |
| inner_verifier | AssertEqE | BNE | <span style="color: red">(+4 [+3.0%])</span> <div style='text-align: right'>136</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <span style="color: red">(+1,008 [+25.9%])</span> <div style='text-align: right'>4,894</div>  |
| inner_verifier | AssertEqV | BNE | <span style="color: red">(+117 [+10.1%])</span> <div style='text-align: right'>1,276</div>  |
| inner_verifier | AssertEqVI | BNE | <span style="color: red">(+8 [+6.6%])</span> <div style='text-align: right'>130</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: red">(+1,260 [+1.2%])</span> <div style='text-align: right'>105,343</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: red">(+1,260 [+1.2%])</span> <div style='text-align: right'>105,343</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: red">(+653 [+0.3%])</span> <div style='text-align: right'>195,641</div>  |
| inner_verifier | DivEIN | BBE4DIV | <span style="color: red">(+1 [+3.3%])</span> <div style='text-align: right'>31</div>  |
| inner_verifier | DivEIN | STOREW | <span style="color: red">(+4 [+3.3%])</span> <div style='text-align: right'>124</div>  |
| inner_verifier | DivFIN | DIV | <span style="color: red">(+3 [+4.2%])</span> <div style='text-align: right'>75</div>  |
| inner_verifier | For | ADD | <span style="color: red">(+40,253 [+7.7%])</span> <div style='text-align: right'>561,711</div>  |
| inner_verifier | For | BNE | <span style="color: red">(+45,191 [+8.4%])</span> <div style='text-align: right'>584,454</div>  |
| inner_verifier | For | JAL | <span style="color: red">(+4,938 [+27.7%])</span> <div style='text-align: right'>22,743</div>  |
| inner_verifier | For | LOADW | <span style="color: red">(+189 [+21.4%])</span> <div style='text-align: right'>1,071</div>  |
| inner_verifier | For | STOREW | <span style="color: red">(+4,749 [+28.1%])</span> <div style='text-align: right'>21,672</div>  |
| inner_verifier | Halt | TERMINATE | <div style='text-align: right'>1</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <span style="color: red">(+3,617 [+40.5%])</span> <div style='text-align: right'>12,542</div>  |
| inner_verifier | IfEq | BNE | <span style="color: red">(+1,464 [+28.2%])</span> <div style='text-align: right'>6,653</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: red">(+6,777 [+5.6%])</span> <div style='text-align: right'>127,291</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+2,678 [+30.4%])</span> <div style='text-align: right'>11,489</div>  |
| inner_verifier | IfNe | BEQ | <span style="color: red">(+3,170 [+49.6%])</span> <div style='text-align: right'>9,555</div>  |
| inner_verifier | IfNe | JAL | <span style="color: red">(+5 [+25.0%])</span> <div style='text-align: right'>25</div>  |
| inner_verifier | IfNeI | BEQ | <span style="color: red">(+191 [+21.6%])</span> <div style='text-align: right'>1,077</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: red">(+16 [+0.1%])</span> <div style='text-align: right'>12,384</div>  |
| inner_verifier | ImmF | STOREW | <span style="color: red">(+2,362 [+17.7%])</span> <div style='text-align: right'>15,719</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: red">(+1,853 [+8.8%])</span> <div style='text-align: right'>23,016</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: red">(+1,372 [+3.3%])</span> <div style='text-align: right'>42,584</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: red">(+5,548 [+0.7%])</span> <div style='text-align: right'>805,528</div>  |
| inner_verifier | LoadF | LOADW | <span style="color: red">(+3,009 [+27.5%])</span> <div style='text-align: right'>13,948</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: red">(+1,415 [+0.5%])</span> <div style='text-align: right'>300,284</div>  |
| inner_verifier | LoadV | LOADW | <span style="color: red">(+3,893 [+35.5%])</span> <div style='text-align: right'>14,871</div>  |
| inner_verifier | LoadV | LOADW2 | <span style="color: red">(+13,597 [+22.1%])</span> <div style='text-align: right'>75,198</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: red">(+1,560 [+0.4%])</span> <div style='text-align: right'>409,566</div>  |
| inner_verifier | MulEF | MUL | <span style="color: red">(+428 [+27.0%])</span> <div style='text-align: right'>2,012</div>  |
| inner_verifier | MulEFI | MUL | <div style='text-align: right'>1,432</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: red">(+41 [+1.6%])</span> <div style='text-align: right'>2,599</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: red">(+164 [+1.6%])</span> <div style='text-align: right'>10,396</div>  |
| inner_verifier | MulF | MUL | <span style="color: red">(+4,288 [+19.5%])</span> <div style='text-align: right'>26,293</div>  |
| inner_verifier | MulFI | MUL | <span style="color: red">(+1 [+8.3%])</span> <div style='text-align: right'>13</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <span style="color: red">(+3,347 [+43.3%])</span> <div style='text-align: right'>11,081</div>  |
| inner_verifier | NegE | MUL | <span style="color: red">(+4 [+2.2%])</span> <div style='text-align: right'>188</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+3,213 [+48.3%])</span> <div style='text-align: right'>9,870</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+195 [+1.5%])</span> <div style='text-align: right'>13,031</div>  |
| inner_verifier | StoreE | STOREW | <span style="color: red">(+1,692 [+15.5%])</span> <div style='text-align: right'>12,600</div>  |
| inner_verifier | StoreE | STOREW2 | <span style="color: red">(+1,532 [+13.9%])</span> <div style='text-align: right'>12,516</div>  |
| inner_verifier | StoreF | STOREW | <span style="color: red">(+2,194 [+19.6%])</span> <div style='text-align: right'>13,406</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: red">(+1,271 [+1.3%])</span> <div style='text-align: right'>102,853</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: red">(+26,348 [+14.0%])</span> <div style='text-align: right'>214,569</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: red">(+29,965 [+15.1%])</span> <div style='text-align: right'>227,793</div>  |
| inner_verifier | StoreV | STOREW | <span style="color: red">(+341 [+25.6%])</span> <div style='text-align: right'>1,674</div>  |
| inner_verifier | StoreV | STOREW2 | <span style="color: red">(+7,373 [+32.0%])</span> <div style='text-align: right'>30,429</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: red">(+320 [+2.3%])</span> <div style='text-align: right'>14,093</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: red">(+3,282 [+0.3%])</span> <div style='text-align: right'>1,171,122</div>  |
| inner_verifier | SubEF | SUB | <span style="color: red">(+1,094 [+0.3%])</span> <div style='text-align: right'>390,374</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,288</div>  |
| inner_verifier | SubEI | ADD | <span style="color: red">(+8 [+3.3%])</span> <div style='text-align: right'>248</div>  |
| inner_verifier | SubV | SUB | <span style="color: red">(+2,626 [+18.9%])</span> <div style='text-align: right'>16,519</div>  |
| inner_verifier | SubVI | SUB | <span style="color: red">(+145 [+11.7%])</span> <div style='text-align: right'>1,384</div>  |
| inner_verifier | SubVIN | SUB | <span style="color: red">(+105 [+31.2%])</span> <div style='text-align: right'>441</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir |  | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Audit | AddE | FE4ADD | <div style='text-align: right'>76</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | <div style='text-align: right'>41</div>  |
| bench_program_inner | Audit | AddF | ADD | <div style='text-align: right'>19</div>  |
| bench_program_inner | FieldArithmeticAir | AddF | ADD | <div style='text-align: right'>31</div>  |
| bench_program_inner | Audit | AddVI | ADD | <div style='text-align: right'>38</div>  |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | <div style='text-align: right'>186</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit | Alloc | LOADW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>124</div>  |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | <div style='text-align: right'>62</div>  |
| bench_program_inner | FieldArithmeticAir | For | ADD | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | For | BNE | <div style='text-align: right'>186</div>  |
| bench_program_inner | CoreAir | For | JAL | <div style='text-align: right'>62</div>  |
| bench_program_inner | Audit | For | STOREW | <div style='text-align: right'>19</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | Halt | TERMINATE | <div style='text-align: right'>62</div>  |
| bench_program_inner | CoreAir | IfEqI | BNE | <div style='text-align: right'>124</div>  |
| bench_program_inner | Audit | ImmE | STOREW | <div style='text-align: right'>152</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>496</div>  |
| bench_program_inner | Audit | ImmF | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>124</div>  |
| bench_program_inner | Audit | ImmV | STOREW | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>186</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Audit | Keccak256 | KECCAK256 | <div style='text-align: right'>722</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Audit | StoreV | STOREW2 | <div style='text-align: right'>38</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>124</div>  |
| inner_verifier | Audit |  | JAL | <div style='text-align: right'>19</div>  |
| inner_verifier | CoreAir |  | JAL | <div style='text-align: right'>66</div>  |
| inner_verifier | Audit |  | STOREW | <div style='text-align: right'>38</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>132</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: red">(+6,952 [+0.6%])</span> <div style='text-align: right'>1,112,012</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: red">(+4,108 [+0.6%])</span> <div style='text-align: right'>657,098</div>  |
| inner_verifier | Audit | AddE | FE4ADD | <div style='text-align: right'>2,077,764</div>  |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | <span style="color: red">(+32,636 [+0.4%])</span> <div style='text-align: right'>9,206,058</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: red">(+220 [+25.3%])</span> <div style='text-align: right'>1,089</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: red">(+260 [+25.3%])</span> <div style='text-align: right'>1,287</div>  |
| inner_verifier | Audit | AddEFFI | LOADW | <div style='text-align: right'>380</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <span style="color: red">(+1,320 [+16.3%])</span> <div style='text-align: right'>9,438</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: red">(+220 [+25.3%])</span> <div style='text-align: right'>1,089</div>  |
| inner_verifier | Audit | AddEFFI | STOREW | <div style='text-align: right'>1,140</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <span style="color: red">(+3,960 [+16.3%])</span> <div style='text-align: right'>28,314</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>572</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>338</div>  |
| inner_verifier | Audit | AddEFI | ADD | <div style='text-align: right'>2,052</div>  |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | <div style='text-align: right'>5,208</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: red">(+6,490 [+1.8%])</span> <div style='text-align: right'>377,036</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: red">(+3,835 [+1.8%])</span> <div style='text-align: right'>222,794</div>  |
| inner_verifier | Audit | AddEI | ADD | <div style='text-align: right'>1,132,096</div>  |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | <span style="color: red">(+41,664 [+2.0%])</span> <div style='text-align: right'>2,104,156</div>  |
| inner_verifier | Audit | AddFI | ADD | <div style='text-align: right'>437</div>  |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | <span style="color: red">(+71,114 [+18.6%])</span> <div style='text-align: right'>453,251</div>  |
| inner_verifier | Audit | AddV | ADD | <div style='text-align: right'>57</div>  |
| inner_verifier | FieldArithmeticAir | AddV | ADD | <span style="color: red">(+79,577 [+45.8%])</span> <div style='text-align: right'>253,363</div>  |
| inner_verifier | Audit | AddVI | ADD | <span style="color: red">(+19 [+0.1%])</span> <div style='text-align: right'>15,010</div>  |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | <span style="color: red">(+810,991 [+9.8%])</span> <div style='text-align: right'>9,111,985</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | <span style="color: red">(+258,633 [+37.1%])</span> <div style='text-align: right'>956,381</div>  |
| inner_verifier | Audit | Alloc | LOADW | <div style='text-align: right'>1,634</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <span style="color: red">(+550,638 [+37.1%])</span> <div style='text-align: right'>2,036,166</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | <span style="color: red">(+146,506 [+34.8%])</span> <div style='text-align: right'>567,579</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <span style="color: red">(+22 [+3.0%])</span> <div style='text-align: right'>748</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <span style="color: red">(+13 [+3.0%])</span> <div style='text-align: right'>442</div>  |
| inner_verifier | CoreAir | AssertEqE | BNE | <span style="color: red">(+264 [+3.0%])</span> <div style='text-align: right'>8,976</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | CoreAir | AssertEqEI | BNE | <div style='text-align: right'>264</div>  |
| inner_verifier | CoreAir | AssertEqF | BNE | <span style="color: red">(+66,528 [+25.9%])</span> <div style='text-align: right'>323,004</div>  |
| inner_verifier | CoreAir | AssertEqV | BNE | <span style="color: red">(+7,722 [+10.1%])</span> <div style='text-align: right'>84,216</div>  |
| inner_verifier | CoreAir | AssertEqVI | BNE | <span style="color: red">(+528 [+6.6%])</span> <div style='text-align: right'>8,580</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: red">(+83,160 [+1.2%])</span> <div style='text-align: right'>6,952,638</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: red">(+83,160 [+1.2%])</span> <div style='text-align: right'>6,952,638</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: red">(+24,090 [+0.3%])</span> <div style='text-align: right'>8,588,536</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: red">(+14,235 [+0.3%])</span> <div style='text-align: right'>5,075,044</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | <span style="color: red">(+26,773 [+0.3%])</span> <div style='text-align: right'>8,021,281</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+22 [+1.8%])</span> <div style='text-align: right'>1,232</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+13 [+1.8%])</span> <div style='text-align: right'>728</div>  |
| inner_verifier | Audit | DivEIN | BBE4DIV | <div style='text-align: right'>304</div>  |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | <span style="color: red">(+41 [+3.3%])</span> <div style='text-align: right'>1,271</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <span style="color: red">(+11 [+2.6%])</span> <div style='text-align: right'>440</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>117</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <span style="color: red">(+264 [+3.3%])</span> <div style='text-align: right'>8,184</div>  |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | <span style="color: red">(+93 [+4.2%])</span> <div style='text-align: right'>2,325</div>  |
| inner_verifier | FieldArithmeticAir | For | ADD | <span style="color: red">(+1,247,843 [+7.7%])</span> <div style='text-align: right'>17,413,041</div>  |
| inner_verifier | CoreAir | For | BNE | <span style="color: red">(+2,982,606 [+8.4%])</span> <div style='text-align: right'>38,573,964</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <span style="color: red">(+66 [+15.8%])</span> <div style='text-align: right'>484</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <span style="color: red">(+78 [+15.8%])</span> <div style='text-align: right'>572</div>  |
| inner_verifier | CoreAir | For | JAL | <span style="color: red">(+325,908 [+27.7%])</span> <div style='text-align: right'>1,501,038</div>  |
| inner_verifier | Audit | For | LOADW | <div style='text-align: right'>399</div>  |
| inner_verifier | CoreAir | For | LOADW | <span style="color: red">(+12,474 [+21.4%])</span> <div style='text-align: right'>70,686</div>  |
| inner_verifier | Audit | For | STOREW | <div style='text-align: right'>969</div>  |
| inner_verifier | CoreAir | For | STOREW | <span style="color: red">(+313,434 [+28.1%])</span> <div style='text-align: right'>1,430,352</div>  |
| inner_verifier | CoreAir | Halt | TERMINATE | <div style='text-align: right'>66</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,452</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <span style="color: red">(+238,722 [+40.5%])</span> <div style='text-align: right'>827,772</div>  |
| inner_verifier | CoreAir | IfEq | BNE | <span style="color: red">(+96,624 [+28.2%])</span> <div style='text-align: right'>439,098</div>  |
| inner_verifier | CoreAir | IfEqI | BNE | <span style="color: red">(+447,282 [+5.6%])</span> <div style='text-align: right'>8,401,206</div>  |
| inner_verifier | CoreAir | IfEqI | JAL | <span style="color: red">(+176,748 [+30.4%])</span> <div style='text-align: right'>758,274</div>  |
| inner_verifier | CoreAir | IfNe | BEQ | <span style="color: red">(+209,220 [+49.6%])</span> <div style='text-align: right'>630,630</div>  |
| inner_verifier | CoreAir | IfNe | JAL | <span style="color: red">(+330 [+25.0%])</span> <div style='text-align: right'>1,650</div>  |
| inner_verifier | CoreAir | IfNeI | BEQ | <span style="color: red">(+12,606 [+21.6%])</span> <div style='text-align: right'>71,082</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: red">(+66 [+2.0%])</span> <div style='text-align: right'>3,300</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: red">(+39 [+2.0%])</span> <div style='text-align: right'>1,950</div>  |
| inner_verifier | Audit | ImmE | STOREW | <div style='text-align: right'>214,624</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <span style="color: red">(+1,056 [+0.1%])</span> <div style='text-align: right'>817,344</div>  |
| inner_verifier | Audit | ImmF | STOREW | <div style='text-align: right'>2,337</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <span style="color: red">(+155,892 [+17.7%])</span> <div style='text-align: right'>1,037,454</div>  |
| inner_verifier | Audit | ImmV | STOREW | <div style='text-align: right'>15,048</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <span style="color: red">(+122,298 [+8.8%])</span> <div style='text-align: right'>1,519,056</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: red">(+5,148 [+8.3%])</span> <div style='text-align: right'>66,836</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: red">(+3,042 [+8.3%])</span> <div style='text-align: right'>39,494</div>  |
| inner_verifier | Audit | LoadE | LOADW | <div style='text-align: right'>503,120</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <span style="color: red">(+90,552 [+3.3%])</span> <div style='text-align: right'>2,810,544</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <span style="color: red">(+6,930 [+30.5%])</span> <div style='text-align: right'>29,634</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <span style="color: red">(+4,095 [+30.5%])</span> <div style='text-align: right'>17,511</div>  |
| inner_verifier | Audit | LoadE | LOADW2 | <div style='text-align: right'>76</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <span style="color: red">(+366,168 [+0.7%])</span> <div style='text-align: right'>53,164,848</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <span style="color: red">(+5,544 [+26.1%])</span> <div style='text-align: right'>26,796</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <span style="color: red">(+3,276 [+26.1%])</span> <div style='text-align: right'>15,834</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <span style="color: red">(+2,142 [+26.1%])</span> <div style='text-align: right'>10,353</div>  |
| inner_verifier | Audit | LoadF | LOADW | <div style='text-align: right'>494</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <span style="color: red">(+198,594 [+27.5%])</span> <div style='text-align: right'>920,568</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <span style="color: red">(+110 [+18.9%])</span> <div style='text-align: right'>693</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <span style="color: red">(+65 [+18.5%])</span> <div style='text-align: right'>416</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <span style="color: red">(+85 [+22.7%])</span> <div style='text-align: right'>459</div>  |
| inner_verifier | Audit | LoadF | LOADW2 | <div style='text-align: right'>532</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <span style="color: red">(+93,390 [+0.5%])</span> <div style='text-align: right'>19,818,744</div>  |
| inner_verifier | Audit | LoadV | LOADW | <span style="color: red">(+38 [+0.3%])</span> <div style='text-align: right'>13,718</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <span style="color: red">(+256,938 [+35.5%])</span> <div style='text-align: right'>981,486</div>  |
| inner_verifier | Audit | LoadV | LOADW2 | <div style='text-align: right'>1,615</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <span style="color: red">(+897,402 [+22.1%])</span> <div style='text-align: right'>4,963,068</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: red">(+7,920 [+1.6%])</span> <div style='text-align: right'>500,962</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: red">(+4,680 [+1.6%])</span> <div style='text-align: right'>296,023</div>  |
| inner_verifier | Audit | MulE | BBE4MUL | <div style='text-align: right'>1,215,620</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | <span style="color: red">(+63,960 [+0.4%])</span> <div style='text-align: right'>16,792,206</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: red">(+2,310 [+29.2%])</span> <div style='text-align: right'>10,208</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: red">(+1,365 [+29.2%])</span> <div style='text-align: right'>6,032</div>  |
| inner_verifier | Audit | MulEF | MUL | <div style='text-align: right'>608</div>  |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | <span style="color: red">(+13,268 [+27.0%])</span> <div style='text-align: right'>62,372</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,694</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,001</div>  |
| inner_verifier | Audit | MulEFI | MUL | <div style='text-align: right'>24,244</div>  |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | <div style='text-align: right'>44,392</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: red">(+1,936 [+1.2%])</span> <div style='text-align: right'>158,796</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: red">(+1,144 [+1.2%])</span> <div style='text-align: right'>93,834</div>  |
| inner_verifier | Audit | MulEI | BBE4MUL | <div style='text-align: right'>154,660</div>  |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | <span style="color: red">(+1,681 [+1.6%])</span> <div style='text-align: right'>106,559</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: red">(+847 [+1.5%])</span> <div style='text-align: right'>56,892</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: red">(+468 [+1.4%])</span> <div style='text-align: right'>33,462</div>  |
| inner_verifier | Audit | MulEI | STOREW | <div style='text-align: right'>57</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <span style="color: red">(+10,824 [+1.6%])</span> <div style='text-align: right'>686,136</div>  |
| inner_verifier | Audit | MulF | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulF | MUL | <span style="color: red">(+132,928 [+19.5%])</span> <div style='text-align: right'>815,083</div>  |
| inner_verifier | Audit | MulFI | MUL | <div style='text-align: right'>19</div>  |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | <span style="color: red">(+31 [+8.3%])</span> <div style='text-align: right'>403</div>  |
| inner_verifier | Audit | MulV | MUL | <div style='text-align: right'>12,901</div>  |
| inner_verifier | FieldArithmeticAir | MulV | MUL | <div style='text-align: right'>21,142</div>  |
| inner_verifier | Audit | MulVI | MUL | <div style='text-align: right'>133</div>  |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | <span style="color: red">(+103,757 [+43.3%])</span> <div style='text-align: right'>343,511</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: red">(+22 [+1.9%])</span> <div style='text-align: right'>1,210</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: red">(+13 [+1.9%])</span> <div style='text-align: right'>715</div>  |
| inner_verifier | Audit | NegE | MUL | <div style='text-align: right'>2,356</div>  |
| inner_verifier | FieldArithmeticAir | NegE | MUL | <span style="color: red">(+124 [+2.2%])</span> <div style='text-align: right'>5,828</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+138,600 [+50.2%])</span> <div style='text-align: right'>414,876</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+81,900 [+50.2%])</span> <div style='text-align: right'>245,154</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+53,550 [+50.2%])</span> <div style='text-align: right'>160,293</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <span style="color: red">(+1,343,034 [+48.3%])</span> <div style='text-align: right'>4,125,660</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+17,028 [+2.8%])</span> <div style='text-align: right'>615,505</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+10,062 [+2.8%])</span> <div style='text-align: right'>364,130</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+6,630 [+2.8%])</span> <div style='text-align: right'>239,904</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: red">(+81,510 [+1.5%])</span> <div style='text-align: right'>5,446,958</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <span style="color: red">(+2,310 [+31.1%])</span> <div style='text-align: right'>9,746</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <span style="color: red">(+1,365 [+31.1%])</span> <div style='text-align: right'>5,759</div>  |
| inner_verifier | Audit | StoreE | STOREW | <span style="color: red">(+32,148 [+15.5%])</span> <div style='text-align: right'>239,400</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <span style="color: red">(+111,672 [+15.5%])</span> <div style='text-align: right'>831,600</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <span style="color: red">(+3,696 [+8.2%])</span> <div style='text-align: right'>48,972</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <span style="color: red">(+2,184 [+8.2%])</span> <div style='text-align: right'>28,938</div>  |
| inner_verifier | Audit | StoreE | STOREW2 | <span style="color: red">(+8,360 [+31.2%])</span> <div style='text-align: right'>35,112</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <span style="color: red">(+101,112 [+13.9%])</span> <div style='text-align: right'>826,056</div>  |
| inner_verifier | Audit | StoreF | STOREW | <span style="color: red">(+41,686 [+19.6%])</span> <div style='text-align: right'>254,714</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <span style="color: red">(+144,804 [+19.6%])</span> <div style='text-align: right'>884,796</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: red">(+286 [+0.1%])</span> <div style='text-align: right'>522,357</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: red">(+169 [+0.1%])</span> <div style='text-align: right'>309,088</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: red">(+119 [+0.1%])</span> <div style='text-align: right'>203,728</div>  |
| inner_verifier | Audit | StoreF | STOREW2 | <span style="color: red">(+16,188 [+29.2%])</span> <div style='text-align: right'>71,592</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <span style="color: red">(+83,886 [+1.3%])</span> <div style='text-align: right'>6,788,298</div>  |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | <span style="color: red">(+816,788 [+14.0%])</span> <div style='text-align: right'>6,651,639</div>  |
| inner_verifier | Audit | StoreHintWord | SHINTW | <span style="color: red">(+569,335 [+15.1%])</span> <div style='text-align: right'>4,328,067</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <span style="color: red">(+1,977,690 [+15.1%])</span> <div style='text-align: right'>15,034,338</div>  |
| inner_verifier | Audit | StoreV | STOREW | <span style="color: red">(+6,479 [+25.6%])</span> <div style='text-align: right'>31,806</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <span style="color: red">(+22,506 [+25.6%])</span> <div style='text-align: right'>110,484</div>  |
| inner_verifier | Audit | StoreV | STOREW2 | <span style="color: red">(+142,082 [+32.8%])</span> <div style='text-align: right'>575,472</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <span style="color: red">(+486,618 [+32.0%])</span> <div style='text-align: right'>2,008,314</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: red">(+11,572 [+2.6%])</span> <div style='text-align: right'>464,970</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: red">(+6,838 [+2.6%])</span> <div style='text-align: right'>274,755</div>  |
| inner_verifier | Audit | SubE | FE4SUB | <div style='text-align: right'>958,132</div>  |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | <span style="color: red">(+13,120 [+2.3%])</span> <div style='text-align: right'>577,813</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: red">(+12,034 [+0.3%])</span> <div style='text-align: right'>4,294,114</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <span style="color: red">(+216,612 [+0.3%])</span> <div style='text-align: right'>77,294,052</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: red">(+12,034 [+0.3%])</span> <div style='text-align: right'>4,294,114</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: red">(+14,222 [+0.3%])</span> <div style='text-align: right'>5,074,862</div>  |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | <span style="color: red">(+33,914 [+0.3%])</span> <div style='text-align: right'>12,101,594</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>299</div>  |
| inner_verifier | Audit | SubEFI | ADD | <div style='text-align: right'>22,800</div>  |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | <div style='text-align: right'>39,928</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: red">(+66 [+3.4%])</span> <div style='text-align: right'>1,980</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: red">(+39 [+3.4%])</span> <div style='text-align: right'>1,170</div>  |
| inner_verifier | Audit | SubEI | ADD | <div style='text-align: right'>608</div>  |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | <span style="color: red">(+248 [+3.3%])</span> <div style='text-align: right'>7,688</div>  |
| inner_verifier | Audit | SubV | SUB | <div style='text-align: right'>76</div>  |
| inner_verifier | FieldArithmeticAir | SubV | SUB | <span style="color: red">(+81,406 [+18.9%])</span> <div style='text-align: right'>512,089</div>  |
| inner_verifier | Audit | SubVI | SUB | <div style='text-align: right'>13,357</div>  |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | <span style="color: red">(+4,495 [+11.7%])</span> <div style='text-align: right'>42,904</div>  |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | <span style="color: red">(+3,255 [+31.2%])</span> <div style='text-align: right'>13,671</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | <div style='text-align: right'>576</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,392</div>  | <div style='text-align: right'>115</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>62</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | FieldArithmeticAir | <div style='text-align: right'>1,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | FieldExtensionArithmeticAir | <div style='text-align: right'>77</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| bench_program_inner | MemoryAuditAir | <div style='text-align: right'>4,480</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | ProgramAir<BabyBear> | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>9</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>360,710,144</div>  | <div style='text-align: right'>113</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>66</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | FieldArithmeticAir | <div style='text-align: right'>98,566,144</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | FieldExtensionArithmeticAir | <div style='text-align: right'>59,768,832</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>373</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | RangeTupleCheckerAir<2> | <div style='text-align: right'>18,874,368</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | MemoryAuditAir | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>20</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7ae64b1b5dac841b57b39e7eeea60803813fff54/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/7ae64b1b5dac841b57b39e7eeea60803813fff54
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11267318429)
