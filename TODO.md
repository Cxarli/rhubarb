# TODO

* add generator back to input to compare optimisation
* optimisation
  * move as many clauses as possible to the front: `1 + (2 + 3) => (1 + 2) + 3`
  * remove`*0` and `*1`, `+0` and `-0`, `/1`
* use LXI to set register pairs at once
* add bitshifts and bitwise operators
  * optimise `*2` to `<< 1` and `/2` to `>> 1`
