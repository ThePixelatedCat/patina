use std::{collections::HashMap, iter};

/// A concrete type that has been fully inferred
#[derive(Debug)]
enum Type {
    Int,
    UInt,
    Byte,
    Float,
    Bool,
    Str,
    Char,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Func(Vec<Type>, Box<Type>),
    Struct(String, Vec<Type>),
    Enum(String, Vec<Type>),
}

/// A identifier to uniquely refer to our type terms
pub type TypeId = usize;

/// Information about a type term
#[derive(Clone, Debug)]
enum TypeInfo {
    // No information about the type of this type term
    Unknown,
    // This type term is the same as another type term
    Ref(TypeId),
    Int,
    UInt,
    Byte,
    Float,
    Bool,
    Str,
    Char,
    Array(TypeId),
    Tuple(Vec<TypeId>),
    Func(Vec<TypeId>, TypeId),
    Struct(String, Vec<TypeId>),
    Enum(String, Vec<TypeId>),
}

#[derive(Default)]
struct Engine {
    id_counter: usize, // Used to generate unique IDs
    vars: HashMap<TypeId, TypeInfo>,
}

impl Engine {
    /// Create a new type term with whatever we have about its type
    pub fn insert(&mut self, info: TypeInfo) -> TypeId {
        // Generate a new ID for our type term
        self.id_counter += 1;
        let id = self.id_counter;
        self.vars.insert(id, info);
        id
    }

    /// Make the types of two type terms equivalent (or produce an error if
    /// there is a conflict between them)
    pub fn unify(&mut self, a: TypeId, b: TypeId) -> Result<(), String> {
        use TypeInfo::*;
        match (self.vars[&a].clone(), self.vars[&b].clone()) {
            // Follow any references
            (Ref(a), _) => self.unify(a, b),
            (_, Ref(b)) => self.unify(a, b),

            // When we don't know anything about either term, assume that
            // they match and make the one we know nothing about reference the
            // one we may know something about
            (Unknown, _) => {
                self.vars.insert(a, TypeInfo::Ref(b));
                Ok(())
            }
            (_, Unknown) => {
                self.vars.insert(b, TypeInfo::Ref(a));
                Ok(())
            }

            // Primitives are trivial to unify
            (Int, Int) => Ok(()),
            (UInt, UInt) => Ok(()),
            (Byte, Byte) => Ok(()),
            (Float, Float) => Ok(()),
            (Bool, Bool) => Ok(()),
            (Str, Str) => Ok(()),
            (Char, Char) => Ok(()),

            // When unifying complex types, we must check their sub-types. This
            // can be trivially implemented for tuples, sum types, etc.
            (Tuple(a), Tuple(b)) => self.unify_vec(a, b),
            (Array(a_item), Array(b_item)) => self.unify(a_item, b_item),
            (Func(a_i, a_o), Func(b_i, b_o)) => {
                self.unify_vec(a_i, b_i).and_then(|_| self.unify(a_o, b_o))
            }
            (Struct(a_n, a_g), Struct(b_n, b_g)) => (a_n == b_n)
                .then_some(())
                .ok_or_else(|| format!("Conflict between {a_n:?} and {b_n:?}"))
                .and_then(|()| self.unify_vec(a_g, b_g)),

            // If no previous attempts to unify were successful, raise an error
            (a, b) => Err(format!("Conflict between {a:?} and {b:?}")),
        }
    }

    pub fn unify_vec(&mut self, a: Vec<TypeId>, b: Vec<TypeId>) -> Result<(), String> {
        iter::zip(a, b).try_for_each(|(a, b)| self.unify(a, b))
    }

    /// Attempt to reconstruct a concrete type from the given type term ID. This
    /// may fail if we don't yet have enough information to figure out what the
    /// type is.
    pub fn reconstruct(&self, id: TypeId) -> Result<Type, String> {
        use TypeInfo::*;
        match &self.vars[&id] {
            Unknown => Err("Cannot infer".to_string()),
            Ref(id) => self.reconstruct(*id),
            Int => Ok(Type::Int),
            UInt => Ok(Type::UInt),
            Byte => Ok(Type::Byte),
            Float => Ok(Type::Float),
            Str => Ok(Type::Str),
            Char => Ok(Type::Char),
            Bool => Ok(Type::Bool),
            Tuple(types) => Ok(Type::Tuple(self.reconstruct_vec(types)?)),
            Array(item) => Ok(Type::Array(Box::new(self.reconstruct(*item)?))),
            Func(i, o) => Ok(Type::Func(
                self.reconstruct_vec(i)?,
                Box::new(self.reconstruct(*o)?),
            )),
            Struct(name, generics) => {
                Ok(Type::Struct(name.clone(), self.reconstruct_vec(generics)?))
            }
            Enum(name, generics) => Ok(Type::Struct(name.clone(), self.reconstruct_vec(generics)?)),
        }
    }

    pub fn reconstruct_vec(&self, ids: &[TypeId]) -> Result<Vec<Type>, String> {
        ids.iter().map(|i| self.reconstruct(*i)).collect()
    }
}