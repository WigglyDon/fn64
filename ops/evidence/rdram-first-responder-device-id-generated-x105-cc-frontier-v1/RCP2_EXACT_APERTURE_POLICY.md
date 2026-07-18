# RCP 2.0 exact aperture policy

The fixed Machine MI_VERSION word selects the source-labelled RCP 2.0 path by
guest CPU comparison. Generated spacing is `0x400`; r11 remains
`0xFFFFFFFFA3F00000`; r17 becomes `0xFFFFFFFFA3F08000`.

Only physical `0x03F08004` is classified as the bounded first-responder target.
There is no bit mask, range, array, responder index, or branch-selection state
inside Rdram.
