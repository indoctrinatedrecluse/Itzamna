#[derive(Debug, Clone)]
pub struct DoraMetrics {
    pub deployment_frequency: f64,
    pub lead_time_hours: f64,
    pub change_failure_rate: f64,
    pub mttr_minutes: f64,
    pub tier: String,
}

#[derive(Debug, Clone)]
pub struct ChurnMetrics {
    pub lines_added: u64,
    pub lines_deleted: u64,
    pub lines_modified: u64,
    pub net_growth: i64,
}

#[derive(Debug, Clone)]
pub struct ContributorInsight {
    pub name: String,
    pub email: String,
    pub commit_count: usize,
    pub ownership_percent: f64,
}

#[derive(Debug, Clone)]
pub struct BusFactor {
    pub score: usize,
    pub risk_level: String,
}

#[derive(Debug, Clone)]
pub struct RepoHealthOverview {
    pub health_score: u32,
    pub dora: DoraMetrics,
    pub churn: ChurnMetrics,
    pub top_contributors: Vec<ContributorInsight>,
    pub bus_factor: BusFactor,
}
