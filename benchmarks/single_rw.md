## Benchmark for ReadWrite
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 36_405_248 | 1430.00 | 214.00 | 20.50 | 172.00 | 1023.50 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageReadAir          | 32_768     | 1_343_488   | 0     | [33] | [8] |
| IndexedPageWriteAir  | 32_768     | 5_079_040   | 0     | [33, 82] | [40] |
| PageOfflineChecker   | 131_072    | 27_394_048  | 0     | [161] | [48] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |
| ExecutionAir         | 32_768     | 1_409_024   | 0     | [35] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/cf1119e0fc3d7de4ce04e32a2f4de414f2b83c2d
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11206713156)
