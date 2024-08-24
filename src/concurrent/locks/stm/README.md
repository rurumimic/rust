# STM

- wiki: [Software transactional memory](https://en.wikipedia.org/wiki/Software_transactional_memory)

## Conceptual advantages and disadvantages

- Any operation performed within a transaction must be idempotent.
- If an operation has side effects, a rollback operation must be included.
- Deadlock and livelock are either prevented entirely.
- Composable operations.

## Transaction Locking II

### Write Transaction

1. Read global version-clock
  - copy global version-clock to local read-version
2. Run Transaction
  - save a address and data to a local write-set
  - read from write-set or memory
  - if stripe is locked and version of stripe is higher than read-version, abort transaction
3. Lock write-set
  - lock all stripes in write-set
  - if can't lock, abort transaction
4. increment global version-clock
  - atomically increment global version-clock
  - save incremented version to local write-version
5. validate read-set
  - if `read-version + 1 == write-version`, skip
  - if stripe's version is locked by other thread or stripe's version is higher than read-version, abort transaction
6. commit and release
  - write data to the addresses in the write-set
  - set stripe's version to the write-version
  - atomically unlock

### Read Transaction

1. Read global version-clock
  - copy global version-clock to local read-version
2. Run Transaction
  - if stripe is locked and version of stripe is higher than read-version, abort transaction
3. Commit

