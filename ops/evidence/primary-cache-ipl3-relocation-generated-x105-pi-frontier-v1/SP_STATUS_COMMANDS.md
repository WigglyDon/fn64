# SP_STATUS commands

| PC | Word | Physical target | Resulting halt | broke | interrupt pending | single step | interrupt on break |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0xA4000490 | 0x000000CE | 0x04040010 | true | false | false | true | false |
| 0xA4000508 | 0x000000AD | 0x04040010 | false | false | false | false | false |

The first command halts/configures the SP before relocation. The second occurs
in the JR delay slot and records the generated start request. Both preserve
unrelated Machine truth and use ordinary store cadence.
