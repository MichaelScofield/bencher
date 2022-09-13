use std::fmt;

use bencher_rbac::Organization;
use bencher_rbac::User as RbacUser;
use oso::{Oso, PolarClass, ToPolar};

use crate::{
    model::{organization::QueryOrganization, project::QueryProject, user::auth::AuthUser},
    ApiError,
};

pub type Context = tokio::sync::Mutex<ApiContext>;

pub struct ApiContext {
    pub secret_key: String,
    pub rbac: Rbac,
    pub db_conn: diesel::SqliteConnection,
}

pub struct Rbac(pub Oso);

impl From<Oso> for Rbac {
    fn from(oso: Oso) -> Self {
        Self(oso)
    }
}

impl Rbac {
    pub fn is_allowed<Actor, Action, Resource>(
        &self,
        actor: Actor,
        action: Action,
        resource: Resource,
    ) -> Result<bool, ApiError>
    where
        Actor: ToPolar,
        Action: ToPolar,
        Resource: ToPolar,
    {
        self.0.is_allowed(actor, action, resource).map_err(|e| {
            let err = ApiError::IsAllowed(e);
            tracing::info!("{err}");
            err
        })
    }

    pub fn unwrap_is_allowed<Actor, Action, Resource>(
        &self,
        actor: Actor,
        action: Action,
        resource: Resource,
    ) -> bool
    where
        Actor: ToPolar,
        Action: ToPolar,
        Resource: ToPolar,
    {
        self.is_allowed(actor, action, resource).unwrap_or_default()
    }

    pub fn is_allowed_organization(
        &self,
        rbac_user: RbacUser,
        action: bencher_rbac::organization::Permission,
        organization: &QueryOrganization,
    ) -> bool {
        self.unwrap_is_allowed(rbac_user, action, organization)
    }

    pub fn is_allowed_project(
        &self,
        rbac_user: RbacUser,
        action: bencher_rbac::project::Permission,
        project: &QueryProject,
    ) -> bool {
        self.unwrap_is_allowed(rbac_user, action, project)
    }
}
