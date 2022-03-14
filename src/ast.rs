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
use std::fmt::{self, Write};

#[derive(Debug)]
pub struct Graph {
    pub is_strict: bool,
    pub graph_type: GraphType,
    pub id: ID,
    pub stmt_list: Vec<Stmt>,
}
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_strict {
            f.write_str("strict ")?;
        }
        writeln!(f, "{} {} {{", self.graph_type, self.id)?;
        let write_stmt: Box<dyn Fn(&Stmt, &mut fmt::Formatter) -> fmt::Result> =
            match self.graph_type {
                GraphType::Graph => Box::new(|stmt, f| writeln!(f, "{stmt}")),
                GraphType::Digraph => Box::new(|stmt, f| writeln!(f, "{stmt:#}")),
            };
        for stmt in &self.stmt_list {
            write_stmt(stmt, f)?;
        }
        f.write_char('}')
    }
}
#[derive(Debug)]
pub enum GraphType {
    Graph,
    Digraph,
}
impl fmt::Display for GraphType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            GraphType::Graph => "graph",
            GraphType::Digraph => "digraph",
        })
    }
}
#[derive(Debug, PartialEq)]
pub enum ID {
    Name(String),
    Integer(isize),
    Float(f64),
    StringLiteral(String),
}
impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ID::Name(name) => f.write_str(name),
            ID::Integer(i) => write!(f, "{}", i),
            ID::Float(n) => write!(f, "{}", n),
            ID::StringLiteral(s) => write!(f, r#""{}""#, s),
        }
    }
}
#[derive(Debug)]
pub enum Stmt {
    Node {
        node_id: NodeID,
        attrs: Option<AttrList>,
    },
    Edge {
        start: EdgePoint,
        rhs: Vec<EdgePoint>,
        attrs: Option<AttrList>,
    },
    Attr {
        ty: AttrStmtType,
        attrs: AttrList,
    },
    Assign(ID, ID),
    Subgraph(Subgraph),
}
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Node { node_id, attrs } => {
                write!(f, "{node_id}")?;
                if let Some(attrs) = attrs {
                    write!(f, " {}", AttrListDisplay(attrs))?;
                }
                Ok(())
            }
            Stmt::Edge { start, rhs, attrs } => {
                write!(f, "{start} ")?;
                if f.alternate() {
                    write!(f, "{:#}", EdgePointsDisplay(&rhs))?;
                } else {
                    write!(f, "{}", EdgePointsDisplay(&rhs))?;
                }
                if let Some(attrs) = attrs {
                    write!(f, "{}", AttrListDisplay(attrs))?;
                }
                Ok(())
            }
            Stmt::Attr { ty, attrs } => write!(f, "{ty} {}", AttrListDisplay(attrs)),
            Stmt::Assign(lhs, rhs) => write!(f, "{lhs} {rhs}"),
            Stmt::Subgraph(subgraph) => write!(f, "{subgraph}"),
        }?;
        f.write_char(';')
    }
}

#[derive(Debug)]
pub struct NodeID {
    pub id: ID,
    pub port: Option<Port>,
}
impl fmt::Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(port) = &self.port {
            write!(f, "{}:{port}", self.id)?
        } else {
            write!(f, "{}", self.id)?;
        }
        Ok(())
    }
}
pub type AttrList = Vec<AList>;
struct AttrListDisplay<'a>(&'a AttrList);
impl<'a> fmt::Display for AttrListDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(' ')?;
        for a_list in self.0 {
            write!(f, "{}", AListDisplay(a_list))?;
        }
        Ok(())
    }
}
pub type AList = Vec<(ID, ID)>;
struct AListDisplay<'a>(&'a AList);
impl<'a> fmt::Display for AListDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[ ")?;
        for (lhs, rhs) in self.0 {
            //TODO: dont do trailing semicolon
            write!(f, "{lhs} = {rhs}; ")?;
        }
        f.write_char(']')
    }
}

#[derive(Debug)]
pub enum Port {
    ID(ID),
    IDCompass(ID, CompassPt),
    Compass(CompassPt),
}
impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Port::ID(id) => write!(f, "{id}"),
            Port::IDCompass(id, compass) => write!(f, "{id}:{compass}"),
            Port::Compass(compass) => write!(f, "{compass}"),
        }
    }
}
#[derive(Debug)]
pub enum CompassPt {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    C,
    Underscore,
}
impl fmt::Display for CompassPt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CompassPt::*;
        write!(
            f,
            "{}",
            match self {
                N => "n",
                NE => "ne",
                E => "e",
                SE => "se",
                S => "s",
                SW => "sw",
                W => "w",
                NW => "nw",
                C => "c",
                Underscore => "_",
            }
        )
    }
}

#[derive(Debug)]
pub enum EdgePoint {
    NodeID(NodeID),
    Subgraph(Subgraph),
}

impl fmt::Display for EdgePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EdgePoint::NodeID(id) => write!(f, "{id}"),
            EdgePoint::Subgraph(subgraph) => write!(f, "{subgraph}"),
        }
    }
}

struct EdgePointsDisplay<'a>(&'a [EdgePoint]);
impl<'a> fmt::Display for EdgePointsDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //FIXME Oh god this is a terrible hack why did i do this
        let edge_op = if f.alternate() { "->" } else { "--" };
        for edge_point in self.0 {
            write!(f, "{edge_op} {edge_point}")?;
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Subgraph {
    pub id: Option<ID>,
    pub stmt_list: Vec<Stmt>,
}
impl fmt::Display for Subgraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "subgraph ")?;
        if let Some(id) = &self.id {
            write!(f, "{id} ")?;
        }
        writeln!(f, "{{")?;
        for stmt in &self.stmt_list {
            writeln!(f, "{stmt}")?;
        }
        f.write_char('}')
    }
}

#[derive(Debug)]
pub enum AttrStmtType {
    Graph,
    Node,
    Edge,
}
impl fmt::Display for AttrStmtType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AttrStmtType::*;
        f.write_str(match self {
            Graph => "graph",
            Node => "node",
            Edge => "edge",
        })
    }
}
