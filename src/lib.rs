//! # Mini Database
//!
//! A simple, lightweight database implementation with log-based persistence.
//!
//! ## Overview
//!
//! This database provides basic CRUD operations with persistence through an append-only log.
//! It features:
//! - In-memory storage with disk persistence
//! - Index-based lookups for efficient ID-based queries
//! - Log replay for data recovery after restart
//! - Simple JSON-based storage format
//!
//! ## Modules
//!
//! - `model`: Data structures for database rows
//! - `engine`: Core database operations and management
//! - `errors`: Custom error types for database operations
//! - `parser`: Command parsing and execution
//! - `storage`: Persistence layer with append-only log
//! - `index`: In-memory indexing for fast lookups

pub mod model;
pub mod engine;
pub mod errors;
pub mod parser;
pub mod storage;
pub mod index;