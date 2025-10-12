# Learning Rust
Learning some Rust using the AOC 2024 puzzles up to day 10 (easier ones)

* Day 1 and 2: Just learning how to get a file in and do anything with Rust
* Day 3: Added in testing in the Rust way. Added a lib file with tests on a part1 and part2 function with the example input. Then piped my individual input to the tested part1 and part2 functions from the main file. Since the VS Code debugger seems... imperfect, I am ending up with a lot of debug prints. Will have to think of a way to deal with that or to better use the debugger
* Day 4: I seem to have figured out the debugger, so I am able to look for my logic errors that way
* Day 5: Good practice fixing type errors and a move error. I could do fewer passes through the input, but seems fine. I had a logic error with the multiple after pages case, but fixed. For part 2 I did some code copy and a clone because it was just easier, but should have done a refactor if it was something real. Also would have refactored the testing to not repeat the file read if it was something real. Even more, I could have been more efficient in part1 by doing an outer loop break when detecting a bad line after fixing my initial logic error. Maybe I can fix all this later if I need some additional practice content
* Day 6: Changed tests to only read in from the test file once for both part tests. This was harder than I expected it would be and I don't know if I really needed the once_cell crate that the LLM suggested I use there. However, since tests run in parallel, maybe I do?  
Part 2 I just did a brute force replacement of all "." cells. This worked, but I needed a --release compile to get it to run in a few seconds. This is probably a good place to both improve the logic and parallelize. I also need to start reading the solutions of others as it took a long time to get all the typing correct while knowing the approach just fine. I need to see how other "Rustations" do this

I did go and look at some code for AOC from others doing it in Rust. There are a number of folks doing speed optimization in the language and that was interesting to read about. I think I will continue on to Day 10 and then maybe go back and refactor them. Even though others seem to have less testing with less overhead, I think I will stick with that for now as it seems to fit well

* Day 7: For now, just did a bunch of clones and no short circuit for going over the target. On the refactor I will do those and make it parallel. I tried to stick to usize and I think that resulted in less casting and the final numbers would not have fit in u32 anyway. It takes a couple seconds for part 2
* Day 8: I still had to fix a bunch of items to get things to compile, but it seemed to go much smoother with less stumbling with typing. I'm sure it could be improved, but it finished quick and there was no cloning. I rebuilt some data types that I could have carried from part1 to part2, but that seems minor
* Day 9: Brute force part 1 was fine and didn't take too long. Part 2 was still brute force with a little early exit. It took a bit to get the stepping and loop exits correct, but worked and didn't take too long in run time. Might rewrite with a different structure when I come back through to refactor. I think my Part 2 code was a little hard to follow
* Day 10: Part 1, I was looking for a chance to use petgraph and compare with the use of NetworkX with Python. NetworkX and Python is much easier to use. I will see if petgraph is a bunch faster when I go back and refactor the days and compare to whatever I did in Python.  
Part 2: I had a good idea. Seemed to work and was able to just use petgraph's neighbors in a fixed length loop. May not have really needed a graph library for this day, but it was good to learn a bit about petgraph

