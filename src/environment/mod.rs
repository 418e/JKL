use crate::{expressions::TronType, utils::TronError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod clock_impl;
use clock_impl::clock_impl;
mod get_globals;
use get_globals::get_globals;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Rc<RefCell<HashMap<String, TronType>>>,
    pub value_types: Rc<RefCell<HashMap<String, String>>>,
    locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: get_globals(),
            value_types: Rc::new(RefCell::new(HashMap::new())),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        }
    }
    pub fn get_value_type(&self, name: &str) -> Option<String> {
        self.value_types.borrow().get(name).cloned()
    }
    pub fn set_value_type(&self, name: String, type_annotation: String) {
        self.value_types.borrow_mut().insert(name, type_annotation);
    }
    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        for (key, val) in locals.iter() {
            self.locals.borrow_mut().insert(*key, *val);
        }
    }
    pub fn enclose(&self) -> Environment {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            value_types: self.value_types.clone(),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        }
    }
    pub fn define(&self, name: String, value: TronType) {
        self.values.borrow_mut().insert(name, value);
    }
    pub fn get(&self, name: &str, expr_id: usize) -> Option<TronType> {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.get_internal(name, distance)
    }
    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<TronType> {
        if distance.is_none() {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get_internal(name, distance),
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    None => {
                        TronError::throw("E3004", 0, vec![]);
                        Some(TronType::Null)
                    }
                    Some(env) => {
                        assert!(distance > 0);
                        env.get_internal(name, Some(distance - 1))
                    }
                }
            }
        }
    }
    pub fn assign(&self, name: &str, value: TronType, expr_id: usize) -> bool {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.assign_internal(name, value, distance)
    }
    fn assign_internal(&self, name: &str, value: TronType, distance: Option<usize>) -> bool {
        if distance.is_none() {
            match &self.enclosing {
                Some(env) => env.assign_internal(name, value, distance),
                None => match self.values.borrow_mut().insert(name.to_string(), value) {
                    Some(_) => true,
                    None => false,
                },
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow_mut().insert(name.to_string(), value);
                true
            } else {
                match &self.enclosing {
                    None => {
                        TronError::throw("E3005", 0, vec![]);
                        false
                    }
                    Some(env) => env.assign_internal(name, value, Some(distance - 1)),
                };
                true
            }
        }
    }
}
