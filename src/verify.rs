//! Completeness checker for per-machine results files.
//!
//! A `results/<id>/results.json` report is a full cartesian product of
//! **(algorithm, variant) × test size × thread count**. This module verifies
//! that every expected cell is present exactly once, so a partial report — a
//! crashed run, a `--filter`ed run, a stale algorithm — is caught before it
//! lands (PR validation) or after a code change (post-hoc).
//!
//! The expected `(algorithm, variant)` list comes from [`crate::registry`], so
//! verification reflects the *enabled* Cargo feature set **and** what the
//! current host's CPU exposes (HW-accelerated variants whose instructions are
//! missing are filtered out by the registry, so they aren't expected here).
//! Sizes and thread counts are read per-file from each report's own `config`,
//! so custom-size or non-default-concurrency runs are checked against what
//! they declared, not a hardcoded baseline.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The subset of `schema/results.v2.schema.json` needed for verification.
#[derive(Deserialize)]
struct ResultsFile {
    platform: Platform,
    config: Config,
    results: Vec<ResultRow>,
}

#[derive(Deserialize)]
struct Platform {
    id: String,
}

#[derive(Deserialize)]
struct Config {
    sizes_bytes: Vec<u64>,
    concurrency_levels: Vec<u32>,
}

#[derive(Deserialize)]
struct ResultRow {
    algorithm: String,
    variant: String,
    size_bytes: u64,
    threads: u32,
}

/// An `(algorithm, variant, size_bytes, threads)` coordinate.
pub type Cell = (String, String, u64, u32);

/// Verification outcome for a single results file.
pub struct FileReport {
    /// Path to the `results.json` that was checked.
    pub path: PathBuf,
    /// `platform.id` from the report.
    pub platform_id: String,
    /// Expected cells (algorithms × sizes × threads) for this file's config.
    pub expected: usize,
    /// Total result rows present in the file.
    pub found: usize,
    /// Expected cells with no matching row.
    pub missing: Vec<Cell>,
    /// Cells present more than once, with their occurrence count.
    pub duplicates: Vec<(Cell, usize)>,
    /// Algorithm names in the file that are absent from the registry.
    pub unknown_algorithms: BTreeSet<String>,
    /// Rows whose `size_bytes`/`threads` are not declared in the file's config.
    pub inconsistent: Vec<Cell>,
}

impl FileReport {
    /// Whether the file contains exactly the expected set of cells.
    pub fn is_ok(&self) -> bool {
        self.missing.is_empty()
            && self.duplicates.is_empty()
            && self.unknown_algorithms.is_empty()
            && self.inconsistent.is_empty()
    }
}

/// Core completeness check: diff `file`'s rows against the expected cartesian
/// product of `algorithms` × the file's own configured sizes and thread counts.
fn check(path: PathBuf, file: &ResultsFile, algorithms: &[(String, String)]) -> FileReport {
    let known: BTreeSet<(&str, &str)> = algorithms
        .iter()
        .map(|(n, v)| (n.as_str(), v.as_str()))
        .collect();

    // Expected cells: every (algorithm, variant) at every configured (size, threads).
    let mut expected: BTreeSet<Cell> = BTreeSet::new();
    for (name, variant) in algorithms {
        for &size in &file.config.sizes_bytes {
            for &threads in &file.config.concurrency_levels {
                expected.insert((name.clone(), variant.clone(), size, threads));
            }
        }
    }

    // Count occurrences of each cell across the file's rows.
    let mut seen: BTreeMap<Cell, usize> = BTreeMap::new();
    let mut unknown_algorithms: BTreeSet<String> = BTreeSet::new();
    let mut inconsistent: Vec<Cell> = Vec::new();
    for row in &file.results {
        let cell = (
            row.algorithm.clone(),
            row.variant.clone(),
            row.size_bytes,
            row.threads,
        );
        if !known.contains(&(row.algorithm.as_str(), row.variant.as_str())) {
            unknown_algorithms.insert(format!("{} [{}]", row.algorithm, row.variant));
        }
        if !file.config.sizes_bytes.contains(&row.size_bytes)
            || !file.config.concurrency_levels.contains(&row.threads)
        {
            inconsistent.push(cell.clone());
        }
        *seen.entry(cell).or_insert(0) += 1;
    }

    let missing: Vec<Cell> = expected
        .iter()
        .filter(|cell| !seen.contains_key(*cell))
        .cloned()
        .collect();
    let mut duplicates: Vec<(Cell, usize)> =
        seen.into_iter().filter(|(_, count)| *count > 1).collect();
    duplicates.sort();
    inconsistent.sort();

    FileReport {
        path,
        platform_id: file.platform.id.clone(),
        expected: expected.len(),
        found: file.results.len(),
        missing,
        duplicates,
        unknown_algorithms,
        inconsistent,
    }
}

/// Verify a single `results.json` against the canonical `algorithms` list.
fn verify_file(path: &Path, algorithms: &[(String, String)]) -> Result<FileReport, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let file: ResultsFile = serde_json::from_str(&text)
        .map_err(|e| format!("failed to parse {}: {e}", path.display()))?;
    Ok(check(path.to_path_buf(), &file, algorithms))
}

