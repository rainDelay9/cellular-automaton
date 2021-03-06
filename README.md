# cellular-automaton

This cli enables creation and simulation of a toroidal [cellular automaton](https://mathworld.wolfram.com/CellularAutomaton.html "Cellular Automaton - Wolfram"). 

![Conway's GoL gif](https://upload.wikimedia.org/wikipedia/commons/4/4f/Animated_Hwss.gif)

Heavyweight spaceship (above) is a periodic configuration in GoL. It can be seen in one of the provided example configs.

## Example

Before we begin configuration - a look at an example output.

![Example output gif](resources/cellular-automaton.gif)

Example of an execution of a GoL Pulsar - oscillator with period 3. As you can see, my gif making abilities are... well... less than ideal.


## Configuring an Automaton

You are provided with two examples of configuration under config/rule110 and config/game_of_life (which is set to the glider configuration - feel free to play around). Currently too large of dimensions do not print very well. This will be fixed in the future.
to create your cellular automaton you need to supply it with three configuration files:
1. Dimensions of the board:

   Since this is a toroidal implementation, something like
```json
{
	"dimensions": [30,20,10]
}
```
   would give you a 3-dimensional automaton of dimensions 30x20x10.

   See config/rule110/dimensions.json for reference.

2. Coordinates of starting state 

```json
{
	"coordinates": [
		[0, 15, 8],
		[1, 1, 6],
		[15, 5, 0],
		[20, 17, 4],
		[11, 11, 1],
		[12, 11, 3],
		[13, 0, 7]
	]
}
```

   See config/rule110/coordinates.json for reference.

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
  
  where "neighberhood" is a vector of the encompassing coordinate values of size (3^{dimensions} - 1) of the immediate neighbors, "current" is the current value of the cell, and "next" is the value of the cell in the next generation.
  
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
path-to-cellular-automaton-bin/cellular-automaton --help
```
for instructions on how to use the cli,
or
```
cargo run -- [parameters for cli]
```
for another method of running.

