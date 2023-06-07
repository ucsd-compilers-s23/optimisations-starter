
Example of use:

```
→ cat tests/bigloop.snek
(let ((n input))
  (loop
    (if (= n 0) (break 100)
        (set! n (- n 1)))))
→ make profile

... lots of output ...
---- profile_bigloop stdout ----
valgrind is only available on Linux. Using usr/bin/time to gauge instructions instead (use it only for relative comparisons on your system)
Instructions executed: 3111162207

Instructions in generated .s: 66
Instructions by type in your generated assembly before linking is:
  30 mov
   5 push
   5 pop
   4 test
   4 call
   3 jne
   3 jmp
   2 sub
   2 je
   2 cmp
   1 xor
   1 ret
   1 or
   1 jo
   1 cmove
   1 add


perf is only available on Linux. Using usr/bin/time instead.
Time taken in ms (seconds on MacOS):
1 0.33
2 0.36
3 0.34
4 0.30
5 0.34
```

This prints

1. The number of instructions counted in the execution (counted by `callgrind`
on Linux or `/usr/bin/time -l` on OSX)
2. A summary of the instructions used (statically counted)
3. Several trials of wall-clock time for the program

Try to improve on what forest-flame can do!
