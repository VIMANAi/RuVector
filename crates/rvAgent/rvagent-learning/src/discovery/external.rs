//! External research source integrations
//!
//! Incorporates multi-domain discovery approaches from mcp-brain-server (PR #263).
//! Fetches real-world data from open scientific APIs for the daily learning loop.

use super::{DiscoveryCategory, DiscoveryLog, QualityAssessment, ToolUsage, ToolType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Discovery domains (aligned with mcp-brain-server trainer.rs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDomain {
    /// Space science - NASA, ESA, arXiv astro-ph
    SpaceScience,
    /// Earth science - climate, geology, oceanography
    EarthScience,
    /// Academic research - arXiv, SSRN, preprints
    AcademicResearch,
    /// Economics and finance - FRED, BLS, market data
    EconomicsFinance,
    /// Medical and genomics - PubMed, NCBI
    MedicalGenomics,
    /// Materials and physics - arXiv cond-mat, materials databases
    MaterialsPhysics,
    /// Local codebase scanning
    Codebase,
}

impl std::fmt::Display for DiscoveryDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpaceScience => write!(f, "space-science"),
            Self::EarthScience => write!(f, "earth-science"),
            Self::AcademicResearch => write!(f, "academic-research"),
            Self::EconomicsFinance => write!(f, "economics-finance"),
            Self::MedicalGenomics => write!(f, "medical-genomics"),
            Self::MaterialsPhysics => write!(f, "materials-physics"),
            Self::Codebase => write!(f, "codebase"),
        }
    }
}

/// Configuration for multi-domain discovery (from PR #263 TrainerConfig)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Minimum confidence to submit a discovery
    pub min_confidence: f64,
    /// Maximum discoveries per cycle
    pub max_per_cycle: usize,
    /// Duplicate detection threshold (cosine similarity)
    pub duplicate_threshold: f64,
    /// Active discovery domains
    pub active_domains: Vec<DiscoveryDomain>,
    /// Whether to trigger SONA learning after ingestion
    pub trigger_sona: bool,
    /// API request delay (ms) for rate limiting
    pub api_delay_ms: u64,
    /// PubMed search queries
    pub pubmed_queries: Vec<String>,
    /// arXiv categories to search
    pub arxiv_categories: Vec<String>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.70,
            max_per_cycle: 50,
            duplicate_threshold: 0.95,
            active_domains: vec![
                DiscoveryDomain::MedicalGenomics,
                DiscoveryDomain::SpaceScience,
                DiscoveryDomain::AcademicResearch,
                DiscoveryDomain::Codebase,
            ],
            trigger_sona: true,
            api_delay_ms: 500,
            pubmed_queries: vec![
                "machine learning".to_string(),
                "neural network optimization".to_string(),
                "transformer architecture".to_string(),
            ],
            arxiv_categories: vec![
                "cs.AI".to_string(),
                "cs.LG".to_string(),
                "cs.CL".to_string(),
            ],
        }
    }
}

/// Errors from external source fetching
#[derive(Debug, Error)]
pub enum ExternalSourceError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parse failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("XML parse failed: {0}")]
    XmlError(String),
    #[error("Rate limited, retry after {0}ms")]
    RateLimited(u64),
    #[error("API returned error: {0}")]
    ApiError(String),
}

/// A discovery from an external source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDiscovery {
    pub id: String,
    pub domain: DiscoveryDomain,
    pub title: String,
    pub content: String,
    pub source_url: String,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
    pub confidence: f64,
    pub timestamp: DateTime<Utc>,
}

