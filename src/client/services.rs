use super::LarkClient;
use crate::service;

impl LarkClient {
    service_accessor!(admin, admin, v1, V1);
    service_accessor!(acs, acs, v1, V1);
    service_accessor!(aily, aily, v1, V1);
    service_accessor!(application, application, v1, V1);
    service_accessor!(application_v5, application, v5, V5);
    service_accessor!(application_v6, application, v6, V6);
    service_accessor!(application_v7, application, v7, V7);
    service_accessor!(attendance, attendance, v1, V1);
    service_accessor!(approval, approval, v4, V4);
    service_accessor!(auth, auth, v3, V3);
    service_accessor!(authen, authen, v1, V1);
    service_accessor!(bitable, bitable, v1, V1);
    service_accessor!(calendar, calendar, v4, V4);
    service_accessor!(contact, contact, v3, V3);
    service_accessor!(corehr, corehr, v1, V1);
    service_accessor!(corehr_v2, corehr, v2, V2);
    service_accessor!(docx, docx, v1, V1);
    service_accessor!(drive, drive, v1, V1);
    service_accessor!(drive_v2, drive, v2, V2);
    service_accessor!(helpdesk, helpdesk, v1, V1);
    service_accessor!(hire, hire, v1, V1);
    service_accessor!(hire_v2, hire, v2, V2);
    service_accessor!(im, im, v1, V1);
    service_accessor!(sheets, sheets, v3, V3);
    service_accessor!(spark, spark, v1, V1);
    service_accessor!(task, task, v1, V1);
    service_accessor!(im_v2, im, v2, V2);
    service_accessor!(task_v2, task, v2, V2);
    service_accessor!(tenant, tenant, v2, V2);
    service_accessor!(trust_party, trust_party, v1, V1);
    service_accessor!(baike, baike, v1, V1);
    service_accessor!(lingo, lingo, v1, V1);
    service_accessor!(mail, mail, v1, V1);
    service_accessor!(minutes, minutes, v1, V1);
    service_accessor!(okr, okr, v1, V1);
    service_accessor!(translation, translation, v1, V1);
    service_accessor!(search, search, v2, V2);
    service_accessor!(vc, vc, v1, V1);
    service_accessor!(wiki_v2, wiki, v2, V2);
    service_accessor!(wiki_v1, wiki, v1, V1);
    service_accessor!(passport, passport, v1, V1);
    service_accessor!(report, report, v1, V1);
    service_accessor!(workplace, workplace, v1, V1);
    service_accessor!(face_detection, face_detection, v1, V1);
    service_accessor!(human_authentication, human_authentication, v1, V1);
    service_accessor!(optical_char_recognition, optical_char_recognition, v1, V1);
    service_accessor!(speech_to_text, speech_to_text, v1, V1);
    service_accessor!(verification, verification, v1, V1);
    service_accessor!(document_ai, document_ai, v1, V1);
    service_accessor!(mdm, mdm, v1, V1);
    service_accessor!(mdm_v3, mdm, v3, V3);
    service_accessor!(personal_settings, personal_settings, v1, V1);
    service_accessor!(security_and_compliance, security_and_compliance, v1, V1);
    service_accessor!(security_and_compliance_v2, security_and_compliance, v2, V2);
    service_accessor!(moments, moments, v1, V1);
    service_accessor!(meeting_room, meeting_room, v1, V1);
    service_accessor!(ehr, ehr, v1, V1);
    service_accessor!(compensation, compensation, v1, V1);
    service_accessor!(payroll, payroll, v1, V1);
    service_accessor!(performance, performance, v1, V1);
    service_accessor!(performance_v2, performance, v2, V2);
    service_accessor!(directory, directory, v1, V1);
    service_accessor!(docs, docs, v1, V1);
    service_accessor!(apaas, apaas, v1, V1);
    service_accessor!(base_v2, base, v2, V2);
    service_accessor!(block, block, v1, V1);
    service_accessor!(block_v2, block, v2, V2);
    service_accessor!(board, board, v1, V1);
    service_accessor!(cardkit, cardkit, v1, V1);
    service_accessor!(event, event, v1, V1);

    pub fn ext(&self) -> service::ext::ExtService<'_> {
        service::ext::ExtService::new(&self.config)
    }

    pub fn go_v397(&self) -> service::go_v397::GoV397<'_> {
        service::go_v397::GoV397::new(&self.config)
    }

    /// Create a [`WsClient`](crate::ws::WsClient) that shares this client's token cache.
    #[cfg(feature = "ws")]
    pub fn ws_client(&self, dispatcher: crate::event::EventDispatcher) -> crate::ws::WsClient {
        crate::ws::WsClient::new(self.config.clone(), dispatcher)
    }
}
