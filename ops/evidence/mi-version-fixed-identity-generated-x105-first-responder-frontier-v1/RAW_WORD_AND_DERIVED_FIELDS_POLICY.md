# Raw word and derived fields policy

`0x02020102` is the only stored canonical truth. Accessors derive byte fields
with shifts and masks. There is no setter, duplicate mutable field storage, or
stored branch-selection flag.
