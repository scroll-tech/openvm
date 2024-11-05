| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| revm_contract_deployment | <span style="color: red">(+2,921 [+0.0%])</span> <div style='text-align: right'>76,372,298</div>  | <span style="color: red">(+179 [+0.0%])</span> <div style='text-align: right'>1,879,512</div>  | <span style="color: red">(+60.0 [+0.4%])</span> <div style='text-align: right'>14,489.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true | <span style="color: red">(+319.0 [+0.7%])</span> <div style='text-align: right'>47,846.0</div>  | <span style="color: red">(+2,921 [+0.0%])</span> <div style='text-align: right'>76,372,298</div>  | <span style="color: red">(+179 [+0.0%])</span> <div style='text-align: right'>1,879,512</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_contract_deployment | BitwiseOperationLookup | true | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | Memory AccessAdapter<8> | true | <span style="color: green">(-13 [-0.1%])</span> <div style='text-align: right'>11,553</div>  |
| revm_contract_deployment | Memory Boundary | true | <span style="color: green">(-26 [-0.1%])</span> <div style='text-align: right'>23,106</div>  |
| revm_contract_deployment | Memory Merkle | true | <span style="color: green">(-92 [-0.4%])</span> <div style='text-align: right'>24,346</div>  |
| revm_contract_deployment | ProgramChip | true | <div style='text-align: right'>250,207</div>  |
| revm_contract_deployment | RangeTupleChecker | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true |  | ADD | <span style="color: red">(+43 [+0.0%])</span> <div style='text-align: right'>151,742</div>  |
| revm_contract_deployment | true |  | AND | <span style="color: red">(+23 [+0.0%])</span> <div style='text-align: right'>116,919</div>  |
| revm_contract_deployment | true |  | AUIPC | <span style="color: red">(+2 [+0.2%])</span> <div style='text-align: right'>963</div>  |
| revm_contract_deployment | true |  | BEQ | <span style="color: red">(+15 [+0.1%])</span> <div style='text-align: right'>13,746</div>  |
| revm_contract_deployment | true |  | BGE | <div style='text-align: right'>10,842</div>  |
| revm_contract_deployment | true |  | BGEU | <div style='text-align: right'>11,680</div>  |
| revm_contract_deployment | true |  | BLT | <div style='text-align: right'>151</div>  |
| revm_contract_deployment | true |  | BLTU | <div style='text-align: right'>6,902</div>  |
| revm_contract_deployment | true |  | BNE | <span style="color: red">(+3 [+0.0%])</span> <div style='text-align: right'>34,377</div>  |
| revm_contract_deployment | true |  | HINT_STOREW | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | true |  | JAL | <span style="color: red">(+4 [+0.1%])</span> <div style='text-align: right'>3,304</div>  |
| revm_contract_deployment | true |  | JALR | <span style="color: red">(+3 [+0.1%])</span> <div style='text-align: right'>2,099</div>  |
| revm_contract_deployment | true |  | LOADB | <div style='text-align: right'>11,806</div>  |
| revm_contract_deployment | true |  | LOADBU | <span style="color: red">(+19 [+0.0%])</span> <div style='text-align: right'>44,493</div>  |
| revm_contract_deployment | true |  | LOADH | <div style='text-align: right'>5</div>  |
| revm_contract_deployment | true |  | LOADHU | <div style='text-align: right'>20</div>  |
| revm_contract_deployment | true |  | LOADW | <span style="color: red">(+15 [+0.0%])</span> <div style='text-align: right'>316,656</div>  |
| revm_contract_deployment | true |  | LUI | <div style='text-align: right'>3,973</div>  |
| revm_contract_deployment | true |  | MUL | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>2,119</div>  |
| revm_contract_deployment | true |  | MULH | <div style='text-align: right'>3</div>  |
| revm_contract_deployment | true |  | MULHU | <div style='text-align: right'>1,234</div>  |
| revm_contract_deployment | true |  | OR | <span style="color: red">(+12 [+0.0%])</span> <div style='text-align: right'>133,829</div>  |
| revm_contract_deployment | true |  | PHANTOM | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | true |  | SLL | <span style="color: red">(+16 [+0.0%])</span> <div style='text-align: right'>133,764</div>  |
| revm_contract_deployment | true |  | SLT | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | SLTU | <div style='text-align: right'>14,115</div>  |
| revm_contract_deployment | true |  | SRA | <div style='text-align: right'>493</div>  |
| revm_contract_deployment | true |  | SRL | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>130,632</div>  |
| revm_contract_deployment | true |  | STOREB | <div style='text-align: right'>13,828</div>  |
| revm_contract_deployment | true |  | STOREH | <div style='text-align: right'>14</div>  |
| revm_contract_deployment | true |  | STOREW | <span style="color: red">(+9 [+0.0%])</span> <div style='text-align: right'>258,400</div>  |
| revm_contract_deployment | true |  | SUB | <span style="color: red">(+3 [+0.1%])</span> <div style='text-align: right'>2,249</div>  |
| revm_contract_deployment | true |  | XOR | <span style="color: red">(+8 [+0.0%])</span> <div style='text-align: right'>455,946</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: red">(+1,548 [+0.0%])</span> <div style='text-align: right'>5,462,712</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: red">(+828 [+0.0%])</span> <div style='text-align: right'>4,209,084</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: red">(+42 [+0.2%])</span> <div style='text-align: right'>20,223</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: red">(+390 [+0.1%])</span> <div style='text-align: right'>357,396</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>346,944</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>373,760</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,832</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>220,864</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: red">(+78 [+0.0%])</span> <div style='text-align: right'>893,802</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>79,794</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>26,095</div>  |
| revm_contract_deployment | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>61,400</div>  |
| revm_contract_deployment | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>98,688</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: red">(+72 [+0.1%])</span> <div style='text-align: right'>59,472</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: red">(+84 [+0.1%])</span> <div style='text-align: right'>58,772</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>413,210</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: red">(+760 [+0.0%])</span> <div style='text-align: right'>1,779,720</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>323</div>  |
| revm_contract_deployment | Boundary | true |  | LOADBU | <div style='text-align: right'>760</div>  |
| revm_contract_deployment | Merkle | true |  | LOADBU | <span style="color: green">(-128 [-3.4%])</span> <div style='text-align: right'>3,648</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>800</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADHU | <div style='text-align: right'>85</div>  |
| revm_contract_deployment | Boundary | true |  | LOADHU | <div style='text-align: right'>200</div>  |
| revm_contract_deployment | Merkle | true |  | LOADHU | <div style='text-align: right'>384</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: red">(+600 [+0.0%])</span> <div style='text-align: right'>12,666,240</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-17 [-0.1%])</span> <div style='text-align: right'>20,978</div>  |
| revm_contract_deployment | Boundary | true |  | LOADW | <span style="color: green">(-40 [-0.1%])</span> <div style='text-align: right'>49,360</div>  |
| revm_contract_deployment | Merkle | true |  | LOADW | <span style="color: green">(-128 [-0.1%])</span> <div style='text-align: right'>92,288</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>71,514</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: red">(+62 [+0.1%])</span> <div style='text-align: right'>65,689</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>51</div>  |
| revm_contract_deployment | Boundary | true |  | MUL | <div style='text-align: right'>120</div>  |
| revm_contract_deployment | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULH | <div style='text-align: right'>117</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>48,126</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_contract_deployment | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_contract_deployment | Merkle | true |  | MULHU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: red">(+432 [+0.0%])</span> <div style='text-align: right'>4,817,844</div>  |
| revm_contract_deployment | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>822</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: red">(+848 [+0.0%])</span> <div style='text-align: right'>7,089,492</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>74</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>522,255</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>26,129</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: red">(+106 [+0.0%])</span> <div style='text-align: right'>6,923,496</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>553,120</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,023</div>  |
| revm_contract_deployment | Boundary | true |  | STOREB | <div style='text-align: right'>4,760</div>  |
| revm_contract_deployment | Merkle | true |  | STOREB | <span style="color: green">(-192 [-1.0%])</span> <div style='text-align: right'>18,176</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>560</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: red">(+360 [+0.0%])</span> <div style='text-align: right'>10,336,000</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREW | <span style="color: green">(-204 [-0.1%])</span> <div style='text-align: right'>146,727</div>  |
| revm_contract_deployment | Boundary | true |  | STOREW | <span style="color: green">(-480 [-0.1%])</span> <div style='text-align: right'>345,240</div>  |
| revm_contract_deployment | Merkle | true |  | STOREW | <span style="color: green">(-2,496 [-0.4%])</span> <div style='text-align: right'>562,176</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: red">(+108 [+0.1%])</span> <div style='text-align: right'>80,964</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: red">(+288 [+0.0%])</span> <div style='text-align: right'>16,414,056</div>  |

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| revm_contract_deployment | VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| revm_contract_deployment | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <span style="color: green">(-1.0 [-1.2%])</span> <div style='text-align: right'>85.0</div>  | <div style='text-align: right'>4,583.0</div>  | <div style='text-align: right'>3,803.0</div>  | <span style="color: green">(-1.0 [-0.9%])</span> <div style='text-align: right'>107.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+2,921 [+0.0%])</span> <div style='text-align: right'>76,372,298</div>  | <span style="color: red">(+179 [+0.0%])</span> <div style='text-align: right'>1,879,512</div>  | <span style="color: red">(+60.0 [+0.4%])</span> <div style='text-align: right'>14,489.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | ProgramAir | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| revm_contract_deployment | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,343,488</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | PhantomAir | 0 | <div style='text-align: right'>4,608</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>256</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,261,568</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>55,050,240</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>524,288</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>117,440,512</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>50,176</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,024</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>284,672</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>2,048</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>253,952</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | Poseidon2VmAir<BabyBear> | 0 | <div style='text-align: right'>41,091,072</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| revm_contract_deployment | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| revm_contract_deployment | 0 | <span style="color: red">(+60.0 [+0.6%])</span> <div style='text-align: right'>9,906.0</div>  | <div style='text-align: right'>363,216,416</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f3e8226cba2bb2627510e9675f3d25c7808b4f35/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/f3e8226cba2bb2627510e9675f3d25c7808b4f35
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11679526544)
