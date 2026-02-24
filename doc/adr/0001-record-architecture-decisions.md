# 1. Record architecture decisions

Date: 2026-02-24

## Status

Accepted

## Context

We need to record the architectural decisions made on this project for future reference and for the orchestrator to maintain context across sessions.

## Decision

We will use Architecture Decision Records, as described by Michael Nygard in [Documenting Architecture Decisions](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions).

ADRs will be stored in `doc/adr/` and numbered sequentially.

## Consequences

- Architectural decisions will be documented and versioned alongside code
- The orchestrator can reference prior decisions when making new ones
- Human operators can understand the rationale behind design choices
- ADRs are append-only — superseded decisions are marked as such, not deleted
