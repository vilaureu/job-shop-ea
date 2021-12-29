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

## Method

This multi-threaded program uses an evolutionary algorithm to solve job-shop
scheduling problems.
It supports reproducible calculations.

### State

The state of a single individual/schedule is represented as a list of job
numbers.
This list specifies the order in which operations are inserted into the
schedule.
For example, the list `[0, 1, 1, 0]` is converted to a schedule as follows:

1. Take the first operation of job 0 and schedule it at the earliest time at
   the corresponding machine.
   Generally, this takes the end time of the previous operation of the same
   job and on the same machine into account.
2. Take the first operation of job 1 and schedule it.
3. Take the second operation of job 1 and schedule it.
4. Take the second operation of job 0 and schedule it.

As a result, all valid states are permutations of each other.

### Parent Selection

Parents are selected probabilistically weighted by their fitness (the inverse
of their total time).

### Recombination

This program uses _Modified Crossover_ [0] to merge states and generate a new
permutation.
This simply copies a random amount of job numbers from the beginning of the
state of the first parent and then fills up the child state with the remaining
numbers from the second parent.

### Mutation

Mutations are performed probabilistically by swapping two job numbers in the
state of an individual at random.

### Survivor Selection

Survivor selection happens deterministically only using fitness as an criterion.

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

## References

[0] Lawrence Davis. 1985. Applying adaptive algorithms to epistatic domains. In
Proceedings of the 9th international joint conference on Artificial
intelligence - Volume 1 (IJCAI'85). Morgan Kaufmann Publishers Inc., San
Francisco, CA, USA, 162–164.
