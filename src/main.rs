use askama::Template;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub skills: SkillGroups,
    pub projects: Vec<Project>,
    pub publications: Vec<Publication>,
}

#[derive(Clone)]
pub struct Profile {
    pub name: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub summary: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub linkedin: &'static str,
    pub location: &'static str,
    pub stats: Vec<Stat>,
}

#[derive(Clone)]
pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
}

#[derive(Clone)]
pub struct Experience {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub location: &'static str,
    pub highlights: Vec<&'static str>,
    pub is_current: bool,
}

#[derive(Clone)]
pub struct SkillGroups {
    pub quant_finance: Vec<&'static str>,
    pub engineering: Vec<&'static str>,
    pub certifications: Vec<&'static str>,
    pub languages: Vec<&'static str>,
}

#[derive(Clone)]
pub struct Project {
    pub name: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub description: &'static str,
    pub tech: Vec<&'static str>,
    pub highlights: Vec<&'static str>,
}

#[derive(Clone)]
pub struct Publication {
    pub title: &'static str,
    pub description: &'static str,
}

// ============================================================================
// TEMPLATES
// ============================================================================

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate {
    pub title: String,
    pub profile: Profile,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub profile: Profile,
    pub stats: Vec<Stat>,
}

#[derive(Template)]
#[template(path = "experience.html")]
pub struct ExperienceTemplate {
    pub experiences: Vec<Experience>,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "skills.html")]
pub struct SkillsTemplate {
    pub skills: SkillGroups,
}

#[derive(Template)]
#[template(path = "research.html")]
pub struct ResearchTemplate {
    pub publications: Vec<Publication>,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub profile: Profile,
}

// Custom response wrapper for Askama templates
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", err),
            )
                .into_response(),
        }
    }
}

// ============================================================================
// HANDLERS
// ============================================================================

async fn index_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(LayoutTemplate {
        title: format!("{} | Portfolio", state.profile.name),
        profile: state.profile.clone(),
    })
}

async fn home_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {
        profile: state.profile.clone(),
        stats: state.profile.stats.clone(),
    })
}

async fn experience_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(ExperienceTemplate {
        experiences: state.experiences.clone(),
    })
}

async fn projects_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(ProjectsTemplate {
        projects: state.projects.clone(),
    })
}

async fn skills_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(SkillsTemplate {
        skills: state.skills.clone(),
    })
}

async fn research_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(ResearchTemplate {
        publications: state.publications.clone(),
    })
}

async fn contact_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(ContactTemplate {
        profile: state.profile.clone(),
    })
}

// ============================================================================
// DATA INITIALIZATION
// ============================================================================

