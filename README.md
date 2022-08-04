# Soroban-compatible primitive Expressions Evaluator

Implementation of fully-contextual helper functions to evaluate a very basic subset of expressions and returning a boolean.

For instance, if you want your smart contract to evaluate this string: `a + b >= c` where `a = 1`, `b = 2`, and `c = 3`, you might find this code useful. Since this code was extracted from a larger contract I'm building that only needed to evaluate very simple expressions, the helper funcitons only allow additions (as well as subtractions). No logical operators or other operations are supported.

### Evaluating an expression
As showed in the `src/test.rs` file, expressions are a structure were the expression string is a vector of `Symbols`, where each synbol is either an operator or a variable.

The functions assume that everything in the expression is either an operator or a variable to be added to the context, so in an expression like `a - 2 == 1`, you'll have to define to which value `1` and `2` correspond to.

Also, since everything is based on the `Symbol` type, operators can't contain characters like `==` or `+`, as a result, these are the supported operators:

- `eq` -> `==`
- `gt` -> `>=`
- `gg` -> `>`
- `lt` -> `<=`
- `ll` -> `<<`

Finally, let's build an expression in our tests:

```rust
use super::{VarMap, new_expr, eval};
use soroban_sdk::{Env, FixedBinary, vec, Symbol};

#[test]
fn test() {
	...
	let expr = new_expr(
		vec![&env,
			Symbol::from_str("a"),
			Symbol::from_str("add"),
			Symbol::from_str("b"),
			Symbol::from_str("eq"),
			Symbol::from_str("6"),
		],
			
		vec![&env,
			VarMap {
				name: Symbol::from_str("a"),
				value:5
			},
			VarMap {
				 name: Symbol::from_str("6"),
				 value:6
			 },
			 VarMap {
				 name: Symbol::from_str("b"),
				 value:1
			 }
	]
    ); // a add b eq 6 where a = 5 and b = 2 (and 6 = 6)
	   // a + b == 6
	   
	let result: bool = eval(&env, expr);
	assert_eq!(result, true); // test should pass since 5 + 1 == 6 is true
	...
}
```

## Implementing in Soroban Contracts
These functions are designed to be helpers function to easily be used from a contract function invocation.


**This implementation is not optimized nor production-ready**
