# cellular-automaton

This cli enables creation and simulation of a toroidal [cellular automaton](https://mathworld.wolfram.com/CellularAutomaton.html "Cellular Automaton - Wolfram"). 

## Configuring an Automaton

to create your cellular automaton you need to supply it with the following configuration files:
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

   Note that neighborhood size is calculated as 3^{num of dimensions} (you will get an error if a rule does not conform to that standard). This can be a pain in the ass for anything larger than two dimensions. I know. 

   Another important note is that this implementation only supports "nearest neighbor"-neighborhoods, I.e only cells that share and edge or a vertex with the middle cell are considered neighboring cells.

   Example of Rule110 config:
```json
{
	"rules": [
		{
			"neighborhood": [0, 0, 0],
			"cell": 0
		},
		{
			"neighborhood": [0, 0, 1],
			"cell": 1
		},
		{
			"neighborhood": [0, 1, 0],
			"cell": 1
		},
		{
			"neighborhood": [0, 1, 1],
			"cell": 1
		},
		{
			"neighborhood": [1, 0, 0],
			"cell": 0
		},
		{
			"neighborhood": [1, 0, 1],
			"cell": 1
		},
		{
			"neighborhood": [1, 1, 0],
			"cell": 1
		},
		{
			"neighborhood": [1, 1, 1],
			"cell": 0
		}
	]
}
```
   (Also available in config/rule110_rules.json)

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

