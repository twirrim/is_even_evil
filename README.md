Inspired by this bit of delightful evil https://andreasjhkarlsson.github.io/jekyll/update/2023/12/27/4-billion-if-statements.html

This will make a rust binary that tells you if the provided number argument is odd or even.

At the moment, based on u16, so maximum of 65535, and doesn't handle negative numbers!

# Initial observations

Yes, this is terrible.  Tickling my fancy in my post-Christmas, pre-New Years "What day is it" fuzz.

Originally I wrote this using an if / else if ... form, but at u16 it triggered a stack overflow at compile time, while creating the file.
Switching to match statements format doesn't trigger that.

At u32 / 100, it created a 1G file, and managed to consume about 50G of RAM (between swap and physical in my 32G machine) before OOM killing, around the 5 minute mark.
at u32 / 1000, it created a 104M file, and seems to be sitting steady at around 6G of memory consumption while compiling.  u32 / 1000 == 429,4967.

If I get _really_ bored some day, I'll experiment with modifying that number and seeing how it maps to compilation time and memory consumption of rustc
