# create_words
```
$ cargo run --release электровоз dat/dict.txt > электровоз
```
```
$ cargo run этажерка dat/dict.txt > этажерка
```
```
$valgrind ./target/release/create_words этажерка dat/dict.txt > этажерка
==9597== Memcheck, a memory error detector
==9597== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==9597== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==9597== Command: ./target/release/create_words этажерка dat/dict.txt
==9597==
==9597==
==9597== HEAP SUMMARY:
==9597==     in use at exit: 0 bytes in 0 blocks
==9597==   total heap usage: 0 allocs, 0 frees, 0 bytes allocated
==9597==
==9597== All heap blocks were freed -- no leaks are possible
==9597==
==9597== For counts of detected and suppressed errors, rerun with: -v
==9597== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```