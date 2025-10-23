🧩 mini_db — Sequential Project Specification (Part 1)
📘 Goal

You’ll build a tiny database in Rust, step by step — starting with a command-line tool that can store and retrieve data from memory. Later, you’ll add persistence, querying, and concurrency.

🪜 Step 1 — Create Your Project
Objective

Initialize a new Rust binary project called mini_db.

Instructions

In your terminal:

cargo new mini_db
cd mini_db

Confirm it runs:

cargo run

You should see Hello, world!.

🪜 Step 2 — Define the Data Model
Objective

Represent a single database row.

Specification

Create a new file: src/model.rs

Define a struct named Row.

Each row should have:

id: u32

name: String

age: u8

Derive at least Debug, Clone, and PartialEq traits.

Example Behavior

If you print a row, it should look like:

Row { id: 1, name: "Alice", age: 30 }

🪜 Step 3 — Create the Database Structure
Objective

Store multiple Rows in memory.

Specification

In a new file src/engine.rs define:

A struct named Database.

It should hold a vector of rows: Vec<Row>.

A method new() returning an empty database.

Methods to Define
Method Parameters Returns Description
new none Self Creates an empty DB
insert id: u32, name: String, age: u8 Result<(), String> Adds a row to memory
select_all none Vec<Row> Returns all rows currently stored
Behavior

After inserting multiple rows, calling select_all should return them all in the order inserted.

Crates

No external crates yet — just standard std and your own modules.

🪜 Step 4 — Add a Command Loop (REPL)
Objective

Let the user type commands into the terminal.

Specification

In src/main.rs:

Import your engine::Database.

Create a loop that:

Prompts for user input.

Reads a line.

Passes it to a function that interprets the command.

The commands supported at this stage:

insert <id> <name> <age>

select

exit

Methods to Define
Method Parameters Returns Description
handle_command input: &str, db: &mut Database none Parses user command and executes action
Example Interaction

> insert 1 alice 30
> Inserted row with id 1.
> select
> Row { id: 1, name: "alice", age: 30 }
> exit
> Bye!

Crates

Still none — all done with std::io.

🪜 Step 5 — Add Basic Error Handling
Objective

Replace generic strings with structured errors.

Specification

Add a new file: src/error.rs

Define an enum DbError with variants:

InvalidCommand

DuplicateId

ParseError

Update your methods (like insert) to return Result<_, DbError> instead of Result<_, String>.

Crate to Add
cargo add thiserror

Use #[derive(thiserror::Error, Debug)] and give each variant a friendly message.

Example Behavior

If the user types:

> insert 1

Output:

Error: invalid command syntax

If the user tries to insert the same id twice:

> insert 1 alice 30
> insert 1 bob 28

Output:

Error: duplicate id 1

🪜 Step 6 — Separate Parsing Logic
Objective

Keep command parsing isolated from execution.

Specification

Create src/parser.rs

Define:

pub enum Command {
Insert { id: u32, name: String, age: u8 },
SelectAll,
Exit,
}

Implement:

pub fn parse(input: &str) -> Result<Command, DbError>

It should split by whitespace and interpret user commands.

Behavior Example
Input Output (Command)
"insert 1 alice 30" Command::Insert { id: 1, name: "alice", age: 30 }
"select" Command::SelectAll
"exit" Command::Exit
🪜 Step 7 — Wire Everything Together
Objective

Main REPL loop now uses the parser and executes commands.

Specification

In src/main.rs:

Read a line of input.

Call parser::parse(input).

Match on the Command result and call the appropriate method in Database.

Print output or errors.

Example Interaction

> insert 2 bob 25
> Inserted row with id 2.
> select
> Row { id: 1, name: "alice", age: 30 }
> Row { id: 2, name: "bob", age: 25 }
> exit
> Goodbye!

🪜 Step 8 — Testing the Core
Objective

Validate correctness before adding persistence.

Tests to Write (tests/basic.rs)

Creating an empty DB returns no rows.

