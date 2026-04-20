# Cross-Doc Audit: Spec Coherence & Roadmap Fix Scope

**Date:** 2026-04-20  
**Purpose:** Inventory all documentation references to claims being revised in the spec coherence fixes plan  
**Scope:** All `*.md` files in `docs/` tree (excluding `docs/superpowers/`)

---

## Executive Summary

This audit inventories every reference to five claim categories across the Ryo documentation tree, classifying each hit as "needs update", "needs rewrite", or "minor mention". The goal is to identify the full scope of companion-doc updates required for the spec coherence fix plan.

### Summary Table

| Category | Needs Update | Minor Mentions | Total Hits | Files Affected |
|----------|--------------|----------------|------------|----------------|
| A. Performance Positioning | 12 | 34 | 46 | 10 files |
| B. Ownership Lite Framing | 28 | 103 | 131 | 15 files |
| C. AI-Era Framing | 4 | 3 | 7 | 3 files |
| D. Loop Keywords | 3 | 3 | 6 | 5 files |
| E. Milestone References | 1 | 200+ | 200+ | 10 files |

**Key Finding:** No category exceeds 20 "needs update" files. Safe to proceed with companion-doc updates as planned.

---

## Category A: Performance Positioning

**Grep Query:** `"competitive performance|faster than|~5-10%|Cranelift|LLVM|GC|no garbage"`

### Needs Update (12 hits)

1. **docs/specification.md:63** — Section 1 vision paragraph: "competitive performance"
2. **docs/specification.md:70** — Section 1.1 performance bullet: "Competitive Performance" + "~5-10% runtime overhead"
3. **docs/specification.md:124-129** — Performance overhead table (5 rows mentioning "~5-10%")
4. **docs/specification.md:161** — Build command comment: "~5-10% overhead"
5. **docs/specification.md:671** — Error creation note: "~5-10% overhead"
6. **docs/specification.md:672** — Error propagation note: "~5-10% overhead"
7. **docs/specification.md:1859** — Runtime overhead bullet: "~5-10% estimated"
8. **docs/specification.md:1964** — Trace collection mode note: "~5-10% runtime overhead"
9. **docs/specification.md:2106** — Performance overhead table row: "~5-10%" (3 columns)
10. **docs/index.md:25** — Homepage tagline: "competitive performance"
11. **docs/design_issues.md:33** — Error Handling Overhead issue: "~5-10% overhead"
12. **docs/design_issues.md:34** — Issue description: "10x-100x slower than the operation itself"

**Impact:** These are the primary targets for Pass 1 (specification.md performance edits) and Task 4 (rewrite Section 1.1 comparison block). The design_issues.md entries need updating to reflect the new "lazy symbol resolution" approach.

### Minor Mentions (34 hits)

- **docs/specification.md** (18 hits): Cranelift backend references (6×), "no GC" mentions (5×), LLVM comparison (2×), GC pauses (3×), DWARF debug format (2×)
- **docs/quickstart.md** (3 hits): Cranelift IR generation
- **docs/getting_started.md** (3 hits): Cranelift compilation, "no GC"
- **docs/language_comparison.md** (6 hits): GC overhead comparisons
- **docs/dev/compilation_pipeline.md** (19 hits): Cranelift implementation details
- **docs/dev/implementation_roadmap.md** (9 hits): Cranelift mentions, M3 milestone
- **docs/dev/considerations.md** (3 hits): Cranelift limitations, LLVM discussion
- **docs/dev/built_in.md** (2 hits): Cranelift intrinsics
- **docs/troubleshooting.md** (1 hit): Cranelift architecture support

**Classification Rationale:** "Cranelift" and "no GC" are factual technical details, not performance claims. These mentions are accurate and require no change unless the backend or memory management strategy changes (out of scope for this plan).

---

## Category B: Ownership Lite Framing

**Grep Query:** `"Ownership Lite|no lifetime|without lifetime|shared\[|mutex\[|Arc<|borrow"`

### Needs Update (28 hits)