fn init_app_state() -> AppState {
    let profile = Profile {
        name: "Lorenzo Pulcini",
        title: "MS Quantitative Finance at WashU | Ex-Accenture Sr. Engineer",
        tagline: "Bridging Financial Theory and Software",
        summary: "I thrive at the intersection of groundbreaking ideas and scalable execution. \
                  With a background in economics and software engineering, I bridge the gap between \
                  financial theory and software. Currently seeking Fall 2026 Internships.",
        email: "pulcini@wustl.edu",
        phone: "+1 (314) 565-4894",
        linkedin: "linkedin.com/in/lorenzo-pulcini",
        location: "St Louis, Missouri, United States",
        stats: vec![
            Stat { value: "4.0", label: "GPA @ WashU" },
            Stat { value: "2", label: "Startups Co-Founded" },
            Stat { value: "L1", label: "CFA Candidate" },
        ],
    };

    let experiences = vec![
        Experience {
            company: "Washington University in St. Louis",
            role: "Teaching Assistant",
            period: "Jan 2026 - Present",
            location: "St Louis, MO",
            highlights: vec![
                "Teaching Assistant for Investment Theory and Methods in Fintech",
            ],
            is_current: true,
        },
        Experience {
            company: "Washington University in St. Louis",
            role: "Data Analyst - Campus Operations",
            period: "Oct 2025 - Present",
            location: "St Louis, MO",
            highlights: vec![
                "Developing reproducible Python/SQL ETL pipelines to clean and transform complex utilization datasets",
                "Bridging raw data and strategic planning by quantifying space usage patterns for resource allocation",
            ],
            is_current: true,
        },
        Experience {
            company: "Accenture",
            role: "Senior Software Engineer",
            period: "Apr 2023 - Jul 2025",
            location: "Milan, Italy",
            highlights: vec![
                "Architected systems processing high-frequency transaction data (>1M/day) for major financial institution",
                "Led migration of core transaction proxy from Java to Go, achieving 10% throughput increase",
                "Engineered RAG system enabling natural language queries on internal documentation",
                "Designed Kafka-based monitoring pipelines handling 500K+ daily messages with statistical anomaly detection",
                "Owned performance stress testing and incident response for critical financial infrastructure",
            ],
            is_current: false,
        },
        Experience {
            company: "Raccoon Fantasy",
            role: "Co-Founder & CTO",
            period: "Jan 2020 - Jan 2024",
            location: "Milan, Italy",
            highlights: vec![
                "Designed dynamic in-game market economy with liquidity monitoring and pricing algorithms",
                "Managed engineering roadmap and sprint planning, translating economic requirements into technical specs",
                "Built scalable multiplayer infrastructure supporting 150+ concurrent players",
            ],
            is_current: false,
        },
        Experience {
            company: "Similar",
            role: "Co-Founder & COO",
            period: "Jan 2020 - Jan 2024",
            location: "Milan, Italy",
            highlights: vec![
                "Built personalized recommendation engine handling complete ML pipeline (design, benchmarking, optimization)",
                "Secured 3rd place at startup incubator competition, attracting initial investor funding",
            ],
            is_current: false,
        },
    ];

    let skills = SkillGroups {
        quant_finance: vec![
            "Derivatives Pricing",
            "Asset Pricing",
            "Stochastic Calculus",
            "Fixed Income & Derivatives",
            "Time-Series Analysis",
            "Portfolio Optimization",
            "Backtesting",
            "Risk Modeling (VaR/ES)",
        ],
        engineering: vec![
            "Python (NumPy, pandas, PyTorch)",
            "Go (Golang)",
            "Rust",
            "SQL",
            "Kafka",
            "Event-Driven Architecture",
            "Microservices",
            "Distributed Systems",
            "ETL Pipelines",
            "K8s",
            "CI/CD",
        ],
        certifications: vec![
            "CFA Level 1 Candidate",
            "Bloomberg Market Concepts",
            "Quantum Machine Learning (IBM)",
            "MITx Micromasters in Data, Economics, and Design of Policy",
        ],
        languages: vec![
            "Italian (Native)",
            "English (Fluent)",
            "Spanish (Medium)",
        ],
    };

    let projects = vec![
        Project {
            name: "Quant Sim",
            role: "Developer",
            period: "2025 - Present",
            description: "Financial models showcase and simulation",
            tech: vec!["Stochastic Processes", "Financial Modeling", "Monte Carlo Simulations","Rust"],
            highlights: vec![
                "Brownian Motion",
                "Glosten-Milgrom Model",
                "Probability Theory",
            ],
        },
        Project {
            name: "Portfolio Optimization Suite",
            role: "Developer",
            period: "2025",
            description: "Mean-variance and Black-Litterman optimization routines built from scratch",
            tech: vec!["Python", "Black–Litterman", "Optimization"],
            highlights: vec![
                "Efficient frontier visualization",
                "Scenario analysis tools",
                "Risk-adjusted return metrics",
            ],
        },
        Project {
            name: "Market Forecasting & Alpha Research",
            role: "Quantitative Researcher",
            period: "2025 - Present",
            description: "LSTM and Transformer-based models for asset price forecasting using limit order book and macro features",
            tech: vec!["PyTorch", "Time-Series", "Backtesting"],
            highlights: vec![
                "Walk-forward validation framework",
                "Drift diagnostics implementation",
                "Limit order book feature engineering",
            ],
        },
        Project {
            name: "Raccoon Fantasy",
            role: "Co-Founder & CTO",
            period: "2020 - 2024",
            description: "Blockchain-based multiplayer game with dynamic market economy and pricing algorithms",
            tech: vec!["Node.js", "Game Dev", "Market Microstructure"],
            highlights: vec![
                "150+ concurrent players supported",
                "Liquidity monitoring systems",
                "In-game economic modeling",
            ],
        },
        Project {
            name: "Similar",
            role: "Co-Founder & COO",
            period: "2021 - 2022",
            description: "Film & TV Series recommendation system",
            tech: vec!["ML Development", "ETL Pipeline", "ML Testing"],
            highlights: vec![
                "ML Model to recommend based on user past views",
                "Testing pipelines and performance metrics",
                "Incubator pitch and funding",
            ],
        },
        Project {
            name: "IBM Watson Insurance AI",
            role: "Developer",
            period: "2018",
            description: "Watson-backed insurance application using image recognition for damage detection",
            tech: vec!["IBM Watson", "AI", "Image Recognition"],
            highlights: vec![
                "70% reduction in assessment times",
                "Featured on IBM AI Developer blog",
                "End-to-end ML pipeline",
            ],
        },
    ];

    let publications = vec![
        Publication {
            title: "Impact of US and EU Sanctions on the Russian Economy",
            description: "Applied econometric techniques to analyze macroeconomic consequences of US/EU sanctions on Russia. Research paper for BSc thesis at Bocconi University.",
        },
    ];

    AppState {
        profile,
        experiences,
        skills,
        projects,
        publications,
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize app state
    let state = Arc::new(init_app_state());

    // Build router
    let app = Router::new()
        // Main page
        .route("/", get(index_handler))
        // HTMX partials
        .route("/partials/home", get(home_partial))
        .route("/partials/experience", get(experience_partial))
        .route("/partials/projects", get(projects_partial))
        .route("/partials/skills", get(skills_partial))
        .route("/partials/research", get(research_partial))
        .route("/partials/contact", get(contact_partial))
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        // State and middleware
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("🚀 Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
