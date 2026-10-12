# CompatLab Roadmap

> **Vision:** CompatLab is the main, language-neutral compatibility testing platform in the Compat ecosystem. It can test a single-language project or coordinate compatibility experiments across a multilingual repository. Language-specific capabilities are provided by adapters such as Python/CompatPy, JavaScript/Node.js, GDScript/Godot, and Rust.
>
> **Guiding principle:** Build one reliable experiment platform, support single-language projects as first-class citizens, and grow into multilingual and cross-component testing without pretending every language works the same way.
>
> **Scope rule:** CompatLab owns project discovery, experiment planning and orchestration, process/resource controls, normalized outcomes, reports, and the adapter contract. Adapters own language- and toolchain-specific behavior. Consumer tools such as CompatPy may request experiments and present results, but must not need to reimplement the engine.

This roadmap describes the intended journey from the current prototype to a mature, cross-platform compatibility platform. It includes core correctness, stable contracts, single-language testing, multilingual project discovery, cross-language integration testing, adapter development, security, release engineering, and long-term ecosystem capabilities. Checkboxes are implementation claims, not aspirations: mark an item complete only after the work exists and its acceptance criteria have been verified. Future capabilities remain planned until implemented.

## Project principles

- **Single-language and multilingual projects are equally valid.** A repository with one language must not pay unnecessary complexity costs for multilingual support.
- **Python first, never Python-only.** Python is the first adapter and proves the architecture; it is not the permanent center of the product.
- **Windows, Linux, and macOS are first-class platforms.** Cross-platform behavior must be tested, not inferred from a successful build.
- **Preserve the current workspace.** Keep `compatlab-core` and `compatlab-cli` unless an evidence-based change is justified.
- **No corner-cutting on correctness.** Do not weaken a valid test to make it pass. Diagnose the implementation or the test setup and fix the real defect.
- **One logical change per commit.** Inspect focused diffs, run relevant tests and quality checks, and only then commit.
- **Prefer explicit contracts over clever abstractions.** Model real shared behavior and keep language-specific semantics in adapters.
- **Separate component checks from integration checks.** A component passing alone does not prove it works with the other components in a project.
- **Make unsupported behavior explicit.** An unavailable runtime, unsupported project type, skipped check, and failed check are different outcomes.
- **Be honest about evidence.** A smoke test does not prove universal compatibility. Reports must say what was tested and what remains unknown.
- **Be honest about isolation.** A virtual environment or temporary directory is not a security sandbox.
- **Secure by default where feasible.** Avoid leaking secrets, executing user code unintentionally, or implying that arbitrary package builds are safe.
- **Keep external consumers decoupled.** CompatPy and future tools should use documented interfaces, not scrape human-readable terminal output or depend on private Rust internals.

## Roadmap at a glance

