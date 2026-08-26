pub mod cli;
pub mod entity;
pub mod harness;
pub mod llm;
pub mod operator;
pub mod perception;
pub mod trace;
pub mod world;

pub use cli::PrimordiaRepl;
pub use entity::{Entity, LifecyclePhase, Spatial};
pub use harness::{GenesisSpec, HarnessReport, HarnessStep, Scenario, SimulationHarness};
pub use llm::{
    create_llm_client_from_env, AnthropicLlmClient, GeminiLlmClient, LlmClient, LlmProtocol,
    OpenAiLlmClient,
};
pub use operator::{
    AutonomousAgencyOperator, CausalExecutor, CausalOperator, CosmicLawOperator,
    MindInhabitationContext, MindInhabitationOperator, MorphogenesisContext,
    MorphogenesisOperator, SelfEvolutionOperator,
};
pub use perception::{PerceptionEngine, SensoryHorizon};
pub use trace::{CausalSpan, CausalityTracer};
pub use world::{ChronicleEvent, PrimordiaWorld, WorldSnapshot};
