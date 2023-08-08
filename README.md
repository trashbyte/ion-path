## ion-path

An alternative to XPath / JsonPath / YAML Path / etc for [Amazon Ion](https://amazon-ion.github.io/ion-docs/).

Interoperates with (and depends on) the [ion-rs](https://github.com/amazon-ion/ion-rust) crate.

Currently *very* early in development and probably very buggy. Don't depend on it for anything important. However, if you'd like to try it out and find the aforementioned bugs, I'd appreciate it!

### Query Feature Roadmap

- [x] Query by field name
  - [x] `/key` searches for a field named "key" in the current context.
- [x] Query by index
  - [x] `/3` returns the fourth item (zero-based index `3`) in a sequence.
- [x] Query by slice
  - [x] `/1:3` returns items at indices 1 through 3 (inclusive).
  - [x] `/4:`, `/:-1` - supports half-open ranges and negative indices.
  - [x] `/3:-2:2` - supports Python-style `start:end(:step)` syntax.
- [x] Query by annotations
  - [x] `/A::B::*` returns all elements annotated with both `A` and `B`
  - [x] `/(A|B)::*` returns all elements annotated with either `A` or `B`
- [x] Path subquery predicate
  - [x] `/*[subfield]` filters matched elements to those with a field called "subfield".
  - [x] `/*[a/b[c/d != "e"]/f]` - supports all top-level query features and can be nested arbitrarily deep.
- [x] Value comparison predicate
  - [x] `/*[field >= value]` filters results to elements with a field called "field", whose values are greater than `value`.
  - [x] supports all common comparison operators (`==` or `=`, `!=`, `<`, `>`, `<=`, `>=`)
  - [x] left-hand side can be any valid path, with any features and nested arbitrarily deep
  - [x] right-hand side can be any non-collection Ion literal type (no List, SExp, or Struct)
  - [x] can also compare against self by omitting the LHS: `/*[!= null]` matches all (`*`) that are not `null`.
  - [x] can also match against the root level element(s) by using an absolute path: `//A::*[/B::*[valid=true]]` returns all elements annotated with `A` at any level of the document, but only if the document has a root level element annotated with `B` that has the field `valid: true`. 
- [x] Combining predicates
  - [x] `/A[B = "C"][D/E != "F"]` - predicates can be chained to filter elements to those that match all of the predicates.
  - [x] `/A[B = "C" or D/E != "F"]` - predicates can be combined using "or" to filter elements to those that match any of the predicates.
- [ ] Recursive descent
  - [ ] `//` searches children at any depth recursively

### Other Feature Roadmap

- [ ] Visitor types/traits for patching Ion documents / `Element`s.
- [ ] Open to suggestions for other features!

### Limitations

- Symbol IDs are not resolved (a symbol `$10` is parsed as a symbol with the name `'$10'`.)
- Line breaks inside query strings may not be handled correctly in some cases. `ion-path` was made with the assumption 
that queries would almost always be written on one line. Feel free to open an issue if `ion-path` fails to handle your
use case properly.
- Supports BigInt literals for value matching, but indices/slice bounds are `i32`.
- Some operations with `Decimal`s do not respect `-0 != 0`.