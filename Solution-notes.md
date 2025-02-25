There were several different approaches to the problem of alternating the moves and keeping track of where the "real" and the "Robo" Santas were.

**Keep track of how many moves have been made**

- Create a [`Santas`](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-alwin/blob/a532fc6c12d29721acf9c590ac0401e6b418d4a9/src/bin/part2.rs#L23)
  struct that has the position of the two Santas _and_ the total number of moves made so far.
- [Use the number of moves to decide whether the next move goes](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-alwin/blob/a532fc6c12d29721acf9c590ac0401e6b418d4a9/src/bin/part2.rs#L103)
  to the "real" Santa or the "Robo" Santa.
- The `Pos + Direction` logic is duplicated.

**Keep a boolean to track which Santa goes next**

This is very similar to the previous one, but:

- The data is all [just added to `VisitedHouses`](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-linnea-tristan/blob/ed0680cb83ad52fb3d6f466f971406b2c0ab1f1d/src/bin/part2.rs#L34)
  instead of making a new `Santas` type.
- The state is stored as a `bool` instead of counting the number of moves.
- They use the `Add` trait [to simplify the move logic](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-linnea-tristan/blob/ed0680cb83ad52fb3d6f466f971406b2c0ab1f1d/src/bin/part2.rs#L65).

**Attach the index to the moves**

[This uses a `for` loop and `.enumerate()` to attach indices to moves](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-andreas/blob/fd9e28cae851deb5975152563b0fbb3f5ddc8d41/src/bin/part2.rs#L116).
The logic is then similar to the that used in the first solution, but without needing to track the number of moves in the `Santas` type.

**Attach the index to the moves at the time of parsing**

This is essentially the same as the previous solution, but it attaches the index [when the moves are parsed](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-ken_malena/blob/cb936a035caa3022bcd43c3b036fc0e9649575ef/src/bin/part2.rs#L63)
instead of [when they're processed](https://github.com/UMM-CSci-4611-S25/aoc-2015-day-3-part-2-ken_malena/blob/cb936a035caa3022bcd43c3b036fc0e9649575ef/src/bin/part2.rs#L115).
This works, but arguably complicates things somewhat. I guess the question here is whether one needs the attached index anywhere other than the processing step.
If you don't (which we don't here), then it's probably easier to do it at processing time. If you do, however, then this is a good approach.