impl ExternalDiscovery {
    /// Convert to DiscoveryLog for unified processing
    pub fn to_discovery_log(&self) -> DiscoveryLog {
        let category = match self.domain {
            DiscoveryDomain::SpaceScience => DiscoveryCategory::Other,
            DiscoveryDomain::MedicalGenomics => DiscoveryCategory::Other,
            DiscoveryDomain::AcademicResearch => DiscoveryCategory::Other,
            DiscoveryDomain::EconomicsFinance => DiscoveryCategory::Other,
            DiscoveryDomain::EarthScience => DiscoveryCategory::Other,
            DiscoveryDomain::MaterialsPhysics => DiscoveryCategory::Other,
            DiscoveryDomain::Codebase => DiscoveryCategory::Architecture,
        };

        let mut log = DiscoveryLog::new(category, &self.title, &self.content)
            .with_tags(self.tags.clone())
            .with_tool(ToolUsage {
                tool_type: ToolType::Custom,
                tool_name: format!("{} API", self.domain),
                usage_description: format!("Fetched from {}", self.source_url),
                duration_ms: None,
                input_summary: None,
                output_summary: Some(format!("{} authors", self.authors.len())),
            });

        log.quality = QualityAssessment {
            novelty: self.confidence,
            usefulness: 0.7,
            clarity: 0.8,
            correctness: self.confidence,
            generalizability: 0.6,
            composite: self.confidence * 0.8,
            confidence: self.confidence,
            method: super::AssessmentMethod::Heuristic,
        };

        log
    }
}

/// Multi-domain research client
pub struct ResearchClient {
    http: reqwest::Client,
    config: DiscoveryConfig,
}

impl ResearchClient {
    pub fn new(config: DiscoveryConfig) -> Self {
        let http = reqwest::Client::builder()
            .user_agent("rvagent-learning/1.0 (https://pi.ruv.io; benevolent-discovery)")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("HTTP client");
        Self { http, config }
    }

    /// Fetch discoveries from PubMed (NCBI E-utilities)
    pub async fn fetch_pubmed(&self, query: &str, max_results: usize) -> Result<Vec<ExternalDiscovery>, ExternalSourceError> {
        // Rate limit
        tokio::time::sleep(std::time::Duration::from_millis(self.config.api_delay_ms)).await;

        // Search for PMIDs
        let search_url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term={}&retmax={}&retmode=json&sort=date",
            urlencoding::encode(query),
            max_results
        );

        let resp: serde_json::Value = self.http.get(&search_url).send().await?.json().await?;

        let pmids: Vec<String> = resp["esearchresult"]["idlist"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        if pmids.is_empty() {
            return Ok(vec![]);
        }

        // Rate limit before fetch
        tokio::time::sleep(std::time::Duration::from_millis(self.config.api_delay_ms)).await;

        // Fetch article summaries
        let fetch_url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=pubmed&id={}&retmode=json",
            pmids.join(",")
        );

        let summary: serde_json::Value = self.http.get(&fetch_url).send().await?.json().await?;

        let mut discoveries = Vec::new();
        if let Some(result) = summary.get("result") {
            for pmid in &pmids {
                if let Some(article) = result.get(pmid) {
                    let title = article["title"].as_str().unwrap_or("Untitled").to_string();
                    let source = article["source"].as_str().unwrap_or("").to_string();

                    // Get authors
                    let authors: Vec<String> = article["authors"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|a| a["name"].as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();

                    discoveries.push(ExternalDiscovery {
                        id: pmid.clone(),
                        domain: DiscoveryDomain::MedicalGenomics,
                        title: title.clone(),
                        content: format!("Published in {}. {}", source, title),
                        source_url: format!("https://pubmed.ncbi.nlm.nih.gov/{}/", pmid),
                        authors,
                        tags: vec!["pubmed".to_string(), "medical".to_string()],
                        confidence: 0.85,
                        timestamp: Utc::now(),
                    });
                }
            }
        }

        Ok(discoveries)
    }

