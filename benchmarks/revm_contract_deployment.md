| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| revm_contract_deployment | <span style="color: green">(-55,858,569 [-73.1%])</span> <div style='text-align: right'>20,515,733</div>  | <span style="color: green">(-1,568,984 [-83.5%])</span> <div style='text-align: right'>310,284</div>  | <span style="color: green">(-6,516.0 [-57.4%])</span> <div style='text-align: right'>4,841.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true | <span style="color: green">(-39,463.0 [-80.6%])</span> <div style='text-align: right'>9,487.0</div>  | <span style="color: green">(-55,858,569 [-73.1%])</span> <div style='text-align: right'>20,515,733</div>  | <span style="color: green">(-1,568,984 [-83.5%])</span> <div style='text-align: right'>310,284</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-749,807 [-87.1%])</span> <div style='text-align: right'>110,749</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <span style="color: green">(-8 [-0.1%])</span> <div style='text-align: right'>14,109</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: green">(-253,326 [-95.6%])</span> <div style='text-align: right'>11,533</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-14,441 [-30.0%])</span> <div style='text-align: right'>33,654</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <span style="color: green">(-519 [-1.8%])</span> <div style='text-align: right'>29,056</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: green">(-2,364 [-32.5%])</span> <div style='text-align: right'>4,911</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: green">(-201 [-9.6%])</span> <div style='text-align: right'>1,895</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>11,811</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-548,218 [-86.6%])</span> <div style='text-align: right'>85,145</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>1,237</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: green">(-2 [-0.1%])</span> <div style='text-align: right'>2,115</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: green">(-100 [-10.4%])</span> <div style='text-align: right'>861</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | KeccakVmAir | true | <div style='text-align: right'>2,184</div>  |
| revm_contract_deployment | Memory AccessAdapter<8> | true | <span style="color: red">(+195 [+1.7%])</span> <div style='text-align: right'>11,767</div>  |
| revm_contract_deployment | Memory Boundary | true | <span style="color: red">(+390 [+1.7%])</span> <div style='text-align: right'>23,534</div>  |
| revm_contract_deployment | Memory Merkle | true | <span style="color: red">(+364 [+1.5%])</span> <div style='text-align: right'>24,818</div>  |
| revm_contract_deployment | PhantomAir | true | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | ProgramChip | true | <span style="color: green">(-746 [-0.3%])</span> <div style='text-align: right'>256,032</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true |  | ADD | <span style="color: green">(-60,337 [-39.8%])</span> <div style='text-align: right'>91,352</div>  |
| revm_contract_deployment | true |  | AND | <span style="color: green">(-109,213 [-93.4%])</span> <div style='text-align: right'>7,668</div>  |
| revm_contract_deployment | true |  | AUIPC | <span style="color: green">(-100 [-10.4%])</span> <div style='text-align: right'>861</div>  |
| revm_contract_deployment | true |  | BEQ | <span style="color: green">(-95 [-0.7%])</span> <div style='text-align: right'>13,627</div>  |
| revm_contract_deployment | true |  | BGE | <div style='text-align: right'>10,842</div>  |
| revm_contract_deployment | true |  | BGEU | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>11,676</div>  |
| revm_contract_deployment | true |  | BLT | <div style='text-align: right'>151</div>  |
| revm_contract_deployment | true |  | BLTU | <span style="color: green">(-515 [-7.5%])</span> <div style='text-align: right'>6,387</div>  |
| revm_contract_deployment | true |  | BNE | <span style="color: green">(-14,346 [-41.7%])</span> <div style='text-align: right'>20,027</div>  |
| revm_contract_deployment | true |  | HINT_STOREW | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | true |  | JAL | <span style="color: green">(-93 [-2.8%])</span> <div style='text-align: right'>3,204</div>  |
| revm_contract_deployment | true |  | JALR | <span style="color: green">(-201 [-9.6%])</span> <div style='text-align: right'>1,895</div>  |
| revm_contract_deployment | true |  | KECCAK256 | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | LOADB | <div style='text-align: right'>11,806</div>  |
| revm_contract_deployment | true |  | LOADBU | <span style="color: green">(-24,282 [-54.6%])</span> <div style='text-align: right'>20,180</div>  |
| revm_contract_deployment | true |  | LOADH | <div style='text-align: right'>5</div>  |
| revm_contract_deployment | true |  | LOADHU | <div style='text-align: right'>20</div>  |
| revm_contract_deployment | true |  | LOADW | <span style="color: green">(-286,447 [-90.5%])</span> <div style='text-align: right'>30,191</div>  |
| revm_contract_deployment | true |  | LUI | <span style="color: green">(-2,271 [-57.1%])</span> <div style='text-align: right'>1,707</div>  |
| revm_contract_deployment | true |  | MUL | <span style="color: green">(-2 [-0.1%])</span> <div style='text-align: right'>2,115</div>  |
| revm_contract_deployment | true |  | MULH | <div style='text-align: right'>3</div>  |
| revm_contract_deployment | true |  | MULHU | <div style='text-align: right'>1,234</div>  |
| revm_contract_deployment | true |  | OR | <span style="color: green">(-126,663 [-94.7%])</span> <div style='text-align: right'>7,145</div>  |
| revm_contract_deployment | true |  | PHANTOM | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | true |  | SLL | <span style="color: green">(-126,654 [-94.7%])</span> <div style='text-align: right'>7,082</div>  |
| revm_contract_deployment | true |  | SLT | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | SLTU | <span style="color: green">(-8 [-0.1%])</span> <div style='text-align: right'>14,107</div>  |
| revm_contract_deployment | true |  | SRA | <div style='text-align: right'>493</div>  |
| revm_contract_deployment | true |  | SRL | <span style="color: green">(-126,672 [-97.0%])</span> <div style='text-align: right'>3,958</div>  |
| revm_contract_deployment | true |  | STOREB | <span style="color: green">(-12,168 [-88.0%])</span> <div style='text-align: right'>1,660</div>  |
| revm_contract_deployment | true |  | STOREH | <span style="color: green">(-2 [-14.3%])</span> <div style='text-align: right'>12</div>  |
| revm_contract_deployment | true |  | STOREW | <span style="color: green">(-225,319 [-87.2%])</span> <div style='text-align: right'>33,082</div>  |
| revm_contract_deployment | true |  | SUB | <span style="color: green">(-279 [-12.4%])</span> <div style='text-align: right'>1,967</div>  |
| revm_contract_deployment | true |  | XOR | <span style="color: green">(-453,315 [-99.4%])</span> <div style='text-align: right'>2,617</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-2,172,132 [-39.8%])</span> <div style='text-align: right'>3,288,672</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: green">(-3,931,668 [-93.4%])</span> <div style='text-align: right'>276,048</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: green">(-2,100 [-10.4%])</span> <div style='text-align: right'>18,081</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | AUIPC | <span style="color: red">(+17 [+50.0%])</span> <div style='text-align: right'>51</div>  |
| revm_contract_deployment | Boundary | true |  | AUIPC | <span style="color: red">(+40 [+50.0%])</span> <div style='text-align: right'>120</div>  |
| revm_contract_deployment | Merkle | true |  | AUIPC | <span style="color: red">(+64 [+1.9%])</span> <div style='text-align: right'>3,520</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: green">(-2,470 [-0.7%])</span> <div style='text-align: right'>354,302</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>346,944</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <span style="color: green">(-128 [-0.0%])</span> <div style='text-align: right'>373,632</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,832</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <span style="color: green">(-16,480 [-7.5%])</span> <div style='text-align: right'>204,384</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: green">(-372,996 [-41.7%])</span> <div style='text-align: right'>520,702</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>79,794</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>26,095</div>  |
| revm_contract_deployment | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>61,400</div>  |
| revm_contract_deployment | Merkle | true |  | HINT_STOREW | <span style="color: green">(-128 [-0.1%])</span> <div style='text-align: right'>98,560</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: green">(-1,674 [-2.8%])</span> <div style='text-align: right'>57,672</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: green">(-5,628 [-9.6%])</span> <div style='text-align: right'>53,060</div>  |
| revm_contract_deployment | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>6,910,176</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>413,210</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: green">(-971,280 [-54.6%])</span> <div style='text-align: right'>807,200</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>323</div>  |
| revm_contract_deployment | Boundary | true |  | LOADBU | <div style='text-align: right'>760</div>  |
| revm_contract_deployment | Merkle | true |  | LOADBU | <span style="color: green">(-640 [-14.1%])</span> <div style='text-align: right'>3,904</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>800</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADHU | <div style='text-align: right'>85</div>  |
| revm_contract_deployment | Boundary | true |  | LOADHU | <div style='text-align: right'>200</div>  |
| revm_contract_deployment | Merkle | true |  | LOADHU | <span style="color: green">(-256 [-50.0%])</span> <div style='text-align: right'>256</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-11,457,880 [-90.5%])</span> <div style='text-align: right'>1,207,640</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADW | <span style="color: green">(-646 [-3.0%])</span> <div style='text-align: right'>20,536</div>  |
| revm_contract_deployment | Boundary | true |  | LOADW | <span style="color: green">(-1,520 [-3.0%])</span> <div style='text-align: right'>48,320</div>  |
| revm_contract_deployment | Merkle | true |  | LOADW | <span style="color: green">(-320 [-0.3%])</span> <div style='text-align: right'>92,672</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <span style="color: green">(-40,878 [-57.1%])</span> <div style='text-align: right'>30,726</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: green">(-62 [-0.1%])</span> <div style='text-align: right'>65,565</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MUL | <span style="color: green">(-17 [-33.3%])</span> <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | MUL | <span style="color: green">(-40 [-33.3%])</span> <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULH | <div style='text-align: right'>117</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>48,126</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_contract_deployment | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: green">(-4,559,868 [-94.7%])</span> <div style='text-align: right'>257,220</div>  |
| revm_contract_deployment | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>822</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: green">(-6,712,662 [-94.7%])</span> <div style='text-align: right'>375,346</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>74</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <span style="color: green">(-296 [-0.1%])</span> <div style='text-align: right'>521,959</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>26,129</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: green">(-6,713,616 [-97.0%])</span> <div style='text-align: right'>209,774</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <span style="color: green">(-486,720 [-88.0%])</span> <div style='text-align: right'>66,400</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREB | <span style="color: green">(-17 [-0.8%])</span> <div style='text-align: right'>2,006</div>  |
| revm_contract_deployment | Boundary | true |  | STOREB | <span style="color: green">(-40 [-0.8%])</span> <div style='text-align: right'>4,720</div>  |
| revm_contract_deployment | Merkle | true |  | STOREB | <span style="color: red">(+896 [+4.7%])</span> <div style='text-align: right'>20,032</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <span style="color: green">(-80 [-14.3%])</span> <div style='text-align: right'>480</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-9,012,760 [-87.2%])</span> <div style='text-align: right'>1,323,280</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREW | <span style="color: red">(+3,978 [+2.7%])</span> <div style='text-align: right'>150,824</div>  |
| revm_contract_deployment | Boundary | true |  | STOREW | <span style="color: red">(+9,360 [+2.7%])</span> <div style='text-align: right'>354,880</div>  |
| revm_contract_deployment | Merkle | true |  | STOREW | <span style="color: red">(+12,096 [+2.1%])</span> <div style='text-align: right'>575,040</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: green">(-10,044 [-12.4%])</span> <div style='text-align: right'>70,812</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: green">(-16,319,340 [-99.4%])</span> <div style='text-align: right'>94,212</div>  |

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
| revm_contract_deployment | KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| revm_contract_deployment | VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <span style="color: green">(-9.0 [-11.0%])</span> <div style='text-align: right'>73.0</div>  | <span style="color: green">(-2,334.0 [-49.5%])</span> <div style='text-align: right'>2,383.0</div>  | <span style="color: green">(-2,054.0 [-52.4%])</span> <div style='text-align: right'>1,868.0</div>  | <span style="color: red">(+9.0 [+7.6%])</span> <div style='text-align: right'>128.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-55,858,569 [-73.1%])</span> <div style='text-align: right'>20,515,733</div>  | <span style="color: green">(-1,568,984 [-83.5%])</span> <div style='text-align: right'>310,284</div>  | <span style="color: green">(-6,516.0 [-57.4%])</span> <div style='text-align: right'>4,841.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | ProgramAir | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| revm_contract_deployment | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,343,488</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | PhantomAir | 0 | <div style='text-align: right'>4,608</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>256</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <span style="color: green">(-106,430,464 [-87.5%])</span> <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <span style="color: green">(-917,504 [-87.5%])</span> <div style='text-align: right'>131,072</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,261,568</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-53,329,920 [-96.9%])</span> <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: green">(-507,904 [-96.9%])</span> <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: green">(-102,760,448 [-87.5%])</span> <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <span style="color: green">(-917,504 [-87.5%])</span> <div style='text-align: right'>131,072</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: green">(-131,072 [-50.0%])</span> <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <span style="color: green">(-2,048 [-50.0%])</span> <div style='text-align: right'>2,048</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>50,176</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,024</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>454,656</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>284,672</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>2,048</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>253,952</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | KeccakVmAir | 0 | <div style='text-align: right'>18,235,392</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>4,096</div>  |
| revm_contract_deployment | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>41,091,072</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| revm_contract_deployment | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- |
| revm_contract_deployment | 0 | <span style="color: green">(-4,182.0 [-63.0%])</span> <div style='text-align: right'>2,458.0</div>  | <span style="color: green">(-244,416,512 [-67.3%])</span> <div style='text-align: right'>118,799,904</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/529ac9766c61de45a974884ad5102288e5bd0b76/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/529ac9766c61de45a974884ad5102288e5bd0b76
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11717597072)
