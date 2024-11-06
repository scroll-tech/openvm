| group | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- |
| revm_contract_deployment | <div style='text-align: right'>76,374,302</div>  | <div style='text-align: right'>1,879,268</div>  | <span style="color: green">(-185.0 [-1.3%])</span> <div style='text-align: right'>14,572.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | collect_metrics | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true | <span style="color: red">(+399.0 [+0.8%])</span> <div style='text-align: right'>49,020.0</div>  | <div style='text-align: right'>76,374,302</div>  | <div style='text-align: right'>1,879,268</div>  |

| group | chip_name | collect_metrics | rows_used |
| --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true | <div style='text-align: right'>860,556</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true | <div style='text-align: right'>14,117</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true | <div style='text-align: right'>264,859</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true | <div style='text-align: right'>48,095</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true | <div style='text-align: right'>29,575</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true | <div style='text-align: right'>7,275</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true | <div style='text-align: right'>2,096</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true | <div style='text-align: right'>11,811</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true | <div style='text-align: right'>633,363</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true | <div style='text-align: right'>1,237</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true | <div style='text-align: right'>2,117</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true | <div style='text-align: right'>961</div>  |
| revm_contract_deployment | BitwiseOperationLookupAir<8> | true | <div style='text-align: right'>65,536</div>  |
| revm_contract_deployment | Memory AccessAdapter<8> | true | <div style='text-align: right'>11,572</div>  |
| revm_contract_deployment | Memory Boundary | true | <div style='text-align: right'>23,144</div>  |
| revm_contract_deployment | Memory Merkle | true | <div style='text-align: right'>24,454</div>  |
| revm_contract_deployment | PhantomAir | true | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | ProgramChip | true | <div style='text-align: right'>256,778</div>  |
| revm_contract_deployment | RangeTupleCheckerAir<2> | true | <div style='text-align: right'>524,288</div>  |

| group | collect_metrics | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| revm_contract_deployment | true |  | ADD | <div style='text-align: right'>151,689</div>  |
| revm_contract_deployment | true |  | AND | <div style='text-align: right'>116,881</div>  |
| revm_contract_deployment | true |  | AUIPC | <div style='text-align: right'>961</div>  |
| revm_contract_deployment | true |  | BEQ | <div style='text-align: right'>13,722</div>  |
| revm_contract_deployment | true |  | BGE | <div style='text-align: right'>10,842</div>  |
| revm_contract_deployment | true |  | BGEU | <div style='text-align: right'>11,680</div>  |
| revm_contract_deployment | true |  | BLT | <div style='text-align: right'>151</div>  |
| revm_contract_deployment | true |  | BLTU | <div style='text-align: right'>6,902</div>  |
| revm_contract_deployment | true |  | BNE | <div style='text-align: right'>34,373</div>  |
| revm_contract_deployment | true |  | HINT_STOREW | <div style='text-align: right'>3,069</div>  |
| revm_contract_deployment | true |  | JAL | <div style='text-align: right'>3,297</div>  |
| revm_contract_deployment | true |  | JALR | <div style='text-align: right'>2,096</div>  |
| revm_contract_deployment | true |  | LOADB | <div style='text-align: right'>11,806</div>  |
| revm_contract_deployment | true |  | LOADBU | <div style='text-align: right'>44,462</div>  |
| revm_contract_deployment | true |  | LOADH | <div style='text-align: right'>5</div>  |
| revm_contract_deployment | true |  | LOADHU | <div style='text-align: right'>20</div>  |
| revm_contract_deployment | true |  | LOADW | <div style='text-align: right'>316,638</div>  |
| revm_contract_deployment | true |  | LUI | <div style='text-align: right'>3,978</div>  |
| revm_contract_deployment | true |  | MUL | <div style='text-align: right'>2,117</div>  |
| revm_contract_deployment | true |  | MULH | <div style='text-align: right'>3</div>  |
| revm_contract_deployment | true |  | MULHU | <div style='text-align: right'>1,234</div>  |
| revm_contract_deployment | true |  | OR | <div style='text-align: right'>133,808</div>  |
| revm_contract_deployment | true |  | PHANTOM | <div style='text-align: right'>137</div>  |
| revm_contract_deployment | true |  | SLL | <div style='text-align: right'>133,736</div>  |
| revm_contract_deployment | true |  | SLT | <div style='text-align: right'>2</div>  |
| revm_contract_deployment | true |  | SLTU | <div style='text-align: right'>14,115</div>  |
| revm_contract_deployment | true |  | SRA | <div style='text-align: right'>493</div>  |
| revm_contract_deployment | true |  | SRL | <div style='text-align: right'>130,630</div>  |
| revm_contract_deployment | true |  | STOREB | <div style='text-align: right'>13,828</div>  |
| revm_contract_deployment | true |  | STOREH | <div style='text-align: right'>14</div>  |
| revm_contract_deployment | true |  | STOREW | <div style='text-align: right'>258,401</div>  |
| revm_contract_deployment | true |  | SUB | <div style='text-align: right'>2,246</div>  |
| revm_contract_deployment | true |  | XOR | <div style='text-align: right'>455,932</div>  |

