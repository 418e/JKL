use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expressions::{CallableImpl, StdFunctionImpl, TronType};

use super::clock_impl;

pub fn get_globals() -> Rc<RefCell<HashMap<String, TronType>>> {
    let mut env = HashMap::new();
    let fun_impl: StdFunctionImpl = StdFunctionImpl {
        name: "clock".to_string(),
        arity: 0,
        function: Rc::new(clock_impl),
    };
    let callable_impl = CallableImpl::StdFunction(fun_impl);
    env.insert("clock".to_string(), TronType::Callable(callable_impl));
    Rc::new(RefCell::new(env))
}
