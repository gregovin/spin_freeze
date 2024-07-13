spin_freeze is a command line tool for computing when spinners will freeze and how
To use it, you first must install spin_freeze. You can do so with [cargo](https://crates.io/crates/spin_freeze), or by installing the precompiled binaries and running them from a shell or adding them to your path.

Then, you simply run the spin_freeze binary(if you added it to your path, or cargo installed run `spin_freeze`) command with either wait or cycle as an argument
In wait mode you will be asked details relevant for assuming you spend the freeze doing no inputs

In cycle mode you will be asked details relevant for spending that time doing cyclic inputs

After you select your mode, you will be prompted for the following:

-   TimeActive: enter the time active value as measured on the last frame before the commands will be placed. You are encouraged to use a lot of decimals.
-   ChapterTime: enter the chapter time in frames at the same point you measured TimeActive
-   Frames to Freeze: enter the number of frames(chapterTime differential) from the end of the wait(or cycle assuming perfect alignment) to the frame freeze should occur
-   Cycle Length(only for cycle mode): enter the length of the cycle in frames. Exclude freeze frames.
-   Before/after(only for cycle mode): if the cycle does not align, allows you to decide if you will insert the waiting frames before or after before generating the commands. Enter b or before to select before, will default to after