1. [Core reliability and correctness](#phase-1-core-reliability-and-correctness)
2. [Experiment and report format](#phase-2-experiment-and-report-format)
3. [CLI workflow](#phase-3-cli-workflow)
4. [Python runtime and compatibility capabilities](#phase-4-python-runtime-and-compatibility-capabilities)
5. [Isolation, process control, and operational safety](#phase-5-isolation-process-control-and-operational-safety)
6. [Cross-platform support](#phase-6-cross-platform-support)
7. [CompatPy integration](#phase-7-compatpy-integration)
8. [Advanced experiment capabilities](#phase-8-advanced-experiment-capabilities)
9. [Extensible architecture](#phase-9-extensible-architecture)
10. [Documentation, packaging, and stable release](#phase-10-documentation-packaging-and-stable-release)
11. [JavaScript and Node.js adapter](#phase-11-javascript-and-nodejs-adapter)
12. [GDScript and Godot adapter](#phase-12-gdscript-and-godot-adapter)
13. [Rust toolchain and crate adapter](#phase-13-rust-toolchain-and-crate-adapter)
14. [Wider Compat ecosystem](#phase-14-wider-compat-ecosystem)
15. [Advanced compatibility intelligence](#phase-15-advanced-compatibility-intelligence)
16. [Project and workspace model](#phase-16-project-and-workspace-model)
17. [Project discovery and component inventory](#phase-17-project-discovery-and-component-inventory)
18. [Multilingual project orchestration](#phase-18-multilingual-project-orchestration)
19. [Cross-language integration experiments](#phase-19-cross-language-integration-experiments)
20. [Adapter lifecycle and capability contract](#phase-20-adapter-lifecycle-and-capability-contract)
21. [Configuration, policy, and reproducible plans](#phase-21-configuration-policy-and-reproducible-plans)
22. [Security, trust boundaries, and sandbox strategy](#phase-22-security-trust-boundaries-and-sandbox-strategy)
23. [Execution graph, artifacts, and dependency relationships](#phase-23-execution-graph-artifacts-and-dependency-relationships)
24. [Project-level reporting and result interpretation](#phase-24-project-level-reporting-and-result-interpretation)
25. [Quality engineering and acceptance strategy](#phase-25-quality-engineering-and-acceptance-strategy)
26. [Performance, scale, and resource governance](#phase-26-performance-scale-and-resource-governance)
27. [Developer experience and ecosystem adoption](#phase-27-developer-experience-and-ecosystem-adoption)
28. [Long-term integrations and research candidates](#phase-28-long-term-integrations-and-research-candidates)

---

## Phase 1: Core reliability and correctness

**Goal:** Make `compatlab-core` trustworthy before expanding its features.

### 1.1. Establish the baseline

- [x] Create the Cargo workspace.
- [x] Separate `compatlab-core` and `compatlab-cli`.
- [x] Define `Experiment` and `ExperimentConfig`.
- [x] Define experiment statuses and stages.
- [x] Define `ExperimentReport`.
- [x] Add JSON serialization and deserialization.
- [x] Inspect Python interpreter versions.
- [x] Create isolated Python virtual environments.
- [x] Install packages with pip.
- [x] Check whether installed modules import successfully.
- [x] Add a process timeout helper.
- [x] Test invalid interpreter handling.
- [x] Test successful local package installation and import.
- [x] Test missing-module failure.
- [x] Test timeout process termination.
- [x] Add timeout error mapping.
- [x] Run formatting and Clippy successfully.

These checkmarks reflect the implementation and test results observed during development. They are not proof that every edge case has been covered.

### 1.2. Finish timeout handling

- [x] Decide and document whether `timeout_seconds` is a per-operation limit or an overall experiment deadline. The current implementation applies it separately to installation and import.
- [x] Define and test zero-second timeout behavior.
- [x] Test a process that exits before its deadline.
- [x] Test a process that exceeds its deadline.
- [ ] Verify installation timeouts produce `TimedOut`.
- [ ] Verify import-check timeouts produce `TimedOut`.
- [ ] Verify process-launch failures produce `Error`, not `TimedOut`.
- [ ] Verify the report stage identifies the operation that failed.
- [ ] Verify duration is populated on every return path.
- [ ] Handle output collection correctly after process termination.
- [ ] Investigate descendant-process cleanup and orphaned subprocesses.
- [ ] Verify timeout behavior on Windows, Linux, and macOS.

### 1.3. Complete report lifecycle tests

- [ ] Create a reusable test report fixture.
- [ ] Test every `ExperimentStatus`.
- [ ] Test every `ExperimentStage`.
- [ ] Test reports after interpreter inspection fails.
- [ ] Test reports after virtual environment creation fails.
- [ ] Test reports after package installation fails.
- [ ] Test reports after import fails.
- [ ] Test reports after timeout.
- [ ] Test stdout and stderr preservation.
- [ ] Test optional fields and JSON round-tripping for all outcomes.
- [ ] Document the meaning of each status and stage.

### 1.4. Improve failure handling

- [ ] Distinguish invalid input from runtime failures.
- [ ] Distinguish process-launch failures from unsuccessful exit codes.
- [ ] Preserve useful Python and pip diagnostics.
- [ ] Avoid replacing a useful error message with a less useful one.
- [ ] Verify temporary directories are cleaned up after early returns.
- [ ] Test paths containing spaces and non-ASCII characters.
- [ ] Test missing or inaccessible interpreter paths.
- [ ] Test malformed package requirements.
- [ ] Test malformed import names.
- [ ] Avoid panics for ordinary user errors.
- [ ] Replace `unwrap()` in production paths with suitable error handling.

### 1.5. Validate experiment inputs

- [ ] Reject empty interpreter paths.
- [ ] Reject empty package requirements.
- [ ] Reject empty import names.
- [ ] Validate timeout configuration.
- [ ] Define zero-timeout behavior.
- [ ] Define reasonable maximum timeout limits.
- [ ] Reject malformed or unsupported configuration values.
- [ ] Add tests for every validation rule.

**Completion gate:** Normal runs, major failure paths, timeouts, and cleanup behavior are covered by automated tests. Ordinary invalid input does not cause unexpected panics.

---

## Phase 2: Experiment and report format

**Goal:** Let CompatPy and other programs describe experiments without depending on CompatLab's internal Rust structures.

The existing `docs/experiment-format.md` is the natural home for the specification.

### 2.1. Define the experiment schema

- [ ] Document current experiment fields.
- [ ] Define a schema version.
- [ ] Define required and optional fields.
- [ ] Define package requirement syntax.
- [ ] Define import-check configuration.
- [ ] Define timeout units and semantics.
- [ ] Define environment configuration.
- [ ] Define supported experiment types.
- [ ] Decide how unknown fields are handled.
- [ ] Define defaults and validation errors.
- [ ] Decide how language-neutral runtime identifiers will work in the future.
- [ ] Add valid and invalid example JSON files.

### 2.2. Define the report schema

- [ ] Add a report schema version.
- [ ] Define stable status and stage values.
- [ ] Define error categories.
- [ ] Define duration units.
- [ ] Define stdout and stderr representation.
- [ ] Define runtime and platform information.
- [ ] Define package and module metadata.
- [ ] Define optional diagnostic details.
- [ ] Define how reports evolve without breaking consumers.
- [ ] Add examples for success, failure, timeout, and execution error.

### 2.3. Add schema compatibility tests

- [ ] Deserialize documented experiment examples.
- [ ] Serialize reports into documented shapes.
- [ ] Test missing required fields.
- [ ] Test invalid enum values.
- [ ] Test unknown fields according to the chosen policy.
- [ ] Test schema-version mismatches.
- [ ] Define and test backward-compatibility expectations.
- [ ] Keep examples synchronized with automated tests.

### 2.4. Document the contract

- [ ] Write the experiment format specification.
- [ ] Write the report format specification.
- [ ] Document statuses, stages, and errors.
- [ ] Document versioning policy.
- [ ] Document compatibility guarantees.
- [ ] Explain how other tools consume reports.

**Completion gate:** An external program can generate an experiment and consume its report using the documented contract without importing the CompatLab Rust library.

---

## Phase 3: CLI workflow

**Goal:** Make CompatLab usable by people and automation without requiring consumers to write Rust code.

### 3.1. Establish command structure

- [ ] Inspect the current `compatlab-cli` implementation.
- [ ] Define the top-level help screen.
- [ ] Define command names and arguments.
- [ ] Define consistent error output.
- [ ] Define process exit codes.
- [ ] Add version output.
- [ ] Add examples to help text.

Possible future commands (proposals, not commitments):

```text
compatlab
├── run
├── validate
├── report
├── runtime
└── version
```

### 3.2. Implement experiment execution

- [ ] Load an experiment from a JSON file.
- [ ] Validate input before execution.
- [ ] Run the experiment.
- [ ] Display a human-readable summary.
- [ ] Print actionable failure information.
- [ ] Export a JSON report.
- [ ] Support a configurable output path.
- [ ] Handle missing input files.
- [ ] Handle invalid JSON.
- [ ] Handle unwritable output paths.
- [ ] Keep machine-readable output free of debug noise.

### 3.3. Implement report utilities

- [ ] Read an existing report.
- [ ] Display report details.
- [ ] Validate a report.
- [ ] Return predictable exit codes based on outcome.
- [ ] Support quiet mode for scripts.
- [ ] Support verbose diagnostics.
- [ ] Decide whether one invocation can emit multiple reports.

### 3.4. Test the CLI

- [ ] Test successful execution.
- [ ] Test failed installation.
- [ ] Test failed import.
- [ ] Test timeout.
- [ ] Test malformed input and missing files.
- [ ] Test exit codes.
- [ ] Test JSON output.
- [ ] Test command help.
- [ ] Test paths containing spaces.
- [ ] Run CLI tests on Windows, Linux, and macOS.

**Completion gate:** A user can run a documented experiment file from the terminal, receive a useful summary, and save a machine-readable report.

---

## Phase 4: Python runtime and compatibility capabilities

**Goal:** Move from one manually specified interpreter to a useful Python compatibility engine.

### 4.1. Interpreter discovery

- [ ] Separate interpreter discovery from interpreter inspection.
- [ ] Discover Python executables on Windows.
- [ ] Discover Python executables on Linux.
- [ ] Discover Python executables on macOS.
- [ ] Inspect discovered versions.
- [ ] Identify duplicate interpreter paths.
- [ ] Handle inaccessible and broken interpreters.
- [ ] Return stable runtime identifiers.
- [ ] Allow explicit interpreter selection.
- [ ] Document limitations of automatic discovery.

### 4.2. Package compatibility

- [ ] Support exact package versions.
- [ ] Support version ranges.
- [ ] Support optional package extras.
- [ ] Validate requirement syntax.
- [ ] Distinguish installation failures from import failures.
- [ ] Capture the installed package version.
- [ ] Capture interpreter version, platform, and architecture.
- [ ] Record whether installation used a wheel or source build when available.
- [ ] Report dependency-resolution failures clearly.

### 4.3. Improve import checking

- [ ] Support multiple import checks.
- [ ] Report each import independently.
- [ ] Capture import exceptions.
- [ ] Distinguish missing modules from runtime exceptions.
- [ ] Consider optional post-import assertions or smoke tests.
- [ ] Define how individual failures affect the overall result.
- [ ] Test packages whose import names differ from their distribution names.

### 4.4. Compatibility matrices

- [ ] Define a matrix of interpreters and package versions.
- [ ] Generate experiments from a matrix.
- [ ] Run experiments independently.
- [ ] Collect individual reports.
- [ ] Summarize matrix results.
- [ ] Identify version-specific failures.
- [ ] Support selecting subsets of combinations.
- [ ] Avoid redundant runs when caching is eventually implemented.

**Completion gate:** CompatLab can test a package across multiple Python versions and produce structured results that explain compatibility differences.

---

## Phase 5: Isolation, process control, and operational safety

**Goal:** Make experiments predictable and prevent one experiment from corrupting another.

### 5.1. Environment isolation

- [ ] Use a separate virtual environment for each experiment.
- [ ] Ensure the system Python environment is not modified.
- [ ] Define environment-variable handling.
- [ ] Define working-directory behavior.
- [ ] Control pip configuration where appropriate.
- [ ] Avoid inheriting unintended Python path overrides.
- [ ] Ensure temporary directories are cleaned up.
- [ ] Test isolation between consecutive experiments.

### 5.2. Process management

- [ ] Define timeout semantics.
- [ ] Implement robust process termination.
- [ ] Investigate descendant-process cleanup.
- [ ] Prevent orphaned package-build processes.
- [ ] Handle interrupted execution.
- [ ] Handle unexpected child-process exits.
- [ ] Avoid unbounded stdout/stderr growth.
- [ ] Avoid deadlocks while collecting output.
- [ ] Test cancellation behavior.

### 5.3. Resource controls

- [ ] Set maximum execution duration.
- [ ] Consider output-size limits.
- [ ] Consider configurable temporary-storage limits.
- [ ] Define behavior when disk space runs out.
- [ ] Avoid uncontrolled parallel execution.
- [ ] Define cleanup behavior after crashes.
- [ ] Document isolation guarantees and limitations.

### 5.4. Dependency and network behavior

- [ ] Define whether experiments may use the network.
- [ ] Support offline tests when dependencies are available.
- [ ] Make package indexes configurable.
- [ ] Avoid embedding credentials in reports.
- [ ] Redact sensitive environment values.
- [ ] Define configuration for private package sources.
- [ ] Document that third-party build scripts can execute code during installation.

**Security note:** A virtual environment isolates Python dependencies; it is not a security sandbox. Untrusted packages may execute code during installation. Do not claim protection against malicious packages solely because a virtual environment is used.

**Completion gate:** Experiments have clear resource limits, predictable cleanup, and documented isolation boundaries.

---

## Phase 6: Cross-platform support

**Goal:** Support Windows, Linux, and macOS as part of the design.

### 6.1. Establish a CI matrix

- [ ] Add Windows CI.
- [ ] Add Linux CI.
- [ ] Add macOS CI.
- [ ] Install the required stable Rust toolchain.
- [ ] Install a supported Python runtime on each runner.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo check --workspace --all-targets`.
- [ ] Run `cargo test --workspace`.
- [ ] Run Clippy with warnings denied.
- [ ] Run local-package installation/import integration tests.
- [ ] Test timeout termination.
- [ ] Publish useful test output on failures.

### 6.2. Verify platform-specific behavior

- [ ] Verify Windows virtual-environment paths.
- [ ] Verify Unix virtual-environment paths.
- [ ] Test executable paths containing spaces.
- [ ] Test paths with non-ASCII characters.
- [ ] Verify Linux permissions and executable handling.
- [ ] Verify macOS runtime discovery.
- [ ] Test ARM64 and x86-64 where practical.
- [ ] Avoid assuming every OS has identical Python versions.
- [ ] Avoid tests that depend on globally installed packages.
- [ ] Keep tests deterministic and reasonably fast.
- [ ] Test cleanup after forced timeouts on every OS.

### 6.3. Test real user environments

- [ ] Test multiple installed Python versions.
- [ ] Test missing and unlaunchable interpreters.
- [ ] Test offline installation where feasible.
- [ ] Test pip installation errors.
- [ ] Test read-only directories and insufficient permissions.
- [ ] Test filesystem edge cases.
- [ ] Document platform-specific limitations.

### 6.4. Define support guarantees

- [ ] Define minimum supported OS versions.
- [ ] Define supported CPU architectures.
- [ ] Define a minimum supported Rust version if needed.
- [ ] Define Python versions supported by CompatLab itself.
- [ ] Distinguish tested platforms from theoretically compatible platforms.
- [ ] Keep CI aligned with the support policy.

**Completion gate:** All three operating systems pass the relevant tests, with platform-specific tests where necessary. A green Windows build alone is not proof of cross-platform support.

---

## Phase 7: CompatPy integration

**Goal:** Turn CompatLab's original motivation into a working integration.

### 7.1. Define the integration contract

- [ ] Decide how CompatPy invokes CompatLab.
- [ ] Define which component generates experiment definitions.
- [ ] Define which component discovers Python interpreters.
- [ ] Decide whether CompatPy calls the CLI or Rust library.
- [ ] Use the documented JSON contract to decouple components.
- [ ] Define how CompatPy receives results.
- [ ] Define version compatibility between CompatPy and CompatLab.
- [ ] Define behavior when the installed CompatLab version is incompatible.

### 7.2. Implement the first integration

- [ ] Generate an experiment from a CompatPy check.
- [ ] Pass the selected Python runtime.
- [ ] Pass the requested package version.
- [ ] Pass the correct import name.
- [ ] Execute the experiment.
- [ ] Parse the JSON report.
- [ ] Translate results into CompatPy's result model.
- [ ] Display meaningful compatibility information.
- [ ] Handle missing or incompatible CompatLab installations.
- [ ] Add an integration test for a known compatible package.
- [ ] Add an integration test for a known incompatible scenario.

### 7.3. Improve the user experience

- [ ] Explain which Python versions passed.
- [ ] Explain which versions failed.
- [ ] Distinguish installation failures from import failures.
- [ ] Show concise diagnostics.
- [ ] Link failures to detailed reports.
- [ ] Avoid claiming universal compatibility from a single smoke test.
- [ ] Document precisely what CompatLab tested.

**Completion gate:** CompatPy can request a real experiment, consume the result, and provide actionable compatibility information without reimplementing the execution engine.

---

## Phase 8: Advanced experiment capabilities

**Goal:** Make CompatLab useful beyond one-package-at-a-time checks.

### 8.1. Batch execution

- [ ] Run multiple experiments from one input.
- [ ] Track each experiment independently.
- [ ] Produce an aggregate summary.
- [ ] Continue after an individual experiment fails.
- [ ] Define how global errors affect batch results.
- [ ] Support selecting specific experiments.
- [ ] Preserve submitted experiment identifiers.

### 8.2. Controlled concurrency

- [ ] Add a configurable concurrency limit.
- [ ] Keep concurrent experiments isolated.
- [ ] Avoid oversubscribing CPU, memory, and disk.
- [ ] Ensure one timeout does not corrupt other reports.
- [ ] Handle cancellation of pending experiments.
- [ ] Test concurrent package installation.
- [ ] Keep sequential execution available for debugging.

### 8.3. Reproducibility

- [ ] Record runtime and platform details.
- [ ] Record requested and resolved package versions.
- [ ] Record experiment configuration.
- [ ] Record timestamps where useful.
- [ ] Define stable experiment identifiers.
- [ ] Capture relevant environment metadata without leaking secrets.
- [ ] Support saving complete experiment records.
- [ ] Document sources of nondeterminism.

### 8.4. Caching and comparison

- [ ] Design cache keys around the full experiment configuration.
- [ ] Avoid reusing results after relevant inputs change.
- [ ] Define cache expiration and invalidation.
- [ ] Allow caching to be disabled.
- [ ] Compare reports from different runs.
- [ ] Identify changes in pass/fail outcomes.
- [ ] Compare runtime and installation behavior.
- [ ] Avoid presenting cached results as newly executed experiments.

### 8.5. Diagnostics and observability

- [ ] Add structured diagnostic categories.
- [ ] Make logs useful without exposing secrets.
- [ ] Add detailed failure context.
- [ ] Record experiment-level timing.
- [ ] Track setup, installation, and check durations.
- [ ] Support machine-readable aggregate results.
- [ ] Make debugging possible without enormous terminal output.

**Completion gate:** CompatLab can run batches or matrices, aggregate results, explain failures, and preserve enough information to reproduce an experiment.

---

## Phase 9: Extensible architecture

**Goal:** Add future languages without turning the core into a collection of language-specific conditionals.

This phase follows the working Python implementation so the architecture is based on real requirements.

### 9.1. Identify shared concepts

- [ ] Identify genuinely language-neutral fields.
- [ ] Separate experiment orchestration from runtime execution.
- [ ] Separate common reporting from language-specific diagnostics.
- [ ] Define shared concepts such as runtime, task, outcome, and artifact.
- [ ] Identify concepts that must remain language-specific.
- [ ] Avoid forcing Python-specific fields into every future experiment.

### 9.2. Define runtime adapters

- [ ] Define how an adapter identifies its language.
- [ ] Define runtime discovery and validation.
- [ ] Define environment preparation.
- [ ] Define execution and compatibility checks.
- [ ] Define failure reporting.
- [ ] Define timeout and cancellation behavior.
- [ ] Define adapter-specific configuration.
- [ ] Define a capability model.
- [ ] Decide whether adapters start as internal modules or external extensions.

### 9.3. Keep the extension boundary small

- [ ] Introduce only interfaces required by real implementations.
- [ ] Keep the Python adapter working during refactoring.
- [ ] Avoid a dynamic plugin loader until needed.
- [ ] Avoid a universal scripting language for experiments.
- [ ] Avoid an unnecessarily complicated trait hierarchy.
- [ ] Ensure shared reports can represent language-specific details.
- [ ] Test that a new adapter can be added without rewriting orchestration.

### 9.4. Define extension compatibility

- [ ] Version the adapter contract.
- [ ] Document capability negotiation.
- [ ] Define behavior for unsupported operations.
- [ ] Define errors for unavailable runtimes.
- [ ] Validate adapter configuration.
- [ ] Document how contributors add a language.
- [ ] Create a reference adapter example.

**Completion gate:** The Python implementation still works, and a second language can be added behind a defined boundary without rewriting orchestration or reporting.

---

## Phase 10: Documentation, packaging, and stable release

**Goal:** Make CompatLab installable, understandable, and maintainable by people other than its creator.

### 10.1. Complete documentation

- [ ] Write the root `README.md`.
- [ ] Explain what CompatLab does.
- [ ] Explain the relationship between CompatLab and CompatPy.
- [ ] Document supported platforms.
- [ ] Document installation requirements.
- [ ] Provide a quick-start example.
- [ ] Document the CLI.
- [ ] Document experiment and report formats.
- [ ] Document timeout behavior.
- [ ] Document isolation limitations.
- [ ] Add troubleshooting instructions.
- [ ] Add a contribution guide.
- [ ] Add a security policy.
- [ ] Add a changelog.
- [ ] Document release and versioning processes.

### 10.2. Prepare distribution

- [ ] Define supported release targets.
- [ ] Build Windows binaries.
- [ ] Build Linux binaries.
- [ ] Build macOS binaries.
- [ ] Include ARM64 builds where supported.
- [ ] Test each binary on its intended platform.
- [ ] Define archive formats and naming conventions.
- [ ] Provide checksums.
- [ ] Consider signed artifacts where practical.
- [ ] Test installation on a clean machine.
- [ ] Document upgrades and uninstall procedures.

### 10.3. Establish release automation

- [ ] Define versioning rules.
- [ ] Validate release tags.
- [ ] Build release artifacts in CI.
- [ ] Run tests before publishing.
- [ ] Attach artifacts to GitHub Releases.
- [ ] Generate release notes.
- [ ] Publish checksums.
- [ ] Document rollback and correction procedures.
- [ ] Test the complete release workflow before the first stable release.

### 10.4. Establish project maintenance

- [ ] Define issue labels.
- [ ] Add bug-report templates.
- [ ] Add feature-request templates.
- [ ] Add pull-request guidance.
- [ ] Document code review expectations.
- [ ] Keep dependencies maintained.
- [ ] Add dependency update checks where useful.
- [ ] Establish a breaking-change policy.
- [ ] Document support and deprecation policies.

**Completion gate:** A new user can install CompatLab, run an example, understand the output, and find documentation for supported features.

**Release strategy:** Early versions may evolve as the engine and experiment contract mature. Call it `1.0.0` only when the CLI, schema, core behavior, and support policy are stable enough to promise compatibility.

---

## Phase 11: JavaScript and Node.js adapter

**Goal:** Test JavaScript runtime, package, and module compatibility.

### 11.1. Define the compatibility model

- [ ] Choose the initial JavaScript runtime; start with Node.js.
- [ ] Define runtime version discovery.
- [ ] Define package-manager support; start with a deliberately small scope such as npm.
- [ ] Define dependency isolation.
- [ ] Define initial compatibility checks.
- [ ] Define how package and module names are represented.

### 11.2. Implement the adapter

- [ ] Discover Node.js installations.
- [ ] Inspect Node.js versions.
- [ ] Create isolated project directories.
- [ ] Install dependencies.
- [ ] Run a module-resolution check.
- [ ] Run an import smoke test.
- [ ] Capture stdout and stderr.
- [ ] Apply timeouts and process cleanup.
- [ ] Produce standard CompatLab reports.
- [ ] Test success and failure scenarios.

### 11.3. Validate the integration

- [ ] Test multiple Node.js versions.
- [ ] Test package-version constraints.
- [ ] Test missing packages.
- [ ] Test incompatible runtime requirements.
- [ ] Test failed installation.
- [ ] Test failed module loading.
- [ ] Test Windows, Linux, and macOS.
- [ ] Document Node.js-specific limitations.

**Completion gate:** CompatLab can test a JavaScript package against selected Node.js versions using the shared lifecycle and reporting model.

---

## Phase 12: GDScript and Godot adapter

**Goal:** Test compatibility across Godot versions and project configurations.

GDScript execution is closely tied to the Godot engine, so its adapter must model engine-specific behavior rather than pretending it is a conventional package-import workflow.

### 12.1. Define compatibility

- [ ] Choose supported Godot versions.
- [ ] Define Godot installation discovery.
- [ ] Define project setup requirements.
- [ ] Define headless execution behavior.
- [ ] Define script parsing and loading checks.
- [ ] Decide whether checks run inside the engine.
- [ ] Define asset and project-setting handling.
- [ ] Define engine-specific error reporting.

### 12.2. Implement the adapter

- [ ] Discover Godot executables.
- [ ] Inspect engine versions.
- [ ] Prepare isolated project copies.
- [ ] Validate project configuration.
- [ ] Run headless project checks.
- [ ] Check GDScript parsing and loading.
- [ ] Capture engine output.
- [ ] Apply timeouts and cleanup.
- [ ] Translate engine results into CompatLab reports.

### 12.3. Validate real projects

- [ ] Test a minimal valid Godot project.
- [ ] Test invalid GDScript.
- [ ] Test missing resources.
- [ ] Test engine-version incompatibilities.
- [ ] Test projects with different Godot versions.
- [ ] Test headless execution across supported platforms.
- [ ] Document rendering and hardware-dependent limitations.

**Completion gate:** CompatLab can evaluate basic script and project compatibility across selected Godot versions.

---

## Phase 13: Rust toolchain and crate adapter

**Goal:** Test Rust toolchains, crates, workspaces, and compilation targets.

CompatLab being written in Rust does not automatically provide Rust compatibility testing. The adapter's purpose is to test Rust projects.

### 13.1. Define Rust experiment types

- [ ] Define Rust toolchain selection.
- [ ] Define support for stable and other selected toolchains.
- [ ] Define crate and workspace inputs.
- [ ] Define dependency-resolution behavior.
- [ ] Define target-triple configuration.
- [ ] Define feature selection.
- [ ] Define check, build, and test operations.
- [ ] Define what a Rust compatibility result means.

### 13.2. Implement the adapter

- [ ] Discover Rust toolchains.
- [ ] Inspect `rustc` and Cargo versions.
- [ ] Prepare an isolated workspace or project copy.
- [ ] Run dependency resolution.
- [ ] Run `cargo check`.
- [ ] Optionally run `cargo build`.
- [ ] Optionally run tests.
- [ ] Capture compiler diagnostics.
- [ ] Apply timeouts and process cleanup.
- [ ] Produce standard CompatLab reports.

### 13.3. Validate Rust compatibility

- [ ] Test a crate that compiles successfully.
- [ ] Test compilation errors.
- [ ] Test missing dependencies.
- [ ] Test feature-specific behavior.
- [ ] Test supported target triples.
- [ ] Test toolchain-specific differences.
- [ ] Test multi-crate workspaces.
- [ ] Test Windows, Linux, and macOS where appropriate.

**Completion gate:** CompatLab can run reproducible Rust compilation and test experiments across selected toolchains and targets.

---

## Phase 14: Wider Compat ecosystem

**Goal:** Make CompatLab a reusable foundation for more than CompatPy.

### 14.1. Define ecosystem boundaries

- [ ] Document CompatLab's responsibilities.
- [ ] Document CompatPy's responsibilities.
- [ ] Identify other potential Compat tools.
- [ ] Establish shared schema and report conventions.
- [ ] Avoid duplicating core execution logic in consumers.
- [ ] Avoid coupling CompatLab to a single frontend.
- [ ] Define version compatibility between consumers and CompatLab.

### 14.2. Build shared integration patterns

- [ ] Provide a documented CLI integration.
- [ ] Provide JSON integration examples.
- [ ] Provide Rust library usage examples.
- [ ] Establish stable error-reporting conventions.
- [ ] Define artifact-storage conventions.
- [ ] Provide integration-test fixtures.
- [ ] Document batch-experiment requests.
- [ ] Document how consumers interpret results.

### 14.3. Consider ecosystem tooling

- [ ] Evaluate a shared compatibility report viewer.
- [ ] Consider report comparison across releases.
- [ ] Consider a common experiment registry.
- [ ] Consider reusable runtime-discovery utilities.
- [ ] Consider shared configuration formats.
- [ ] Consider a common library for other Compat tools.
- [ ] Extract separate crates only when real consumers need them.

**Completion gate:** Multiple tools can use CompatLab without depending on one another's internals, and common compatibility functionality has a clear owner.

---

## Phase 15: Advanced compatibility intelligence

**Goal:** Improve how CompatLab selects, compares, and explains compatibility experiments.

These are long-term goals, not requirements for the first stable release.

### 15.1. Rich compatibility analysis

- [ ] Compare results across runtime versions.
- [ ] Identify earliest known passing versions.
- [ ] Identify latest known passing versions.
- [ ] Distinguish confirmed results from untested combinations.
- [ ] Detect regressions between runs.
- [ ] Track compatibility trends across package releases.
- [ ] Explain which checks caused failures.
- [ ] Avoid claiming compatibility from insufficient evidence.

### 15.2. Better experiment selection

- [ ] Select relevant runtime versions automatically.
- [ ] Support user-defined compatibility policies.
- [ ] Prioritize versions based on project requirements.
- [ ] Reduce redundant experiments.
- [ ] Support explicit exclusion rules.
- [ ] Explain why an experiment was selected.
- [ ] Allow users to override automatic selections.

### 15.3. CI and developer workflows

- [ ] Provide documented CI integration.
- [ ] Allow experiments to run on pull requests.
- [ ] Compare new results with a baseline.
- [ ] Detect newly introduced compatibility failures.
- [ ] Export results in formats suitable for CI summaries.
- [ ] Define optional quality gates.
- [ ] Keep local and CI results reproducible.

### 15.4. Future platform possibilities

- [ ] Evaluate more package managers.
- [ ] Evaluate additional language runtimes.
- [ ] Evaluate container-based execution for stronger isolation.
- [ ] Evaluate remote execution on separate machines.
- [ ] Evaluate a report viewer.
- [ ] Evaluate experiment-result storage.
- [ ] Evaluate integrations with other developer tools.
- [ ] Evaluate community-contributed language adapters.

Every item in this phase is conditional. Implement it when there is a clear use case, not merely because it sounds impressive.

---

## Recommended execution order

1. **Finish Phase 1.** Complete timeout status mapping, report lifecycle coverage, input validation, process cleanup, and failure-path tests.
2. **Specify Phase 2.** Document the initial experiment and report contracts before external consumers depend on them.
3. **Build Phase 3.** Make the CLI useful for validation, execution, and report inspection, adding discovery commands when the project model is ready.
4. **Strengthen Phases 4-6 continuously.** Improve Python capability and isolation while maintaining Windows, Linux, and macOS CI.
5. **Integrate CompatPy in Phase 7.** Prove the consumer boundary using the documented contract.
6. **Build Phase 8.** Make batch and matrix execution reliable before coordinating whole projects.
7. **Complete Phase 9 from evidence.** Introduce the smallest useful adapter boundary while preserving the existing workspace and working Python behavior.
8. **Complete the initial release requirements in Phase 10.** Publish only what is implemented, tested, documented, and supported.
9. **Build Phases 16-17.** Introduce the project/component model and safe, explainable discovery.
10. **Build Phase 18.** Coordinate independent component experiments and aggregate outcomes without hiding unsupported or untested scope.
11. **Prove Phase 19 with one real integration scenario.** Keep integration checks separate from independent component checks.
12. **Harden Phases 20-25.** Establish adapter conformance, configuration, trust boundaries, execution graphs when needed, report semantics, and quality gates.
13. **Implement Phases 11-13 incrementally.** Add JavaScript/Node.js, GDScript/Godot, and Rust adapters one at a time using the same conformance and cross-platform standards.
14. **Grow Phases 14-15 and 26-27.** Expand ecosystem integrations, compatibility intelligence, performance, and contributor experience based on measured needs.
15. **Evaluate Phase 28 candidates individually.** Remote execution, hosted services, additional languages, and advanced automation require explicit use cases and acceptance criteria.

This order is deliberately dependency-aware, not a demand to delay all releases until every phase is complete. The project should deliver useful increments, but must not skip a prerequisite simply to claim a feature sooner.

## Architectural rule

**Standardize the experiment lifecycle, not the behavior of every language.**

Python package installation and imports, JavaScript module loading, Godot project execution, and Rust compilation have different semantics. They can share orchestration, structured results, timeouts, diagnostics, and reporting while retaining language-specific checks.

That is how CompatLab can grow without becoming one enormous collection of language-specific conditionals.

## Current next step

Continue with **Phase 1: Core reliability and correctness**. In particular, finish timeout-status coverage for installation and import operations, verify stage/duration reporting, and test cleanup. Do not start a broad refactor or add a second adapter yet. Make the current engine reliable first, then specify the stable experiment/report contract before extracting language-specific behavior.

The longer-term architectural destination is explicit: **CompatLab is the main application and engine; language adapters are optional capabilities; a project can contain one or many language components; and project-wide integration checks are distinct from individual component checks.** This destination guides design decisions now, but does not justify implementing every future abstraction before it is needed.

---

_This roadmap is an evolving plan, not a promise that every future feature will be implemented. Revisit it as CompatLab and its consumers reveal real requirements._

---

## Phase 16: Project and workspace model

**Goal:** Introduce a first-class project model without making a single package or directory unnecessarily complicated.

A project is the root unit a user asks CompatLab to inspect. It may contain one component or many. A component is a testable unit owned by a language adapter, such as a Python package, Node.js workspace, Rust crate, or Godot project. A repository may contain nested projects, shared tooling, generated code, examples, and unrelated directories, so discovery must be explicit and explainable.

### 16.1. Define project concepts

- [ ] Define `Project` independently of a language-specific experiment.
- [ ] Define a stable project identifier and display name.
- [ ] Define project root and allowed filesystem boundaries.
- [ ] Define `Component` and its stable identifier.
- [ ] Define component kind, language, root directory, and adapter identifier.
- [ ] Define relationships between components.
- [ ] Define project metadata separately from discovered facts.
- [ ] Distinguish user-declared metadata from auto-detected metadata.
- [ ] Define a project manifest format and schema version.
- [ ] Decide whether a manifest is optional, required, or generated by discovery.
- [ ] Support an explicit manifest that overrides ambiguous automatic detection.
- [ ] Define how nested projects are represented.
- [ ] Define how a component may belong to more than one logical project.
- [ ] Define path normalization and path identity across Windows, Linux, and macOS.
- [ ] Define behavior for symlinks and paths escaping the project root.
- [ ] Define how case-sensitive and case-insensitive filesystems affect matching.
- [ ] Document the difference between repository, project, component, experiment, and report.

### 16.2. Model project composition

- [ ] Represent a single-language project as one project with one or more components.
- [ ] Represent a multilingual project without a special alternate execution engine.
- [ ] Support multiple components written in the same language.
- [ ] Support multiple components using different versions of the same runtime.
- [ ] Support components that share dependencies or generated artifacts.
- [ ] Support optional components.
- [ ] Support explicitly excluded components.
- [ ] Define how examples, tests, benchmarks, and production components are classified.
- [ ] Define how monorepos and nested workspaces are represented.
- [ ] Avoid assuming one repository equals one package.
- [ ] Avoid assuming every directory with a manifest should be tested.
- [ ] Define whether generated/vendor directories are ignored by default.
- [ ] Provide a way to inspect the detected project model without executing code.

### 16.3. Manifest and discovery precedence

- [ ] Define precedence between CLI flags, explicit project manifest, adapter metadata, and heuristics.
- [ ] Report conflicts between user configuration and detected metadata.
- [ ] Never silently overwrite explicit user configuration with a heuristic.
- [ ] Validate the manifest before execution.
- [ ] Provide clear errors for unknown adapter names and unsupported manifest versions.
- [ ] Provide migration guidance when the manifest schema changes.
- [ ] Include a complete minimal manifest example.
- [ ] Include a multilingual manifest example.
- [ ] Include examples for explicit exclusions and component relationships.
- [ ] Test round-trip serialization of project manifests.

### 16.4. Project model acceptance criteria

- [ ] A single-language project can be represented without multilingual-only fields being required.
- [ ] A multilingual repository can represent each component independently.
- [ ] Component identifiers remain stable across repeated discovery when the inputs have not changed.
- [ ] Ambiguous discovery is reported rather than silently guessed.
- [ ] The project model can be serialized and consumed outside the Rust library.
- [ ] Unit and integration tests cover manifest parsing, validation, and path handling.

**Completion gate:** CompatLab can describe a project as one or more independently testable components, regardless of how many languages it contains, using a documented and versioned model.

---

## Phase 17: Project discovery and component inventory

**Goal:** Identify what is present in a repository, which adapters may handle it, and what evidence supports each detection.

Discovery must be a separate operation from execution. Listing a repository must not install dependencies, run build scripts, or execute project code.

### 17.1. Safe filesystem discovery

- [ ] Walk project directories using an explicit traversal policy.
- [ ] Respect user-defined include and exclude paths.
- [ ] Define default exclusions for version-control internals, build output, caches, virtual environments, vendor directories, and generated artifacts.
- [ ] Make exclusions visible and configurable.
- [ ] Prevent accidental traversal outside the selected root through symlinks.
- [ ] Detect permission errors and report incomplete discovery.
- [ ] Handle very large repositories without unbounded memory use.
- [ ] Avoid following symlink loops.
- [ ] Define behavior for junctions and reparse points on Windows.
- [ ] Normalize discovered paths for stable reporting.
- [ ] Support a dry-run or discovery-only mode.
- [ ] Ensure discovery does not modify the repository.
- [ ] Record discovery warnings and skipped paths.
- [ ] Allow users to inspect why a path was included or excluded.

### 17.2. Language and project markers

- [ ] Define evidence markers for each adapter.
- [ ] Detect Python project metadata and requirements files.
- [ ] Detect JavaScript/Node.js manifests and lockfiles.
- [ ] Detect Rust `Cargo.toml` files and workspace declarations.
- [ ] Detect Godot project configuration and GDScript files.
- [ ] Treat file extensions as evidence, not proof of an executable project.
- [ ] Distinguish language presence from a runnable component.
- [ ] Distinguish an adapter being installed from an adapter being able to run.
- [ ] Support multiple markers in one directory.
- [ ] Handle ambiguous or conflicting markers.
- [ ] Record detection confidence or evidence without pretending heuristics are infallible.
- [ ] Let users override automatic component classification.
- [ ] Define how unknown languages are represented.
- [ ] Keep unknown files visible in discovery output without claiming support.

### 17.3. Workspace and monorepo discovery

- [ ] Detect Python workspace and package layouts.
- [ ] Detect npm workspaces and nested package manifests.
- [ ] Detect Cargo workspaces and member crates.
- [ ] Detect repositories with multiple independent projects.
- [ ] Detect common source layouts such as `apps/`, `packages/`, `crates/`, and `services/` only as hints.
- [ ] Avoid hard-coding one repository layout as universal.
- [ ] Identify likely shared configuration files.
- [ ] Identify lockfiles and the components they govern.
- [ ] Identify generated code and build outputs where practical.
- [ ] Report overlapping component roots.
- [ ] Define whether parent and child components may both run experiments.
- [ ] Allow users to split or merge detected components through configuration.

### 17.4. Discovery output and tests

- [ ] Provide machine-readable discovery output.
- [ ] Provide human-readable component inventory.
- [ ] Include detection evidence, root paths, adapter identifiers, and warnings.
- [ ] Test empty repositories.
- [ ] Test repositories with one supported language.
- [ ] Test multilingual repositories.
- [ ] Test nested workspaces.
- [ ] Test misleading file extensions and incomplete manifests.
- [ ] Test excluded directories and symlinks.
- [ ] Test inaccessible paths.
- [ ] Test deterministic output ordering.
- [ ] Test that discovery does not execute subprocesses or modify files.

**Completion gate:** Users can inspect a trustworthy component inventory before any potentially expensive or code-executing experiment begins.

---

## Phase 18: Multilingual project orchestration

**Goal:** Coordinate independent component experiments and produce a single project-level result without conflating component success with whole-project success.

### 18.1. Plan generation

- [ ] Define a `ProjectPlan` separate from execution results.
- [ ] Convert a project manifest and discovery results into a plan.
- [ ] Assign stable identifiers to planned experiments.
- [ ] Record why each experiment was included.
- [ ] Record prerequisites and declared dependencies.
- [ ] Support user-selected components.
- [ ] Support selecting a subset of runtimes or versions.
- [ ] Support excluding expensive or optional checks.
- [ ] Validate the complete plan before executing any experiment.
- [ ] Report unsupported components before execution.
- [ ] Provide a plan preview and estimated experiment count.
- [ ] Define whether an invalid component blocks the entire plan or only that component.
- [ ] Ensure a plan can be serialized and rerun.
- [ ] Define plan versioning and migration rules.

### 18.2. Independent component execution

- [ ] Execute each component using its matching adapter.
- [ ] Preserve independent component outcomes.
- [ ] Prevent one component's working directory or environment from leaking into another.
- [ ] Allow an individual component to fail without erasing unrelated results.
- [ ] Define fail-fast and continue-on-error policies.
- [ ] Define how user cancellation affects active and pending components.
- [ ] Keep experiment IDs stable across retries of the same plan.
- [ ] Record skipped components and their reasons.
- [ ] Handle missing adapters and runtimes explicitly.
- [ ] Make execution order deterministic when ordering matters.
- [ ] Permit bounded concurrency after isolation and cancellation are reliable.
- [ ] Ensure project-level status is derived from documented rules.

### 18.3. Project-level outcome semantics

- [ ] Define statuses for all-passed, partially-passed, failed, timed-out, blocked, skipped, and incomplete projects.
- [ ] Distinguish a project with unsupported components from a fully tested project.
- [ ] Distinguish component failure from plan/configuration failure.
- [ ] Distinguish infrastructure failure from incompatibility.
- [ ] Define whether warnings can coexist with success.
- [ ] Define how optional components affect the aggregate result.
- [ ] Define how skipped experiments affect confidence.
- [ ] Never label an incompletely tested project as fully compatible.
- [ ] Include the exact scope of tested components in the summary.
- [ ] Test all combinations of component outcomes that affect aggregate status.

### 18.4. Project plan reproducibility

- [ ] Store the discovery snapshot or sufficient metadata to explain the plan.
- [ ] Store the selected adapter versions.
- [ ] Store relevant runtime selections.
- [ ] Store the plan configuration and schema version.
- [ ] Detect when source manifests change after planning.
- [ ] Define whether execution uses a frozen plan or re-discovers components.
- [ ] Make re-planning explicit.
- [ ] Support comparing plans from different commits.
- [ ] Document what is and is not guaranteed reproducible.

**Completion gate:** A multilingual repository can run independent component experiments through one plan and receive a truthful aggregate report, with no loss of per-component details.

---

## Phase 19: Cross-language integration experiments

**Goal:** Test combinations of components that must work together, not only whether each component works independently.

This phase follows independent component testing. Cross-language integration is more complex because it may require building artifacts, launching services, configuring protocols, and coordinating multiple processes.

### 19.1. Define integration relationships

- [ ] Define an explicit relationship between two or more components.
- [ ] Represent producer and consumer roles where applicable.
- [ ] Represent build-time, link-time, load-time, and runtime dependencies.
- [ ] Represent a Python package that loads a native Rust extension.
- [ ] Represent a frontend that communicates with a backend service.
- [ ] Represent generated bindings or foreign-function interfaces.
- [ ] Represent shared schemas and protocol contracts.
- [ ] Represent a Godot project that consumes native or networked components where appropriate.
- [ ] Define integration test inputs, setup steps, checks, and cleanup.
- [ ] Keep integration declarations separate from assumptions inferred by discovery.
- [ ] Report unknown relationships instead of inventing them.
- [ ] Allow integration checks to be optional or required by policy.

### 19.2. Integration lifecycle

- [ ] Define setup, build, launch, readiness, check, teardown, and cleanup stages.
- [ ] Give each stage a timeout and diagnostic record.
- [ ] Support dependencies between stages.
- [ ] Ensure teardown runs after success, failure, timeout, or cancellation.
- [ ] Handle partial startup when only some services launch.
- [ ] Capture logs per component and per stage.
- [ ] Avoid conflating readiness failure with application incompatibility.
- [ ] Define how ports, temporary paths, and other resources are allocated.
- [ ] Avoid fixed-port collisions during parallel tests.
- [ ] Test process cleanup when one service times out.
- [ ] Define behavior when a dependent component cannot be built.
- [ ] Ensure teardown errors do not erase the primary failure.

### 19.3. Integration contracts

- [ ] Define generic integration metadata without pretending protocols are identical.
- [ ] Support explicit health checks or readiness probes.
- [ ] Support command-based checks where appropriate.
- [ ] Support protocol-specific checks through adapters.
- [ ] Define input/output artifact contracts between components.
- [ ] Validate that required artifacts exist before downstream execution.
- [ ] Record versions and hashes of produced artifacts where useful.
- [ ] Distinguish ABI, API, protocol, and data-format compatibility.
- [ ] Avoid claiming cross-language compatibility solely from successful builds.
- [ ] Provide sample integration fixtures for native extension and service-based projects.

### 19.4. Integration test matrix

- [ ] Test each component independently before the combined integration test.
- [ ] Test a known-good combination.
- [ ] Test a component that builds but fails to load.
- [ ] Test a consumer with an incompatible producer version.
- [ ] Test service readiness timeout.
- [ ] Test startup failure and teardown.
- [ ] Test artifact missing or corrupt.
- [ ] Test incompatible protocol/schema versions.
- [ ] Test cancellation during setup and during execution.
- [ ] Test cross-platform integration where the toolchains are supported.
- [ ] Define how integration outcomes roll up into project status.

**Completion gate:** CompatLab can represent and execute at least one real multi-component integration scenario, clean up all participating processes, and report the precise stage and relationship that failed.

---

## Phase 20: Adapter lifecycle and capability contract

**Goal:** Establish one stable internal adapter contract while allowing each language to expose only the capabilities it genuinely supports.

### 20.1. Adapter identity and metadata

- [ ] Define adapter ID, display name, language, version, and contract version.
- [ ] Define supported project markers and component types.
- [ ] Define supported operating systems and CPU architectures.
- [ ] Define runtime discovery capabilities.
- [ ] Define supported experiment types.
- [ ] Define adapter configuration schema.
- [ ] Define required external tools.
- [ ] Define how missing tools are diagnosed.
- [ ] Define how adapters declare optional features.
- [ ] Define how an adapter reports its own version and build metadata.
- [ ] Avoid relying on display names as machine identifiers.

### 20.2. Adapter lifecycle

- [ ] Define discovery hooks.
- [ ] Define validation hooks.
- [ ] Define runtime inspection hooks.
- [ ] Define environment preparation hooks.
- [ ] Define execution hooks.
- [ ] Define output collection hooks.
- [ ] Define cleanup hooks.
- [ ] Define cancellation and timeout integration.
- [ ] Define which lifecycle methods may mutate files or launch processes.
- [ ] Define cleanup behavior if preparation only partially succeeds.
- [ ] Define how adapter errors are normalized.
- [ ] Ensure adapter errors retain language-specific diagnostic detail.

### 20.3. Capabilities and unsupported operations

- [ ] Define capability negotiation before an experiment starts.
- [ ] Let adapters declare unsupported experiment types.
- [ ] Distinguish unsupported, unavailable, disabled, and failed capabilities.
- [ ] Avoid silent fallback to a weaker check.
- [ ] Record when a requested check is skipped or downgraded.
- [ ] Define a capability matrix in documentation.
- [ ] Test capability negotiation with fake adapters.
- [ ] Ensure the core does not need language-name conditionals for ordinary lifecycle orchestration.

### 20.4. Extension delivery strategy

- [ ] Start with statically compiled internal adapters.
- [ ] Keep the adapter boundary suitable for future separate crates.
- [ ] Do not implement a dynamic plugin loader without a concrete use case.
- [ ] If external adapters become necessary, define discovery, installation, trust, versioning, and compatibility rules first.
- [ ] Never load an external adapter silently.
- [ ] Define how adapter crashes are contained if external execution is supported.
- [ ] Define signature or provenance expectations for distributed adapters if applicable.
- [ ] Document the difference between built-in adapters and third-party extensions.

### 20.5. Adapter conformance suite

- [ ] Create reusable conformance tests for adapter metadata.
- [ ] Test invalid configuration rejection.
- [ ] Test success, failure, timeout, cancellation, and unsupported outcomes.
- [ ] Test diagnostic preservation.
- [ ] Test cleanup after partial setup.
- [ ] Test deterministic report structure.
- [ ] Provide a reference adapter fixture.
- [ ] Require every new adapter to pass the conformance suite before being considered supported.

**Completion gate:** A second adapter can be added without rewriting the core lifecycle, and it must pass a shared conformance suite.

---

## Phase 21: Configuration, policy, and reproducible plans

**Goal:** Make behavior configurable and predictable for individual developers, CI, and large repositories.

### 21.1. Configuration sources

- [ ] Define project configuration file discovery.
- [ ] Define CLI override behavior.
- [ ] Define environment-variable configuration where justified.
- [ ] Define precedence and conflict reporting.
- [ ] Avoid ambiguous implicit defaults.
- [ ] Provide `config show` or equivalent effective-configuration output.
- [ ] Redact secrets from configuration output.
- [ ] Validate all configuration before execution.
- [ ] Version the configuration schema.
- [ ] Provide migration instructions for breaking configuration changes.

### 21.2. Compatibility policies

- [ ] Define a policy for required and optional components.
- [ ] Define allowed and required runtime ranges.
- [ ] Define required experiment types.
- [ ] Define whether warnings or skipped checks fail CI.
- [ ] Define policy for unsupported adapters.
- [ ] Define policy for network access.
- [ ] Define resource and time budgets.
- [ ] Define retry policy only for explicitly retryable failures.
- [ ] Explain which policy rule caused a project to pass or fail.
- [ ] Keep policy decisions separate from raw experiment outcomes.

### 21.3. Plan locking and repeatability

- [ ] Define whether a plan can be locked to exact runtime and adapter versions.
- [ ] Record dependency lockfile identity where applicable.
- [ ] Record source revision or working-tree state when available.
- [ ] Detect changed inputs before execution.
- [ ] Define behavior for dirty working trees.
- [ ] Support exporting a plan for CI.
- [ ] Support re-running a stored plan.
- [ ] Distinguish exact reproducibility from best-effort repeatability.
- [ ] Define how runtime auto-discovery results are frozen in a plan.
- [ ] Test that configuration changes invalidate or regenerate affected plans.

### 21.4. Configuration testing

- [ ] Test default configuration.
- [ ] Test each override source.
- [ ] Test precedence conflicts.
- [ ] Test unknown keys.
- [ ] Test invalid types and ranges.
- [ ] Test schema-version mismatches.
- [ ] Test secret redaction.
- [ ] Test that invalid configuration prevents execution from starting.
- [ ] Keep examples synchronized with the schema and implementation.

**Completion gate:** Users can inspect the effective configuration, understand why a plan was generated, and reproduce the intended experiment selection.

---

## Phase 22: Security, trust boundaries, and sandbox strategy

**Goal:** Make security claims accurate, minimize accidental damage, and provide a path toward stronger isolation for untrusted code.

### 22.1. Threat model

- [ ] Document whether CompatLab is intended to run trusted projects, untrusted repositories, or both.
- [ ] Identify code execution points such as package installation, build scripts, imports, tests, and project launch commands.
- [ ] Identify filesystem, network, process, credential, and resource risks.
- [ ] Identify threats from malicious dependencies and compromised toolchains.
- [ ] Identify risks from adapter bugs and malformed reports.
- [ ] Define the trust boundary between CompatLab, adapters, and tested code.
- [ ] Document which threats the current implementation does not mitigate.
- [ ] Review the threat model whenever new execution modes are added.

### 22.2. Safe defaults

- [ ] Require explicit project root and clearly scoped paths.
- [ ] Avoid executing project code during discovery.
- [ ] Avoid invoking shell strings when structured process arguments are sufficient.
- [ ] Avoid writing outside designated temporary and output directories by default.
- [ ] Avoid changing global runtime or package-manager configuration.
- [ ] Avoid embedding credentials in command lines where possible.
- [ ] Redact tokens and secrets from logs and reports.
- [ ] Do not store environment dumps by default.
- [ ] Define behavior for symbolic links and path traversal.
- [ ] Make network use and package downloads visible.
- [ ] Require explicit configuration for privileged or host-mutating operations.
- [ ] Keep cleanup scoped to directories created by CompatLab.

### 22.3. Isolation levels

- [ ] Document the guarantees of a temporary directory.
- [ ] Document the guarantees of a language virtual environment.
- [ ] Define an isolation capability model per adapter.
- [ ] Evaluate process-level restrictions where portable.
- [ ] Evaluate container-based execution for stronger isolation.
- [ ] Evaluate OS sandboxing mechanisms where practical.
- [ ] Evaluate disposable virtual machines or remote workers for high-risk jobs.
- [ ] Make isolation level visible in reports.
- [ ] Avoid claiming a container or virtual environment is perfectly secure.
- [ ] Define how network and filesystem access restrictions are configured.
- [ ] Test isolation boundaries with controlled fixtures before making security claims.

### 22.4. Secret and artifact handling

- [ ] Define a redaction policy for known secret patterns.
- [ ] Avoid including credentials in stdout/stderr by default where feasible.
- [ ] Define artifact retention and deletion.
- [ ] Prevent one experiment from reading another experiment's private artifacts.
- [ ] Set restrictive permissions on temporary data where supported.
- [ ] Avoid retaining package caches containing private artifacts without user consent.
- [ ] Define safe behavior for private package indexes.
- [ ] Document that redaction is defense in depth and cannot guarantee detection of every secret.

### 22.5. Security validation

- [ ] Test path traversal inputs.
- [ ] Test symlink escape scenarios.
- [ ] Test malformed configuration and report payloads.
- [ ] Test child-process cleanup.
- [ ] Test secret redaction.
- [ ] Test cleanup after forced termination.
- [ ] Add dependency auditing where practical.
- [ ] Publish a vulnerability reporting policy.
- [ ] Document supported security update practices.

**Completion gate:** Security boundaries are documented, unsafe assumptions are avoided, and any stronger isolation claims are backed by tests and platform-specific evidence.

---

## Phase 23: Execution graph, artifacts, and dependency relationships

**Goal:** Represent work as a graph when experiments have prerequisites, generated artifacts, or integration dependencies.

A graph is not required for the first single-component runner. Introduce it when project plans need to express real dependencies between tasks.

### 23.1. Execution graph model

- [ ] Define task nodes and dependency edges.
- [ ] Define ordering constraints separately from data dependencies.
- [ ] Detect dependency cycles before execution.
- [ ] Validate missing dependency references.
- [ ] Define task states such as pending, ready, running, succeeded, failed, blocked, cancelled, and timed out.
- [ ] Define how a failed prerequisite affects dependent tasks.
- [ ] Support independent branches running concurrently.
- [ ] Preserve deterministic plan serialization.
- [ ] Provide a human-readable graph view.
- [ ] Explain why a task was blocked.
- [ ] Test graph validation and cycle detection.

### 23.2. Artifact model

- [ ] Define artifact identity, type, producer, and provenance.
- [ ] Record path or storage reference without assuming all artifacts are files.
- [ ] Record hashes when integrity verification is useful.
- [ ] Define artifact lifetime and cleanup ownership.
- [ ] Define how artifacts are passed to downstream experiments.
- [ ] Validate required artifacts before use.
- [ ] Prevent ambiguous artifact names from silently resolving to the wrong producer.
- [ ] Distinguish temporary artifacts from user-requested retained outputs.
- [ ] Avoid copying huge artifacts unnecessarily.
- [ ] Define how artifact metadata is represented in reports.

### 23.3. Incremental execution

- [ ] Determine whether a task's inputs changed before reusing its result.
- [ ] Define cache keys for task inputs, adapter version, runtime, and relevant environment.
- [ ] Invalidate dependent tasks when an upstream artifact changes.
- [ ] Never reuse results when required provenance is missing.
- [ ] Make cache hits visible.
- [ ] Permit a forced clean run.
- [ ] Test partial reruns after one component changes.
- [ ] Keep incremental execution optional until its correctness is established.

### 23.4. Graph execution correctness

- [ ] Test independent tasks executing without ordering assumptions.
- [ ] Test blocked descendants after prerequisite failure.
- [ ] Test cancellation propagation.
- [ ] Test timeout propagation.
- [ ] Test artifact cleanup on failure.
- [ ] Test concurrent tasks do not share mutable workspaces accidentally.
- [ ] Test resuming or re-running a partially completed graph.
- [ ] Test report integrity after multiple failures.

**Completion gate:** A multi-component plan with real prerequisites can execute predictably, pass artifacts safely, and explain blocked or failed downstream tasks.

---

## Phase 24: Project-level reporting and result interpretation

**Goal:** Make results useful to both humans and automation while preserving exact evidence.

### 24.1. Layered report structure

- [ ] Define a project report containing project metadata, plan metadata, component reports, integration reports, and summary.
- [ ] Keep raw experiment results available beneath aggregate summaries.
- [ ] Include requested scope and actual executed scope.
- [ ] Include skipped, unsupported, blocked, and not-run items.
- [ ] Include adapter and runtime identities.
- [ ] Include timestamps and durations with documented semantics.
- [ ] Include diagnostics by stage.
- [ ] Include links or references to retained logs and artifacts where applicable.
- [ ] Define stable identifiers for cross-referencing records.
- [ ] Keep reports usable when a component fails to start.

### 24.2. Explainability and evidence

- [ ] State exactly what was tested.
- [ ] State what was not tested.
- [ ] Distinguish direct observations from inferred conclusions.
- [ ] Link each aggregate failure to underlying experiments.
- [ ] Explain the policy rule that determined overall status.
- [ ] Include the command or operation category without exposing secrets.
- [ ] Preserve concise diagnostic summaries and detailed raw output separately.
- [ ] Avoid claiming a package or project is universally compatible.
- [ ] Distinguish tested compatibility from metadata-based prediction.
- [ ] Define how stale or cached evidence is labeled.

### 24.3. Report formats and presentation

- [ ] Provide stable JSON for tools.
- [ ] Provide readable CLI summaries.
- [ ] Consider JUnit XML for test-oriented CI systems.
- [ ] Consider SARIF only if diagnostics map meaningfully to its model.
- [ ] Consider Markdown summaries for pull requests.
- [ ] Keep presentation formats derived from the canonical report model.
- [ ] Define output-size and truncation behavior.
- [ ] Ensure truncation is explicit and does not destroy the canonical retained result.
- [ ] Test report serialization for partial and interrupted runs.

### 24.4. Report comparison

- [ ] Compare project reports from different revisions.
- [ ] Identify newly passing and newly failing components.
- [ ] Identify changes in skipped or unsupported scope.
- [ ] Identify changes in runtime coverage.
- [ ] Distinguish a genuine regression from a changed test matrix.
- [ ] Show changes in duration without over-interpreting noisy measurements.
- [ ] Preserve the baseline report identity.
- [ ] Test comparisons when schemas differ across report versions.

**Completion gate:** A developer can understand both the overall project result and the evidence for each component, while automation can consume stable structured data.

---

## Phase 25: Quality engineering and acceptance strategy

**Goal:** Define rigorous quality gates so new adapters and project-level features do not reduce reliability.

### 25.1. Testing layers

- [ ] Unit-test pure validation and schema logic.
- [ ] Unit-test orchestration and status aggregation.
- [ ] Use controlled fake adapters for lifecycle edge cases.
- [ ] Add integration tests with small local fixtures.
- [ ] Add end-to-end tests through the CLI.
- [ ] Add cross-platform tests for OS-dependent behavior.
- [ ] Add contract tests for every adapter.
- [ ] Add regression tests for every confirmed defect.
- [ ] Keep network-dependent tests separate from deterministic core tests.
- [ ] Make test prerequisites and skips explicit.
- [ ] Ensure tests clean up all temporary processes and directories.

### 25.2. Test integrity rules

- [ ] Never weaken a valid assertion merely to make a failing test pass.
- [ ] Investigate whether a failure is caused by implementation, test setup, environment, or an invalid expectation.
- [ ] Use deterministic fixtures where possible.
- [ ] Avoid arbitrary sleeps when a synchronization mechanism is available.
- [ ] Where timing is inherently involved, use generous but meaningful bounds and test the actual semantic outcome.
- [ ] Test both expected result and relevant side effects.
- [ ] Do not let a child-process test accidentally match the parent test or run recursively.
- [ ] Verify tests fail when the relevant implementation behavior is deliberately broken where practical.
- [ ] Do not suppress compiler or Clippy warnings to get a green build.
- [ ] Keep negative-test stderr understandable and intentional.
- [ ] Document unavoidable platform-specific test constraints.

### 25.3. Continuous integration

- [ ] Keep formatting checks enabled.
- [ ] Run workspace checks and builds.
- [ ] Run the complete test suite.
- [ ] Run Clippy with warnings denied.
- [ ] Run adapter-specific checks only where the required tools are installed.
- [ ] Test supported operating systems and architectures.
- [ ] Pin third-party CI actions according to project policy.
- [ ] Review CI deprecation notices.
- [ ] Cache dependencies without making cache contents a correctness requirement.
- [ ] Publish test logs and diagnostics for failed jobs.
- [ ] Keep CI duration bounded and divide slow integration suites from fast checks.
- [ ] Test release artifacts separately from debug builds.

### 25.4. Acceptance criteria per feature

- [ ] Every roadmap item that changes behavior has an explicit acceptance criterion.
- [ ] Every bug fix includes a regression test when feasible.
- [ ] Every schema change updates examples and contract tests.
- [ ] Every adapter addition passes the adapter conformance suite.
- [ ] Every new platform claim is supported by CI or documented manual verification.
- [ ] Every security claim identifies its tested boundary.
- [ ] Every completion checkbox is supported by repository evidence.
- [ ] Maintain a distinction between implemented, tested, documented, and released.

**Completion gate:** Quality checks are repeatable, failures are actionable, and new features cannot be declared done solely because the code compiles.

---

## Phase 26: Performance, scale, and resource governance

**Goal:** Keep CompatLab responsive and bounded as project size, language count, and experiment matrices grow.

### 26.1. Measure before optimizing

- [ ] Measure discovery time on small, medium, and large repositories.
- [ ] Measure plan-generation cost.
- [ ] Measure per-experiment setup and execution time.
- [ ] Measure memory and temporary-storage use.
- [ ] Measure concurrency overhead.
- [ ] Define practical performance budgets for common workflows.
- [ ] Avoid premature micro-optimization.
- [ ] Keep benchmark fixtures representative and reproducible.

### 26.2. Bounded execution

- [ ] Define maximum active experiments.
- [ ] Define per-adapter concurrency limits where necessary.
- [ ] Define per-project time and resource budgets.
- [ ] Define output buffering limits.
- [ ] Define temporary storage cleanup and quotas where feasible.
- [ ] Avoid launching an unbounded number of runtimes or builds.
- [ ] Apply backpressure to report and log collection.
- [ ] Define behavior when a resource budget is exhausted.
- [ ] Report resource-limit termination distinctly from compatibility failure.

### 26.3. Large matrices and scheduling

- [ ] Estimate the number of planned combinations before execution.
- [ ] Warn when a matrix expands unexpectedly.
- [ ] Allow users to cap total experiments.
- [ ] Support prioritizing required combinations.
- [ ] Support stopping after a policy-defined condition when explicitly requested.
- [ ] Preserve unexecuted combinations in the report.
- [ ] Avoid hiding untested combinations in a summary.
- [ ] Define fair scheduling between components when useful.
- [ ] Ensure retries do not multiply work without bounds.

### 26.4. Performance acceptance

- [ ] Benchmark discovery and planning separately from execution.
- [ ] Test cancellation responsiveness.
- [ ] Test output collection under heavy output.
- [ ] Test many small experiments.
- [ ] Test a small number of long-running experiments.
- [ ] Verify that resource limits remain enforced under concurrency.
- [ ] Document hardware and environment used for benchmarks.

**Completion gate:** Large projects and matrices remain observable and bounded, and performance optimizations preserve correctness.

---

## Phase 27: Developer experience and ecosystem adoption

**Goal:** Make CompatLab understandable, scriptable, contributor-friendly, and practical in real workflows.

### 27.1. First-run experience

- [ ] Provide a quick start that works on a clean supported machine.
- [ ] Provide a command to inspect available adapters and runtimes.
- [ ] Provide a discovery-only command.
- [ ] Provide a plan preview before execution.
- [ ] Explain missing runtimes and installation requirements.
- [ ] Provide actionable errors without requiring debug logs.
- [ ] Keep verbose diagnostics available for deeper investigation.
- [ ] Avoid network access or package installation during simple help/version commands.

### 27.2. CI and editor integration

- [ ] Document CI usage with JSON output and exit codes.
- [ ] Provide examples for GitHub Actions and other CI systems.
- [ ] Support baseline comparisons in CI.
- [ ] Define pull-request summary output.
- [ ] Provide machine-readable output suitable for editor tooling.
- [ ] Consider editor integrations only after CLI and schema stability.
- [ ] Avoid making editor integrations mandatory for core functionality.
- [ ] Document local-versus-CI configuration differences.

### 27.3. Contributor workflow

- [ ] Document the repository architecture.
- [ ] Document how to run all checks locally.
- [ ] Document how to add a core feature.
- [ ] Document how to add or modify an adapter.
- [ ] Provide small fixture projects for each adapter.
- [ ] Add issue and pull-request templates.
- [ ] Define review expectations for tests, errors, and cross-platform behavior.
- [ ] Document how roadmap checkboxes are verified.
- [ ] Keep architecture decision records for major decisions.
- [ ] Record rejected alternatives and why they were rejected when useful.

### 27.4. User support and feedback

- [ ] Document how to report a bug with a redacted report.
- [ ] Provide a diagnostic bundle that excludes secrets by default.
- [ ] Define support expectations for stable and experimental adapters.
- [ ] Distinguish bugs in CompatLab from incompatibilities in tested projects.
- [ ] Provide troubleshooting for toolchain discovery and permission failures.
- [ ] Maintain a known-limitations page.
- [ ] Collect feature requests without promising every adapter or runtime.

**Completion gate:** A new user can discover, plan, execute, and interpret a project test; a new contributor can understand the architecture and add a tested feature.

---

## Phase 28: Long-term integrations and research candidates

**Goal:** Preserve ambitious options without turning every possibility into a near-term dependency.

These are research candidates. Investigate each one with a concrete use case, a small prototype, an explicit decision, and documented trade-offs before committing to implementation.

### 28.1. Additional language and runtime adapters

- [ ] Evaluate additional Python implementations where relevant.
- [ ] Evaluate alternative JavaScript runtimes after Node.js support is proven.
- [ ] Evaluate TypeScript-specific compilation and type-checking workflows.
- [ ] Evaluate Java and JVM ecosystems.
- [ ] Evaluate .NET and C# projects.
- [ ] Evaluate Go modules and toolchains.
- [ ] Evaluate C and C++ compiler/toolchain compatibility.
- [ ] Evaluate PHP, Ruby, Swift, Kotlin, Lua, and other ecosystems based on actual demand.
- [ ] Evaluate additional game engines and scripting environments.
- [ ] Evaluate shader languages and other specialized toolchains only if a clear testing model exists.
- [ ] Define a consistent process for accepting, maintaining, and deprecating adapters.
- [ ] Avoid promising support for a language before its toolchain and test semantics are defined.

### 28.2. Remote and distributed execution

- [ ] Identify use cases that cannot be served by local execution.
- [ ] Define worker identity and capability reporting.
- [ ] Define authenticated job submission and result retrieval.
- [ ] Define artifact transfer and integrity checks.
- [ ] Define cancellation and timeout behavior across network boundaries.
- [ ] Define worker trust and isolation guarantees.
- [ ] Define scheduling across OS and architecture pools.
- [ ] Define offline and unavailable-worker behavior.
- [ ] Avoid distributed execution until local plans and reports are stable.

### 28.3. Containers and disposable environments

- [ ] Compare containers, virtual machines, and native process isolation.
- [ ] Define the threat model for each isolation mode.
- [ ] Define image or environment provenance.
- [ ] Define caching without compromising isolation.
- [ ] Define resource and network restrictions.
- [ ] Test cleanup after worker crashes.
- [ ] Document platform-specific differences.
- [ ] Never treat a container as a complete security boundary without qualification.

### 28.4. Long-term report services and user interfaces

- [ ] Evaluate a local report viewer.
- [ ] Evaluate historical result storage.
- [ ] Evaluate dashboards for matrix trends.
- [ ] Evaluate authenticated multi-user services only if collaboration requires them.
- [ ] Define retention, deletion, access control, and privacy requirements before storing reports remotely.
- [ ] Keep the core CLI and report format usable without a hosted service.
- [ ] Avoid adding a server/database dependency to users who only need local tests.

### 28.5. Intelligence and automated recommendations

- [ ] Evaluate suggesting runtime versions based on project metadata.
- [ ] Evaluate identifying likely compatibility boundaries from observed results.
- [ ] Evaluate prioritizing experiments based on changed dependencies.
- [ ] Explain the evidence behind every recommendation.
- [ ] Distinguish suggestions from verified experiment results.
- [ ] Never invent passing versions for untested combinations.
- [ ] Keep heuristic recommendations overridable and optional.
- [ ] Require measured benefit before adding complex automated selection.

**Completion gate:** A research candidate is promoted into committed roadmap work only after its use case, scope, risks, acceptance criteria, and maintenance cost are understood.

---

## Global definition of done

Every substantial feature should satisfy the applicable checks below before it is marked complete.

- [ ] The behavior and scope are documented.
- [ ] Inputs are validated and invalid input produces actionable errors.
- [ ] Success, failure, timeout, cancellation, and unsupported cases are defined where relevant.
- [ ] Diagnostics preserve the root cause without leaking secrets.
- [ ] Resource and cleanup behavior are tested.
- [ ] Relevant Windows, Linux, and macOS behavior is tested or the limitation is documented.
- [ ] Automated tests cover normal and important negative cases.
- [ ] Regression tests exist for confirmed bugs where feasible.
- [ ] Formatting, workspace checks, tests, and Clippy pass without warnings.
- [ ] CLI and JSON contracts are updated when externally visible behavior changes.
- [ ] Documentation examples are tested or checked against the implementation.
- [ ] The focused diff contains only the intended logical change.
- [ ] The implementation does not claim more compatibility or isolation than the evidence supports.
- [ ] The feature is usable by a consumer without relying on undocumented internal details.
- [ ] The roadmap checkbox is marked only after the acceptance criteria are verified.

## Non-goals and guardrails

These are not permanent bans. They are guardrails against building complexity without evidence.

- Do not implement every language adapter before the adapter contract has been proven.
- Do not require multilingual configuration for single-language projects.
- Do not treat file-extension detection as proof that a component is runnable.
- Do not conflate project discovery, plan generation, and execution.
- Do not conflate independent component checks with cross-language integration tests.
- Do not make dynamic plugin loading a prerequisite for the first adapter architecture.
- Do not build remote execution, a hosted dashboard, or a database before local workflows justify them.
- Do not claim that package metadata alone proves runtime compatibility.
- Do not claim that successful compilation proves integration compatibility.
- Do not silently skip unsupported checks or hide untested matrix combinations.
- Do not weaken correct tests to conceal implementation defects.
- Do not use one giant commit for unrelated architectural changes.
- Do not make external network calls during discovery unless the user explicitly requested them and the behavior is documented.
- Do not execute project code merely to discover what files exist.
- Do not silently change a user's project files during experiments.
- Do not retain sensitive logs, credentials, or artifacts without a clear policy.

## Updated execution strategy

1. **Finish Phase 1:** timeout status mapping, report lifecycle, input validation, process cleanup, and failure-path tests.
2. **Specify Phase 2:** freeze the initial language-neutral experiment and report contract before consumers depend on it.
3. **Build Phase 3:** make the CLI useful for validation, discovery, execution, and report inspection as those commands become available.
4. **Strengthen Phases 4-6:** make Python experiments correct, isolated, bounded, and continuously tested across supported platforms.
5. **Integrate CompatPy in Phase 7:** prove the consumer boundary using the documented contract.
6. **Build batch and matrix support in Phase 8:** establish reliable multi-experiment behavior before project-wide orchestration.
7. **Design Phase 9 from evidence:** extract the adapter interface while keeping the existing workspace and Python behavior working.
8. **Ship the initial documented CLI and stable contract in Phase 10:** release only after the promised behavior and support matrix are tested.
9. **Build the project model and discovery in Phases 16-17:** distinguish repositories, projects, components, and experiments.
10. **Implement multilingual orchestration in Phase 18:** coordinate independently testable components and aggregate results honestly.
11. **Add the first real integration experiment in Phase 19:** prove that component-level success and system-level success are different.
12. **Harden adapter, configuration, security, graph, and report contracts in Phases 20-25:** promote only the abstractions required by real workflows.
13. **Add language adapters incrementally in Phases 11-13:** use the same conformance suite and platform standards for each.
14. **Address performance and adoption in Phases 26-27:** use measured project needs rather than speculation.
15. **Evaluate Phase 28 candidates individually:** no future language, service, or automation is considered implemented until its own acceptance criteria pass.

The phases are a roadmap, not a requirement to finish every future feature before useful releases. Work can proceed in small increments, but no phase should be declared complete while its acceptance criteria remain unmet.

---

_This roadmap is intentionally ambitious and comprehensive. It is a planning contract for scope and quality, not a claim that all features already exist. Revisit priorities when implementation evidence or real user needs justify a change._