1. **docs/specification.md:23** — Roadmap preamble: "Ownership Lite model"
2. **docs/specification.md:49** — Open question: "Ownership Lite clear and practical?"
3. **docs/specification.md:68** — Section 1.1 bullet: "Simplified borrowing model compared to Rust (no manual lifetimes)"
4. **docs/specification.md:82** — Feature matrix row: "Ownership & borrowing (Ownership Lite)"
5. **docs/specification.md:114** — Key differentiators: "no lifetimes"
6. **docs/specification.md:199** — Readable by default principle: "parameter borrowing"
7. **docs/specification.md:365** — Slice intro: "scope-locked"
8. **docs/specification.md:371** — Function parameters: "compiler handles borrowing implicitly"
9. **docs/specification.md:383** — Rationale: "Under Ownership Lite, borrows are parameter-passing conventions"
10. **docs/specification.md:970** — Section 5 title: "Memory Management: Ownership Lite"
11. **docs/specification.md:972** — Section 5 intro: "Rust-level safety without lifetime annotations"
12. **docs/specification.md:987** — Parameter-passing intro: "not general-purpose type constructors"
13. **docs/specification.md:992-1009** — Parameter-passing table and Rule 2 (immutable borrows implicit)
14. **docs/specification.md:1056-1076** — Rule 5: "returning a borrow requires lifetime tracking"
15. **docs/specification.md:1076-1102** — Rule 6: "never &T" in struct fields
16. **docs/specification.md:1102-1114** — Rule 7: "One Writer OR Many Readers"
17. **docs/specification.md:1182-1220** — Section 5.6: `shared[T]` / ARC framing (5 mentions)
18. **docs/specification.md:1224-1256** — Section 5.7: Scope-locked views (4 mentions)
19. **docs/index.md:42-45** — Feature #2: "Ownership Lite Memory Model" (4 lines)
20. **docs/index.md:91** — Simplified borrowing rules bullet
21. **docs/index.md:209** — Key trade-off: "Simplified borrowing (no lifetimes)"
22. **docs/index.md:224** — Open question #2: "Ownership Lite practical?"
23. **docs/design_issues.md:5** — Last updated note: "After Ownership Lite rewrite"
24. **docs/design_issues.md:118-156** — Resolved Issues section (4 entries mentioning "Ownership Lite")
25. **docs/language_comparison.md:95** — Rust comparison: "Scope-based borrowing only"
26. **docs/language_comparison.md:130** — Ryo advantage: "Simpler borrowing rules (no explicit lifetimes)"
27. **docs/language_comparison.md:227** — When to choose Ryo: "Simpler mental model preferred"
28. **docs/getting_started.md:1164** — Comparison table: "Ownership with simplified borrowing"

**Impact:** These are the primary targets for Task 6 (revise spec Section 5.6 framing) and Task 7 (revise spec Section 5.8 summary). The "Ownership Lite" term and "no lifetimes" framing need consistent adjustment across all docs.

### Minor Mentions (103 hits)

- **docs/specification.md** (42 hits): Technical uses of "borrow", `shared[T]`, `mutex[T]` in code examples and rules
- **docs/getting_started.md** (2 hits): Borrow example, "no GC" mention
- **docs/design_issues.md** (16 hits): Borrow rules discussion, resolved issues
- **docs/examples/README.md** (4 hits): Example descriptions
- **docs/dev/implementation_roadmap.md** (33 hits): Milestone descriptions for M15, M19, M20, borrow checker tasks
- **docs/dev/project_structure.md** (1 hit): Borrow checker stub
- **docs/dev/dyn_trait.md** (1 hit): "Ownership Lite" philosophy mention
- **docs/dev/std_ext.md** (1 hit): Thread safety
- **docs/language_comparison.md** (2 hits): Memory management comparisons
- **docs/index.md** (1 hit): "no GC" in feature list

**Classification Rationale:** Technical uses of "borrow", `shared[T]`, and `mutex[T]` in code examples are accurate syntax. Only high-level framing paragraphs and section titles need adjustment.

---

## Category C: AI-Era Framing

**Grep Query:** `"AI agent|AI-era|AI-writes|majority of.*code|human review|2026"`

### Needs Update (4 hits)

1. **docs/specification.md:182** — Section 1.2 intro: "As of 2026, the majority of application code is written by AI agents"
2. **docs/specification.md:193** — Principle #1: "Verbose safety patterns cost the AI nothing"
3. **docs/specification.md:199** — Principle #4: "can a human reviewer understand"
4. **docs/specification.md:924** — Constrained types rationale: "For the AI-writes, human-reviews workflow"

**Impact:** These are the primary targets for Task 9 (rewrite spec Section 1.2 AI-era framing). The 2026 date and "majority of code" claim need adjustment to match the revised framing.

### Minor Mentions (3 hits)

- **docs/specification.md:1332** — Named arguments rationale: "For the AI-writes, human-reviews workflow"
- **docs/specification.md:2321** — Contracts rationale: "For the AI-writes, human-reviews workflow"
- **docs/dev/implementation_roadmap.md:870** — Named arguments rationale (duplicate)

**Classification Rationale:** These are rationale justifications that reference the AI-era workflow as context for design decisions. They remain accurate as long as the core principle (AI writes, human reviews) is retained — only the "majority of code" quantification needs softening.

---

## Category D: Loop Keywords

