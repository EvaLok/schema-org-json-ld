# Language Evaluation: Next Implementation Target

## Scope and baseline

This review compares the current PHP implementation under `php/src/v1/` and the TypeScript implementation under `ts/src/` to determine the best next language target for `schema-org-json-ld`.

The current library architecture is consistent across both implementations:

- schema types are small data containers
- serialization is centralized in `JsonLdGenerator` / `JsonLdGenerator::SchemaToObject()`
- controlled vocabularies use enums
- hyphenated JSON-LD keys use property maps
- some schema types rely on inheritance chains
- tests construct schemas, serialize them, and assert on the resulting JSON structure

> Note: the issue text mentions `toArray()` / `toJsonLd()` style methods, but the current repository uses centralized generator helpers rather than per-class serialization methods. That difference does not materially change the portability analysis; it mostly affects how reflective or generic the serializer must be in the target language.

## Comparison matrix

| Language | Pattern fit | Testing fit | Estimated effort | Why it fits / struggles |
| --- | --- | --- | --- | --- |
| **Kotlin / Java** | **Very strong** | **Strong** | **Medium** | Natural mapping for class hierarchies, enums, property annotations/maps, and immutable data containers. Kotlin especially matches the TypeScript options-object ergonomics through named arguments and data classes. |
| **C# / .NET** | **Strong** | **Strong** | **Medium** | Good support for enums, inheritance, records/classes, JSON attributes, and predictable testing with xUnit/NUnit. Slightly more serializer-configuration work than Kotlin for schema.org key remapping and null omission conventions. |
| **Python** | **Moderate to strong** | **Very strong** | **Medium** | Excellent JSON handling and low-friction test porting with pytest, but weaker static guarantees around enums, unions, and inheritance-heavy schema modeling unless the project adopts dataclasses or Pydantic-style discipline. |
| **Go** | **Moderate** | **Moderate** | **High** | JSON support is excellent and table-driven tests are easy to scale, but lack of inheritance makes the existing subtype hierarchy the least portable of the top candidates. More manual serializer code would be required. |
| **Ruby** | **Moderate** | **Strong** | **Medium to high** | Ruby can model the API ergonomically and pairs well with RSpec, but it shares Python’s dynamic-typing tradeoffs without Python’s larger downstream ecosystem and library momentum. |

## Ranked recommendation

### 1. Kotlin (with Java interoperability as a secondary benefit)

Kotlin is the best next target.

It maps most naturally to the library’s existing patterns:

- **Options-style construction** translates well to named arguments and data-class-like modeling.
- **Enums** are first-class and ergonomic.
- **Inheritance hierarchies** are natural and preserve the current subtype structure better than Go.
- **Property mapping** can be expressed clearly with annotations or explicit serializer metadata.
- **Null handling** is explicit and close to the TypeScript and PHP designs.

Kotlin also offers a practical ecosystem advantage: a Kotlin implementation can serve Kotlin-first users while still being comfortable for Java consumers. That broadens reach without forcing the project into the verbosity of a Java-first design.

### 2. C# /.NET

C# is the strongest non-JVM alternative.

It provides:

- mature JSON tooling
- strong typing and inheritance support
- clean enum support
- modern record/class syntax for schema containers
- highly portable test patterns through xUnit or NUnit

Compared with Kotlin, C# is slightly less natural for reproducing the TypeScript-style “single options object” ergonomics unless the library deliberately standardizes on records or request objects. It is still a strong candidate and likely easier to keep structurally close to the current codebase than Python or Go.

### 3. Python

Python is the best ecosystem-driven option, but not the best pattern-fit option.

If the deciding factor is install-base and downstream adoption, Python deserves serious consideration. However, the current codebase relies on explicit type relationships, controlled vocabularies, and inheritance in ways that are more naturally expressed in Kotlin or C# than in idiomatic Python without extra framework choices.

Python becomes much more attractive if the project prioritizes:

- fastest contributor onboarding
- widest packaging reach
- easiest prototype iteration

It becomes less attractive if the project prioritizes:

- long-term parity enforcement across many schema classes
- strong guarantees around schema shape
- minimal ambiguity in union-heavy fields

### 4. Go

Go is attractive for deployment simplicity and strong JSON tooling, but it is a worse fit for the current object model. The library’s subtype hierarchy and serializer conventions would need a more opinionated redesign, especially around inheritance and heterogeneous field types.

### 5. Ruby

Ruby is viable, but it is the least compelling of the listed candidates when balancing ecosystem impact and architectural fit. It can express the DSL-like side of the library well, yet offers fewer advantages than Python in reach and fewer advantages than Kotlin/C# in type fidelity.

