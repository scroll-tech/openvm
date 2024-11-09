| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | <div style='text-align: right'>2</div>  | <span style="color: green">(-6,411 [-0.0%])</span> <div style='text-align: right'>20,516,699</div>  | <span style="color: green">(-162 [-0.1%])</span> <div style='text-align: right'>310,277</div>  | <span style="color: red">(+74.0 [+1.3%])</span> <div style='text-align: right'>5,642.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true | <span style="color: green">(-93.0 [-1.0%])</span> <div style='text-align: right'>9,393.0</div>  | <span style="color: green">(-6,411 [-0.0%])</span> <div style='text-align: right'>20,516,699</div>  | <span style="color: green">(-162 [-0.1%])</span> <div style='text-align: right'>310,277</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <span style="color: green">(-99 [-0.1%])</span> <div style='text-align: right'>110,741</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>14,109</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <span style="color: green">(-18 [-0.2%])</span> <div style='text-align: right'>11,529</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <span style="color: green">(-16 [-0.0%])</span> <div style='text-align: right'>33,652</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>29,056</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <span style="color: green">(-5 [-0.1%])</span> <div style='text-align: right'>4,911</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <span style="color: red">(+4 [+0.2%])</span> <div style='text-align: right'>1,898</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>11,811</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <span style="color: green">(-33 [-0.0%])</span> <div style='text-align: right'>85,145</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>1,237</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>2,117</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <span style="color: red">(+3 [+0.3%])</span> <div style='text-align: right'>863</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | KeccakVmAir | true | <div style='text-align: right'>2,184</div>  |
| revm_contract_deployment | Memory AccessAdapter<8> | true | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>11,769</div>  |
| revm_contract_deployment | Memory Boundary | true | <span style="color: green">(-4 [-0.0%])</span> <div style='text-align: right'>23,538</div>  |
| revm_contract_deployment | Memory Merkle | true | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>24,856</div>  |
| revm_contract_deployment | PhantomAir | true | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | ProgramChip | true | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>256,032</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true |  | ADD | <span style="color: green">(-55 [-0.1%])</span> <div style='text-align: right'>91,345</div>  |
| revm_contract_deployment | true |  | AND | <span style="color: green">(-22 [-0.3%])</span> <div style='text-align: right'>7,667</div>  |
| revm_contract_deployment | true |  | AUIPC | <span style="color: red">(+3 [+0.3%])</span> <div style='text-align: right'>863</div>  |
| revm_contract_deployment | true |  | BEQ | <span style="color: green">(-18 [-0.1%])</span> <div style='text-align: right'>13,621</div>  |
| revm_contract_deployment | true |  | BGE | <div style='text-align: right'>10,842</div>  |
| revm_contract_deployment | true |  | BGEU | <div style='text-align: right'>11,676</div>  |
| revm_contract_deployment | true |  | BLT | <div style='text-align: right'>151</div>  |
| revm_contract_deployment | true |  | BLTU | <div style='text-align: right'>6,387</div>  |
| revm_contract_deployment | true |  | BNE | <span style="color: red">(+2 [+0.0%])</span> <div style='text-align: right'>20,031</div>  |
| revm_contract_deployment | true |  | HINT_STOREW | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | true |  | JAL | <span style="color: green">(-5 [-0.2%])</span> <div style='text-align: right'>3,204</div>  |
| revm_contract_deployment | true |  | JALR | <span style="color: red">(+4 [+0.2%])</span> <div style='text-align: right'>1,898</div>  |
| revm_contract_deployment | true |  | KECCAK256 | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | LOADB | <div style='text-align: right'>11,806</div>  |
| revm_contract_deployment | true |  | LOADBU | <span style="color: green">(-17 [-0.1%])</span> <div style='text-align: right'>20,175</div>  |
| revm_contract_deployment | true |  | LOADH | <div style='text-align: right'>5</div>  |
| revm_contract_deployment | true |  | LOADHU | <div style='text-align: right'>20</div>  |
| revm_contract_deployment | true |  | LOADW | <span style="color: green">(-10 [-0.0%])</span> <div style='text-align: right'>30,194</div>  |
| revm_contract_deployment | true |  | LUI | <div style='text-align: right'>1,707</div>  |
| revm_contract_deployment | true |  | MUL | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>2,117</div>  |
| revm_contract_deployment | true |  | MULH | <div style='text-align: right'>3</div>  |
| revm_contract_deployment | true |  | MULHU | <div style='text-align: right'>1,234</div>  |
| revm_contract_deployment | true |  | OR | <span style="color: green">(-15 [-0.2%])</span> <div style='text-align: right'>7,139</div>  |
| revm_contract_deployment | true |  | PHANTOM | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | true |  | SLL | <span style="color: green">(-20 [-0.3%])</span> <div style='text-align: right'>7,076</div>  |
| revm_contract_deployment | true |  | SLT | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | SLTU | <div style='text-align: right'>14,107</div>  |
| revm_contract_deployment | true |  | SRA | <div style='text-align: right'>493</div>  |
| revm_contract_deployment | true |  | SRL | <span style="color: red">(+2 [+0.1%])</span> <div style='text-align: right'>3,960</div>  |
| revm_contract_deployment | true |  | STOREB | <div style='text-align: right'>1,660</div>  |
| revm_contract_deployment | true |  | STOREH | <div style='text-align: right'>12</div>  |
| revm_contract_deployment | true |  | STOREW | <span style="color: green">(-6 [-0.0%])</span> <div style='text-align: right'>33,084</div>  |
| revm_contract_deployment | true |  | SUB | <span style="color: red">(+3 [+0.2%])</span> <div style='text-align: right'>1,970</div>  |
| revm_contract_deployment | true |  | XOR | <span style="color: green">(-10 [-0.4%])</span> <div style='text-align: right'>2,620</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <span style="color: green">(-1,980 [-0.1%])</span> <div style='text-align: right'>3,288,420</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <span style="color: green">(-792 [-0.3%])</span> <div style='text-align: right'>276,012</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <span style="color: red">(+63 [+0.3%])</span> <div style='text-align: right'>18,123</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>51</div>  |
| revm_contract_deployment | Boundary | true |  | AUIPC | <div style='text-align: right'>120</div>  |
| revm_contract_deployment | Merkle | true |  | AUIPC | <div style='text-align: right'>3,520</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <span style="color: green">(-468 [-0.1%])</span> <div style='text-align: right'>354,146</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>346,944</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>373,632</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,832</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>204,384</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <span style="color: red">(+52 [+0.0%])</span> <div style='text-align: right'>520,806</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>79,794</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>26,095</div>  |
| revm_contract_deployment | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>61,400</div>  |
| revm_contract_deployment | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>98,560</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <span style="color: green">(-90 [-0.2%])</span> <div style='text-align: right'>57,672</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <span style="color: red">(+112 [+0.2%])</span> <div style='text-align: right'>53,144</div>  |
| revm_contract_deployment | KeccakVmAir | true |  | KECCAK256 | <div style='text-align: right'>6,910,176</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>413,210</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <span style="color: green">(-680 [-0.1%])</span> <div style='text-align: right'>807,000</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>323</div>  |
| revm_contract_deployment | Boundary | true |  | LOADBU | <div style='text-align: right'>760</div>  |
| revm_contract_deployment | Merkle | true |  | LOADBU | <span style="color: green">(-448 [-10.3%])</span> <div style='text-align: right'>3,904</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>800</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADHU | <div style='text-align: right'>85</div>  |
| revm_contract_deployment | Boundary | true |  | LOADHU | <div style='text-align: right'>200</div>  |
| revm_contract_deployment | Merkle | true |  | LOADHU | <span style="color: green">(-64 [-20.0%])</span> <div style='text-align: right'>256</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <span style="color: green">(-400 [-0.0%])</span> <div style='text-align: right'>1,207,760</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADW | <span style="color: red">(+17 [+0.1%])</span> <div style='text-align: right'>20,519</div>  |
| revm_contract_deployment | Boundary | true |  | LOADW | <span style="color: red">(+40 [+0.1%])</span> <div style='text-align: right'>48,280</div>  |
| revm_contract_deployment | Merkle | true |  | LOADW | <span style="color: green">(-320 [-0.3%])</span> <div style='text-align: right'>92,416</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>30,726</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <span style="color: red">(+62 [+0.1%])</span> <div style='text-align: right'>65,627</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | MUL | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULH | <div style='text-align: right'>117</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>48,126</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_contract_deployment | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <span style="color: green">(-540 [-0.2%])</span> <div style='text-align: right'>257,004</div>  |
| revm_contract_deployment | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>822</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <span style="color: green">(-1,060 [-0.3%])</span> <div style='text-align: right'>375,028</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>74</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>521,959</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>26,129</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <span style="color: red">(+106 [+0.1%])</span> <div style='text-align: right'>209,880</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>66,400</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,006</div>  |
| revm_contract_deployment | Boundary | true |  | STOREB | <div style='text-align: right'>4,720</div>  |
| revm_contract_deployment | Merkle | true |  | STOREB | <span style="color: red">(+896 [+4.7%])</span> <div style='text-align: right'>19,904</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>480</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <span style="color: green">(-240 [-0.0%])</span> <div style='text-align: right'>1,323,360</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREW | <span style="color: green">(-51 [-0.0%])</span> <div style='text-align: right'>150,875</div>  |
| revm_contract_deployment | Boundary | true |  | STOREW | <span style="color: green">(-120 [-0.0%])</span> <div style='text-align: right'>355,000</div>  |
| revm_contract_deployment | Merkle | true |  | STOREW | <span style="color: green">(-256 [-0.0%])</span> <div style='text-align: right'>576,640</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <span style="color: red">(+108 [+0.2%])</span> <div style='text-align: right'>70,920</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <span style="color: green">(-360 [-0.4%])</span> <div style='text-align: right'>94,320</div>  |

