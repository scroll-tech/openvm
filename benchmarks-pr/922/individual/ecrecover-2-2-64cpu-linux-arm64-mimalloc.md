| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-19,860,079 [-6.5%])</span> <div style='text-align: right'>286,875,981</div>  | <span style="color: green">(-554,042 [-9.6%])</span> <div style='text-align: right'>5,232,849</div>  | <span style="color: green">(-11,246.0 [-29.5%])</span> <div style='text-align: right'>26,823.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| ecrecover_program | true | <span style="color: green">(-51,902.0 [-60.2%])</span> <div style='text-align: right'>34,384.0</div>  | <span style="color: green">(-19,860,079 [-6.5%])</span> <div style='text-align: right'>286,875,981</div>  | <span style="color: green">(-554,042 [-9.6%])</span> <div style='text-align: right'>5,232,849</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| ecrecover_program | ProgramChip | true | <span style="color: green">(-1,498 [-8.5%])</span> <div style='text-align: right'>16,147</div>  |
| ecrecover_program | VmConnectorAir | true | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | true | <span style="color: green">(-38 [-0.1%])</span> <div style='text-align: right'>63,436</div>  |
| ecrecover_program | Merkle | true | <span style="color: green">(-18 [-0.0%])</span> <div style='text-align: right'>63,790</div>  |
| ecrecover_program | AccessAdapter<2> | true | <div style='text-align: right'>650</div>  |
| ecrecover_program | AccessAdapter<4> | true | <div style='text-align: right'>364</div>  |
| ecrecover_program | AccessAdapter<8> | true | <span style="color: red">(+12 [+0.0%])</span> <div style='text-align: right'>253,590</div>  |
| ecrecover_program | AccessAdapter<16> | true | <span style="color: red">(+22 [+0.0%])</span> <div style='text-align: right'>95,138</div>  |
| ecrecover_program | AccessAdapter<32> | true | <span style="color: red">(+12 [+0.0%])</span> <div style='text-align: right'>47,570</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>2,556</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>2,551</div>  |
| ecrecover_program | KeccakVmAir | true | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true | <div style='text-align: right'>16,045</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true | <div style='text-align: right'>26</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true | <div style='text-align: right'>1,281</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>174</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <span style="color: green">(-15 [-0.0%])</span> <div style='text-align: right'>184,740</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: green">(-7,719 [-3.8%])</span> <div style='text-align: right'>195,166</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-18,655 [-35.3%])</span> <div style='text-align: right'>34,171</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-37,235 [-35.3%])</span> <div style='text-align: right'>68,352</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: green">(-14,548 [-28.6%])</span> <div style='text-align: right'>36,313</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <span style="color: green">(-19,017 [-10.7%])</span> <div style='text-align: right'>159,090</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-28,268 [-10.0%])</span> <div style='text-align: right'>253,699</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <span style="color: red">(+105 [+0.1%])</span> <div style='text-align: right'>74,297</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-190,487 [-12.4%])</span> <div style='text-align: right'>1,346,654</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: green">(-20,376 [-3.8%])</span> <div style='text-align: right'>515,741</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: green">(-10,917 [-3.3%])</span> <div style='text-align: right'>322,815</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-206,904 [-9.3%])</span> <div style='text-align: right'>2,016,480</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | true | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>2,675</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | true | <span style="color: green">(-56 [-0.0%])</span> <div style='text-align: right'>127,226</div>  |
| ecrecover_program | VariableRangeCheckerAir | true | <div style='text-align: right'>262,144</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| ecrecover_program | true |  | ADD | <span style="color: green">(-158,182 [-9.6%])</span> <div style='text-align: right'>1,485,144</div>  |
| ecrecover_program | true |  | AND | <span style="color: green">(-29,351 [-8.6%])</span> <div style='text-align: right'>310,749</div>  |
| ecrecover_program | true |  | AUIPC | <span style="color: green">(-18,655 [-35.3%])</span> <div style='text-align: right'>34,171</div>  |
| ecrecover_program | true |  | BEQ | <span style="color: green">(-10,787 [-9.3%])</span> <div style='text-align: right'>105,824</div>  |
| ecrecover_program | true |  | BGE | <div style='text-align: right'>9,005</div>  |
| ecrecover_program | true |  | BGEU | <span style="color: green">(-2,546 [-32.6%])</span> <div style='text-align: right'>5,261</div>  |
| ecrecover_program | true |  | BLT | <span style="color: red">(+7 [+11.7%])</span> <div style='text-align: right'>67</div>  |
| ecrecover_program | true |  | BLTU | <span style="color: green">(-16,478 [-10.2%])</span> <div style='text-align: right'>144,757</div>  |
| ecrecover_program | true |  | BNE | <span style="color: green">(-17,481 [-10.6%])</span> <div style='text-align: right'>147,875</div>  |
| ecrecover_program | true |  | EcAddNe | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>2,551</div>  |
| ecrecover_program | true |  | EcDouble | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>2,556</div>  |
| ecrecover_program | true |  | HINT_STOREW | <div style='text-align: right'>174</div>  |
| ecrecover_program | true |  | IS_EQ | <div style='text-align: right'>16,049</div>  |
| ecrecover_program | true |  | JAL | <span style="color: green">(-4,884 [-22.7%])</span> <div style='text-align: right'>16,649</div>  |
| ecrecover_program | true |  | JALR | <span style="color: green">(-37,235 [-35.3%])</span> <div style='text-align: right'>68,352</div>  |
| ecrecover_program | true |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program | true |  | LOADB | <span style="color: red">(+100 [+0.1%])</span> <div style='text-align: right'>74,292</div>  |
| ecrecover_program | true |  | LOADBU | <span style="color: green">(-150 [-1.1%])</span> <div style='text-align: right'>13,230</div>  |
| ecrecover_program | true |  | LOADH | <div style='text-align: right'>5</div>  |
| ecrecover_program | true |  | LOADW | <span style="color: green">(-106,325 [-15.3%])</span> <div style='text-align: right'>588,881</div>  |
| ecrecover_program | true |  | LUI | <span style="color: green">(-9,664 [-33.0%])</span> <div style='text-align: right'>19,664</div>  |
| ecrecover_program | true |  | MUL | <span style="color: green">(-7,719 [-3.8%])</span> <div style='text-align: right'>195,166</div>  |
| ecrecover_program | true |  | MULHU | <span style="color: green">(-15 [-0.0%])</span> <div style='text-align: right'>184,740</div>  |
| ecrecover_program | true |  | ModularAddSub | <div style='text-align: right'>1,292</div>  |
| ecrecover_program | true |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program | true |  | OR | <span style="color: green">(-2,220 [-1.1%])</span> <div style='text-align: right'>198,486</div>  |
| ecrecover_program | true |  | PHANTOM | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>2,675</div>  |
| ecrecover_program | true |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program | true |  | SLL | <span style="color: green">(-20,223 [-7.5%])</span> <div style='text-align: right'>249,449</div>  |
| ecrecover_program | true |  | SLTU | <span style="color: green">(-10,917 [-3.3%])</span> <div style='text-align: right'>322,815</div>  |
| ecrecover_program | true |  | SRA | <div style='text-align: right'>2,562</div>  |
| ecrecover_program | true |  | SRL | <span style="color: green">(-2,715 [-1.0%])</span> <div style='text-align: right'>263,730</div>  |
| ecrecover_program | true |  | STOREB | <span style="color: green">(-641 [-0.6%])</span> <div style='text-align: right'>115,051</div>  |
| ecrecover_program | true |  | STOREH | <span style="color: red">(+5 [+100.0%])</span> <div style='text-align: right'>10</div>  |
| ecrecover_program | true |  | STOREW | <span style="color: green">(-83,376 [-11.7%])</span> <div style='text-align: right'>629,482</div>  |
| ecrecover_program | true |  | SUB | <span style="color: green">(-11,562 [-39.9%])</span> <div style='text-align: right'>17,421</div>  |
| ecrecover_program | true |  | XOR | <span style="color: green">(-5,589 [-54.4%])</span> <div style='text-align: right'>4,680</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-5,694,552 [-9.6%])</span> <div style='text-align: right'>53,465,184</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ADD | <span style="color: red">(+34 [+200.0%])</span> <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary | true |  | ADD | <span style="color: red">(+80 [+200.0%])</span> <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle | true |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: green">(-1,056,636 [-8.6%])</span> <div style='text-align: right'>11,186,964</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-391,755 [-35.3%])</span> <div style='text-align: right'>717,591</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: green">(-280,462 [-9.3%])</span> <div style='text-align: right'>2,751,424</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>288,160</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <span style="color: green">(-81,472 [-32.6%])</span> <div style='text-align: right'>168,352</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <span style="color: red">(+224 [+11.7%])</span> <div style='text-align: right'>2,144</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <span style="color: green">(-527,296 [-10.2%])</span> <div style='text-align: right'>4,632,224</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-454,506 [-10.6%])</span> <div style='text-align: right'>3,844,750</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcAddNe | <span style="color: red">(+619 [+0.0%])</span> <div style='text-align: right'>1,579,069</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcAddNe | <span style="color: red">(+275 [+0.1%])</span> <div style='text-align: right'>255,275</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcAddNe | <span style="color: red">(+246 [+0.1%])</span> <div style='text-align: right'>209,346</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcAddNe | <span style="color: red">(+306 [+0.1%])</span> <div style='text-align: right'>347,106</div>  |
| ecrecover_program | Boundary | true |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle | true |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | true |  | EcDouble | <span style="color: red">(+543 [+0.0%])</span> <div style='text-align: right'>1,387,908</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | EcDouble | <div style='text-align: right'>127,750</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | EcDouble | <div style='text-align: right'>104,755</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | EcDouble | <div style='text-align: right'>173,740</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>4,524</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>1,513</div>  |
| ecrecover_program | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle | true |  | HINT_STOREW | <span style="color: green">(-256 [-3.9%])</span> <div style='text-align: right'>6,272</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | IS_EQ | <div style='text-align: right'>2,664,134</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | IS_EQ | <div style='text-align: right'>675,250</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | IS_EQ | <div style='text-align: right'>553,705</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | IS_EQ | <div style='text-align: right'>918,272</div>  |
| ecrecover_program | Boundary | true |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle | true |  | IS_EQ | <span style="color: red">(+128 [+33.3%])</span> <div style='text-align: right'>512</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: green">(-87,912 [-22.7%])</span> <div style='text-align: right'>299,682</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-1,042,580 [-35.3%])</span> <div style='text-align: right'>1,913,856</div>  |
| ecrecover_program | AccessAdapter<2> | true |  | KECCAK256 | <div style='text-align: right'>3,575</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | KECCAK256 | <div style='text-align: right'>2,145</div>  |
| ecrecover_program | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <span style="color: red">(+3,500 [+0.1%])</span> <div style='text-align: right'>2,600,220</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: green">(-6,000 [-1.1%])</span> <div style='text-align: right'>529,200</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADBU | <span style="color: green">(-68 [-8.9%])</span> <div style='text-align: right'>697</div>  |
| ecrecover_program | Boundary | true |  | LOADBU | <span style="color: green">(-160 [-8.9%])</span> <div style='text-align: right'>1,640</div>  |
| ecrecover_program | Merkle | true |  | LOADBU | <span style="color: red">(+192 [+7.7%])</span> <div style='text-align: right'>2,688</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-4,253,000 [-15.3%])</span> <div style='text-align: right'>23,555,240</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | LOADW | <div style='text-align: right'>643,350</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | LOADW | <div style='text-align: right'>527,547</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-170 [-0.0%])</span> <div style='text-align: right'>997,628</div>  |
| ecrecover_program | Boundary | true |  | LOADW | <span style="color: green">(-400 [-0.1%])</span> <div style='text-align: right'>288,640</div>  |
| ecrecover_program | Merkle | true |  | LOADW | <span style="color: green">(-192 [-0.1%])</span> <div style='text-align: right'>379,968</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <span style="color: green">(-173,952 [-33.0%])</span> <div style='text-align: right'>353,952</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: green">(-239,289 [-3.8%])</span> <div style='text-align: right'>6,050,146</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <span style="color: green">(-585 [-0.0%])</span> <div style='text-align: right'>7,204,860</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | true |  | ModularAddSub | <div style='text-align: right'>257,108</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularAddSub | <div style='text-align: right'>129,200</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularAddSub | <div style='text-align: right'>105,944</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularAddSub | <span style="color: green">(-153 [-0.1%])</span> <div style='text-align: right'>175,593</div>  |
| ecrecover_program | Boundary | true |  | ModularAddSub | <span style="color: green">(-40 [-5.6%])</span> <div style='text-align: right'>680</div>  |
| ecrecover_program | Merkle | true |  | ModularAddSub | <span style="color: red">(+64 [+2.6%])</span> <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | true |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | ModularMulDiv | <div style='text-align: right'>1,750</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | ModularMulDiv | <div style='text-align: right'>1,435</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | ModularMulDiv | <div style='text-align: right'>2,380</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: green">(-79,920 [-1.1%])</span> <div style='text-align: right'>7,145,496</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | OR | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary | true |  | OR | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle | true |  | OR | <div style='text-align: right'>64</div>  |
| ecrecover_program | PhantomAir | true |  | PHANTOM | <span style="color: red">(+12 [+0.1%])</span> <div style='text-align: right'>16,050</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | true |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: green">(-1,071,819 [-7.5%])</span> <div style='text-align: right'>13,220,797</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: green">(-403,929 [-3.3%])</span> <div style='text-align: right'>11,944,155</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>135,786</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: green">(-143,895 [-1.0%])</span> <div style='text-align: right'>13,977,690</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | SRL | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary | true |  | SRL | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <span style="color: green">(-25,640 [-0.6%])</span> <div style='text-align: right'>4,602,040</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREB | <span style="color: red">(+24,125 [+23.4%])</span> <div style='text-align: right'>127,425</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREB | <span style="color: red">(+39,770 [+23.5%])</span> <div style='text-align: right'>208,977</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREB | <span style="color: red">(+16,592 [+11.0%])</span> <div style='text-align: right'>167,824</div>  |
| ecrecover_program | Boundary | true |  | STOREB | <span style="color: red">(+640 [+0.3%])</span> <div style='text-align: right'>191,000</div>  |
| ecrecover_program | Merkle | true |  | STOREB | <span style="color: red">(+2,432 [+0.5%])</span> <div style='text-align: right'>515,008</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <span style="color: red">(+200 [+100.0%])</span> <div style='text-align: right'>400</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-3,335,040 [-11.7%])</span> <div style='text-align: right'>25,179,280</div>  |
| ecrecover_program | AccessAdapter<16> | true |  | STOREW | <span style="color: green">(-23,850 [-5.4%])</span> <div style='text-align: right'>417,700</div>  |
| ecrecover_program | AccessAdapter<2> | true |  | STOREW | <div style='text-align: right'>2,860</div>  |
| ecrecover_program | AccessAdapter<32> | true |  | STOREW | <span style="color: green">(-39,524 [-14.2%])</span> <div style='text-align: right'>238,046</div>  |
| ecrecover_program | AccessAdapter<4> | true |  | STOREW | <div style='text-align: right'>1,716</div>  |
| ecrecover_program | AccessAdapter<8> | true |  | STOREW | <span style="color: green">(-16,371 [-1.6%])</span> <div style='text-align: right'>987,275</div>  |
| ecrecover_program | Boundary | true |  | STOREW | <span style="color: green">(-960 [-0.1%])</span> <div style='text-align: right'>782,520</div>  |
| ecrecover_program | Merkle | true |  | STOREW | <span style="color: green">(-3,008 [-0.3%])</span> <div style='text-align: right'>1,130,432</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: green">(-416,232 [-39.9%])</span> <div style='text-align: right'>627,156</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: green">(-201,204 [-54.4%])</span> <div style='text-align: right'>168,480</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <span style="color: green">(-8.0 [-53.3%])</span> <div style='text-align: right'>7.0</div>  | <span style="color: green">(-664.0 [-7.9%])</span> <div style='text-align: right'>7,780.0</div>  | <span style="color: green">(-577.0 [-9.0%])</span> <div style='text-align: right'>5,850.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-15.0 [-6.8%])</span> <div style='text-align: right'>206.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-19,860,079 [-6.5%])</span> <div style='text-align: right'>286,875,981</div>  | <span style="color: green">(-554,042 [-9.6%])</span> <div style='text-align: right'>5,232,849</div>  | <span style="color: green">(-11,246.0 [-29.5%])</span> <div style='text-align: right'>26,823.0</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>449</div>  | <div style='text-align: right'>411</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | <div style='text-align: right'>456</div>  | <div style='text-align: right'>422</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | <div style='text-align: right'>223</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | <div style='text-align: right'>188</div>  | <div style='text-align: right'>156</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | <div style='text-align: right'>126</div>  | <div style='text-align: right'>94</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <span style="color: green">(-294,912 [-50.0%])</span> <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-16,384 [-50.0%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>3,407,872</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | AccessAdapterAir<2> | 0 | <div style='text-align: right'>35,840</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>1,024</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <div style='text-align: right'>18,944</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>512</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>10,747,904</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <div style='text-align: right'>6,422,528</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <div style='text-align: right'>4,259,840</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>5,615,616</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <div style='text-align: right'>6,008,832</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <div style='text-align: right'>3,637,248</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <div style='text-align: right'>18,464</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <div style='text-align: right'>32</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <div style='text-align: right'>800,768</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>36,438,016</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>29,097,984</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>8,388,608</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>4,063,232</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>23,068,672</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <span style="color: green">(-19,398,656 [-50.0%])</span> <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <span style="color: green">(-262,144 [-50.0%])</span> <div style='text-align: right'>262,144</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>14,548,992</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>234,881,024</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-55,050,240 [-50.0%])</span> <div style='text-align: right'>55,050,240</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: green">(-524,288 [-50.0%])</span> <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <span style="color: green">(-243,269,632 [-50.0%])</span> <div style='text-align: right'>243,269,632</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-2,097,152 [-50.0%])</span> <div style='text-align: right'>2,097,152</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4,096</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>82,182,144</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: green">(-86.0 [-4.3%])</span> <div style='text-align: right'>1,928.0</div>  | <span style="color: green">(-10,496.0 [-38.0%])</span> <div style='text-align: right'>17,115.0</div>  | <span style="color: green">(-318,016,016 [-27.3%])</span> <div style='text-align: right'>845,735,905</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/7790528f6dc54cf48cc1ffc3cb8dce9eee763b40

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12134338510)