## Candidate-specific implementation challenges

### Top candidate: Kotlin

1. **Choosing the construction API**
   - Kotlin can support named parameters directly, which is excellent for consumers.
   - The challenge is deciding whether to mirror the TypeScript “options object” pattern with parameter objects, or to embrace idiomatic Kotlin constructors and keep parity through documentation and tests rather than exact API shape.

2. **Serializer strategy for property maps and null omission**
   - The current PHP/TypeScript design relies on a central serializer that skips nulls, handles arrays/unions, and remaps keys like `mathExpression-input`.
   - Kotlin would need a deliberate choice between reflection-based serialization and explicit serializers/annotations. Either works, but the choice affects maintenance burden and parity enforcement.

3. **Representing union-heavy fields**
   - Fields such as `offers`, `review`, and `size` accept multiple shapes.
   - Kotlin can model these, but doing so cleanly may require sealed interfaces/classes or carefully documented `Any`-like escape hatches. The wrong choice could make the public API either too loose or too cumbersome.

### Second candidate: C#

1. **Balancing ergonomic constructors with strong typing**
   - C# can model schema containers cleanly, but reproducing the TypeScript ergonomics may require records plus init-only properties or dedicated options types.
   - Without a clear convention, the API could become inconsistent across simple and complex schema classes.

2. **Handling schema unions without degrading API clarity**
   - The library contains many “string or schema or array” shapes.
   - C# does not have native union types, so the implementation would need wrappers, overloads, interfaces, or custom converters. This is solvable, but it is a repeated design cost.

3. **Configuring serialization for schema.org conventions**
   - The target implementation must preserve `@context`, `@type`, null omission, enum value emission, and per-property remapping.
   - .NET JSON libraries can do this, but some of the behavior will need attributes or custom converter logic rather than the simpler reflective model used today.

## Challenges by remaining candidates

### Python

1. **Type discipline across a large schema surface**
   - Python can express the models, but maintaining strict parity across dozens of schema classes is harder without enforcing dataclasses/Pydantic-style conventions.

2. **Union-heavy fields become runtime-validated rather than compiler-guided**
   - Many current fields accept multiple shapes. Python handles that flexibly, but correctness shifts more toward tests and runtime validation.

3. **Inheritance and serializer conventions need stronger project rules**
   - Python supports inheritance, but consistency is easier to drift without strong linting/type-checking discipline.

### Go

1. **No class inheritance**
   - Existing chains such as `LocalBusiness -> FoodEstablishment -> Restaurant` do not port directly and would need composition or field embedding.

2. **Union types and heterogeneous arrays are awkward**
   - Fields that currently accept schema-or-array-or-primitive shapes would require interfaces and custom marshaling.

3. **Property remapping and omission logic become more explicit**
   - Go’s JSON tags help with naming, but reproducing all current generator behavior across nested polymorphic structures adds manual effort quickly.

### Ruby

1. **Weaker static guarantees for parity maintenance**
   - Ruby can express the API ergonomically, but long-term drift is harder to control without leaning heavily on tests.

2. **Serializer conventions would rely on discipline over compiler structure**
   - Property maps, null omission, and enum-like vocabularies are implementable, but enforcement is less structural than in Kotlin or C#.

3. **Lower strategic upside than Python**
   - Even if implementation is pleasant, the ecosystem payoff is smaller than Python and the type fidelity is weaker than Kotlin/C#.

## Testing portability assessment

The current PHPUnit and Vitest suites are structurally simple and portable:

1. construct a schema
2. serialize it
3. decode/parse JSON
4. assert required keys and omitted null fields

That pattern ports most cleanly to:

- **pytest** in Python
- **JUnit 5 / Kotest** in Kotlin
- **xUnit / NUnit** in C#

Of those, **pytest is the easiest direct test-authoring experience**, but **Kotlin and C# preserve the production-model structure more faithfully**. Since the implementation burden will dominate over the test burden for this library, pattern fit should carry more weight than raw test familiarity.

## Final recommendation

If the goal is the **best architectural fit with the current library design**, choose **Kotlin** next.

If the goal is the **best balance of strong typing, web ecosystem relevance, and maintainable parity**, rank the candidates:

1. **Kotlin**
2. **C# /.NET**
3. **Python**
4. **Go**
5. **Ruby**

If Eva instead wants to optimize primarily for **ecosystem reach over structural fidelity**, Python is the only candidate that plausibly overtakes Kotlin. Otherwise, Kotlin is the strongest next target after PHP and TypeScript.
