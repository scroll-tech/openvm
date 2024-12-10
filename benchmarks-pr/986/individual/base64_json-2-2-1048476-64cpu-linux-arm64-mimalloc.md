| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <span style="color: green">(-866,804,624 [-98.3%])</span> <div style='text-align: right'>15,116,803</div>  | <span style="color: green">(-6,780,704 [-96.9%])</span> <div style='text-align: right'>217,347</div>  | <span style="color: green">(-38,525.0 [-95.2%])</span> <div style='text-align: right'>1,929.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <div style='text-align: right'>15.0</div>  | <span style="color: red">(+12.0 [+2.5%])</span> <div style='text-align: right'>501.0</div>  | <span style="color: red">(+5.0 [+1.6%])</span> <div style='text-align: right'>323.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+613.0 [+0.8%])</span> <div style='text-align: right'>74,697.0</div>  |

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| KeccakVmAir | <div style='text-align: right'>4,571</div>  | <div style='text-align: right'>321</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 | <span style="color: green">(-38,525.0 [-95.2%])</span> <div style='text-align: right'>1,929.0</div>  | <span style="color: green">(-1,414,524,164 [-96.6%])</span> <div style='text-align: right'>50,533,140</div>  | <span style="color: green">(-866,804,624 [-98.3%])</span> <div style='text-align: right'>15,116,803</div>  | <span style="color: green">(-6,780,704 [-96.9%])</span> <div style='text-align: right'>217,347</div>  | <span style="color: green">(-9,825.0 [-98.3%])</span> <div style='text-align: right'>174.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <span style="color: green">(-289,815 [-93.9%])</span> <div style='text-align: right'>18,961</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <span style="color: green">(-402,076 [-98.7%])</span> <div style='text-align: right'>5,178</div>  |
| app_proof | Merkle | 0 | <span style="color: green">(-588,924 [-99.1%])</span> <div style='text-align: right'>5,524</div>  |
| app_proof | AccessAdapter<8> | 0 | <span style="color: green">(-471,562 [-98.9%])</span> <div style='text-align: right'>5,178</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>1,563</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | <div style='text-align: right'>86</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | <div style='text-align: right'>116</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>1,331</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>2,940</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>5,003</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,738</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>27,336</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,236</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>55,121</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>16,188</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>575</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>89,109</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <span style="color: green">(-621,073 [-100.0%])</span> <div style='text-align: right'>5</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-991,000 [-98.9%])</span> <div style='text-align: right'>10,702</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <span style="color: green">(-2,289,110 [-97.0%])</span> <div style='text-align: right'>69,773</div>  |
| app_proof |  | AND | 0 | <div style='text-align: right'>10,124</div>  |
| app_proof |  | AUIPC | 0 | <div style='text-align: right'>1,331</div>  |
| app_proof |  | BEQ | 0 | <span style="color: green">(-2,206 [-12.4%])</span> <div style='text-align: right'>15,568</div>  |
| app_proof |  | BGE | 0 | <div style='text-align: right'>703</div>  |
| app_proof |  | BGEU | 0 | <div style='text-align: right'>6,863</div>  |
| app_proof |  | BLT | 0 | <div style='text-align: right'>3,354</div>  |
| app_proof |  | BLTU | 0 | <div style='text-align: right'>5,818</div>  |
| app_proof |  | BNE | 0 | <span style="color: green">(-1,333,166 [-99.1%])</span> <div style='text-align: right'>11,768</div>  |
| app_proof |  | HINT_STOREW | 0 | <div style='text-align: right'>1,563</div>  |
| app_proof |  | JAL | 0 | <span style="color: green">(-82,057 [-95.7%])</span> <div style='text-align: right'>3,685</div>  |
| app_proof |  | JALR | 0 | <div style='text-align: right'>2,940</div>  |
| app_proof |  | LOADB | 0 | <div style='text-align: right'>1,236</div>  |
| app_proof |  | LOADBU | 0 | <div style='text-align: right'>23,858</div>  |
| app_proof |  | LOADHU | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | LOADW | 0 | <span style="color: green">(-179,098 [-93.0%])</span> <div style='text-align: right'>13,465</div>  |
| app_proof |  | LUI | 0 | <div style='text-align: right'>1,318</div>  |
| app_proof |  | MUL | 0 | <span style="color: green">(-185,365 [-99.9%])</span> <div style='text-align: right'>116</div>  |
| app_proof |  | MULHU | 0 | <div style='text-align: right'>86</div>  |
| app_proof |  | OR | 0 | <div style='text-align: right'>7,608</div>  |
| app_proof |  | PHANTOM | 0 | <span style="color: green">(-621,073 [-100.0%])</span> <div style='text-align: right'>5</div>  |
| app_proof |  | SLL | 0 | <div style='text-align: right'>7,118</div>  |
| app_proof |  | SLT | 0 | <div style='text-align: right'>5</div>  |
| app_proof |  | SLTU | 0 | <div style='text-align: right'>570</div>  |
| app_proof |  | SRA | 0 | <div style='text-align: right'>8</div>  |
| app_proof |  | SRL | 0 | <div style='text-align: right'>9,062</div>  |
| app_proof |  | STOREB | 0 | <div style='text-align: right'>5,133</div>  |
| app_proof |  | STOREH | 0 | <div style='text-align: right'>10</div>  |
| app_proof |  | STOREW | 0 | <span style="color: green">(-234,546 [-94.9%])</span> <div style='text-align: right'>12,652</div>  |
| app_proof |  | SUB | 0 | <span style="color: green">(-76,147 [-98.2%])</span> <div style='text-align: right'>1,416</div>  |
| app_proof |  | XOR | 0 | <div style='text-align: right'>188</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>2,511,828</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <span style="color: green">(-294,270 [-100.0%])</span> <div style='text-align: right'>85</div>  |
| app_proof | Boundary |  | ADD | 0 | <span style="color: green">(-692,400 [-100.0%])</span> <div style='text-align: right'>200</div>  |
| app_proof | Merkle |  | ADD | 0 | <span style="color: green">(-2,038,912 [-100.0%])</span> <div style='text-align: right'>128</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>364,464</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>27,951</div>  |
| app_proof | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>51</div>  |
| app_proof | Boundary |  | AUIPC | 0 | <div style='text-align: right'>120</div>  |
| app_proof | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,520</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>404,768</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGE | 0 | <div style='text-align: right'>22,496</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>219,616</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | <div style='text-align: right'>107,328</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>186,176</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>305,968</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>40,638</div>  |
| app_proof | AccessAdapter<8> |  | HINT_STOREW | 0 | <div style='text-align: right'>13,277</div>  |
| app_proof | Boundary |  | HINT_STOREW | 0 | <div style='text-align: right'>31,240</div>  |
| app_proof | Merkle |  | HINT_STOREW | 0 | <div style='text-align: right'>50,240</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>66,330</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>82,320</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | <div style='text-align: right'>43,260</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | <div style='text-align: right'>954,320</div>  |
| app_proof | AccessAdapter<8> |  | LOADBU | 0 | <div style='text-align: right'>2,856</div>  |
| app_proof | Boundary |  | LOADBU | 0 | <div style='text-align: right'>6,720</div>  |
| app_proof | Merkle |  | LOADBU | 0 | <div style='text-align: right'>12,352</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADHU | 0 | <div style='text-align: right'>120</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>538,600</div>  |
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <span style="color: green">(-122,842 [-98.5%])</span> <div style='text-align: right'>1,921</div>  |
| app_proof | Boundary |  | LOADW | 0 | <span style="color: green">(-243,680 [-98.2%])</span> <div style='text-align: right'>4,520</div>  |
| app_proof | Merkle |  | LOADW | 0 | <span style="color: green">(-385,344 [-96.9%])</span> <div style='text-align: right'>12,160</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>23,724</div>  |
| app_proof | AccessAdapter<8> |  | LUI | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | LUI | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | <div style='text-align: right'>3,596</div>  |
| app_proof | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | <div style='text-align: right'>3,354</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>273,888</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <span style="color: green">(-3,726,438 [-100.0%])</span> <div style='text-align: right'>30</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>377,254</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLT | 0 | <div style='text-align: right'>185</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>21,090</div>  |
| app_proof | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | SLTU | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRA | 0 | <div style='text-align: right'>424</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | <div style='text-align: right'>480,286</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | <div style='text-align: right'>205,320</div>  |
| app_proof | AccessAdapter<8> |  | STOREB | 0 | <div style='text-align: right'>10,472</div>  |
| app_proof | Boundary |  | STOREB | 0 | <div style='text-align: right'>24,640</div>  |
| app_proof | Merkle |  | STOREB | 0 | <div style='text-align: right'>39,232</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREH | 0 | <div style='text-align: right'>400</div>  |
| app_proof | AccessAdapter<8> |  | STOREH | 0 | <div style='text-align: right'>17</div>  |
| app_proof | Boundary |  | STOREH | 0 | <div style='text-align: right'>40</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>506,080</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <span style="color: green">(-223,601 [-93.6%])</span> <div style='text-align: right'>15,300</div>  |
| app_proof | Boundary |  | STOREW | 0 | <span style="color: green">(-526,120 [-93.6%])</span> <div style='text-align: right'>36,000</div>  |
| app_proof | Merkle |  | STOREW | 0 | <span style="color: green">(-2,735,808 [-97.9%])</span> <div style='text-align: right'>59,072</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>50,976</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>6,768</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: green">(-6.0 [-1.9%])</span> <div style='text-align: right'>311.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-866,804,624 [-98.3%])</span> <div style='text-align: right'>15,116,803</div>  | <span style="color: green">(-6,780,704 [-96.9%])</span> <div style='text-align: right'>217,347</div>  | <span style="color: green">(-38,525.0 [-95.2%])</span> <div style='text-align: right'>1,929.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <span style="color: green">(-8,847,360 [-93.8%])</span> <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-491,520 [-93.8%])</span> <div style='text-align: right'>32,768</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <span style="color: green">(-14,417,920 [-98.2%])</span> <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-516,096 [-98.4%])</span> <div style='text-align: right'>8,192</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <span style="color: green">(-45,711,360 [-99.1%])</span> <div style='text-align: right'>425,984</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <span style="color: green">(-1,040,384 [-99.2%])</span> <div style='text-align: right'>8,192</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <span style="color: green">(-16,965,632 [-98.1%])</span> <div style='text-align: right'>335,872</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-516,096 [-98.4%])</span> <div style='text-align: right'>8,192</div>  |
| app_proof | KeccakVmAir | 0 | <div style='text-align: right'>4,452</div>  | <div style='text-align: right'>3,164</div>  | <div style='text-align: right'>1,288</div>  |  | <div style='text-align: right'>1</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>126,976</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | <div style='text-align: right'>17,792</div>  | <div style='text-align: right'>39</div>  | <div style='text-align: right'>100</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | <div style='text-align: right'>14,208</div>  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>128</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>100,352</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>262,144</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4,096</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>507,904</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>8,192</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>2,424,832</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>32,768</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | <div style='text-align: right'>227,328</div>  | <div style='text-align: right'>35</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>2,048</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>1,720,320</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>16,384</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>78,848</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>1,024</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <span style="color: green">(-14,679,920 [-100.0%])</span> <div style='text-align: right'>144</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-1,048,568 [-100.0%])</span> <div style='text-align: right'>8</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-613,629,952 [-98.4%])</span> <div style='text-align: right'>10,272,768</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <span style="color: green">(-1,032,192 [-98.4%])</span> <div style='text-align: right'>16,384</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: red">(+7.0 [+4.1%])</span> <div style='text-align: right'>176.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f457bcae1d73628d180d4ede35866aa437e3cf26/base64_json-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/f457bcae1d73628d180d4ede35866aa437e3cf26

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12266628442)
