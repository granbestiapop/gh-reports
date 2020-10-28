use super::auth::AuthService;
use super::milestone::MilestoneService;

pub type SharedMilestoneService = std::sync::Arc<MilestoneService>;
pub type SharedAuthService = std::sync::Arc<AuthService>;
pub type SharedContext = std::sync::Arc<Context>;

pub struct Context {
    pub milestone_service: SharedMilestoneService,
    pub auth_service: SharedAuthService,
}

pub fn create_context() -> SharedContext {
    let mservice = SharedMilestoneService::new(MilestoneService::new());
    let authservice = SharedAuthService::new(AuthService::new());
    SharedContext::new(Context {
        milestone_service: mservice,
        auth_service: authservice,
    })
}