| group | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <div style='text-align: right'>109.0</div>  | <span style="color: red">(+14.0 [+0.6%])</span> <div style='text-align: right'>2,393.0</div>  | <span style="color: green">(-5.0 [-0.3%])</span> <div style='text-align: right'>1,867.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-5.0 [-2.5%])</span> <div style='text-align: right'>194.0</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-6,411 [-0.0%])</span> <div style='text-align: right'>20,516,699</div>  | <span style="color: green">(-162 [-0.1%])</span> <div style='text-align: right'>310,277</div>  | <span style="color: red">(+74.0 [+1.3%])</span> <div style='text-align: right'>5,642.0</div>  |

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

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | ProgramAir | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| revm_contract_deployment | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>1,048,576</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>1,703,936</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | AccessAdapterAir<8> | 0 | <div style='text-align: right'>1,343,488</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | PhantomAir | 0 | <div style='text-align: right'>4,608</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>256</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>131,072</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,261,568</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>14,680,064</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>131,072</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>1,818,624</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>16,384</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| revm_contract_deployment | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>131,072</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
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
| revm_contract_deployment | 0 | <span style="color: red">(+60.0 [+1.9%])</span> <div style='text-align: right'>3,249.0</div>  | <div style='text-align: right'>118,799,904</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f8d9332a36ac9816e897543d657db626bedbcf42/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/f8d9332a36ac9816e897543d657db626bedbcf42
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11754355403)
