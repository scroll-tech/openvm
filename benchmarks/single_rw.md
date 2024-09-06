## Benchmark for ReadWrite
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 36_012_032 | 1400.00 | 236.00 | 21.10 | 166.00 | 976.90 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageReadAir          | 32_768     | 1_343_488   | 0     | [33] | [8] |
| IndexedPageWriteAir  | 32_768     | 5_079_040   | 0     | [33, 82] | [40] |
| PageOfflineChecker   | 131_072    | 27_525_120  | 0     | [162] | [48] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
| ExecutionAir         | 32_768     | 1_409_024   | 0     | [35] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/6c8f730f043a5ae164f7341ace83a04e91d0394e
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10729915035)
