use crate::lang::ast;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

type CompStore = HashMap<ast::Id, ast::Structure>;

impl ast::Component {
    //==========================================
    //        Structure Helper Functions
    //==========================================

    pub fn get_wires(&self) -> Vec<ast::Wire> {
        let mut v: Vec<ast::Wire> = Vec::new();
        for structure in self.structure.iter() {
            match structure {
                ast::Structure::Wire { data } => v.push(data.clone()),
                _ => {}
            }
        }
        v
    }

    pub fn get_std(&self) -> Vec<ast::Std> {
        let mut v: Vec<ast::Std> = Vec::new();
        for structure in self.structure.iter() {
            match structure {
                ast::Structure::Std { data } => v.push(data.clone()),
                _ => {}
            }
        }
        v
    }

    pub fn get_decl(&self) -> Vec<ast::Decl> {
        let mut v: Vec<ast::Decl> = Vec::new();
        for structure in self.structure.iter() {
            match structure {
                ast::Structure::Decl { data } => v.push(data.clone()),
                _ => {}
            }
        }
        v
    }
    pub fn get_store(&self) -> CompStore {
        let mut store: CompStore = HashMap::new();
        let std = self.get_std();
        let new = self.get_decl();
        for inst in std {
            store.insert(inst.name.clone(), ast::Structure::Std { data: inst });
        }
        for inst in new {
            store
                .insert(inst.name.clone(), ast::Structure::Decl { data: inst });
        }
        store
    }

    pub fn has_input_port(&self, port: String) -> bool {
        for in_port in &self.inputs {
            if in_port.name == port {
                return true;
            }
        }
        false
    }

    pub fn has_output_port(&self, port: String) -> bool {
        for out_port in &self.outputs {
            if out_port.name == port {
                return true;
            }
        }
        false
    }

    pub fn get_port_width(&self, port: &String) -> i64 {
        for in_port in &self.inputs {
            if in_port.name == *port {
                return in_port.width;
            }
        }
        for out_port in &self.outputs {
            if out_port.name == *port {
                return out_port.width;
            }
        }
        panic!("Non-existent port: Port {}, Component {}", port, self.name)
    }
}