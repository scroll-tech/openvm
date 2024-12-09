| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>2</div>  | <span style="color: green">(-183,473,670 [-64.6%])</span> <div style='text-align: right'>100,539,955</div>  | <span style="color: green">(-3,660,585 [-70.9%])</span> <div style='text-align: right'>1,502,571</div>  | <span style="color: green">(-16,037.0 [-60.4%])</span> <div style='text-align: right'>10,520.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | <div style='text-align: right'>8.0</div>  | <span style="color: green">(-4,738.0 [-64.4%])</span> <div style='text-align: right'>2,618.0</div>  | <span style="color: green">(-3,851.0 [-67.2%])</span> <div style='text-align: right'>1,882.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+3.0 [+1.2%])</span> <div style='text-align: right'>259.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-183,473,670 [-64.6%])</span> <div style='text-align: right'>100,539,955</div>  | <span style="color: green">(-3,660,585 [-70.9%])</span> <div style='text-align: right'>1,502,571</div>  | <span style="color: green">(-16,037.0 [-60.4%])</span> <div style='text-align: right'>10,520.0</div>  |

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

| group | chip_name | rows_used |
| --- | --- | --- |
| ecrecover_program | ProgramChip | <span style="color: green">(-7,464 [-47.0%])</span> <div style='text-align: right'>8,431</div>  |
| ecrecover_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| ecrecover_program | Boundary | <span style="color: green">(-31,834 [-50.2%])</span> <div style='text-align: right'>31,622</div>  |
| ecrecover_program | Merkle | <span style="color: green">(-31,874 [-50.0%])</span> <div style='text-align: right'>31,936</div>  |
| ecrecover_program | AccessAdapter<4> | <span style="color: green">(-330 [-90.7%])</span> <div style='text-align: right'>34</div>  |
| ecrecover_program | AccessAdapter<8> | <span style="color: green">(-126,754 [-50.0%])</span> <div style='text-align: right'>126,872</div>  |
| ecrecover_program | AccessAdapter<16> | <span style="color: green">(-47,468 [-49.9%])</span> <div style='text-align: right'>47,670</div>  |
| ecrecover_program | AccessAdapter<32> | <span style="color: green">(-23,734 [-49.9%])</span> <div style='text-align: right'>23,836</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: green">(-1,279 [-50.0%])</span> <div style='text-align: right'>1,277</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | <span style="color: green">(-1,283 [-50.3%])</span> <div style='text-align: right'>1,268</div>  |
| ecrecover_program | KeccakVmAir | <div style='text-align: right'>120</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | <span style="color: green">(-8,027 [-50.0%])</span> <div style='text-align: right'>8,018</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | <span style="color: green">(-10 [-38.5%])</span> <div style='text-align: right'>16</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | <span style="color: green">(-639 [-49.9%])</span> <div style='text-align: right'>642</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <span style="color: red">(+40 [+23.0%])</span> <div style='text-align: right'>214</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | <span style="color: green">(-193,875 [-99.3%])</span> <div style='text-align: right'>1,291</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <span style="color: green">(-20,287 [-58.2%])</span> <div style='text-align: right'>14,568</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <span style="color: green">(-40,574 [-58.2%])</span> <div style='text-align: right'>29,146</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <span style="color: green">(-17,763 [-54.7%])</span> <div style='text-align: right'>14,707</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <span style="color: green">(-85,476 [-52.6%])</span> <div style='text-align: right'>76,944</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <span style="color: green">(-138,643 [-53.8%])</span> <div style='text-align: right'>119,280</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | <span style="color: green">(-37,514 [-50.2%])</span> <div style='text-align: right'>37,173</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <span style="color: green">(-742,627 [-59.3%])</span> <div style='text-align: right'>510,113</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <span style="color: green">(-439,446 [-85.2%])</span> <div style='text-align: right'>76,595</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <span style="color: green">(-273,229 [-84.3%])</span> <div style='text-align: right'>50,954</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <span style="color: green">(-1,473,868 [-72.5%])</span> <div style='text-align: right'>559,012</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | <span style="color: green">(-1,350 [-50.5%])</span> <div style='text-align: right'>1,325</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | <span style="color: green">(-63,708 [-50.1%])</span> <div style='text-align: right'>63,558</div>  |
| ecrecover_program | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| ecrecover_program |  | ADD | <span style="color: green">(-1,110,478 [-74.2%])</span> <div style='text-align: right'>386,128</div>  |
| ecrecover_program |  | AND | <span style="color: green">(-197,046 [-62.6%])</span> <div style='text-align: right'>117,837</div>  |
| ecrecover_program |  | AUIPC | <span style="color: green">(-20,287 [-58.2%])</span> <div style='text-align: right'>14,568</div>  |
| ecrecover_program |  | BEQ | <span style="color: green">(-56,220 [-52.4%])</span> <div style='text-align: right'>50,976</div>  |
| ecrecover_program |  | BGE | <span style="color: green">(-4,501 [-50.0%])</span> <div style='text-align: right'>4,504</div>  |
| ecrecover_program |  | BGEU | <span style="color: green">(-1,335 [-25.4%])</span> <div style='text-align: right'>3,930</div>  |
| ecrecover_program |  | BLT | <span style="color: green">(-55 [-82.1%])</span> <div style='text-align: right'>12</div>  |
| ecrecover_program |  | BLTU | <span style="color: green">(-79,585 [-53.7%])</span> <div style='text-align: right'>68,498</div>  |
| ecrecover_program |  | BNE | <span style="color: green">(-82,423 [-54.7%])</span> <div style='text-align: right'>68,304</div>  |
| ecrecover_program |  | EcAddNe | <span style="color: green">(-1,283 [-50.3%])</span> <div style='text-align: right'>1,268</div>  |
| ecrecover_program |  | EcDouble | <span style="color: green">(-1,279 [-50.0%])</span> <div style='text-align: right'>1,277</div>  |
| ecrecover_program |  | HINT_STOREW | <span style="color: red">(+40 [+23.0%])</span> <div style='text-align: right'>214</div>  |
| ecrecover_program |  | IS_EQ | <span style="color: green">(-8,022 [-50.0%])</span> <div style='text-align: right'>8,027</div>  |
| ecrecover_program |  | JAL | <span style="color: green">(-7,963 [-49.7%])</span> <div style='text-align: right'>8,062</div>  |
| ecrecover_program |  | JALR | <span style="color: green">(-40,574 [-58.2%])</span> <div style='text-align: right'>29,146</div>  |
| ecrecover_program |  | KECCAK256 | <div style='text-align: right'>5</div>  |
| ecrecover_program |  | LOADB | <span style="color: green">(-37,509 [-50.2%])</span> <div style='text-align: right'>37,173</div>  |
| ecrecover_program |  | LOADBU | <span style="color: green">(-8,048 [-60.8%])</span> <div style='text-align: right'>5,182</div>  |
| ecrecover_program |  | LOADW | <span style="color: green">(-329,405 [-61.0%])</span> <div style='text-align: right'>211,009</div>  |
| ecrecover_program |  | LUI | <span style="color: green">(-9,800 [-59.6%])</span> <div style='text-align: right'>6,645</div>  |
| ecrecover_program |  | MUL | <span style="color: green">(-193,875 [-99.3%])</span> <div style='text-align: right'>1,291</div>  |
| ecrecover_program |  | ModularAddSub | <span style="color: green">(-649 [-50.2%])</span> <div style='text-align: right'>643</div>  |
| ecrecover_program |  | ModularMulDiv | <div style='text-align: right'>27</div>  |
| ecrecover_program |  | OR | <span style="color: green">(-153,401 [-77.0%])</span> <div style='text-align: right'>45,889</div>  |
| ecrecover_program |  | PHANTOM | <span style="color: green">(-1,350 [-50.5%])</span> <div style='text-align: right'>1,325</div>  |
| ecrecover_program |  | SETUP_ISEQ | <div style='text-align: right'>2</div>  |
| ecrecover_program |  | SLL | <span style="color: green">(-213,415 [-85.5%])</span> <div style='text-align: right'>36,154</div>  |
| ecrecover_program |  | SLTU | <span style="color: green">(-273,229 [-84.3%])</span> <div style='text-align: right'>50,954</div>  |
| ecrecover_program |  | SRA | <span style="color: green">(-1,278 [-49.9%])</span> <div style='text-align: right'>1,284</div>  |
| ecrecover_program |  | SRL | <span style="color: green">(-224,753 [-85.2%])</span> <div style='text-align: right'>39,157</div>  |
| ecrecover_program |  | STOREB | <span style="color: green">(-59,129 [-51.2%])</span> <div style='text-align: right'>56,402</div>  |
| ecrecover_program |  | STOREW | <span style="color: green">(-346,035 [-59.3%])</span> <div style='text-align: right'>237,520</div>  |
| ecrecover_program |  | SUB | <span style="color: green">(-8,919 [-51.2%])</span> <div style='text-align: right'>8,502</div>  |
| ecrecover_program |  | XOR | <span style="color: green">(-4,024 [-86.0%])</span> <div style='text-align: right'>656</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <span style="color: green">(-39,977,208 [-74.2%])</span> <div style='text-align: right'>13,900,608</div>  |
| ecrecover_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>51</div>  |
| ecrecover_program | Boundary |  | ADD | <div style='text-align: right'>120</div>  |
| ecrecover_program | Merkle |  | ADD | <div style='text-align: right'>64</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <span style="color: green">(-7,093,656 [-62.6%])</span> <div style='text-align: right'>4,242,132</div>  |
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <span style="color: green">(-426,027 [-58.2%])</span> <div style='text-align: right'>305,928</div>  |
| ecrecover_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| ecrecover_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| ecrecover_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <span style="color: green">(-1,461,720 [-52.4%])</span> <div style='text-align: right'>1,325,376</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | <span style="color: green">(-144,032 [-50.0%])</span> <div style='text-align: right'>144,128</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <span style="color: green">(-42,720 [-25.4%])</span> <div style='text-align: right'>125,760</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <span style="color: green">(-1,760 [-82.1%])</span> <div style='text-align: right'>384</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <span style="color: green">(-2,546,720 [-53.7%])</span> <div style='text-align: right'>2,191,936</div>  |
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <span style="color: green">(-2,142,998 [-54.7%])</span> <div style='text-align: right'>1,775,904</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | <span style="color: green">(-794,177 [-50.3%])</span> <div style='text-align: right'>784,892</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcAddNe | <span style="color: green">(-128,300 [-50.3%])</span> <div style='text-align: right'>126,975</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcAddNe | <span style="color: green">(-105,206 [-50.3%])</span> <div style='text-align: right'>104,140</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcAddNe | <span style="color: green">(-174,488 [-50.3%])</span> <div style='text-align: right'>172,618</div>  |
| ecrecover_program | Boundary |  | EcAddNe | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | EcAddNe | <div style='text-align: right'>192</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcDouble | <span style="color: green">(-694,497 [-50.0%])</span> <div style='text-align: right'>693,411</div>  |
| ecrecover_program | AccessAdapter<16> |  | EcDouble | <span style="color: green">(-63,950 [-50.1%])</span> <div style='text-align: right'>63,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | EcDouble | <span style="color: green">(-52,439 [-50.1%])</span> <div style='text-align: right'>52,316</div>  |
| ecrecover_program | AccessAdapter<8> |  | EcDouble | <span style="color: green">(-86,972 [-50.1%])</span> <div style='text-align: right'>86,768</div>  |
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <span style="color: red">(+1,040 [+23.0%])</span> <div style='text-align: right'>5,564</div>  |
| ecrecover_program | AccessAdapter<16> |  | HINT_STOREW | <div style='text-align: right'>250</div>  |
| ecrecover_program | AccessAdapter<32> |  | HINT_STOREW | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | HINT_STOREW | <span style="color: red">(+340 [+22.5%])</span> <div style='text-align: right'>1,853</div>  |
| ecrecover_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>3,560</div>  |
| ecrecover_program | Merkle |  | HINT_STOREW | <span style="color: red">(+576 [+9.5%])</span> <div style='text-align: right'>6,656</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | <span style="color: green">(-1,331,652 [-50.0%])</span> <div style='text-align: right'>1,332,482</div>  |
| ecrecover_program | AccessAdapter<16> |  | IS_EQ | <span style="color: green">(-336,450 [-49.8%])</span> <div style='text-align: right'>338,800</div>  |
| ecrecover_program | AccessAdapter<32> |  | IS_EQ | <span style="color: green">(-275,889 [-49.8%])</span> <div style='text-align: right'>277,816</div>  |
| ecrecover_program | AccessAdapter<8> |  | IS_EQ | <span style="color: green">(-457,572 [-49.8%])</span> <div style='text-align: right'>460,700</div>  |
| ecrecover_program | Boundary |  | IS_EQ | <div style='text-align: right'>160</div>  |
| ecrecover_program | Merkle |  | IS_EQ | <span style="color: red">(+256 [+57.1%])</span> <div style='text-align: right'>704</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <span style="color: green">(-143,334 [-49.7%])</span> <div style='text-align: right'>145,116</div>  |
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <span style="color: green">(-1,136,072 [-58.2%])</span> <div style='text-align: right'>816,088</div>  |
| ecrecover_program | KeccakVmAir |  | KECCAK256 | <div style='text-align: right'>379,680</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | <span style="color: green">(-1,312,815 [-50.2%])</span> <div style='text-align: right'>1,301,055</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <span style="color: green">(-321,920 [-60.8%])</span> <div style='text-align: right'>207,280</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADBU | <div style='text-align: right'>125</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADBU | <div style='text-align: right'>205</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADBU | <span style="color: green">(-221 [-31.7%])</span> <div style='text-align: right'>476</div>  |
| ecrecover_program | Boundary |  | LOADBU | <span style="color: green">(-720 [-43.9%])</span> <div style='text-align: right'>920</div>  |
| ecrecover_program | Merkle |  | LOADBU | <span style="color: green">(-1,536 [-60.0%])</span> <div style='text-align: right'>1,024</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <span style="color: green">(-13,176,200 [-61.0%])</span> <div style='text-align: right'>8,440,360</div>  |
| ecrecover_program | AccessAdapter<16> |  | LOADW | <span style="color: green">(-319,825 [-49.7%])</span> <div style='text-align: right'>323,525</div>  |
| ecrecover_program | AccessAdapter<32> |  | LOADW | <span style="color: green">(-262,359 [-49.7%])</span> <div style='text-align: right'>265,188</div>  |
| ecrecover_program | AccessAdapter<8> |  | LOADW | <span style="color: green">(-494,513 [-49.6%])</span> <div style='text-align: right'>503,115</div>  |
| ecrecover_program | Boundary |  | LOADW | <span style="color: green">(-140,320 [-48.6%])</span> <div style='text-align: right'>148,320</div>  |
| ecrecover_program | Merkle |  | LOADW | <span style="color: green">(-187,904 [-48.8%])</span> <div style='text-align: right'>197,056</div>  |
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <span style="color: green">(-176,400 [-59.6%])</span> <div style='text-align: right'>119,610</div>  |
| ecrecover_program | AccessAdapter<8> |  | LUI | <div style='text-align: right'>17</div>  |
| ecrecover_program | Boundary |  | LUI | <div style='text-align: right'>40</div>  |
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | <span style="color: green">(-6,010,125 [-99.3%])</span> <div style='text-align: right'>40,021</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | <span style="color: green">(-129,151 [-50.2%])</span> <div style='text-align: right'>127,957</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularAddSub | <span style="color: green">(-64,900 [-50.2%])</span> <div style='text-align: right'>64,300</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularAddSub | <span style="color: green">(-53,218 [-50.2%])</span> <div style='text-align: right'>52,726</div>  |
| ecrecover_program | AccessAdapter<4> |  | ModularAddSub | <div style='text-align: right'>221</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularAddSub | <span style="color: green">(-88,264 [-50.2%])</span> <div style='text-align: right'>87,482</div>  |
| ecrecover_program | Boundary |  | ModularAddSub | <div style='text-align: right'>720</div>  |
| ecrecover_program | Merkle |  | ModularAddSub | <span style="color: green">(-128 [-4.8%])</span> <div style='text-align: right'>2,560</div>  |
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | <div style='text-align: right'>7,047</div>  |
| ecrecover_program | AccessAdapter<16> |  | ModularMulDiv | <span style="color: red">(+250 [+14.3%])</span> <div style='text-align: right'>2,000</div>  |
| ecrecover_program | AccessAdapter<32> |  | ModularMulDiv | <span style="color: red">(+205 [+14.3%])</span> <div style='text-align: right'>1,640</div>  |
| ecrecover_program | AccessAdapter<8> |  | ModularMulDiv | <span style="color: red">(+340 [+14.3%])</span> <div style='text-align: right'>2,720</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <span style="color: green">(-5,522,436 [-77.0%])</span> <div style='text-align: right'>1,652,004</div>  |
| ecrecover_program | PhantomAir |  | PHANTOM | <span style="color: green">(-8,100 [-50.5%])</span> <div style='text-align: right'>7,950</div>  |
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | <div style='text-align: right'>332</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <span style="color: green">(-11,310,995 [-85.5%])</span> <div style='text-align: right'>1,916,162</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <span style="color: green">(-10,109,473 [-84.3%])</span> <div style='text-align: right'>1,885,298</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | <span style="color: green">(-67,734 [-49.9%])</span> <div style='text-align: right'>68,052</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <span style="color: green">(-11,911,909 [-85.2%])</span> <div style='text-align: right'>2,075,321</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <span style="color: green">(-2,365,160 [-51.2%])</span> <div style='text-align: right'>2,256,080</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREB | <span style="color: green">(-53,350 [-49.8%])</span> <div style='text-align: right'>53,825</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREB | <span style="color: green">(-87,658 [-49.9%])</span> <div style='text-align: right'>88,109</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREB | <span style="color: green">(-78,540 [-50.9%])</span> <div style='text-align: right'>75,667</div>  |
| ecrecover_program | Boundary |  | STOREB | <span style="color: green">(-99,760 [-52.1%])</span> <div style='text-align: right'>91,600</div>  |
| ecrecover_program | Merkle |  | STOREB | <span style="color: green">(-264,832 [-51.3%])</span> <div style='text-align: right'>251,264</div>  |
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <span style="color: green">(-13,841,400 [-59.3%])</span> <div style='text-align: right'>9,500,800</div>  |
| ecrecover_program | AccessAdapter<16> |  | STOREW | <span style="color: green">(-220,450 [-50.3%])</span> <div style='text-align: right'>217,500</div>  |
| ecrecover_program | AccessAdapter<32> |  | STOREW | <span style="color: green">(-136,858 [-50.5%])</span> <div style='text-align: right'>134,398</div>  |
| ecrecover_program | AccessAdapter<8> |  | STOREW | <span style="color: green">(-504,424 [-50.4%])</span> <div style='text-align: right'>496,621</div>  |
| ecrecover_program | Boundary |  | STOREW | <span style="color: green">(-395,760 [-50.6%])</span> <div style='text-align: right'>386,760</div>  |
| ecrecover_program | Merkle |  | STOREW | <span style="color: green">(-566,336 [-50.3%])</span> <div style='text-align: right'>558,912</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <span style="color: green">(-321,084 [-51.2%])</span> <div style='text-align: right'>306,072</div>  |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <span style="color: green">(-144,864 [-86.0%])</span> <div style='text-align: right'>23,616</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | ProgramAir | 0 | <div style='text-align: right'>294,912</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| ecrecover_program | PersistentBoundaryAir<8> | 0 | <span style="color: green">(-1,048,576 [-50.0%])</span> <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | MemoryMerkleAir<8> | 0 | <span style="color: green">(-1,703,936 [-50.0%])</span> <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | AccessAdapterAir<4> | 0 | <span style="color: green">(-16,576 [-87.5%])</span> <div style='text-align: right'>2,368</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-448 [-87.5%])</span> <div style='text-align: right'>64</div>  |
| ecrecover_program | AccessAdapterAir<8> | 0 | <span style="color: green">(-5,373,952 [-50.0%])</span> <div style='text-align: right'>5,373,952</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | AccessAdapterAir<16> | 0 | <span style="color: green">(-3,211,264 [-50.0%])</span> <div style='text-align: right'>3,211,264</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-65,536 [-50.0%])</span> <div style='text-align: right'>65,536</div>  |
| ecrecover_program | AccessAdapterAir<32> | 0 | <span style="color: green">(-2,129,920 [-50.0%])</span> <div style='text-align: right'>2,129,920</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-32,768 [-50.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <span style="color: green">(-2,807,808 [-50.0%])</span> <div style='text-align: right'>2,807,808</div>  | <div style='text-align: right'>543</div>  | <div style='text-align: right'>828</div>  |  | <span style="color: green">(-2,048 [-50.0%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | <span style="color: green">(-3,004,416 [-50.0%])</span> <div style='text-align: right'>3,004,416</div>  | <div style='text-align: right'>619</div>  | <div style='text-align: right'>848</div>  |  | <span style="color: green">(-2,048 [-50.0%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | KeccakVmAir | 0 | <div style='text-align: right'>569,856</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>128</div>  |
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | <span style="color: green">(-1,818,624 [-50.0%])</span> <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>166</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: green">(-8,192 [-50.0%])</span> <div style='text-align: right'>8,192</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | <span style="color: green">(-9,232 [-50.0%])</span> <div style='text-align: right'>9,232</div>  | <div style='text-align: right'>261</div>  | <div style='text-align: right'>316</div>  |  | <span style="color: green">(-16 [-50.0%])</span> <div style='text-align: right'>16</div>  |
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | <span style="color: green">(-400,384 [-50.0%])</span> <div style='text-align: right'>400,384</div>  | <div style='text-align: right'>199</div>  | <div style='text-align: right'>192</div>  |  | <span style="color: green">(-1,024 [-50.0%])</span> <div style='text-align: right'>1,024</div>  |
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>15,872</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>256</div>  |
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <span style="color: green">(-28,870,656 [-99.2%])</span> <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-260,096 [-99.2%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <span style="color: green">(-2,408,448 [-75.0%])</span> <div style='text-align: right'>802,816</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <span style="color: green">(-49,152 [-75.0%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: green">(-6,291,456 [-75.0%])</span> <div style='text-align: right'>2,097,152</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <span style="color: green">(-98,304 [-75.0%])</span> <div style='text-align: right'>32,768</div>  |
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <span style="color: green">(-1,015,808 [-50.0%])</span> <div style='text-align: right'>1,015,808</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <span style="color: green">(-16,384 [-50.0%])</span> <div style='text-align: right'>16,384</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <span style="color: green">(-11,534,336 [-50.0%])</span> <div style='text-align: right'>11,534,336</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <span style="color: green">(-9,699,328 [-50.0%])</span> <div style='text-align: right'>9,699,328</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <span style="color: green">(-7,274,496 [-50.0%])</span> <div style='text-align: right'>7,274,496</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <span style="color: green">(-65,536 [-50.0%])</span> <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: green">(-176,160,768 [-75.0%])</span> <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <span style="color: green">(-1,572,864 [-75.0%])</span> <div style='text-align: right'>524,288</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-41,287,680 [-75.0%])</span> <div style='text-align: right'>13,762,560</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: green">(-393,216 [-75.0%])</span> <div style='text-align: right'>131,072</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <span style="color: green">(-35,323,904 [-87.5%])</span> <div style='text-align: right'>5,046,272</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <span style="color: green">(-458,752 [-87.5%])</span> <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <span style="color: green">(-121,634,816 [-50.0%])</span> <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-1,048,576 [-50.0%])</span> <div style='text-align: right'>1,048,576</div>  |
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| ecrecover_program | PhantomAir | 0 | <span style="color: green">(-36,864 [-50.0%])</span> <div style='text-align: right'>36,864</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-2,048 [-50.0%])</span> <div style='text-align: right'>2,048</div>  |
| ecrecover_program | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-41,091,072 [-50.0%])</span> <div style='text-align: right'>41,091,072</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <span style="color: green">(-65,536 [-50.0%])</span> <div style='text-align: right'>65,536</div>  |
| ecrecover_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| ecrecover_program | 0 | <span style="color: green">(-1,157.0 [-59.5%])</span> <div style='text-align: right'>789.0</div>  | <span style="color: green">(-10,142.0 [-58.8%])</span> <div style='text-align: right'>7,113.0</div>  | <span style="color: green">(-540,623,610 [-64.1%])</span> <div style='text-align: right'>303,080,679</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/062a4d976dd51220f169bea42f6bcfa4af9e0c1c/ecrecover-2-2-64cpu-linux-arm64-mimalloc-ecrecover_program.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/062a4d976dd51220f169bea42f6bcfa4af9e0c1c

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12229722409)
