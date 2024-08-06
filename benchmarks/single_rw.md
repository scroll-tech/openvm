--
title: "Benchmark"
layout: default
--

## Benchmark for ReadWrite

| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
| ----------- | ---------------- | ------------------- | ------------------- | ------------------------- | ------------------ |
| 38_764_544  | 1500.00          | 286.00              | 21.80               | 171.00                    | 1021.20            |

### AIR metrics

| Name                | Rows    | Cells      | Prep Cols | Main Cols | Perm Cols |
| ------------------- | ------- | ---------- | --------- | --------- | --------- |
| PageReadAir         | 32_768  | 1_343_488  | 0         | [33]      | [8]       |
| IndexedPageWriteAir | 32_768  | 5_603_328  | 0         | [33, 98]  | [40]      |
| PageOfflineChecker  | 131_072 | 29_753_344 | 0         | [179]     | [48]      |
| RangeCheckerGateAir | 65_536  | 655_360    | 0         | [2]       | [8]       |
| ExecutionAir        | 32_768  | 1_409_024  | 0         | [35]      | [8]       |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/0d17a509a7020270aedf73740d0c764e4ea7d938
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10257789894)
