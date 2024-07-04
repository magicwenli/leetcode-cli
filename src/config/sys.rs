//! System section
//!
//! This section is a set of constants after #88

use serde::{Deserialize, Serialize};

const CATEGORIES: [&str; 4] = ["algorithms", "concurrency", "database", "shell"];

// TODO: find a better solution.
fn categories() -> Vec<String> {
    CATEGORIES.into_iter().map(|s| s.into()).collect()
}

/// Leetcode API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Urls {
    pub base: String,
    pub graphql: String,
    pub login: String,
    pub problems: String,
    pub problem: String,
    pub tag: String,
    pub test: String,
    pub session: String,
    pub submit: String,
    pub submissions: String,
    pub submission: String,
    pub verify: String,
    pub favorites: String,
    pub favorite_delete: String,
    pub solutions: String,
}

impl Default for Urls {
    fn default() -> Self {
        Self::new_with_url("leetcode.com")
    }
}

impl Urls {
    fn new_with_url(url: &str) -> Self {
        Self {
            base: format!("https://{}", url),
            graphql: format!("https://{}/graphql", url),
            login: format!("https://{}/accounts/login/", url),
            problems: format!("https://{}/api/problems/$category/", url),
            problem: format!("https://{}/problems/$slug/description/", url),
            tag: format!("https://{}/tag/$slug/", url),
            test: format!("https://{}/problems/$slug/interpret_solution/", url),
            session: format!("https://{}/session/", url),
            submit: format!("https://{}/problems/$slug/submit/", url),
            submissions: format!("https://{}/submissions/detail/$id/", url),
            submission: format!("https://{}/submissions/detail/$id/", url),
            verify: format!("https://{}/submissions/detail/$id/check/", url),
            favorites: format!("https://{}/list/api/questions", url),
            favorite_delete: format!("https://{}/list/api/questions/$hash/$id", url),
            solutions: format!("https://{}/problems/$slug/solutions/", url),
        }
    }

    pub fn new_with_leetcode_cn() -> Self {
        Self::new_with_url("leetcode.cn")
    }

    /// problem url with specific `$slug`
    pub fn problem(&self, slug: &str) -> String {
        self.problem.replace("$slug", slug)
    }

    /// problems url with specific `$category`
    pub fn problems(&self, category: &str) -> String {
        self.problems.replace("$category", category)
    }

    /// submit url with specific `$slug`
    pub fn submit(&self, slug: &str) -> String {
        self.submit.replace("$slug", slug)
    }

    /// tag url with specific `$slug`
    pub fn tag(&self, slug: &str) -> String {
        self.tag.replace("$slug", slug)
    }

    /// test url with specific `$slug`
    pub fn test(&self, slug: &str) -> String {
        self.test.replace("$slug", slug)
    }

    /// verify url with specific `$id`
    pub fn verify(&self, id: &str) -> String {
        self.verify.replace("$id", id)
    }
}

/// System settings, for leetcode api mainly
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sys {
    #[serde(default = "categories")]
    pub categories: Vec<String>,
    #[serde(default)]
    pub urls: Urls,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            categories: CATEGORIES.into_iter().map(|s| s.into()).collect(),
            urls: Default::default(),
        }
    }
}
