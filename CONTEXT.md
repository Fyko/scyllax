# Scyllax domain context

Request coalescing is required product behavior. It applies only while work is in flight and is scoped to one executor instance and one query type.

## Canonical terms

- **request identity:** An equality-backed description of all inputs that determine complete result equivalence for one query type. A hash may accelerate lookup, but equality establishes identity.
- **leader:** The single database operation admitted for an identity while that operation is in flight.
- **follower:** A caller attached to the leader and resolved with its terminal result instead of starting another database operation.
- **in-flight registry:** The runtime mapping from identities to active leaders and their followers. Entries exist only until resolution; this is not a completed-result cache.
- **routing hint:** Driver locality metadata used to choose where work runs. It never determines identity.
- **terminal outcome:** The one success or typed failure produced by an operation and consistently fanned out to every attached caller.
