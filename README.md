# Job Shop EA

**Evolutionary Algorithm for Solving the Job-shop Scheduling Problem**

## Usage

`$ job-shop-ea --help`

To get reproducible results it is recommended to specify a seed `--seed 0` and
a number of iterations `--iterations 1000`.
You can interrupt the program at any time with `CTRL+C`.

The input files are of the format:

```
2 2
0 10 1 12
1 32 0 76
```

The first line is ignored by this program.
The next lines each specify a single job.
A job is a list of operations, which each consist of first a machine and then
a duration.

The output of the program looks like this:

```
M0: 0/0@0 1/1@10
M1: 1/0@0 0/1@32
```

Each line `MX: [...]` describes the schedule of a single machine `X`.
`J/O@T` states that the operation O of job J is stated at time T.

## License

Copyright 2021 Viktor Reusch

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
