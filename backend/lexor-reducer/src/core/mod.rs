// The internal core module for the graph reduction machine. Contains
// the definitions and implementations of the engine, the arena allocator
// used by the engine, and the nodes forming the graph the machine is
// reducing.
pub mod arena;
pub mod engine;
pub mod node;