Inserting adds a row.

Selecting returns inserted rows.

Duplicate IDs return an error.

Parsing “insert 1 alice 30” returns correct Command.

Run:

cargo test

🪜 Step 9 — Add Documentation & Help Command
Objective

Add internal documentation and a help command.

Specification

Add doc comments (///) to each public struct/method.

Add a command help that prints available commands.

Update the REPL to recognize it.

Behavior Example

> help
> Available commands:
> insert <id> <name> <age>
> select
> exit

✅ At This Point

You have a working in-memory database CLI with:

Command parsing

Error handling

Clean module structure

Tests

🧩 mini_db — Sequential Project Specification (Part 2: Persistence)
📘 Goal

Extend your in-memory database from Part 1 to store data on disk.
When the program restarts, previously inserted rows must still be available.

🪜 Step 1 — Introduce a Storage Module
Objective

Isolate all file-handling logic from the main database code.

Specification

Create a new file: src/storage.rs

Define a public struct Storage that manages:

A path to a data file (PathBuf).

Methods to append entries, read entries, and initialize storage.

Required Methods
Method Parameters Returns Purpose
new path: impl AsRef<Path> Self Create a new storage object.
append_entry &Row Result<(), IoError> Append one record to disk.
load_all none Result<Vec<Row>, IoError> Read all saved records.
Crate to Add
cargo add serde serde_json

Notes

Each row will be written as one line of JSON.

File format example:

{"id":1,"name":"alice","age":30}
{"id":2,"name":"bob","age":25}

Use serde::{Serialize, Deserialize} on your Row struct (in model.rs).

🪜 Step 2 — Update Your Database Struct
Objective

Connect Database to the storage layer.

Specification

In engine.rs:

Add a storage: Storage field.

Change the new() function to accept a file path (for example, "data.jsonl").

On startup:

Call storage.load_all() to populate rows.

When inserting a row:

First append it to the storage file.

Then push it into the in-memory vector.

Behavior

Running twice should show persistent state:

$ cargo run

> insert 1 alice 30
> exit
> $ cargo run
> select
> Row { id: 1, name: "alice", age: 30 }

🪜 Step 3 — Handle I/O Errors Gracefully
Objective

Add robust error handling for file operations.

Specification

Extend your DbError enum (in error.rs) with:

IoError(std::io::Error)

SerializationError(serde_json::Error)

Use the #[from] attribute from thiserror to convert automatically.

Example Behavior

If the data file can’t be opened:

Error: could not open data file at ./data.jsonl (No such file or directory)

🪜 Step 4 — Define the Log Format (Append-Only)
Objective

Think of the storage file as a simple log rather than a static snapshot.

Specification

Never rewrite the file on insert; only append.

Each line represents one database action.

For now, only one action type: Insert(Row).

Define in storage.rs:

pub enum LogEntry {
Insert(Row)
}

(You’ll use it later when you add deletion.)

Behavior

Appending rows produces a file like:

{"Insert":{"id":1,"name":"alice","age":30}}
{"Insert":{"id":2,"name":"bob","age":25}}

Crate to Add (optional but useful)
cargo add fs-err

This crate wraps standard file I/O with better error messages.

🪜 Step 5 — Implement Recovery Logic
Objective

When the database starts, it must rebuild state from the log file.

Specification

In Database::load_all(), iterate over all lines in the file.

For each line:

Deserialize it as a LogEntry.

Apply the operation to the in-memory vector.

Behavior

If the last record is corrupted (e.g., incomplete JSON):

Print a warning, skip that line, continue loading others.

Do not panic.

🪜 Step 6 — Add a “Reset” Command (Optional)
Objective

Let users clear all data for testing.

Specification

Add a command reset to the REPL.

This command:

Clears the in-memory rows.

Truncates the data file (File::create(path) with no append mode).

Example Interaction

> reset
> All data cleared.
> select
> (no rows)

🪜 Step 7 — Write Persistence Tests
Objective

Confirm that data persists across runs.

Tests to Add (tests/persistence.rs)

Start with a temporary directory (use tempfile crate).

cargo add tempfile --dev

Create a database, insert rows, drop it.

Reopen using the same path, verify rows reappear.

Truncate file manually, ensure DB handles it.

Example Test Names

test_persistence_across_sessions

test_corrupted_entry_ignored

Run all:

cargo test

🪜 Step 8 — Add Timestamps to Log Entries (Optional but Good)
Objective

Track when each record was written.

Specification

Add a field timestamp: i64 (Unix epoch seconds) to LogEntry.

Use chrono crate.

Crate to Add
cargo add chrono

Behavior

Your data file now includes timestamps:

{"Insert":{"row":{"id":1,"name":"alice","age":30},"timestamp":1730123456}}

🪜 Step 9 — Graceful Shutdown (Flush)
Objective

Ensure all writes reach disk before exiting.

Specification

When user enters exit:

Explicitly flush the file handle (file.sync_all()).

Wrap file handles in a struct so you can close them safely.

Behavior

If the program crashes mid-insert, only that last record is lost.

🪜 Step 10 — Documentation and README Update
Objective

Explain your storage design.

Checklist for the README:

Describe the on-disk format (JSON lines or binary).

Explain the append-only log model.

Document what happens on startup (replay).

Mention how corrupted lines are handled.

Describe how flushing ensures durability.

✅ End of Part 2

At this stage, your database can:

Run from the terminal.

Persist rows between runs.

Handle file corruption gracefully.

Flush data safely on shutdown.

Pass persistence tests.

🧩 mini_db — Sequential Project Specification (Part 3: Querying & Deletion)
📘 Goal

Add real querying capability and controlled deletion to your persistent database.
You’ll extend your parser, engine, and storage layers so users can filter rows (SELECT WHERE id=…) and remove them (DELETE WHERE id=…).

🪜 Step 1 — Define a New Command Variant
Objective

Extend your command parser to understand SELECT WHERE and DELETE WHERE.

Specification

Open src/parser.rs.

Add two new variants to your Command enum:

SelectById { id: u32 }

DeleteById { id: u32 }

Update the parse() function to recognize:

select where id=<number>
delete where id=<number>

All tokens are lowercase; ignore extra spaces.

Example Input → Expected Output
Input Parsed Command
select where id=2 Command::SelectById { id: 2 }
delete where id=5 Command::DeleteById { id: 5 }
Error Handling

Missing id= → DbError::ParseError

Non-numeric id → DbError::ParseError

Tests to Write

Valid command parses correctly.

Invalid syntax is rejected.

🪜 Step 2 — Update the Storage Log Format
Objective

Your append-only log must now capture deletions too.

Specification

In storage.rs, extend LogEntry enum:

Add Delete { id: u32 } variant.

When writing a delete, append a line like:

{"Delete":{"id":2}}

At startup, when you replay the log:

Apply Insert entries by adding rows.

Apply Delete entries by removing matching rows.

File Format Example
{"Insert":{"id":1,"name":"alice","age":30}}
{"Insert":{"id":2,"name":"bob","age":25}}
{"Delete":{"id":1}}

After replay, only Bob remains.

Crates Used

No new ones beyond serde for serialization.

🪜 Step 3 — Add Delete Behavior to the Database
Objective

Let users remove rows by id, affecting both memory and disk.

Specification

In engine.rs:

Method Parameters Returns Behavior
delete_by_id id: u32 Result<bool, DbError> If row exists, append Delete to log and remove from memory. Return true if something was deleted.
select_by_id id: u32 Result<Option<Row>, DbError> Return the row with matching id or None.
Rules

If no row exists with that id, return Ok(false) for delete.

No panics on missing ids.

Deleting and reinserting same id should work cleanly and persist properly.

🪜 Step 4 — Integrate with the REPL
Objective

Wire the new commands into your interactive loop.

Specification

In main.rs:

When you match on Command, add arms for:

SelectById { id } → call db.select_by_id(id)

DeleteById { id } → call db.delete_by_id(id)

Print messages like:

Found: Row { id: 2, name: "bob", age: 25 }

Row 2 deleted.

No row with id 5.

🪜 Step 5 — Prevent Duplicate IDs on Insert
Objective

Keep the database consistent.

Specification

Before inserting:

Check whether id already exists in memory.

If so, return DbError::DuplicateId.

Do not append to log on failure.

Tests to Write

Insert same id twice → error.

Delete then reinsert → works fine.

🪜 Step 6 — Implement Indexing for Fast Lookups
Objective

Make SELECT WHERE id=... fast instead of linear.

Specification

Create src/index.rs.

Define IdIndex struct mapping u32 → usize (or direct row reference).

Database now contains both:

rows: Vec<Row>

index: IdIndex

Maintain index consistency on insert and delete:

On insert: add mapping to index.

On delete: remove mapping.

Crates

None needed; std::collections::HashMap is sufficient.

Performance Expectation

After 10 000 rows, SELECT WHERE id=... should feel instantaneous.

🪜 Step 7 — Test Rebuild and Index Recovery
Objective

Ensure the index can be rebuilt from log entries after restart.

Specification

At startup, after replaying the log, rebuild the index in memory.

Write a test that:

Inserts 10 rows.

Deletes some.

Restarts the database (reload from file).

Checks that select_by_id returns the correct subset.

Crate for Temp Files

If not already added:

cargo add tempfile --dev

🪜 Step 8 — Extend the Help Command
Objective

Include new commands in the help message.

Output Example
Available commands:
insert <id> <name> <age>
select
select where id=<id>
delete where id=<id>
reset
exit

🪜 Step 9 — Edge Case Testing
Objective

Prove robustness and correctness.

Tests to Add
Scenario Expected Result
Delete non-existent id Returns Ok(false) with message “No row found.”
Corrupted Delete entry in log Startup skips that line gracefully.
Insert then delete then reinsert same id Row appears only once in final state.
Large log replay (>10k entries) Startup completes without errors.

Run cargo test after each phase.

🪜 Step 10 — Performance Measurement (Optional)
Objective

Get a feel for efficiency and index impact.

Specification

Use std::time::Instant to measure:

Time to insert 10 000 rows.

Time to query random ids before and after indexing.

Print simple benchmark results in debug mode.

Output Example
Inserted 10 000 rows in 512 ms
Average SELECT time (indexed): 50 µs

🪜 Step 11 — Update the README
Checklist

Explain query syntax (SELECT WHERE, DELETE WHERE).

Document the log format with Insert/Delete examples.

Describe how the index works and is rebuilt.

List known limitations (no range queries yet).

✅ End of Part 3

At this stage your database can:

Handle inserts, deletes, and selects (by ID and full table).

Recover exact state after restarts.

Maintain an in-memory index for fast lookups.

Gracefully handle corruption and duplicates.

🧩 mini_db — Sequential Project Specification (Part 4: Compaction & Concurrency)
📘 Goal

Keep storage lean by periodically compacting the append-only log into a snapshot.

Make reads and writes thread-safe with clear concurrency rules.

By the end of this part, your DB will start quickly from a snapshot, keep its log small, and support concurrent reads with safe, exclusive writes.

🪜 Step 1 — Introduce a Snapshot File (Checkpoint)
Objective

Create a compact on-disk representation of the current table state to avoid replaying a huge log on startup.

Storage Design

Keep your existing append-only log for durability of new mutations.

Add a snapshot file that contains a full copy of the current table (at the moment of compaction).

On startup: load snapshot first, then apply any log entries newer than the snapshot.

Files

data/mini_db.snapshot (full table at last compaction)

data/mini_db.log (append-only operations since snapshot)

Module & Methods to Define

In storage (new or existing files as you prefer):

Method Parameters Returns Purpose
snapshot_write rows: &[Row], path: &Path Result<(), IoError> Serialize the entire current table to a temporary file, then atomically rename to .snapshot.
snapshot_read path: &Path Result<Vec<Row>, IoError> Load all rows from the snapshot.
log_truncate_or_rotate path: &Path Result<(), IoError> After snapshot, either truncate the log to empty or rotate it to a new file.
log_iter_since path: &Path, since_ts: Option<i64> Result<Iterator<LogEntry>, IoError> (Optional) If you stamp entries, load only entries after snapshot’s watermark. Otherwise, load all and rely on content.

Atomicity rule: Always write snapshot to mini_db.snapshot.tmp, fsync, then rename to mini_db.snapshot.

Crates (if not already added)

serde + your chosen format (serde_json or bincode)

chrono (if you use timestamps in the snapshot header)

fs-err (optional, for clearer I/O errors)

Acceptance Criteria

After compaction, restarting the DB should be noticeably faster than replaying the full log.

Snapshot write is atomic (no partial files observed after crash during compaction).

🪜 Step 2 — Define Compaction Triggers
Objective

Decide when to compact without manual intervention.

Policy (choose at least one):

Size-based: compact when mini_db.log exceeds X MB (e.g., 5–20 MB).

Count-based: compact after N appended entries (e.g., every 50k).

Manual command: compact in REPL (keep this even if you add auto-trigger).

Engine Methods to Define (in engine)
Method Parameters Returns Purpose
should_compact none bool Decide based on thresholds (size / count).
compact none Result<(), DbError> Create snapshot from in-memory state; rotate/truncate log; rebuild index from in-memory data; ensure crash-safety.
Rules

Write-order guarantee: During compaction, do not lose any committed entries. Suspend new writes (see concurrency later).

On success: the log is empty (or newly rotated), snapshot reflects current state.

On failure: leave previous snapshot and log untouched (atomic rename behavior ensures this).

🪜 Step 3 — Startup Sequence with Snapshot
Objective

Load the database efficiently and correctly.

Boot Order

If snapshot exists: load it into memory.

Rebuild the index from loaded rows.

Replay the log after snapshot, applying Insert/Delete in order.

Rebuild (or update) index as you apply the log.

Engine Method to Define/Update
Method Parameters Returns Purpose
load_from_disk paths: {snapshot, log} Result<Self, DbError> Full bootstrap: snapshot → index → log replay → index update.
Acceptance Criteria

Starting from a snapshot + small log should be significantly faster.

If snapshot is absent, behave as before (replay full log).

If snapshot is corrupted: emit clear error and fall back to log replay (document this policy).

🪜 Step 4 — Crash Safety Model for Compaction
Objective

Guarantee no torn states if crash occurs during compaction.

Measures

Write snapshot to a temp file and fsync.

Only then rename to the final snapshot filename.

Only after snapshot rename succeeds, clear/rotate the log (and fsync directory entries if you want to be rigorous).

Storage Methods to Add (if you want explicit fsync)
Method Parameters Returns Purpose
flush_file file_handle Result<(), IoError> Ensure data durability via sync_all.
flush_dir dir_path Result<(), IoError> Ensure directory entry durability (optional on some platforms).

You don’t have to implement full POSIX rigor for this assignment, but document what you do and don’t guarantee.

🪜 Step 5 — Concurrency Model: Readers–Writer
Objective

Allow many readers concurrently, but only one writer (insert/delete/compact) at a time.

Choice A (simplest to implement)

Guard the engine’s mutable state (rows, index, file handles) with a single RwLock.

Reads (select, select_by_id) acquire read lock.

Writes (insert, delete, compact) acquire write lock.

Choice B (more structured)

A single writer thread processes commands from a queue (channel).

Readers can read a snapshot of in-memory state behind an RwLock.

Writers serialize all mutations through the worker; readers acquire only read lock.

Pick one and stick to it.

Crates to Consider

parking_lot (drop-in faster locks)

cargo add parking_lot

(Optional) Channels for Choice B: crossbeam-channel or std::sync::mpsc.

Engine Types & Methods to Define

If Choice A (RwLock):

Type/Method Purpose
DatabaseHandle (public) Holds Arc<RwLock<Database>> and exposes thread-safe methods.
insert(&self, ...) -> Result<(), DbError> Takes write lock inside; appends to log; updates memory/index.
delete_by_id(&self, id) -> Result<bool, DbError> Takes write lock; appends delete; updates memory/index.
select_by_id(&self, id) -> Result<Option<Row>, DbError> Takes read lock; uses index.
select_all(&self) -> Result<Vec<Row>, DbError> Takes read lock; returns copy or iterator policy you design.
compact(&self) -> Result<(), DbError> Takes write lock for whole operation.

If Choice B (Writer thread):

Define a CommandMsg enum (Insert, Delete, Compact, …) sent to the writer.

Writer mutates state and replies via oneshot channel.

Reads may directly hold a read lock to latest state.

Rule: During compact, block new writes and allow read-only ops (if you can, otherwise block briefly). Document the chosen behavior.

🪜 Step 6 — Update the REPL for Concurrency
Objective

Trigger compaction and allow parallel read testing.

REPL additions

Add compact command.

Optionally add spawn_readers <n> <seconds> (for testing parallel selects).

Behavior

compact prints progress and outcome (Compaction complete in X ms).

If concurrency tests run, they should not panic or deadlock.

🪜 Step 7 — Tests: Compaction Correctness
New Test Cases (integration)

Snapshot Correctness

Insert N rows.

Run compact.

Restart DB.

select returns exactly N rows; order/index valid.

Log After Snapshot

Compact.

Insert M more rows.

Restart DB.

Expect N+M rows.

Idempotent Compaction

Compact twice without intervening writes.

State unchanged; startup still fast.

Corrupt Snapshot Fallback (policy-based)

Corrupt snapshot file.

Startup either (a) fails with clear error, or (b) falls back to log replay.

Document which you chose; test it.

🪜 Step 8 — Tests: Concurrency Safety
New Test Cases (integration / threaded)

Readers During Writes

Start K reader threads continuously running select_by_id or select.

Start a writer thread inserting/deleting in a loop.

Run for T seconds.

Assert no panics, no deadlocks, and post-conditions hold (e.g., duplicates disallowed).

Compaction Under Load

While reads are happening, trigger compact.

Check final state matches expected state.

Time the compaction (for your README).

Uniqueness Invariant

With concurrent inserts (same id), ensure only one succeeds (writer-serialized), or your API returns a DuplicateId error deterministically.

For deterministic tests, you can control thread scheduling by batching operations or seeding RNG for id selection.

🪜 Step 9 — Performance & Startup Benchmarks
What to Measure

Startup time before vs. after snapshot (same dataset).

Compaction time for N rows (e.g., 50k).

Query latency (select_by_id) with index vs. without (from earlier part).

README Checklist

Table with dataset size, log size, snapshot size.

Startup times before/after compaction.

Compaction duration and trigger policy.

Concurrency model and guarantees (who blocks whom).

🪜 Step 10 — Operational Commands (Optional but Recommended)
Add CLI/REPL admin commands

status — print counts, log size, snapshot presence, last compact time.

verify — rebuild state from log+snapshot into a temp area and compare with in-memory state; report mismatches.

dump snapshot|log — write human-readable diagnostics for debugging.

These greatly help you (and reviewers) validate correctness.

🪜 Step 11 — Durability Options (Optional)
Flags / configs

--fsync=always | on-compact | never

--compact-threshold-size=<MB>

--compact-threshold-entries=<N>

Document the trade-offs and defaults.

✅ End of Part 4

What you have now

Checkpointing (snapshot) to keep startup fast and log small.

Crash-aware compaction with atomic rename.

Thread-safe API supporting concurrent reads and exclusive writes.

A solid test matrix for snapshot correctness and concurrent behavior.
