# cellular-automaton

This cli enables creation and simulation of a toroidal [cellular automaton](https://mathworld.wolfram.com/CellularAutomaton.html "Cellular Automaton - Wolfram"). 

## Configuring an Automaton

to create your cellular automaton you need to supply it with three configuration files:
1. Dimensions of the board:

   Since this is a toroidal implementation, something like
```json
{
	"dimensions": [30,20,10]
}
```
   would give you a 3-dimensional automaton of dimensions 30x20x10.

   See config/rule110_dimensions.json for reference.

2. Coordinates of starting state 

```json
{
	"coordinates": [[0], [1], [15], [20], [11], [12], [13]]
}
```

   See config/rule110_coordinates.json for reference.

3. Rules - for each neighborhood its resulting middle cell.

  Rules have two forms of configuration - explicit and sum-based.
  
  **Important note:** Both types of configurations must be present, even if unused. For instance, if all of your rules are sum-based, there still needs to be an "explicit_rules" entry in the configuration json (in this case it should be just an empty array).
  
  An explicit rule is of the form:
  
```json
"explicit_rules": [
	{
		"neighborhood": [0, 0, 1, 0, 1, 0, 1, 0],
		"current": 0,
		"next": 1
	},
]
```
  
  where "neighberhood" is a vector of the encompassing coordinate values of size 3^{dimensions} of the immediate neighbors, "current" is the current value of the cell, and "next" is the value of the cell in the next generation.
  
  A sum-based rule is of the form:
```json
"sum_rules": [
	{
		"rule_type": 2,
		"neighborhood": 4,
		"current": 0,
		"next": 1
	},
]
 ```
where "neighborhood" is the sum of neighbor values, "current" is the current cell value and "next" is the cell's value in the next generation.

"rule_type" is one of three options:

0. equals - this rule will be applied if the neighborhood size equals the rule's neighborhood size.
1. larger - this rule will be applied if the neighborhood size is larger than the rule's neighborhood size.
2. smaller - this rule will be applied if the neighborhood size is smaller than the rule's neighborhood size.

Thus, the above rule says that if the current cell value is 0, and there are *less than* 4 activated cells in its neighborhood, then in the next generation the cell's value is 1.

Make sure your rules don't collide, as this can lead to unexpected behavior (the first rule that matches will be apllied).

For examples of config files you can go to config/game_of_life/rules_explicit.json for an example of explicit rule configuration, and config/game_of_life/rules_sum.json for an example of a sum-based rule configuration. You can also mix and match (have both sum-based and explicit rules).

In Conway's Game of Life you can see the big advantage of having sum-based rules. (5 rules as opposed to 310 in the explicit case)

## Building and Running

### Build
use 
```
cargo build [--release]
```
### CLI
use
```
path-to-encrypted-box-bin/encrypted-box --help
```
for instructions on how to use the cli,
or
```
cargo run -- [parameters for cli]
```
for another method of running.

