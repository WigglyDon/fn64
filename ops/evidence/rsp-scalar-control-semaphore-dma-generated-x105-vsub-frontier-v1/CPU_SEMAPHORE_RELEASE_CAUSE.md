# Guest CPU Semaphore Release Cause

The only release observed in the public composition is an ordinary guest CPU
store:

- CPU instruction PC: `0x800000B0`;
- identity: represented `Sw`;
- scalar source: `r0`, architectural available zero;
- stored word: `0x00000000`;
- effective address: `0xFFFFFFFFA404001C`;
- target: singular Sp-owned `SP_SEMAPHORE`;
- old state: set;
- new state: clear.

That CPU instruction commits alone. It does not commit an RSP instruction.
The next selected RSP `Mfc0 SP_SEMAPHORE` observes old clear as zero and
atomically sets the semaphore under the existing owner rule.
