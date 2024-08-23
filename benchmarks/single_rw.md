## Benchmark for ReadWrite
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 38_764_544 | 1530.00 | 301.00 | 25.60 | 175.00 | 1028.40 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageReadAir          | 32_768     | 1_343_488   | 0     | [33] | [8] |
| IndexedPageWriteAir  | 32_768     | 5_603_328   | 0     | [33, 98] | [40] |
| PageOfflineChecker   | 131_072    | 29_753_344  | 0     | [179] | [48] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
| ExecutionAir         | 32_768     | 1_409_024   | 0     | [35] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/5e0da86518761931181f0251ac04c29cb0d6f05a
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10519335567)
