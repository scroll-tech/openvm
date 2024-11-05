| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| revm_contract_deployment | <span style="color: red">(+29,643 [+0.0%])</span> <div style='text-align: right'>76,389,673</div>  | <span style="color: red">(+767 [+0.0%])</span> <div style='text-align: right'>1,879,958</div>  | <span style="color: red">(+53.0 [+0.4%])</span> <div style='text-align: right'>14,569.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true | <span style="color: red">(+1,121.0 [+2.3%])</span> <div style='text-align: right'>50,052.0</div>  | <span style="color: red">(+29,643 [+0.0%])</span> <div style='text-align: right'>76,389,673</div>  | <span style="color: red">(+767 [+0.0%])</span> <div style='text-align: right'>1,879,958</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: red">(+367 [+0.0%])</span> <div style='text-align: right'>860,885</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>14,119</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: red">(+39 [+0.0%])</span> <div style='text-align: right'>264,894</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: red">(+154 [+0.3%])</span> <div style='text-align: right'>48,245</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>29,575</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: red">(+10 [+0.1%])</span> <div style='text-align: right'>7,279</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: red">(+9 [+0.4%])</span> <div style='text-align: right'>2,105</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>11,811</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: red">(+175 [+0.0%])</span> <div style='text-align: right'>633,513</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>1,237</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: red">(+5 [+0.2%])</span> <div style='text-align: right'>2,122</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: red">(+6 [+0.6%])</span> <div style='text-align: right'>967</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | Memory AccessAdapter<8> | true | <span style="color: red">(+7 [+0.1%])</span> <div style='text-align: right'>11,558</div>  |
| revm_contract_deployment | Memory Boundary | true | <span style="color: red">(+14 [+0.1%])</span> <div style='text-align: right'>23,116</div>  |
| revm_contract_deployment | Memory Merkle | true | <span style="color: red">(+68 [+0.3%])</span> <div style='text-align: right'>24,406</div>  |
| revm_contract_deployment | PhantomAir | true | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | ProgramChip | true | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>250,205</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true |  | ADD | <span style="color: red">(+269 [+0.2%])</span> <div style='text-align: right'>151,930</div>  |
| revm_contract_deployment | true |  | AND | <span style="color: red">(+51 [+0.0%])</span> <div style='text-align: right'>116,927</div>  |
| revm_contract_deployment | true |  | AUIPC | <span style="color: red">(+6 [+0.6%])</span> <div style='text-align: right'>967</div>  |
| revm_contract_deployment | true |  | BEQ | <span style="color: red">(+33 [+0.2%])</span> <div style='text-align: right'>13,751</div>  |
| revm_contract_deployment | true |  | BGE | <div style='text-align: right'>10,842</div>  |
| revm_contract_deployment | true |  | BGEU | <div style='text-align: right'>11,680</div>  |
| revm_contract_deployment | true |  | BLT | <div style='text-align: right'>151</div>  |
| revm_contract_deployment | true |  | BLTU | <div style='text-align: right'>6,902</div>  |
| revm_contract_deployment | true |  | BNE | <span style="color: red">(+121 [+0.4%])</span> <div style='text-align: right'>34,494</div>  |
| revm_contract_deployment | true |  | HINT_STOREW | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | true |  | JAL | <span style="color: red">(+8 [+0.2%])</span> <div style='text-align: right'>3,304</div>  |
| revm_contract_deployment | true |  | JALR | <span style="color: red">(+9 [+0.4%])</span> <div style='text-align: right'>2,105</div>  |
| revm_contract_deployment | true |  | LOADB | <div style='text-align: right'>11,806</div>  |
| revm_contract_deployment | true |  | LOADBU | <span style="color: red">(+155 [+0.3%])</span> <div style='text-align: right'>44,613</div>  |
| revm_contract_deployment | true |  | LOADH | <div style='text-align: right'>5</div>  |
| revm_contract_deployment | true |  | LOADHU | <div style='text-align: right'>20</div>  |
| revm_contract_deployment | true |  | LOADW | <span style="color: red">(+19 [+0.0%])</span> <div style='text-align: right'>316,649</div>  |
| revm_contract_deployment | true |  | LUI | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>3,975</div>  |
| revm_contract_deployment | true |  | MUL | <span style="color: red">(+5 [+0.2%])</span> <div style='text-align: right'>2,122</div>  |
| revm_contract_deployment | true |  | MULH | <div style='text-align: right'>3</div>  |
| revm_contract_deployment | true |  | MULHU | <div style='text-align: right'>1,234</div>  |
| revm_contract_deployment | true |  | OR | <span style="color: red">(+24 [+0.0%])</span> <div style='text-align: right'>133,829</div>  |
| revm_contract_deployment | true |  | PHANTOM | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | true |  | SLL | <span style="color: red">(+33 [+0.0%])</span> <div style='text-align: right'>133,765</div>  |
| revm_contract_deployment | true |  | SLT | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | SLTU | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>14,117</div>  |
| revm_contract_deployment | true |  | SRA | <div style='text-align: right'>493</div>  |
| revm_contract_deployment | true |  | SRL | <span style="color: red">(+6 [+0.0%])</span> <div style='text-align: right'>130,636</div>  |
| revm_contract_deployment | true |  | STOREB | <div style='text-align: right'>13,828</div>  |
| revm_contract_deployment | true |  | STOREH | <div style='text-align: right'>14</div>  |
| revm_contract_deployment | true |  | STOREW | <span style="color: red">(+1 [+0.0%])</span> <div style='text-align: right'>258,389</div>  |
| revm_contract_deployment | true |  | SUB | <span style="color: red">(+9 [+0.4%])</span> <div style='text-align: right'>2,255</div>  |
| revm_contract_deployment | true |  | XOR | <span style="color: red">(+14 [+0.0%])</span> <div style='text-align: right'>455,944</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: red">(+9,684 [+0.2%])</span> <div style='text-align: right'>5,469,480</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: red">(+1,836 [+0.0%])</span> <div style='text-align: right'>4,209,372</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: red">(+126 [+0.6%])</span> <div style='text-align: right'>20,307</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: red">(+858 [+0.2%])</span> <div style='text-align: right'>357,526</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>346,944</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>373,760</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,832</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>220,864</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: red">(+3,146 [+0.4%])</span> <div style='text-align: right'>896,844</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>79,794</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>26,095</div>  |
| revm_contract_deployment | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>61,400</div>  |
| revm_contract_deployment | Merkle | true |  | HINT_STOREW | <span style="color: red">(+128 [+0.1%])</span> <div style='text-align: right'>98,816</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: red">(+144 [+0.2%])</span> <div style='text-align: right'>59,472</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: red">(+252 [+0.4%])</span> <div style='text-align: right'>58,940</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>413,210</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: red">(+6,200 [+0.3%])</span> <div style='text-align: right'>1,784,520</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>323</div>  |
| revm_contract_deployment | Boundary | true |  | LOADBU | <div style='text-align: right'>760</div>  |
| revm_contract_deployment | Merkle | true |  | LOADBU | <span style="color: green">(-192 [-5.3%])</span> <div style='text-align: right'>3,456</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>800</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADHU | <div style='text-align: right'>85</div>  |
| revm_contract_deployment | Boundary | true |  | LOADHU | <div style='text-align: right'>200</div>  |
| revm_contract_deployment | Merkle | true |  | LOADHU | <span style="color: green">(-192 [-50.0%])</span> <div style='text-align: right'>192</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: red">(+760 [+0.0%])</span> <div style='text-align: right'>12,665,960</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>20,978</div>  |
| revm_contract_deployment | Boundary | true |  | LOADW | <div style='text-align: right'>49,360</div>  |
| revm_contract_deployment | Merkle | true |  | LOADW | <span style="color: green">(-320 [-0.3%])</span> <div style='text-align: right'>91,456</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <span style="color: red">(+36 [+0.1%])</span> <div style='text-align: right'>71,550</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: red">(+155 [+0.2%])</span> <div style='text-align: right'>65,782</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>51</div>  |
| revm_contract_deployment | Boundary | true |  | MUL | <div style='text-align: right'>120</div>  |
| revm_contract_deployment | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULH | <div style='text-align: right'>117</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>48,126</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_contract_deployment | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_contract_deployment | Merkle | true |  | MULHU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: red">(+864 [+0.0%])</span> <div style='text-align: right'>4,817,844</div>  |
| revm_contract_deployment | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>822</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: red">(+1,749 [+0.0%])</span> <div style='text-align: right'>7,089,545</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>74</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: red">(+74 [+0.0%])</span> <div style='text-align: right'>522,329</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>26,129</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: red">(+318 [+0.0%])</span> <div style='text-align: right'>6,923,708</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>553,120</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,023</div>  |
| revm_contract_deployment | Boundary | true |  | STOREB | <div style='text-align: right'>4,760</div>  |
| revm_contract_deployment | Merkle | true |  | STOREB | <span style="color: red">(+1,792 [+9.8%])</span> <div style='text-align: right'>20,096</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>560</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: red">(+40 [+0.0%])</span> <div style='text-align: right'>10,335,560</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+119 [+0.1%])</span> <div style='text-align: right'>146,812</div>  |
| revm_contract_deployment | Boundary | true |  | STOREW | <span style="color: red">(+280 [+0.1%])</span> <div style='text-align: right'>345,440</div>  |
| revm_contract_deployment | Merkle | true |  | STOREW | <span style="color: red">(+960 [+0.2%])</span> <div style='text-align: right'>563,264</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: red">(+324 [+0.4%])</span> <div style='text-align: right'>81,180</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: red">(+504 [+0.0%])</span> <div style='text-align: right'>16,413,984</div>  |

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
| revm_contract_deployment | <span style="color: green">(-3.0 [-3.4%])</span> <div style='text-align: right'>85.0</div>  | <span style="color: red">(+32.0 [+0.7%])</span> <div style='text-align: right'>4,673.0</div>  | <span style="color: red">(+22.0 [+0.6%])</span> <div style='text-align: right'>3,866.0</div>  | <span style="color: green">(-3.0 [-2.7%])</span> <div style='text-align: right'>108.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+29,643 [+0.0%])</span> <div style='text-align: right'>76,389,673</div>  | <span style="color: red">(+767 [+0.0%])</span> <div style='text-align: right'>1,879,958</div>  | <span style="color: red">(+53.0 [+0.4%])</span> <div style='text-align: right'>14,569.0</div>  |

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
| revm_contract_deployment | 0 | <span style="color: red">(+21.0 [+0.2%])</span> <div style='text-align: right'>9,896.0</div>  | <div style='text-align: right'>363,216,416</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/64fa08bc809386a4f6ba9de679c3d68a342af0a9/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/64fa08bc809386a4f6ba9de679c3d68a342af0a9
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11691040106)