## Reflecting, Refactoring, and Timing
Now that I have a tiny bit of Rust experience and am almost done going through [The Rust Programming Language](https://doc.rust-lang.org/book/) book, I am going to read what I did in Python for the same AOC days, compare, time them and refactor a bit.

* Day 1: My Rust code is not quite twice as long (60 lines) as my Python code (36 lines). I did the same approach in each for part 1 which was to turn each column into a list, sort them and walk through. I did a singly nested for loop in Python for part 2, but in Rust I made a HashMap (would have been a Dict in Python) so that I would only have to walk the list once and saved the counts. So this is not a perfect comparison of approach between the two and I wanted to do the Rust implementation from scratch for these, so that is expected. In some cases, I may refactor to get a better match with the Python for the timing comparison. I will start with just a simple timing of the parsing and the algorithm without the part that reads in the input. So a string will have already been created from an input file, but not parsed yet.  
```
        Python      Rust(debug) (release)   in milliseconds
Part1   0.786       1.794       0.0122
Part2   50.96       0.685       0.0037
```
So I ran each twice and took the second run timing to allow caching to help, but no fancy averaging or anything. Interesting that Python seems to be able to compete with the Rust debug timing in part 1 where the approach is the same. However, if you let Rust optimize, it's not even close as Rust is 64 times faster. The Rust approach for part 2 should be faster as long as using a bit more memory is not to expensive, and so it seems with Rust faster in either case and over 13,000x faster with the better approach and release optimized
* Day 2: My Rust is just over twice as long, but I made a function to test "safety" in Python, which reduced the code size. Part 1 was done in a pretty comparable way between the two with a single walk through each line of numbers. Part 2 was again done a bit different between the two. In Python, I removed each number and tested with a short-circuit if a good result was seen. In Rust, I have a function that will allow another attempt without the current number or the next number and a section to handle the initial number having to be removed. It should be faster, but is harder to read by a good bit and longer to write.
```
        Python      Rust(debug) (release)   in milliseconds
Part1   1.7         2.1         0.17
Part2   5.7         2.8         0.27
```
So things look about as expected and in-line with day01. I am only going to call out the release difference from here forward, which is 10x and 21x faster for Rust part1 and 2. Could I get rid of the .clone() that is used to feed the second function call? I think so. I could try just using a flag to skip an index in the list and pass in the index. I could also use the function in both parts and make the total amount of code shorter. I just did the modification for part 2.
```
        debug   release
Part1   2.3     0.21
Part2   2.5     0.24
```
A bit strange as the first part increased slightly and the second decreased slightly. I guess there are not too many copies and maybe the compiler is rearranging to not do them if not needed. It does not seem easier to read and was harder to implement than I first thought. Maybe not much to be gained for this one
* Day 3: First thing is to clean up all the prints for debugging and to look into fixing all the warnings for unused variables. I didn't clean up ALL the unused warnings in Rust and this will not be an Apples-to-Apples comparison as I used a regex in Python and manual parsing in Rust.
```
        Python      Rust(debug) (release)   in milliseconds
Part1   0.63        0.49        0.054
Part2   1.3         2.2         0.40
```
If you don't count regex as hard, Python is again much easier to write and only takes about 15% the amount of code that Rust does. I introduced Rust style testing in this day so Rust is doing a lot more here. Rust is once again, faster at 11x and 3x faster for part1 and 2 respectively. Why doesn't Rust beat Python by 10x in part 2? I think the regex allows Python do all the searching at once in a compiled call to the regex, so its loop work is not increasing much. Rust has a regex crate, so I could try that or some other speedup, but since it is already running under half a millisecond, I will leave it be for now. I may come back later and try the regex crate just to see how to use it with Rust.
* Day 4: The approach was similar in both languages for this day and was pretty loop heavy in small memory sections.
```
        Python      Rust(debug) (release)   in milliseconds
Part1   18.3        3.3         0.29
Part2   4.2         3.4         0.17
```
So this time Rust is 63x and 24x faster. However, it looks like I did some extra work and storage in Python part 1. Probably because I thought I would be able to use it in part 2 or to provide debugging help. So the difference in part 2 is probably more fair than in part 1.  
This day might be a good test of trying to run something in parallel in Rust where it probably would not be worth it in Python. The Rust results are under a half ms, so unless Rust can efficiently thread, the gain will be pretty limited. Let's see if we can use Rayon here. We should be able to check and count different rows of the matrix of characters independently. If the overhead is low enough, we should see an improvement.  
I had to do a fair rewrite to try this and I only did it on part 1. The result is that it took longer, 0.90 ms. So, either I did it wrong (quite possible) or it take a couple micro seconds to kick off a thread (CoPilot says 2-10), which could kill any gains for this example
* Day 5: Results first this time
```
        Python      Rust(debug) (release)   in milliseconds
Part1   144         17          1.0
Part2   145         328         14.8
```
Rust is 144x and 10x faster. In Python's part 1 I used lists and in Rust I used a hash for the pairs, so not quite the same. In Python part 2, I used a sort with a custom key and in Rust I swapped values that were out of place. I expected this to be worse for Rust, but I guess that doing only bad lines and not having any line be too long makes it OK. Still, let's see if we can get the custom sort working.  
I just made a part2v2 function this time rather than a whole new project and added a bit to main. I already had a hash of hashes to track the order in the Rust version, so I used that inside my sort closure. It worked great and the release runtime for that part is now 1.2 ms and 120x Python. Since Python was a hash of lists, we are back to not exactly comparable. I'm happy with that for now
* Day 6: I had two versions of this in Python, I will include both in the comparison.
```
        Python v2   Rust(debug) (release)   in milliseconds
                                v1      v2      v3
Part1   7.4    5.4  6.0         0.49
Part2   86109  3861 232000      5600    330     105
```
The second version in Python is a parallel one using ProcessPoolExecutor and breaking up the problem in part 2 line-by-line. This is a 23x speed up, Python to Python on a 16 core multi-threaded CPU, so probably in-line with expectations for a good parallel target. I'm unsure where the 2ms came from for part 1, but it could be an artifact of where the first perf_counter() was.  
The original Rust version was the same approach as Python with speedups of 15x for both parts, but with parallel Python part 2 being faster. So of course we will need to try parallel Rust. However, first I wanted to get rid of the clone and fix some unneeded mutable variables, so that would not bog down the parallel version. This helped 6% on its own.  
For the parallel version I modified my now cleaner part2 to use Rayon's par_iter() like I tried in day04. This time it worked nicely with a run time of 330 ms such that Rust is now 11x faster comparing both parallel versions. There is an obvious logic improvement that we should make before we say we are done with this day. We only need to test a block that intersects the original path. Let's make a v3 for Rust to see what that does.  
I needed to adjust part 1 to take a HashMap (it was creating an unused one anyway) so it could be used in v3, but it worked and provided a 3x speedup giving a total 36x speedup vs parallel Python and a final part 2 time of 105 ms
* Day 7: So this was mostly the brute force way.
```
        Python  Rust(debug) (release)   in milliseconds
Part1   367     156         9.6
Part2   26918   2454        283
```
Rust is faster by 38x in part 1 and 95x in part 2. The approaches are similar, but I used nested for loops in Python with the itertools product iterator. In Rust, I used a recursive function. The loops in Python are probably a little cumbersome as the operators are represented with strings and then ifs are used to see what to do. In Python part 2, there is also a string conversion to get length that is probably slower than the computation only method I did in Rust. Since I think a good loop is supposed to be faster than recursion and we should be able to figure out some kind of early exit, let's try another version.  
A looped version for part 1 with a inner-loop early exit (like the Python part 2 version) ran slightly slower at 13 ms. Doing something in parallel here might not improve things either, but we probably have enough room in part 2 to run in parallel. Let's put part 1 back and try that in part 2.  
The debug version of part 2 in parallel was slower, but the release version delivered with 23 ms, 15x faster than the Rust P2 and 1,170x faster than the Python P2. To use Rayon on this, I needed a different tool where I broke up the string by lines first and then fed to `s.lines().par_bridge()` to do the parallelization
* Day 8: ...