/// Verify every `<id>/results.json` directly under `dir`.
///
/// A missing or empty directory yields an empty `Vec` (the caller decides
/// whether that is a warning or a failure). Returns `Err` only on a read or
/// parse error of a file that does exist.
pub fn verify_dir(dir: &Path) -> Result<Vec<FileReport>, String> {
    let algorithms: Vec<(String, String)> = crate::registry()
        .into_iter()
        .map(|a| (a.name.to_string(), a.variant.to_string()))
        .collect();

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("failed to read {}: {e}", dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    entries.sort();

    let mut reports = Vec::new();
    for subdir in entries {
        let file = subdir.join("results.json");
        if file.is_file() {
            reports.push(verify_file(&file, &algorithms)?);
        }
    }
    Ok(reports)
}

/// Human-readable binary size label, e.g. `1024` → `"1 KiB"`.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if value.fract() == 0.0 {
        format!("{} {}", value as u64, UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// Render a cell as `"<algorithm> [<variant>] @ <size>, <n> thread(s)"`.
pub fn format_cell(cell: &Cell) -> String {
    let (alg, variant, size, threads) = cell;
    let plural = if *threads == 1 { "thread" } else { "threads" };
    format!(
        "{alg} [{variant}] @ {}, {threads} {plural}",
        human_size(*size)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a `ResultsFile` whose rows are the product `algorithms × sizes ×
    /// threads`, then optionally mutate `results` before checking.
    fn complete(algorithms: &[(&str, &str)], sizes: &[u64], threads: &[u32]) -> ResultsFile {
        let mut results = Vec::new();
        for (alg, variant) in algorithms {
            for &size in sizes {
                for &t in threads {
                    results.push(ResultRow {
                        algorithm: alg.to_string(),
                        variant: variant.to_string(),
                        size_bytes: size,
                        threads: t,
                    });
                }
            }
        }
        ResultsFile {
            platform: Platform {
                id: "test".to_string(),
            },
            config: Config {
                sizes_bytes: sizes.to_vec(),
                concurrency_levels: threads.to_vec(),
            },
            results,
        }
    }

    fn algs() -> Vec<(String, String)> {
        [("A", "sw"), ("B", "sw"), ("C", "sw")]
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn complete_report_passes() {
        let file = complete(
            &[("A", "sw"), ("B", "sw"), ("C", "sw")],
            &[64, 1024],
            &[1, 8],
        );
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(
            report.is_ok(),
            "expected OK, got missing={:?}",
            report.missing
        );
        assert_eq!(report.expected, 12);
        assert_eq!(report.found, 12);
    }

    #[test]
    fn missing_cell_fails() {
        let mut file = complete(
            &[("A", "sw"), ("B", "sw"), ("C", "sw")],
            &[64, 1024],
            &[1, 8],
        );
        file.results.pop();
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(!report.is_ok());
        assert_eq!(report.missing.len(), 1);
    }

    #[test]
    fn duplicate_cell_fails() {
        let mut file = complete(&[("A", "sw"), ("B", "sw"), ("C", "sw")], &[64], &[1]);
        file.results.push(ResultRow {
            algorithm: "A".to_string(),
            variant: "sw".to_string(),
            size_bytes: 64,
            threads: 1,
        });
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(!report.is_ok());
        assert_eq!(report.duplicates.len(), 1);
        assert_eq!(report.duplicates[0].1, 2);
    }

    #[test]
    fn unknown_algorithm_fails() {
        let mut file = complete(&[("A", "sw"), ("B", "sw"), ("C", "sw")], &[64], &[1]);
        file.results.push(ResultRow {
            algorithm: "Stale".to_string(),
            variant: "sw".to_string(),
            size_bytes: 64,
            threads: 1,
        });
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(!report.is_ok());
        assert!(report.unknown_algorithms.contains("Stale [sw]"));
    }

    #[test]
    fn unknown_variant_of_known_algorithm_fails() {
        let mut file = complete(&[("A", "sw"), ("B", "sw"), ("C", "sw")], &[64], &[1]);
        file.results.push(ResultRow {
            algorithm: "A".to_string(),
            variant: "sha-ext".to_string(),
            size_bytes: 64,
            threads: 1,
        });
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(!report.is_ok());
        assert!(report.unknown_algorithms.contains("A [sha-ext]"));
    }

    #[test]
    fn row_outside_config_is_inconsistent() {
        let mut file = complete(&[("A", "sw"), ("B", "sw"), ("C", "sw")], &[64], &[1]);
        file.results.push(ResultRow {
            algorithm: "A".to_string(),
            variant: "sw".to_string(),
            size_bytes: 999,
            threads: 1,
        });
        let report = check(PathBuf::from("x"), &file, &algs());
        assert!(!report.is_ok());
        assert_eq!(report.inconsistent.len(), 1);
    }
}
