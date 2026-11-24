#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# Dodai Tech — Slim binaries: release profile and size budget
# ──────────────────────────────────────────────────────────────────────────────
#   nu scripts/ci/size_budget.nu profile            the release profile keeps lto, codegen-units = 1, strip
#   nu scripts/ci/size_budget.nu check [<target>]   every built binary is within SIZE_BUDGET_KB
#                                                   (target/[<target>/]release/<bin>; build first)
#   nu scripts/ci/size_budget.nu build              build the host release, then check
# Not part of the quality gate or the release: the security nightly runs `profile` + `build`
# (and `just size` locally). A failure prints how to find the cause (`just bloat`). Libraries have no binaries: skipped.
# ──────────────────────────────────────────────────────────────────────────────

use ../project.nu [BINS SIZE_BUDGET_KB bins_for]

# What the release profile must set to ship slim binaries.
const PROFILE = {lto: [true "fat"], codegen-units: [1], strip: [true "symbols" "debuginfo"], opt-level: ["s" "z"]}

# Problems in a parsed Cargo.toml's `[profile.release]`: one line each, empty when fine.
export def profile_problems [manifest: record]: nothing -> list<string> {
    let rel = ($manifest | get -o profile.release | default {})
    $PROFILE | transpose key allowed | each {|r|
        let v = ($rel | get -o $r.key)
        if $v == null or $v not-in $r.allowed { $"profile.release.($r.key) must be ($r.allowed | first), is ($v | default 'unset')" }
    } | compact
}

# The budget for one binary, in KiB.
export def budget_kb [bin: string, budgets: record]: nothing -> int {
    $budgets | get -o $bin | default $budgets.default
}

# Rows { bin, kb, budget, over } for the sizes (bytes) of built binaries.
export def verdicts [sizes: list<record>, budgets: record]: nothing -> list<record> {
    $sizes | each {|s|
        let kb = ($s.bytes / 1024 | math ceil)
        let budget = (budget_kb $s.bin $budgets)
        {bin: $s.bin, kb: $kb, budget: $budget, over: ($kb > $budget)}
    }
}

def exe [bin: string, target?: string]: nothing -> string {
    if $target != null and ($target | str contains windows) { $"($bin).exe" } else { $bin }
}

# Check the built binaries for `target`; prints a table, returns true when all fit.
export def check_sizes [target?: string]: nothing -> bool {
    let bins = (bins_for $target | get -o bin | default [])
    if ($bins | is-empty) { return true }
    let dir = if $target != null { $"target/($target)/release" } else { "target/release" }
    let sizes = $bins | each {|b| {bin: $b, bytes: (ls ($dir | path join (exe $b $target)) | get size.0 | into int)} }
    let rows = (verdicts $sizes $SIZE_BUDGET_KB)
    print ($rows | table)
    let bad = ($rows | where over)
    if ($bad | is-not-empty) {
        print -e $"❌ ($bad | get bin | str join ', ') over the size budget (scripts/project.nu SIZE_BUDGET_KB). Find the cause: just bloat"
    }
    ($bad | is-empty)
}

def "main profile" [] {
    if ($BINS | is-empty) { print "library: no release binaries"; return }
    let problems = (profile_problems (open Cargo.toml))
    if ($problems | is-not-empty) { $problems | each {|p| print -e $"❌ ($p)" }; exit 1 }
    print "release profile is slim (lto, codegen-units = 1, strip)"
}

def "main check" [target?: string] {
    if not (check_sizes $target) { exit 1 }
}

def "main build" [] {
    ^nu ($env.CURRENT_FILE | path dirname | path join build_release.nu)
    if not (check_sizes) { exit 1 }
}

def main [] { print "usage: nu scripts/ci/size_budget.nu (profile | check [target] | build)" }
