use chrono::NaiveDate;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const REQUIRED_FIELDS: &[&str] = &[
    "id",
    "title",
    "summary",
    "tags",
    "status",
    "sensitivity",
    "created",
    "updated",
    "source",
];

const STATUS_VALUES: &[&str] = &["active", "archived"];
const SENSITIVITY_VALUES: &[&str] = &["public", "personal", "restricted"];
const SOURCE_VALUES: &[&str] = &["manual", "import", "conversation", "generated"];

fn main() {
    let root = std::env::current_dir().expect("failed to read current directory");
    let mut errors = Vec::new();
    let mut ids: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for path in markdown_files(&root) {
        validate_file(&root, &path, &mut ids, &mut errors);
    }

    for (id, paths) in ids {
        if paths.len() > 1 {
            errors.push(format!("duplicate id {id}: {}", paths.join(", ")));
        }
    }

    if errors.is_empty() {
        println!("Mneme record validation passed");
    } else {
        eprintln!("Mneme record validation failed:");
        for error in errors {
            eprintln!("  - {error}");
        }
        std::process::exit(1);
    }
}

fn validate_file(
    root: &Path,
    path: &Path,
    ids: &mut BTreeMap<String, Vec<String>>,
    errors: &mut Vec<String>,
) {
    let rel = rel_path(root, path);
    let in_records = rel.starts_with("records/");
    let frontmatter = match read_frontmatter(path) {
        Ok(frontmatter) => frontmatter,
        Err(message) => {
            if in_records {
                errors.push(format!("{rel}: {message}"));
            }
            return;
        }
    };

    let has_record_id = frontmatter
        .as_ref()
        .and_then(|mapping| string_field(mapping, "id"))
        .is_some_and(|id| id.starts_with("rec_"));

    if in_records {
        let Some(mapping) = frontmatter else {
            errors.push(format!("{rel}: records must have YAML frontmatter"));
            return;
        };

        let keys = mapping
            .keys()
            .filter_map(|key| key.as_str())
            .collect::<BTreeSet<_>>();
        let missing = REQUIRED_FIELDS
            .iter()
            .copied()
            .filter(|field| !keys.contains(field))
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            errors.push(format!(
                "{rel}: missing required fields: {}",
                missing.join(", ")
            ));
        }

        match string_field(&mapping, "id") {
            Some(id) if valid_id(id) => {
                ids.entry(id.to_owned()).or_default().push(rel.clone());
            }
            Some(_) | None => errors.push(format!("{rel}: id must match rec_[a-z0-9]+")),
        }

        require_non_empty_string(&mapping, "title", &rel, errors);
        require_non_empty_string(&mapping, "summary", &rel, errors);
        validate_tags(&mapping, &rel, errors);

        let status = string_field(&mapping, "status");
        match status {
            Some(value) if STATUS_VALUES.contains(&value) => {
                let expected_prefix = format!("records/{value}/");
                if !rel.starts_with(&expected_prefix) {
                    errors.push(format!(
                        "{rel}: status {value:?} must live under {expected_prefix}"
                    ));
                }
            }
            Some(_) | None => errors.push(format!(
                "{rel}: status must be one of {}",
                STATUS_VALUES.join(", ")
            )),
        }

        match string_field(&mapping, "sensitivity") {
            Some("restricted") => {
                errors.push(format!(
                    "{rel}: sensitivity restricted must not be committed under records/"
                ));
            }
            Some(value) if SENSITIVITY_VALUES.contains(&value) => {}
            Some(_) | None => errors.push(format!(
                "{rel}: sensitivity must be one of {}",
                SENSITIVITY_VALUES.join(", ")
            )),
        }

        match string_field(&mapping, "source") {
            Some(value) if SOURCE_VALUES.contains(&value) => {}
            Some(_) | None => errors.push(format!(
                "{rel}: source must be one of {}",
                SOURCE_VALUES.join(", ")
            )),
        }

        let created = validate_date(&mapping, "created", &rel, errors);
        let updated = validate_date(&mapping, "updated", &rel, errors);
        if let (Some(created), Some(updated)) = (created, updated) {
            if updated < created {
                errors.push(format!("{rel}: updated must not be earlier than created"));
            }
        }
    } else if has_record_id {
        errors.push(format!(
            "{rel}: Durable Records must live under records/active or records/archived"
        ));
    }
}

fn markdown_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_markdown_files(root, root, &mut files);
    files
}

fn collect_markdown_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let rel = rel_path(root, &path);
            if ignored_dir(&rel) {
                continue;
            }
            collect_markdown_files(root, &path, files);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            files.push(path);
        }
    }
}

fn ignored_dir(rel: &str) -> bool {
    [".git", "dist", "inbox", "restricted", "target"]
        .iter()
        .any(|dir| rel == *dir || rel.starts_with(&format!("{dir}/")))
}

fn read_frontmatter(path: &Path) -> Result<Option<Mapping>, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("failed to read: {error}"))?;
    if !text.starts_with("---\n") {
        return Ok(None);
    }

    let Some(end) = text[4..].find("\n---\n") else {
        return Err("frontmatter starts but is not closed".to_owned());
    };
    let yaml = &text[4..end + 4];
    let value = serde_yaml::from_str::<Value>(yaml)
        .map_err(|error| format!("invalid YAML frontmatter: {error}"))?;

    match value {
        Value::Mapping(mapping) => Ok(Some(mapping)),
        Value::Null => Ok(Some(Mapping::new())),
        _ => Err("frontmatter must be a YAML mapping".to_owned()),
    }
}

fn string_field<'a>(mapping: &'a Mapping, field: &str) -> Option<&'a str> {
    mapping
        .get(&Value::String(field.to_owned()))
        .and_then(Value::as_str)
}

fn require_non_empty_string(mapping: &Mapping, field: &str, rel: &str, errors: &mut Vec<String>) {
    match string_field(mapping, field) {
        Some(value) if !value.trim().is_empty() => {}
        Some(_) | None => errors.push(format!("{rel}: {field} must be a non-empty string")),
    }
}

fn validate_tags(mapping: &Mapping, rel: &str, errors: &mut Vec<String>) {
    let Some(value) = mapping.get(&Value::String("tags".to_owned())) else {
        errors.push(format!("{rel}: tags must be a list of non-empty strings"));
        return;
    };

    let Some(tags) = value.as_sequence() else {
        errors.push(format!("{rel}: tags must be a list of non-empty strings"));
        return;
    };

    if tags
        .iter()
        .all(|tag| tag.as_str().is_some_and(|tag| !tag.trim().is_empty()))
    {
        return;
    }

    errors.push(format!("{rel}: tags must be a list of non-empty strings"));
}

fn validate_date(
    mapping: &Mapping,
    field: &str,
    rel: &str,
    errors: &mut Vec<String>,
) -> Option<NaiveDate> {
    let value = mapping.get(&Value::String(field.to_owned()));
    let date = match value {
        Some(Value::String(value)) => NaiveDate::parse_from_str(value, "%Y-%m-%d").ok(),
        _ => None,
    };

    if date.is_none() {
        errors.push(format!("{rel}: {field} must be YYYY-MM-DD"));
    }

    date
}

fn valid_id(id: &str) -> bool {
    id.strip_prefix("rec_").is_some_and(|suffix| {
        !suffix.is_empty()
            && suffix
                .chars()
                .all(|char| char.is_ascii_lowercase() || char.is_ascii_digit())
    })
}

fn rel_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
