# Fetch-before-decode boundary

The current instruction-fetch classifier has no SP-IMEM instruction-fetch
route. SP-IMEM addresses remain rejected before reading or decoding any
backing bytes. The opaque implementation adds no route and therefore cannot
turn the private sentinel into an instruction identity.