| group | air_name | collect_metrics | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | ADD | <div style='text-align: right'>5,460,804</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | AND | <div style='text-align: right'>4,207,716</div>  |
| revm_contract_deployment | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | true |  | AUIPC | <div style='text-align: right'>20,181</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | AUIPC | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | AUIPC | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BEQ | <div style='text-align: right'>356,772</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGE | <div style='text-align: right'>346,944</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BGEU | <div style='text-align: right'>373,760</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLT | <div style='text-align: right'>4,832</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | true |  | BLTU | <div style='text-align: right'>220,864</div>  |
| revm_contract_deployment | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | true |  | BNE | <div style='text-align: right'>893,698</div>  |
| revm_contract_deployment | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | true |  | HINT_STOREW | <div style='text-align: right'>79,794</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | HINT_STOREW | <div style='text-align: right'>26,095</div>  |
| revm_contract_deployment | Boundary | true |  | HINT_STOREW | <div style='text-align: right'>61,400</div>  |
| revm_contract_deployment | Merkle | true |  | HINT_STOREW | <div style='text-align: right'>98,688</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | JAL | <div style='text-align: right'>59,346</div>  |
| revm_contract_deployment | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | true |  | JALR | <div style='text-align: right'>58,688</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADB | <div style='text-align: right'>413,210</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADBU | <div style='text-align: right'>1,778,480</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADBU | <div style='text-align: right'>323</div>  |
| revm_contract_deployment | Boundary | true |  | LOADBU | <div style='text-align: right'>760</div>  |
| revm_contract_deployment | Merkle | true |  | LOADBU | <div style='text-align: right'>4,544</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | true |  | LOADH | <div style='text-align: right'>175</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADHU | <div style='text-align: right'>800</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADHU | <div style='text-align: right'>85</div>  |
| revm_contract_deployment | Boundary | true |  | LOADHU | <div style='text-align: right'>200</div>  |
| revm_contract_deployment | Merkle | true |  | LOADHU | <div style='text-align: right'>512</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | LOADW | <div style='text-align: right'>12,665,520</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LOADW | <div style='text-align: right'>21,182</div>  |
| revm_contract_deployment | Boundary | true |  | LOADW | <div style='text-align: right'>49,840</div>  |
| revm_contract_deployment | Merkle | true |  | LOADW | <div style='text-align: right'>92,992</div>  |
| revm_contract_deployment | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | true |  | LUI | <div style='text-align: right'>71,604</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | LUI | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | LUI | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | true |  | MUL | <div style='text-align: right'>65,627</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MUL | <div style='text-align: right'>51</div>  |
| revm_contract_deployment | Boundary | true |  | MUL | <div style='text-align: right'>120</div>  |
| revm_contract_deployment | Merkle | true |  | MUL | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULH | <div style='text-align: right'>117</div>  |
| revm_contract_deployment | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | true |  | MULHU | <div style='text-align: right'>48,126</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | MULHU | <div style='text-align: right'>17</div>  |
| revm_contract_deployment | Boundary | true |  | MULHU | <div style='text-align: right'>40</div>  |
| revm_contract_deployment | Merkle | true |  | MULHU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | OR | <div style='text-align: right'>4,817,088</div>  |
| revm_contract_deployment | PhantomAir | true |  | PHANTOM | <div style='text-align: right'>822</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SLL | <div style='text-align: right'>7,088,008</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLT | <div style='text-align: right'>74</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | true |  | SLTU | <div style='text-align: right'>522,255</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | SLTU | <div style='text-align: right'>34</div>  |
| revm_contract_deployment | Boundary | true |  | SLTU | <div style='text-align: right'>80</div>  |
| revm_contract_deployment | Merkle | true |  | SLTU | <div style='text-align: right'>64</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRA | <div style='text-align: right'>26,129</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | true |  | SRL | <div style='text-align: right'>6,923,390</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREB | <div style='text-align: right'>553,120</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREB | <div style='text-align: right'>2,023</div>  |
| revm_contract_deployment | Boundary | true |  | STOREB | <div style='text-align: right'>4,760</div>  |
| revm_contract_deployment | Merkle | true |  | STOREB | <div style='text-align: right'>19,136</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREH | <div style='text-align: right'>560</div>  |
| revm_contract_deployment | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | true |  | STOREW | <div style='text-align: right'>10,336,040</div>  |
| revm_contract_deployment | AccessAdapter<8> | true |  | STOREW | <div style='text-align: right'>146,846</div>  |
| revm_contract_deployment | Boundary | true |  | STOREW | <div style='text-align: right'>345,520</div>  |
| revm_contract_deployment | Merkle | true |  | STOREW | <div style='text-align: right'>562,944</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | SUB | <div style='text-align: right'>80,856</div>  |
| revm_contract_deployment | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | true |  | XOR | <div style='text-align: right'>16,413,552</div>  |

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
| revm_contract_deployment | <span style="color: red">(+1.0 [+1.2%])</span> <div style='text-align: right'>85.0</div>  | <span style="color: green">(-5.0 [-0.1%])</span> <div style='text-align: right'>4,684.0</div>  | <span style="color: red">(+7.0 [+0.2%])</span> <div style='text-align: right'>3,888.0</div>  | <span style="color: red">(+7.0 [+6.5%])</span> <div style='text-align: right'>115.0</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>76,374,302</div>  | <div style='text-align: right'>1,879,268</div>  | <span style="color: green">(-185.0 [-1.3%])</span> <div style='text-align: right'>14,572.0</div>  |

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
| revm_contract_deployment | 0 | <span style="color: green">(-180.0 [-1.8%])</span> <div style='text-align: right'>9,888.0</div>  | <div style='text-align: right'>363,216,416</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd/revm_contract_deployment-revm_contract_deployment.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/58d2f5a7b285661d4345d2dd0b85b4e656d44ebd
Instance Type: 64cpu-linux-arm64
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11709833384)
