#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct VarMap {
    pub name: Symbol,
    pub value: u32
}

#[contracttype]
#[derive(Clone)]
pub struct Expr {
    pub expression: Vec<Symbol>,
    pub variables: Vec<VarMap>,
}

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum MathEl {
    U32(u32),
    Symbol(Symbol)
}

impl MathEl {
    fn explicit(self) -> u32 {
	let val = match self {
	    MathEl::U32(val) => val,
	    _ => panic!()
	};
	return val
    }
}

fn math_el_add(lhs: MathEl, rhs: MathEl) -> MathEl  {
    let lhs_val = match lhs {
	MathEl::U32(val) => val,
	_ => panic!("failed unluck {:?}", lhs)
    };
    let rhs_val = match rhs {
	MathEl::U32(val) => val,
	_ => panic!()
    };

    MathEl::U32(lhs_val + rhs_val)
}

fn math_el_sub(lhs: MathEl, rhs: MathEl) -> MathEl {
    let lhs_val = match lhs {
	MathEl::U32(val) => val,
	_ => panic!()
    };
    let rhs_val = match rhs {
	MathEl::U32(val) => val,
	_ => panic!()
    };

    MathEl::U32(lhs_val - rhs_val)
}

pub fn new_expr(expr_vec: Vec<Symbol>, vars: Vec<VarMap>) -> Expr {
    Expr {
	expression: expr_vec,
	variables: vars,
    }
}

pub fn eval(env: &Env, expression: Expr) -> bool {
    let mut comp_idxs = Vec::new(&env);
    let mut comp_op: MathEl = MathEl::U32(0);
    let math_expr = to_math(&env, &expression);
    
    for expr_idx in 0..math_expr.len() {
	if math_expr.get(expr_idx).unwrap().unwrap() == MathEl::Symbol(Symbol::from_str("eq")) || math_expr.get(expr_idx).unwrap().unwrap() == MathEl::Symbol(Symbol::from_str("gt")) || math_expr.get(expr_idx).unwrap().unwrap() == MathEl::Symbol(Symbol::from_str("lt")) || math_expr.get(expr_idx).unwrap().unwrap() == MathEl::Symbol(Symbol::from_str("gg")) || math_expr.get(expr_idx).unwrap().unwrap() == MathEl::Symbol(Symbol::from_str("ll")) {
	    comp_idxs.push(expr_idx);
	    comp_op = math_expr.get(expr_idx).unwrap().unwrap()
	}
    };

    if comp_idxs.len() == 1 {
	let left = math_expr.slice(0..comp_idxs.get_unchecked(0).unwrap());
	let right = math_expr.slice(comp_idxs.get_unchecked(0).unwrap()+1..math_expr.len());
	let mut lhs_res: MathEl;
	let mut rhs_res: MathEl;

	lhs_res = math_el_add(left.get_unchecked(0).unwrap(), MathEl::U32(0));
	rhs_res = math_el_add(right.get_unchecked(0).unwrap(), MathEl::U32(0));
	
	if left.len() > 1 {
	    for sub_idx in 0..left.len() {
		if left.get_unchecked(sub_idx).unwrap() == MathEl::Symbol(Symbol::from_str("add")) {
		    lhs_res = math_el_add(lhs_res, left.get_unchecked(sub_idx + 1).unwrap());
		} else if left.get_unchecked(sub_idx).unwrap() == MathEl::Symbol(Symbol::from_str("sub")) {
		    lhs_res = math_el_sub(lhs_res, left.get_unchecked(sub_idx + 1).unwrap());
		};
	    };
	}

	if right.len() > 1 {
	    for sub_idx in 0..right.len() {
		if right.get_unchecked(sub_idx).unwrap() == MathEl::Symbol(Symbol::from_str("add")) {
		    rhs_res = math_el_add(rhs_res, right.get_unchecked(sub_idx + 1).unwrap());
		} else if right.get_unchecked(sub_idx).unwrap() == MathEl::Symbol(Symbol::from_str("sub")) {
		    rhs_res = math_el_sub(rhs_res, right.get_unchecked(sub_idx + 1).unwrap());
		};
	    };
	}

	let op_symbol = match comp_op {
	    MathEl::Symbol(symbol) => symbol,
	    _ => panic!()
	};

	let equal_op = Symbol::from_str("eq");
	let gt_op = Symbol::from_str("gt");
	let gg_op = Symbol::from_str("gg");
	let ll_op = Symbol::from_str("ll");
	let lt_op = Symbol::from_str("lt");

	let lhs_value = lhs_res.explicit();
	let rhs_value = rhs_res.explicit();

	if op_symbol == equal_op {
	    let out: bool = lhs_value == rhs_value;
	    return out
	} else if op_symbol == gt_op {
	    let out: bool = lhs_value >= rhs_value;
	    return out
	} else if op_symbol == gg_op {
	    let out: bool = lhs_value > rhs_value;
	    return out
	} else if op_symbol == ll_op {
	    let out: bool = lhs_value < rhs_value;
	    return out
	} else if op_symbol == lt_op {
	    let out: bool = lhs_value <= rhs_value;
	    return out
	} else {
	    panic!("unsupported comparison operator")
	}
	
    } else {
	panic!("unsupported expression with none, two, or more comparison operators")
    }
}

pub fn to_math(env: &Env, expression: &Expr) -> Vec<MathEl> {
    let mut math_expr: Vec<MathEl> = Vec::new(env);
    let mut var_names: Vec<Symbol> = Vec::new(env);
    for var_idx in 0..expression.variables.len() {
	var_names.push(expression.variables.get(var_idx).unwrap().unwrap().name);
    }

    for sym_idx in 0..expression.expression.len() {
	if &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("eq") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("gt") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("lt") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("gg") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("ll") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("noneq") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("add") || &expression.expression.get(sym_idx).unwrap().unwrap() == &Symbol::from_str("sub") {
	    math_expr.push(MathEl::Symbol(expression.expression.get(sym_idx).unwrap().unwrap()))
	} else {
	    let index = match expression.variables.iter().position(|r| r.unwrap().name == expression.expression.get(sym_idx).unwrap().unwrap()) {
		Some(idx) => idx as u32,
		None => panic!("expression uses undeclared vars")
	    };

	    let value:u32 = expression.variables.get(index).unwrap().unwrap().value;
	    math_expr.push(MathEl::U32(value))
	}
    };

    math_expr
}




pub struct ExprEvalCTR;

#[contractimpl(export_if = "export")]
impl ExprEvalCTR {
    /// Simply store the expression on the ledger data
    pub fn store_expr(env: Env, expression: Expr, expr_id: Symbol) {
	env.contract_data().set(expr_id, expression);
    }

    pub fn eval_expr(env: Env, expr_id: Symbol) -> bool {
	let expression = env.contract_data().get(expr_id).unwrap().unwrap();
	eval(&env, expression)
    }
}



mod test;
