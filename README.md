Inspired by this bit of delightful evil https://andreasjhkarlsson.github.io/jekyll/update/2023/12/27/4-billion-if-statements.html

This will make a rust binary that tells you if the provided number argument is odd or even.

At the moment, based on u32 / 1000, so maximum of 429,496, and doesn't handle negative numbers!

# Initial observations

Yes, this is terrible.  Tickling my fancy in my post-Christmas, pre-New Years "What day is it" fuzz.

Originally I wrote this using an if / else if ... form, but at u16 it triggered a stack overflow at compile time, while creating the file.
Switching to match statements format doesn't trigger that.

At u32 / 100, it created a 1G file, and managed to consume about 50G of RAM (between swap and physical in my 32G machine) before OOM killing, around the 5 minute mark.
at u32 / 1000, it created a 104M file, and seems to be sitting steady at around 6G of memory consumption while compiling.  u32 / 1000 == 429,4967.

If I get _really_ bored some day, I'll experiment with modifying that number and seeing how it maps to compilation time and memory consumption of rustc

# Will it ever compile...?

So far I've had it running on a machine for 3 days, and it's yet to finish compiling.  This seems to be a pathological case, albeit one no one will ever legitimately come across.. I think?
I can't imagine any scenarios where you'd legitimately have 4,294,967 entries in a match statement, at compile time.

## Quick experiment

I threw together a python script, https://gist.github.com/twirrim/f00634367ccf9862975933f184abc4ba to gather some timings

| limit  | debug             | release           |
|--------|-------------------|-------------------|
| 0      | 0.21599388699542  | 0.243635614002415 |
| 10000  | 0.356581996995374 | 2.63057388699963  |
| 20000  | 0.633751568995649 | 8.95849192499736  |
| 30000  | 1.0519599649997   | 20.2605597779984  |
| 40000  | 1.65016355199623  | 36.0358153490015  |
| 50000  | 2.57371079400036  | 61.2215769510003  |
| 60000  | 3.85266107099596  | 87.0177086340045  |
| 70000  | 5.55039826399297  | 110.905351900001  |
| 80000  | 7.81826143500075  | 155.121035224998  |
| 90000  | 10.4216318859981  | 185.115920327     |
| 100000 | 13.2823753760022  | 254.703925580994  |
| 110000 | 16.6019436899951  | 314.851370926997  |
| 120000 | 20.0404017239998  | 368.066561163003  |
| 130000 | 23.8103672759971  | 405.179086242999  |
| 140000 | 28.0492201300003  | 473.558604172002  |
| 150000 | 32.3968182820026  | 541.930442707999  |
| 160000 | 38.0172443449992  | 646.058881256002  |

debug in this case is flat rustc invocation against the generated rust file, release is with the -O flat to enable standard optimisations.

At the risk of sounding like I know what I'm talking about, when I don't, I think that looks maybe quadratic?  Each doubling of match count is resulting in a 4x increase in compilation time:
10000 -> 20000 = 3.4x
20000 -> 40000 = 4.02x
40000 -> 80000 = 4.3x
80000 -> 160000 = 4.16x