    /// Fetch discoveries from arXiv
    pub async fn fetch_arxiv(&self, category: &str, max_results: usize) -> Result<Vec<ExternalDiscovery>, ExternalSourceError> {
        // Rate limit
        tokio::time::sleep(std::time::Duration::from_millis(self.config.api_delay_ms)).await;

        let url = format!(
            "https://export.arxiv.org/api/query?search_query=cat:{}&start=0&max_results={}&sortBy=lastUpdatedDate&sortOrder=descending",
            category,
            max_results
        );

        let resp = self.http.get(&url).send().await?.text().await?;

        // Simple XML parsing for arXiv Atom feed
        let mut discoveries = Vec::new();
        for entry in resp.split("<entry>").skip(1) {
            let title = extract_xml_tag(entry, "title").unwrap_or_default().trim().to_string();
            let summary = extract_xml_tag(entry, "summary").unwrap_or_default().trim().to_string();
            let id = extract_xml_tag(entry, "id").unwrap_or_default();

            // Extract authors
            let authors: Vec<String> = entry
                .split("<author>")
                .skip(1)
                .filter_map(|a| extract_xml_tag(a, "name"))
                .collect();

            if !title.is_empty() {
                discoveries.push(ExternalDiscovery {
                    id: id.clone(),
                    domain: DiscoveryDomain::AcademicResearch,
                    title,
                    content: summary,
                    source_url: id,
                    authors,
                    tags: vec!["arxiv".to_string(), category.to_string()],
                    confidence: 0.80,
                    timestamp: Utc::now(),
                });
            }
        }

        Ok(discoveries)
    }

    /// Fetch all discoveries from configured domains
    pub async fn fetch_all_domains(&self) -> HashMap<DiscoveryDomain, Vec<ExternalDiscovery>> {
        let mut results = HashMap::new();

        for domain in &self.config.active_domains {
            let discoveries = match domain {
                DiscoveryDomain::MedicalGenomics => {
                    let mut all = Vec::new();
                    for query in &self.config.pubmed_queries {
                        match self.fetch_pubmed(query, 10).await {
                            Ok(d) => all.extend(d),
                            Err(e) => tracing::warn!("PubMed fetch failed for '{}': {}", query, e),
                        }
                    }
                    all
                }
                DiscoveryDomain::AcademicResearch => {
                    let mut all = Vec::new();
                    for cat in &self.config.arxiv_categories {
                        match self.fetch_arxiv(cat, 10).await {
                            Ok(d) => all.extend(d),
                            Err(e) => tracing::warn!("arXiv fetch failed for '{}': {}", cat, e),
                        }
                    }
                    all
                }
                _ => Vec::new(), // Other domains not yet implemented
            };

            results.insert(domain.clone(), discoveries);
        }

        results
    }
}

/// Extract content between XML tags (simple helper)
fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}", tag);
    let end_tag = format!("</{}>", tag);

    let start = xml.find(&start_tag)?;
    let after_start = &xml[start..];
    let content_start = after_start.find('>')? + 1;
    let content = &after_start[content_start..];
    let end = content.find(&end_tag)?;

    Some(content[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_domain_display() {
        assert_eq!(DiscoveryDomain::MedicalGenomics.to_string(), "medical-genomics");
        assert_eq!(DiscoveryDomain::SpaceScience.to_string(), "space-science");
    }

    #[test]
    fn test_default_config() {
        let config = DiscoveryConfig::default();
        assert_eq!(config.min_confidence, 0.70);
        assert!(config.active_domains.contains(&DiscoveryDomain::MedicalGenomics));
    }

    #[test]
    fn test_external_to_discovery_log() {
        let ext = ExternalDiscovery {
            id: "123".to_string(),
            domain: DiscoveryDomain::MedicalGenomics,
            title: "Test Paper".to_string(),
            content: "Abstract here".to_string(),
            source_url: "https://pubmed.ncbi.nlm.nih.gov/123/".to_string(),
            authors: vec!["Smith J".to_string()],
            tags: vec!["test".to_string()],
            confidence: 0.9,
            timestamp: Utc::now(),
        };

        let log = ext.to_discovery_log();
        assert_eq!(log.title, "Test Paper");
        assert!(log.quality.composite > 0.5);
    }

    #[test]
    fn test_xml_extraction() {
        let xml = "<entry><title>Test Title</title><summary>Test Summary</summary></entry>";
        assert_eq!(extract_xml_tag(xml, "title"), Some("Test Title".to_string()));
        assert_eq!(extract_xml_tag(xml, "summary"), Some("Test Summary".to_string()));
    }
}
