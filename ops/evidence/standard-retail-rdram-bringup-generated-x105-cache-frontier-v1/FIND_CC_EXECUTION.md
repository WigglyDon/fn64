# FindCC Execution

For each present module, four `FindCC` calls test nominal manual values zero
through seven. With score increments of ten, the guest accumulator is:

`t3 = 10 * (0 + 1 + 2 + 3 + 4 + 5 + 6 + 7) = 280`.

The pinned transform produces `a0 = 280 * 22 - 880 = 5280`. Each successful
call then executes `ConvertManualToAuto` and returns seven. `InitCCValue`
averages four sevens and returns seven.

For the absent probe, four `FindCC` calls each test all 64 nominal values and
return zero. `InitCCValue` returns zero, causing the generated loop1 branch to
terminate without fabricating a third module.
