// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// Julia the Viper - Conformance Suite Runner
// Loads .jtv files from conformance/valid/ and conformance/invalid/
// and asserts that valid files parse successfully and invalid files
// produce parse errors.

use jtv_core::parse_program;
use std::path::PathBuf;

/// Find the conformance directory relative to the workspace root.
fn conformance_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/jtv-core -> workspace root
    manifest_dir.join("../../conformance")
}

/// Read all .jtv files from a directory.
fn collect_jtv_files(dir: &std::path::Path) -> Vec<PathBuf> {
    if !dir.exists() {
        return vec![];
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("Failed to read conformance directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("jtv") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    files.sort();
    files
}

#[test]
fn conformance_valid_files_parse() {
    let valid_dir = conformance_dir().join("valid");
    let files = collect_jtv_files(&valid_dir);

    assert!(
        !files.is_empty(),
        "No valid conformance files found in {:?}",
        valid_dir
    );

    let mut failures = Vec::new();

    for file in &files {
        let source = std::fs::read_to_string(file).expect("Failed to read conformance file");
        match parse_program(&source) {
            Ok(_) => {} // Expected: valid files should parse
            Err(e) => {
                failures.push(format!(
                    "  FAIL: {} -- {}",
                    file.file_name().unwrap().to_str().unwrap(),
                    e
                ));
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "Valid conformance files that failed to parse:\n{}",
            failures.join("\n")
        );
    }
}

#[test]
fn conformance_invalid_files_reject() {
    let invalid_dir = conformance_dir().join("invalid");
    let files = collect_jtv_files(&invalid_dir);

    assert!(
        !files.is_empty(),
        "No invalid conformance files found in {:?}",
        invalid_dir
    );

    let mut failures = Vec::new();

    for file in &files {
        let source = std::fs::read_to_string(file).expect("Failed to read conformance file");
        match parse_program(&source) {
            Ok(_) => {
                failures.push(format!(
                    "  FAIL: {} -- parsed successfully but should have been rejected",
                    file.file_name().unwrap().to_str().unwrap()
                ));
            }
            Err(_) => {} // Expected: invalid files should fail to parse
        }
    }

    if !failures.is_empty() {
        panic!(
            "Invalid conformance files that incorrectly parsed:\n{}",
            failures.join("\n")
        );
    }
}
