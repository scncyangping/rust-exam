use regex::Regex;

pub(crate) fn eq(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source == target)
}

pub(crate) fn not_eq(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source != target)
}

pub(crate) fn contains(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source.contains(target))
}

pub(crate) fn not_contains(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(!source.contains(target))
}

pub(crate) fn regex(source: &str, target: &str) -> anyhow::Result<bool> {
    let re = Regex::new(target)?;
    anyhow::Ok(re.is_match(&source))
}
