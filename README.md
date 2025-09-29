# rust_learning
Learning some Rust using the AOC 2024 puzzles

* Day 1 and 2: Just learning how to get a file in and do anything with Rust
* Day 3: Added in testing in the Rust way. Added a lib file with tests on a part1 and part2 function with the example input. Then piped my individual input to the tested part1 and part2 functions from the main file. Since the VS Code debugger seems... imperfect, I am ending up with a lot of debug prints. Will have to think of a way to deal with that or to better use the debugger
* Day 4: I seem to have figured out the debugger, so I am able to look for my logic errors that way
* Day 5: Good practice fixing type errors and a move error. I could do fewer passes through the input, but seems fine. I had a logic error with the multiple after pages case, but fixed. For part 2 I did some code copy and a clone because it was just easier, but should have done a refactor if it was something real. Also would have refactored the testing to not repeat the file read if it was something real. Even more, I could have been more efficient in part1 by doing an outer loop break when detecting a bad line after fixing my initial logic error. Maybe I can fix all this later if I need some additional practice content
