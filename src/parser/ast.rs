//! The structs and enums needed to build the
//! abstract syntax tree for the program

/// An enum representing the declarations that
/// can be made at a global level
#[derive(Debug, PartialEq)]
enum Decl {}

/// An enum representing the possible statements
#[derive(Debug, PartialEq)]
enum Stmt {}

/// An enum representing the possible expressions
#[derive(Debug, PartialEq)]
enum Expr {}

/// An enum representing the possible elements
/// 
/// Elements produce one or more values
#[derive(Debug, PartialEq)]
enum Elmt {}

/// An enum representing the different kinds of
/// matches that can be made in match expressions
#[derive(Debug, PartialEq)]
enum Mtch {}