**Grep Query:** `"for condition|while keyword|one loop|single loop|three forms"` + `"for [a-z_]+ < |for [a-z_]+ > "` in examples

### Needs Update (3 hits)

1. **docs/specification.md:295-296** — Section 3: "for condition:" + "Ryo has no while keyword"
2. **docs/specification.md:329** — Rationale: "One loop keyword (for) with three forms"
3. **docs/dev/implementation_roadmap.md:784** — M8 task: "for condition: — Ryo has no while keyword"

**Impact:** These are the primary targets for Task 11 (produce while analysis memo) and any resulting spec edits. The "no while keyword" claim needs investigation to determine if it's defensible.

### Minor Mentions (3 hits)

- **docs/getting_started.md:389** — If/else explanation (conditional execution, not loops)
- **docs/CLAUDE.md:29** — Loops quick reference: "three forms, no while keyword"
- **docs/examples/modules/04-nested-modules/src/utils/math/advanced.ryo:33** — Code example: `for i < exp:` (condition-based loop)

**Classification Rationale:** CLAUDE.md is project instructions for AI and should mirror the spec. The example file is correct syntax. Only the spec text and roadmap need adjustment if the design decision changes.

---

## Category E: Milestone References

**Grep Query:** `"M4\.5|M15|M16|M17|M18|M19|M20|M21|M22|M23|Milestone [0-9]+|Phase [23]"`

### Needs Update (1 hit)

1. **docs/design_issues.md:15-20** — Open Issue #1: "Move M20 to start of Phase 3" + "Defer M4.5 closure capture to Phase 3"

**Impact:** This is the primary target for Task 2 (reorder roadmap milestones). Once the roadmap is updated, this design_issues.md entry should be moved to "Resolved Issues".

### Minor Mentions (200+ hits)

All milestone references in:
- **docs/specification.md** (8 hits): Roadmap preambles, future feature notes
- **docs/troubleshooting.md** (10 hits): "Known Limitations (Milestone 3)" section
- **docs/quickstart.md** (6 hits): M3 exit code behavior
- **docs/getting_started.md** (8 hits): M3/M4 status notes
- **docs/design_issues.md** (5 hits): Milestone ordering discussions
- **docs/dev/compilation_pipeline.md** (4 hits): M3/M4 completion notes
- **docs/dev/unsafe.md** (2 hits): M21 reference
- **docs/dev/dyn_trait.md** (2 hits): Phase 2 enums
- **docs/dev/testing.md** (1 hit): M26 proposal
- **docs/dev/proposals.md** (2 hits): Phase 2/3 feature grouping
- **docs/dev/implementation_roadmap.md** (150+ hits): Entire roadmap document

**Classification Rationale:** Milestone references are structural metadata, not claims. They only need updating if:
1. A milestone is renumbered or reordered (Task 2 scope)
2. A feature is moved between phases (Task 2 scope)
3. A milestone description contradicts a spec change (caught by category-specific audits)

Most references are accurate and unchanged. Only the roadmap file itself and cross-references in design_issues.md need adjustment.

---

## Design Issues Log Cross-Reference

This section maps entries in `docs/design_issues.md` to tasks in the spec coherence plan.

### Open Issues

1. **The Logic Paradoxes (Roadmap Breakers)** → **Task 2: Reorder roadmap milestones**
   - Move M20 (`&mut`) to start of Phase 3
   - Defer M4.5 closure capture semantics to Phase 3
   - Status: Planned for Task 2

2. **The "Hardcoded Generics" Trap** → *Out of scope*
   - Design decision, not coherence fix
   - Keep in design_issues.md as-is

3. **Error Handling Overhead** → **Task 3: Rewrite spec Section 1 performance bullet**
   - "~5-10% overhead" claim needs revision
   - Proposal: "Lazy Symbol Resolution + PC Capture"
   - Status: Task 3 will align spec with design_issues.md proposal

4. **Circular Dependencies** → *Out of scope*
   - Design decision, not coherence fix
   - Keep in design_issues.md as-is

5. **Specification Holes** → *Out of scope*
   - Separate from coherence fixes
   - Keep in design_issues.md as-is

6. **Conflicting Syntax: The `!` Operator** → *Out of scope*
   - Design decision, not coherence fix
   - Keep in design_issues.md as-is

### Grey Areas

Issues #7-15 → *Out of scope*
- Design decisions, not coherence fixes
- Keep in design_issues.md as-is

### Resolved Issues

- **Borrow/Move Inconsistency** — Example files already updated
- **Ownership Lite Safety Gap** → **Task 6 & 7: Revise spec Section 5.6 & 5.8 framing**
  - "Ownership Lite" term needs refinement
  - "No lifetimes" framing needs softening
  - Status: Tasks 6 & 7 will revise
