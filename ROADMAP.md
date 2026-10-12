from pathlib import Path

content = r'''# CompatLab Roadmap

> **Vision:** CompatLab is a reusable compatibility experiment engine. It starts by powering CompatPy's Python compatibility checks, then grows to support additional languages and runtimes such as JavaScript/Node.js, GDScript/Godot, and Rust.
>
> **Guiding principle:** Python-first, not Python-only.

This roadmap describes the intended journey from the current prototype to a mature, cross-platform compatibility engine. Checkboxes track planned work; they should only be marked complete when the work is implemented and verified in the repository.

## Project principles

- **Python first, never Python-only.** The first useful implementation targets Python, while avoiding unnecessary assumptions that every future language behaves like Python.
- **Windows, Linux, and macOS are first-class platforms.** Cross-platform support is part of the engineering process, not final polish.
- **Preserve the current workspace.** Keep the existing `compatlab-core` and `compatlab-cli` structure unless a concrete need justifies a change.
- **Prefer small, testable steps.** Each milestone should have a clear completion gate.
- **Avoid premature abstraction.** Design language adapters after the Python lifecycle teaches us what genuinely belongs in the shared engine.
- **Keep responsibilities clear.** CompatLab executes experiments and reports results; CompatPy and future tools decide which experiments to request and how to present results.
- **Be honest about isolation.** A Python virtual environment is dependency isolation, not a security sandbox.

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
- [ ] Test a process that exits before its deadline.
- [ ] Test a process that exceeds its deadline.
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

1. **Finish Phase 1.** Complete report tests, input validation, timeout semantics, and cleanup. Existing passing tests are the starting point, not the finish line.
2. **Complete Phase 2.** Specify the JSON contract before multiple programs depend on it.
3. **Build Phase 3.** Make the CLI useful for real experiments.
4. **Complete Phases 4 and 5.** Improve Python coverage and isolation.
5. **Run Phase 6 continuously.** Add cross-platform CI early and expand it as functionality grows.
6. **Complete Phases 7 and 8.** Integrate CompatPy and add batch/matrix execution.
7. **Complete Phase 9.** Refactor when real requirements establish a useful adapter boundary.
8. **Ship Phase 10.** Release a dependable stable version.
9. **Build Phases 11, 12, and 13.** Add JavaScript, GDScript, and Rust adapters one at a time.
10. **Grow Phases 14 and 15.** Expand the ecosystem and advanced capabilities as actual needs emerge.

## Architectural rule

**Standardize the experiment lifecycle, not the behavior of every language.**

Python package installation and imports, JavaScript module loading, Godot project execution, and Rust compilation have different semantics. They can share orchestration, structured results, timeouts, diagnostics, and reporting while retaining language-specific checks.

That is how CompatLab can grow without becoming one enormous collection of language-specific conditionals.

## Current next step

Return to **Phase 1: Core reliability and correctness**. Do not add new language adapters yet. Make the current engine reliable, document its contract, and keep each change small enough to test and review.

---

_This roadmap is an evolving plan, not a promise that every future feature will be implemented. Revisit it as CompatLab and its consumers reveal real requirements._
'''

path = Path("/mnt/data/ROADMAP.md")
path.write_text(content, encoding="utf-8")
print(f"Created {path} ({len(content.splitlines())} lines, {path.stat().st_size} bytes)")
