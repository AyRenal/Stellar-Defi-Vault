# Storage-layout snapshot (issue #659)

In-place upgrades (`upgrade_wasm`) are only safe when the new code can still
read state written by the old code. Every value the vault stores is encoded in
Soroban's XDR form, which is derived mechanically from the `#[contracttype]`
definition. That means an *incompatible* change to a type changes the bytes on
disk without any compiler error, silently orphaning live state:

- reordering or renaming fields of a `#[contracttype]` struct,
- changing a field's integer width (`u32` → `u64`, `i128` → `i64`),
- reordering or removing `#[contracttype]` enum variants,
- adding a new required field.

`src/test_storage_layout.rs` guards against this. It serialises a curated set of
core keys and values to their XDR form and compares the result against the
committed golden file `test-data/storage_layout.snapshot`.

## What is covered

| Entry | Type |
| --- | --- |
| `DataKey` unit variants (`Admin`, `Token`, `TotalShares`, `TotalDeposited`, `RewardPoolBalance`, `Paused`, `LeaderboardSize`, `VestingPeriod`, `UnstakeFeeBps`) | `struct`/`enum` |
| `DataKey` address-keyed variants (`ShareBalance`, `AccruedReward`, `StakedAtLedger`, `UnbondingPosition`, `VestingEntries`) | `struct`/`enum` |
| `ContractDelegate` | `struct` |

Add a row whenever a new core type is introduced; the point is to cover the
types whose encoding the accounting and per-user state depend on.

## Running

```bash
cargo test --features testutils test_storage_layout
```

The test fails, naming the offending entry, when the encoding no longer matches
the snapshot.

## Updating the snapshot after an intentional change

A layout change is only allowed **together with a migration** for any
already-deployed contract. To refresh the golden file as part of such a change:

1. Add the migration (read the old encoding, rewrite it in the new one).
2. Regenerate the snapshot:

   ```bash
   UPDATE_STORAGE_SNAPSHOT=1 cargo test --features testutils test_storage_layout
   ```

3. Confirm the test passes without the env var:

   ```bash
   cargo test --features testutils test_storage_layout
   ```

4. Commit `test-data/storage_layout.snapshot` **in the same PR as the migration**,
   and note both in the PR description.

If the snapshot file is missing (fresh checkout or after deletion), the test
writes it and reports that it must be committed.
