use std::future::Future;
use std::pin::Pin;

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

impl EventDispatcher {
    pub fn on_p2_corehr_approval_groups_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.approval_groups.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_company_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.company.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_company_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.company.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_company_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.company.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_cost_center_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.cost_center.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_cost_center_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.cost_center.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_cost_center_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.cost_center.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_custom_org_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.custom_org.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_custom_org_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.custom_org.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_custom_org_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.custom_org.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_department_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.department.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_department_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.department.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_employee_domain_event_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employee.domain_event_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_change_status_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_change.status_updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_change_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_change.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_family_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_family.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_family_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_family.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_family_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_family.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_grade_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_grade.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_grade_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_grade.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_grade_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_grade.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_level_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_level.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_level_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_level.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_level_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_level.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_location_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.location.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_location_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.location.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_location_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.location.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_offboarding_checklist_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.offboarding.checklist_updated_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_offboarding_status_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.offboarding.status_updated_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_offboarding_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.offboarding.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_pathway_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.pathway.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_pathway_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.pathway.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_pathway_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.pathway.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_position_created_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.position.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_position_deleted_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.position.deleted_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_position_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.position.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_pre_hire_onboarding_task_changed_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.pre_hire.onboarding_task_changed_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_probation_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.probation.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_approver_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.process.approver.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_cc_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.process.cc.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_node_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.process.node.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_status_update_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.process.status.update_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.process.updated_v2", wrap_handler(handler))
    }

    pub fn on_p2_corehr_process_comment_info_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.process_comment_info.updated_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_signature_file_status_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.signature_file.status_updated_v2",
            wrap_handler(handler),
        )
    }
}
