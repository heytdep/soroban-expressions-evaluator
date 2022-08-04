#![cfg(test)]

use super::{store_expr, eval_expr, ExprEvalCTR, VarMap, new_expr};
use soroban_sdk::{Env, FixedBinary, vec, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, ExprEvalCTR);

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
    ); // a add b eq 3 where a = 1 and b = 2 (and 3 = 3)

    const EXPR_ID: Symbol = Symbol::from_str("FIRST_EXPR");
    store_expr::invoke(&env, &contract_id, &expr, &EXPR_ID); // expression stored on ledger
    let res: bool = eval_expr::invoke(&env, &contract_id, &EXPR_ID);

    assert_eq!(res, true);
}
