#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# Dodai Tech — Checks outside the quality gate (nightly and PR jobs, `just` recipes)
# ──────────────────────────────────────────────────────────────────────────────
#   nu scripts/ci/extra_checks.nu features     cargo hack --each-feature (skips without [features])
#   nu scripts/ci/extra_checks.nu semver       cargo semver-checks against the published crate (library)
#   nu scripts/ci/extra_checks.nu duplicates   report crates present in several versions (never fails)
# ──────────────────────────────────────────────────────────────────────────────

# True when a parsed manifest declares features of its own.
export def has_features [manifest: record]: nothing -> bool {
    ($manifest | get -o features | default {} | columns | is-not-empty)
}

# Why `semver` is skipped, or null when it should run. `status` is the HTTP status
# of crates.io's page for the crate: a crate not yet published has no baseline.
export def semver_skip [status: int]: nothing -> any {
    if $status == 404 { "not published on crates.io yet: no baseline to compare" } else { null }
}

def manifests [] {
    let root = (open Cargo.toml)
    let members = ($root | get -o workspace.members | default [] | each {|g| glob $"($g)/Cargo.toml" } | flatten)
    [$root] | append ($members | each {|m| open $m })
}

def "main features" [] {
    if not (manifests | any {|m| has_features $m }) {
        print -e "no [features] in any manifest: nothing to check"
        return
    }
    cargo hack check --each-feature --workspace --no-dev-deps
}

def "main semver" [] {
    let name = (open Cargo.toml | get package.name)
    let status = (http get --allow-errors --full -H [User-Agent "dodaitech-website-ci"] $"https://crates.io/api/v1/crates/($name)" | get status)
    let why = (semver_skip $status)
    if $why != null { print -e $why; return }
    cargo semver-checks
}

def "main duplicates" [] {
    let out = (cargo tree --workspace --duplicates --edges normal | complete)
    if ($out.stdout | str trim | is-empty) { print "no duplicate dependency versions" } else { print $out.stdout }
}

def main [] { print "usage: nu scripts/ci/extra_checks.nu (features | semver | duplicates)" }
