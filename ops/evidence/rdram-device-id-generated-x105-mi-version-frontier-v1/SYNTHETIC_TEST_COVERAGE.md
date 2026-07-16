# Synthetic test coverage

Core tests cover aliases, generated/arbitrary-high sources, requested base, provenance, wrong values, unknown source, shared base/source rejection, pending-transfer blocking, narrow routes, absent read route, cadence, ordinary/delay-slot AdES, complete preservation, replacement, failed/successful bootstrap, reset, and independent Machines.

The step probe uses public `Machine::step` to commit DEVICE_ID, execute all 14 setup words, and prove atomic MI_VERSION rejection at `32176` commits.
