use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// generate global version sturct
static GITVERSION: Lazy<GitVersion> = Lazy::new(|| {
    serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/GITVERSION"))).unwrap()
});

/// generate global version
pub(crate) static VERSION: Lazy<String> =
    Lazy::new(|| GITVERSION.informational_version.to_string());

#[derive(Debug, Serialize, Deserialize)]
struct GitVersion {
    #[serde(rename = "Major")]
    major: i64,
    #[serde(rename = "Minor")]
    minor: i64,
    #[serde(rename = "Patch")]
    patch: i64,
    #[serde(rename = "PreReleaseTag")]
    pre_release_tag: String,
    #[serde(rename = "PreReleaseTagWithDash")]
    pre_release_tag_with_dash: String,
    #[serde(rename = "PreReleaseLabel")]
    pre_release_label: String,
    #[serde(rename = "PreReleaseLabelWithDash")]
    pre_release_label_with_dash: String,
    #[serde(rename = "PreReleaseNumber")]
    pre_release_number: Option<serde_json::Value>,
    #[serde(rename = "WeightedPreReleaseNumber")]
    weighted_pre_release_number: i64,
    #[serde(rename = "BuildMetaData")]
    build_meta_data: Option<serde_json::Value>,
    #[serde(rename = "BuildMetaDataPadded")]
    build_meta_data_padded: String,
    #[serde(rename = "FullBuildMetaData")]
    full_build_meta_data: String,
    #[serde(rename = "MajorMinorPatch")]
    major_minor_patch: String,
    #[serde(rename = "SemVer")]
    sem_ver: String,
    #[serde(rename = "LegacySemVer")]
    legacy_sem_ver: String,
    #[serde(rename = "LegacySemVerPadded")]
    legacy_sem_ver_padded: String,
    #[serde(rename = "AssemblySemVer")]
    assembly_sem_ver: String,
    #[serde(rename = "AssemblySemFileVer")]
    assembly_sem_file_ver: String,
    #[serde(rename = "FullSemVer")]
    full_sem_ver: String,
    #[serde(rename = "InformationalVersion")]
    informational_version: String,
    #[serde(rename = "BranchName")]
    branch_name: String,
    #[serde(rename = "EscapedBranchName")]
    escaped_branch_name: String,
    #[serde(rename = "Sha")]
    sha: String,
    #[serde(rename = "ShortSha")]
    short_sha: String,
    #[serde(rename = "NuGetVersionV2")]
    nu_get_version_v2: String,
    #[serde(rename = "NuGetVersion")]
    nu_get_version: String,
    #[serde(rename = "NuGetPreReleaseTagV2")]
    nu_get_pre_release_tag_v2: String,
    #[serde(rename = "NuGetPreReleaseTag")]
    nu_get_pre_release_tag: String,
    #[serde(rename = "VersionSourceSha")]
    version_source_sha: String,
    #[serde(rename = "CommitsSinceVersionSource")]
    commits_since_version_source: i64,
    #[serde(rename = "CommitsSinceVersionSourcePadded")]
    commits_since_version_source_padded: String,
    #[serde(rename = "UncommittedChanges")]
    uncommitted_changes: i64,
    #[serde(rename = "CommitDate")]
    commit_date: String,
}
