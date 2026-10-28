# hax bounded integers

Newtypes for bounded integers (`BoundedU8<MIN, MAX>`, `BoundedI64<MIN, MAX>`, ...) carrying their bounds as refinements understood by [hax](https://github.com/pereq-dev/hax): the bounds are available as invariants in the proofs, and checked at construction in debug builds.
