// Copyright (C) 2022 Jade
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use crate::ast::{Graph, GraphType, ID};
use petgraph::graph::{DiGraph as PetDiGraph, UnGraph as PetUnGraph};
use std::collections::HashMap;

pub type DiGraph = PetDiGraph<HashMap<ID, ID>, HashMap<ID, ID>>;
pub type UnGraph = PetUnGraph<HashMap<ID, ID>, HashMap<ID, ID>>;

pub enum PetGraph {
    Directed(DiGraph),
    Undirected(UnGraph),
}

impl Graph {
    pub fn to_petgraph(&self) -> PetGraph {
        match self.graph_type {
            GraphType::Digraph => PetGraph::Directed(self.to_directed()),
            GraphType::Graph => PetGraph::Undirected(self.to_undirected()),
        }
    }
    fn to_directed(&self) -> DiGraph {
        unimplemented!()
    }
    fn to_undirected(&self) -> UnGraph {
        unimplemented!()
    }
}
