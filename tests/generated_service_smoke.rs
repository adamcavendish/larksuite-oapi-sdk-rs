mod common;
#[path = "generated_service_smoke/harness.rs"]
mod harness;
#[path = "generated_service_smoke/prelude.rs"]
mod prelude;

#[path = "generated_service_smoke/acs/access_record.rs"]
mod acs_access_record;
#[path = "generated_service_smoke/acs/device.rs"]
mod acs_device;
#[path = "generated_service_smoke/acs/face_visitor.rs"]
mod acs_face_visitor;
#[path = "generated_service_smoke/acs/rule.rs"]
mod acs_rule;
#[path = "generated_service_smoke/acs/user.rs"]
mod acs_user;
#[path = "generated_service_smoke/admin_v1/audit.rs"]
mod admin_v1_audit;
#[path = "generated_service_smoke/admin_v1/badge.rs"]
mod admin_v1_badge;
#[path = "generated_service_smoke/admin_v1/badge_grant.rs"]
mod admin_v1_badge_grant;
#[path = "generated_service_smoke/admin_v1/stat.rs"]
mod admin_v1_stat;
#[path = "generated_service_smoke/admin_v1/write.rs"]
mod admin_v1_write;
#[path = "generated_service_smoke/application_v1.rs"]
mod application_v1;
#[path = "generated_service_smoke/application_v5_v7.rs"]
mod application_v5_v7;
#[path = "generated_service_smoke/application_v6/app_version.rs"]
mod application_v6_app_version;
#[path = "generated_service_smoke/application_v6/application.rs"]
mod application_v6_application;
#[path = "generated_service_smoke/application_v6/collaboration.rs"]
mod application_v6_collaboration;
#[path = "generated_service_smoke/application_v6/feedback.rs"]
mod application_v6_feedback;
#[path = "generated_service_smoke/application_v6/management.rs"]
mod application_v6_management;
#[path = "generated_service_smoke/application_v6/recommend_rule.rs"]
mod application_v6_recommend_rule;
#[path = "generated_service_smoke/application_v6/utility.rs"]
mod application_v6_utility;
#[path = "generated_service_smoke/application_v6/visibility.rs"]
mod application_v6_visibility;
#[path = "generated_service_smoke/approval/comment.rs"]
mod approval_comment;
#[path = "generated_service_smoke/approval/definition.rs"]
mod approval_definition;
#[path = "generated_service_smoke/approval/external.rs"]
mod approval_external;
#[path = "generated_service_smoke/approval/instance.rs"]
mod approval_instance;
#[path = "generated_service_smoke/approval/task.rs"]
mod approval_task;
#[path = "generated_service_smoke/attendance/approval.rs"]
mod attendance_approval;
#[path = "generated_service_smoke/attendance/archive_rule.rs"]
mod attendance_archive_rule;
#[path = "generated_service_smoke/attendance/file.rs"]
mod attendance_file;
#[path = "generated_service_smoke/attendance/group.rs"]
mod attendance_group;
#[path = "generated_service_smoke/attendance/leave.rs"]
mod attendance_leave;
#[path = "generated_service_smoke/attendance/record.rs"]
mod attendance_record;
#[path = "generated_service_smoke/attendance/shift.rs"]
mod attendance_shift;
#[path = "generated_service_smoke/attendance/user_setting.rs"]
mod attendance_user_setting;
#[path = "generated_service_smoke/auth.rs"]
mod auth;
#[path = "generated_service_smoke/baike/classification.rs"]
mod baike_classification;
#[path = "generated_service_smoke/baike/entity.rs"]
mod baike_entity;
#[path = "generated_service_smoke/baike/file.rs"]
mod baike_file;
#[path = "generated_service_smoke/base.rs"]
mod base;
#[path = "generated_service_smoke/bitable/dashboard.rs"]
mod bitable_dashboard;
#[path = "generated_service_smoke/bitable/field.rs"]
mod bitable_field;
#[path = "generated_service_smoke/bitable/form.rs"]
mod bitable_form;
#[path = "generated_service_smoke/bitable/record.rs"]
mod bitable_record;
#[path = "generated_service_smoke/bitable/role.rs"]
mod bitable_role;
#[path = "generated_service_smoke/bitable/table.rs"]
mod bitable_table;
#[path = "generated_service_smoke/bitable/view.rs"]
mod bitable_view;
#[path = "generated_service_smoke/bitable/workflow.rs"]
mod bitable_workflow;
#[path = "generated_service_smoke/block.rs"]
mod block;
#[path = "generated_service_smoke/board.rs"]
mod board;
#[path = "generated_service_smoke/calendar/acl.rs"]
mod calendar_acl;
#[path = "generated_service_smoke/calendar/attendee.rs"]
mod calendar_attendee;
#[path = "generated_service_smoke/calendar/calendar.rs"]
mod calendar_calendar;
#[path = "generated_service_smoke/calendar/chat_member.rs"]
mod calendar_chat_member;
#[path = "generated_service_smoke/calendar/event.rs"]
mod calendar_event;
#[path = "generated_service_smoke/calendar/freebusy.rs"]
mod calendar_freebusy;
#[path = "generated_service_smoke/calendar/timezone.rs"]
mod calendar_timezone;
#[path = "generated_service_smoke/cardkit.rs"]
mod cardkit;
#[path = "generated_service_smoke/contact_basic.rs"]
mod contact_basic;
#[path = "generated_service_smoke/contact/department.rs"]
mod contact_department;
#[path = "generated_service_smoke/contact/employee_type.rs"]
mod contact_employee_type;
#[path = "generated_service_smoke/contact/functional_role_member.rs"]
mod contact_functional_role_member;
#[path = "generated_service_smoke/contact/group.rs"]
mod contact_group;
#[path = "generated_service_smoke/contact/job.rs"]
mod contact_job;
#[path = "generated_service_smoke/contact/scope.rs"]
mod contact_scope;
#[path = "generated_service_smoke/contact/unit.rs"]
mod contact_unit;
#[path = "generated_service_smoke/contact/user.rs"]
mod contact_user;
#[path = "generated_service_smoke/contact/workplace.rs"]
mod contact_workplace;
#[path = "generated_service_smoke/corehr/job.rs"]
mod corehr_job;
#[path = "generated_service_smoke/corehr/offboarding.rs"]
mod corehr_offboarding;
#[path = "generated_service_smoke/corehr/organization.rs"]
mod corehr_organization;
#[path = "generated_service_smoke/corehr/reference.rs"]
mod corehr_reference;
#[path = "generated_service_smoke/corehr/v2_job.rs"]
mod corehr_v2_job;
#[path = "generated_service_smoke/corehr/v2_signature.rs"]
mod corehr_v2_signature;
#[path = "generated_service_smoke/corehr/v2_workflow.rs"]
mod corehr_v2_workflow;
#[path = "generated_service_smoke/directory/collaboration_rule.rs"]
mod directory_collaboration_rule;
#[path = "generated_service_smoke/directory/collaboration_tenant.rs"]
mod directory_collaboration_tenant;
#[path = "generated_service_smoke/directory/department.rs"]
mod directory_department;
#[path = "generated_service_smoke/directory/employee.rs"]
mod directory_employee;
#[path = "generated_service_smoke/directory/share_entity.rs"]
mod directory_share_entity;
#[path = "generated_service_smoke/directory/user.rs"]
mod directory_user;
#[path = "generated_service_smoke/docs.rs"]
mod docs;
#[path = "generated_service_smoke/docx/announcement.rs"]
mod docx_announcement;
#[path = "generated_service_smoke/docx/block.rs"]
mod docx_block;
#[path = "generated_service_smoke/docx/building_block.rs"]
mod docx_building_block;
#[path = "generated_service_smoke/docx/document.rs"]
mod docx_document;
#[path = "generated_service_smoke/drive/comment.rs"]
mod drive_comment;
#[path = "generated_service_smoke/drive/event.rs"]
mod drive_event;
#[path = "generated_service_smoke/drive/export.rs"]
mod drive_export;
#[path = "generated_service_smoke/drive/file.rs"]
mod drive_file;
#[path = "generated_service_smoke/drive/like.rs"]
mod drive_like;
#[path = "generated_service_smoke/drive/media.rs"]
mod drive_media;
#[path = "generated_service_smoke/drive/permission.rs"]
mod drive_permission;
#[path = "generated_service_smoke/ehr.rs"]
mod ehr;
#[path = "generated_service_smoke/helpdesk/agent.rs"]
mod helpdesk_agent;
#[path = "generated_service_smoke/helpdesk/category.rs"]
mod helpdesk_category;
#[path = "generated_service_smoke/helpdesk/download.rs"]
mod helpdesk_download;
#[path = "generated_service_smoke/helpdesk/faq.rs"]
mod helpdesk_faq;
#[path = "generated_service_smoke/helpdesk/ticket.rs"]
mod helpdesk_ticket;
#[path = "generated_service_smoke/hire/agency.rs"]
mod hire_agency;
#[path = "generated_service_smoke/hire/application.rs"]
mod hire_application;
#[path = "generated_service_smoke/hire/auxiliary_read.rs"]
mod hire_auxiliary_read;
#[path = "generated_service_smoke/hire/catalog.rs"]
mod hire_catalog;
#[path = "generated_service_smoke/hire/external_read.rs"]
mod hire_external_read;
#[path = "generated_service_smoke/hire/interview.rs"]
mod hire_interview;
#[path = "generated_service_smoke/hire/job.rs"]
mod hire_job;
#[path = "generated_service_smoke/hire/job_requirement.rs"]
mod hire_job_requirement;
#[path = "generated_service_smoke/hire/job_utility_read.rs"]
mod hire_job_utility_read;
#[path = "generated_service_smoke/hire/offer.rs"]
mod hire_offer;
#[path = "generated_service_smoke/hire/read_models.rs"]
mod hire_read_models;
#[path = "generated_service_smoke/hire/reference.rs"]
mod hire_reference;
#[path = "generated_service_smoke/hire/referral_talent.rs"]
mod hire_referral_talent;
#[path = "generated_service_smoke/hire/talent.rs"]
mod hire_talent;
#[path = "generated_service_smoke/hire/task.rs"]
mod hire_task;
#[path = "generated_service_smoke/hire/utility_test.rs"]
mod hire_utility_test;
#[path = "generated_service_smoke/hire/v2.rs"]
mod hire_v2;
#[path = "generated_service_smoke/hire/website.rs"]
mod hire_website;
#[path = "generated_service_smoke/human_authentication.rs"]
mod human_authentication;
#[path = "generated_service_smoke/im/announcement.rs"]
mod im_announcement;
#[path = "generated_service_smoke/im/chat.rs"]
mod im_chat;
#[path = "generated_service_smoke/im/download.rs"]
mod im_download;
#[path = "generated_service_smoke/im/message.rs"]
mod im_message;
#[path = "generated_service_smoke/im/moderation.rs"]
mod im_moderation;
#[path = "generated_service_smoke/im/pin.rs"]
mod im_pin;
#[path = "generated_service_smoke/im/reaction.rs"]
mod im_reaction;
#[path = "generated_service_smoke/im_v2.rs"]
mod im_v2;
#[path = "generated_service_smoke/lingo/classification.rs"]
mod lingo_classification;
#[path = "generated_service_smoke/lingo/download.rs"]
mod lingo_download;
#[path = "generated_service_smoke/lingo/draft_file.rs"]
mod lingo_draft_file;
#[path = "generated_service_smoke/lingo/entity.rs"]
mod lingo_entity;
#[path = "generated_service_smoke/mail/mailgroup.rs"]
mod mail_mailgroup;
#[path = "generated_service_smoke/mail/mailgroup_member.rs"]
mod mail_mailgroup_member;
#[path = "generated_service_smoke/mail/public_mailbox.rs"]
mod mail_public_mailbox;
#[path = "generated_service_smoke/mail/public_mailbox_member.rs"]
mod mail_public_mailbox_member;
#[path = "generated_service_smoke/mdm.rs"]
mod mdm;
#[path = "generated_service_smoke/minutes.rs"]
mod minutes;
#[path = "generated_service_smoke/okr.rs"]
mod okr;
#[path = "generated_service_smoke/passport.rs"]
mod passport;
#[path = "generated_service_smoke/payroll.rs"]
mod payroll;
#[path = "generated_service_smoke/performance/activity.rs"]
mod performance_activity;
#[path = "generated_service_smoke/performance/additional_information.rs"]
mod performance_additional_information;
#[path = "generated_service_smoke/performance/metric.rs"]
mod performance_metric;
#[path = "generated_service_smoke/performance/metric_tag.rs"]
mod performance_metric_tag;
#[path = "generated_service_smoke/performance/review_user.rs"]
mod performance_review_user;
#[path = "generated_service_smoke/personal_settings.rs"]
mod personal_settings;
#[path = "generated_service_smoke/recognition_and_speech.rs"]
mod recognition_and_speech;
#[path = "generated_service_smoke/report.rs"]
mod report;
#[path = "generated_service_smoke/search/data_record.rs"]
mod search_data_record;
#[path = "generated_service_smoke/search/data_source.rs"]
mod search_data_source;
#[path = "generated_service_smoke/search/message.rs"]
mod search_message;
#[path = "generated_service_smoke/search/schema.rs"]
mod search_schema;
#[path = "generated_service_smoke/security_and_compliance.rs"]
mod security_and_compliance;
#[path = "generated_service_smoke/sheets.rs"]
mod sheets;
#[path = "generated_service_smoke/spark/app.rs"]
mod spark_app;
#[path = "generated_service_smoke/spark/metadata.rs"]
mod spark_metadata;
#[path = "generated_service_smoke/spark/record.rs"]
mod spark_record;
#[path = "generated_service_smoke/spark/storage.rs"]
mod spark_storage;
#[path = "generated_service_smoke/spark/view.rs"]
mod spark_view;
#[path = "generated_service_smoke/task/comment.rs"]
mod task_comment;
#[path = "generated_service_smoke/task/core.rs"]
mod task_core;
#[path = "generated_service_smoke/task/member.rs"]
mod task_member;
#[path = "generated_service_smoke/task/reminder.rs"]
mod task_reminder;
#[path = "generated_service_smoke/task/task.rs"]
mod task_task;
#[path = "generated_service_smoke/task_v2/attachment.rs"]
mod task_v2_attachment;
#[path = "generated_service_smoke/task_v2/comment.rs"]
mod task_v2_comment;
#[path = "generated_service_smoke/task_v2/custom_field.rs"]
mod task_v2_custom_field;
#[path = "generated_service_smoke/task_v2/section.rs"]
mod task_v2_section;
#[path = "generated_service_smoke/task_v2/task.rs"]
mod task_v2_task;
#[path = "generated_service_smoke/task_v2/tasklist.rs"]
mod task_v2_tasklist;
#[path = "generated_service_smoke/tenant.rs"]
mod tenant;
#[path = "generated_service_smoke/translation.rs"]
mod translation;
#[path = "generated_service_smoke/trust_party.rs"]
mod trust_party;
#[path = "generated_service_smoke/vc/meeting.rs"]
mod vc_meeting;
#[path = "generated_service_smoke/vc/reporting.rs"]
mod vc_reporting;
#[path = "generated_service_smoke/vc/reserve.rs"]
mod vc_reserve;
#[path = "generated_service_smoke/vc/room.rs"]
mod vc_room;
#[path = "generated_service_smoke/verification.rs"]
mod verification;
#[path = "generated_service_smoke/wiki.rs"]
mod wiki;
#[path = "generated_service_smoke/workplace.rs"]
mod workplace;