- **Iterator Invalidation** — Already resolved in spec
- **Hidden String Allocations** — Already resolved in spec
- **Resource Management Syntax** — Already resolved in spec

### Immediate Action Plan Alignment

| Design Issues Priority | Corresponding Plan Task | Status |
|------------------------|-------------------------|--------|
| 1. Reorder Phase 3 | Task 2: Reorder roadmap milestones | Planned |
| 2. Add never type | Out of scope | Keep in design_issues.md |
| 3. Grey area decisions | Out of scope | Keep in design_issues.md |

---

## Observed But Out of Scope

The following references were found but are NOT targets for this plan:

1. **Generic type syntax:** 46 uses of `list[T]`, `map[K,V]`, `shared[T]`, `mutex[T]` across all docs
   - Accurate syntax, not a claim
   - No action needed

2. **Error union syntax:** 23 uses of `!T` in specification.md
   - Accurate syntax, not a claim
   - Design Issues #6 tracks potential syntax change, but that's a separate decision

3. **Concurrency milestone numbers:** M32-M36 references in spec Section 9
   - These are future milestones, no reordering needed
   - No action needed

4. **M3 "Hello, Exit Code" references:** 8 mentions across quickstart/getting_started/troubleshooting
   - Historical milestone, completed
   - No action needed

5. **CLAUDE.md project instructions:** 1 reference to "three forms, no while keyword"
   - Should mirror spec, will update after Task 11 if needed
   - Wait for spec decision first

---

## Recommendations

### For Task 5, 8, 10, 15 (Companion Doc Updates)

Use this audit to identify files needing updates after spec changes:

- **Pass 2 (Task 5):** After Task 3/4 revise performance claims
  - Update: docs/index.md, docs/design_issues.md, docs/language_comparison.md
  - Verify: docs/getting_started.md comparison table

- **Pass 3 (Task 8):** After Task 6/7 revise Ownership Lite framing
  - Update: docs/index.md, docs/language_comparison.md, docs/getting_started.md, docs/examples/README.md
  - Verify: docs/design_issues.md resolved issues section

- **Pass 4 (Task 10):** After Task 9 revises AI-era framing
  - Update: docs/CLAUDE.md (if rationale principles change)
  - Verify: No other files affected (only 4 hits, all in spec)

- **Pass 1 review (Task 15):** After Task 2 reorders roadmap
  - Update: docs/design_issues.md (move Issue #1 to Resolved)
  - Verify: No cross-references broken by renumbering

### For Uncertainty Flagging

If any of the following appear during spec edits, stop and report:

1. A performance claim that contradicts Cranelift's documented capabilities
2. A memory safety claim that contradicts the 7 ownership rules
3. An AI-era framing that implies a timeline or adoption rate
4. A loop syntax example that doesn't match the final `while` decision

### For Scope Creep Prevention

Do NOT revise during companion doc updates:

1. Technical Cranelift references (accurate)
2. Syntax examples using `shared[T]`, `mutex[T]` (accurate)
3. Milestone numbers in historical notes (M3, M4 complete)
4. Design Issues entries marked "Out of scope" above

---

## Audit Methodology

1. Ran grep commands per Task 1 instructions
2. Excluded `docs/superpowers/` per instructions
3. Classified each hit using these criteria:
   - **Needs update:** Text actively makes a claim being revised in the plan
   - **Needs rewrite:** Text structure prevents simple wording change
   - **Minor mention:** Factual technical detail or passing reference
4. Default classification: "Minor mention" unless hit contradicts planned change
5. Cross-referenced design_issues.md to identify relationships to plan tasks

**Grep versions used:**
- BSD grep (macOS default)
- Extended regex mode (-E)
- Recursive (-r) with line numbers (-n)
- Filtered with grep -v to exclude superpowers/

**Files excluded:** All files under `docs/superpowers/` per instructions

---

## Completion Checklist

- [x] Created docs/analysis/ directory
- [x] Ran all 6 grep queries
- [x] Classified hits for categories A/B/C/D/E
- [x] Wrote audit file with sections per instructions
- [x] Added Summary table
- [x] Added Design Issues cross-reference section
- [x] Verified no category has >20 "needs update" files
- [ ] Commit audit file (Step 5)
- [ ] Report summary to human (Step 6)

---

## Next Steps

1. Review this audit with the human (Task 1, Step 6)
2. If approved, proceed to Task 2 (reorder roadmap milestones)
3. Use this audit as reference during all companion-doc update tasks (5, 8, 10, 15)
4. Archive this audit in docs/analysis/ for future spec coherence audits

**End of Audit**
