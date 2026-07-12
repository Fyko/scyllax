# ADR 0001: Request coalescing contract

- Status: Accepted
- Date: 2026-07-11

## Context

Request coalescing is core Scyllax behavior, but its semantics have been distributed across hash generation, channels, and result cloning. Treating a hash, routing metadata, or a convenient subset of values as identity can join requests whose observable results differ. Paging, cancellation, shutdown, concurrency, and output ownership also need stable rules before their implementations evolve.

## Decision

Scyllax adopts the following behavioral contract:

1. Identity uses hash and equality; hash equality alone is insufficient. Hashing narrows candidate lookup, and equality confirms complete identity.
2. By default, identity includes every query value. A custom identity is an explicit expert override whose author promises that requests it equates have equal observable results.
3. Routing and partition hints are separate from identity. They influence driver locality, never request equivalence.
4. Distinct identities may run concurrently behind a configured bound.
5. Followers receive the leader's same terminal outcome: one success or one typed failure is consistently fanned out.
6. Follower cancellation detaches only that follower. The leader continues while followers remain. Cancellation of all followers may cancel the leader only when cleanup is deterministic.
7. No completed result survives removal from the in-flight registry. Coalescing is not a cache.
8. Query type, query values, read mode, page size or state, and selected profile are identity inputs whenever they alter returned values, cardinality, ordering, cursor position, or failure semantics.
9. A paged cursor position is a separate identity. The initial implementation coalesces individual page fetches or explicitly bounded collected reads; it never shares one mutable stream.
10. Shutdown stops admission, resolves or cancels leaders according to a documented policy, resolves followers, and terminates associated tasks.
11. Internal fan-out may share outcomes. Compatibility methods may clone owned output until a shared public mode exists.

## Consequences

- Identity representations must retain equality-checkable inputs rather than only a digest.
- Query APIs that add result-affecting controls must account for them in identity before coalescing.
- Runtime implementations may change their map, channel, semaphore, or task primitives without changing this contract.
- Cancellation and shutdown paths must deterministically resolve attached callers and remove registry entries.
- Coalescing reduces duplicate in-flight work but provides no completed-result reuse.

## Rejected alternatives

- **Hash-only identity:** collisions can combine observably different requests.
- **Routing-key identity:** locality metadata does not describe the complete result.
- **Global cross-executor coalescing:** executor-local policy and lifecycle boundaries must remain independent.
- **Silent partial rows:** incomplete success violates the single terminal-outcome guarantee.
- **Implicit caching:** retaining completed results changes freshness, memory, and failure semantics beyond coalescing.
