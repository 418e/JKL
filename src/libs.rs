use crate::environment::*;
use crate::expr::*;
use crate::natives::*;
use std::rc::Rc;

pub fn include_array_natives(environment: &mut Environment) {
    environment.define(
        "push".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "push".to_string(),
            arity: 1,
            fun: Rc::new(native_push as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "join".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "join".to_string(),
            arity: 1,
            fun: Rc::new(native_join as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "pop".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "pop".to_string(),
            arity: 1,
            fun: Rc::new(native_pop as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "shift".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "shift".to_string(),
            arity: 1,
            fun: Rc::new(native_shift as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
}

pub fn include_math_natives(environment: &mut Environment) {
    environment.define(
        "sin".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "sin".to_string(),
            arity: 1,
            fun: Rc::new(native_sin as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "asin".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "asin".to_string(),
            arity: 1,
            fun: Rc::new(native_asin as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "cos".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "cos".to_string(),
            arity: 1,
            fun: Rc::new(native_cos as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "acos".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "acos".to_string(),
            arity: 1,
            fun: Rc::new(native_acos as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "tan".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "tan".to_string(),
            arity: 1,
            fun: Rc::new(native_tan as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "atan".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "atan".to_string(),
            arity: 1,
            fun: Rc::new(native_atan as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "round".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "round".to_string(),
            arity: 1,
            fun: Rc::new(native_round as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "floor".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "floor".to_string(),
            arity: 1,
            fun: Rc::new(native_floor as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "ceil".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "ceil".to_string(),
            arity: 1,
            fun: Rc::new(native_ceil as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "pow".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "pow".to_string(),
            arity: 2,
            fun: Rc::new(native_pow as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "root".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "root".to_string(),
            arity: 2,
            fun: Rc::new(native_root as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "random".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "random".to_string(),
            arity: 2,
            fun: Rc::new(native_random as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "min".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "min".to_string(),
            arity: 1,
            fun: Rc::new(native_min as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "max".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "max".to_string(),
            arity: 1,
            fun: Rc::new(native_max as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "log".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log".to_string(),
            arity: 1,
            fun: Rc::new(native_log as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "log2".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log2".to_string(),
            arity: 1,
            fun: Rc::new(native_log2 as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
    environment.define(
        "log10".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log10".to_string(),
            arity: 1,
            fun: Rc::new(native_log10 as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );

    environment.define(
        "to_degrees".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "to_degrees".to_string(),
            arity: 1,
            fun: Rc::new(native_todgrees as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );

    environment.define(
        "to_radians".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "to_radians".to_string(),
            arity: 1,
            fun: Rc::new(native_toradians as fn(&Vec<LiteralValue>) -> LiteralValue),
        })),
    );
}
