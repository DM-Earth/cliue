/// Raw task structure from a response.
#[derive(serde::Deserialize)]
struct RawTask {
    biz_id: String,
    biz_tag: String,
    created_at: Option<String>,
    domain: String,
    #[serde(rename = "type")]
    enhancer_type: Enhancer,
    executor_id: u32,
    /// yyyy-mm-dd hh:mm:ss
    expired_at: Option<String>,
    id: u32,
    ignored: bool,
    is_top: bool,
    is_top_sort: u16,
    read: bool,
    reason: String,
    school_id: u32,
    status: Status,
    title: String,
}

#[derive(serde::Deserialize)]
struct RawTaskRelated {
    status: Status,
    assignment_id: u32,
    assignee_id: u32,
    team_id: Option<u32>,
    task_id: u32,
    is_excellent: bool,
    archived_at: bool,
    id: u32,
    school_id: u32,
    creator_id: u32,
    role_id: u32,
    parent_task_id: Option<u32>,
    overall_task_status: Status,
    title: String,
}

struct RawTaskLabels {}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "submitted")]
    Submitted,

    #[serde(rename = "rejected")]
    Rejected,

    #[serde(rename = "published")]
    Published,

    #[serde(rename = "some_published")]
    SomePublished,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Enhancer {
    #[serde(rename = "seiue.personal_task")]
    PersonalTask,

    #[serde(rename = "seiue.class_activity_task")]
    ClassActivityTask,

    #[serde(rename = "seiue.class_homework_task")]
    ClassHomeworkTask,

    #[serde(rename = "seiue.class_document_task")]
    ClassDocumentTask,

    #[serde(rename = "seiue.class_website_task")]
    ClassWebsiteTask,

    #[serde(rename = "seiue.class_questionnaire_task")]
    ClassQuestionnaireTask,

    #[serde(rename = "seiue.class_quiz_task")]
    ClassQuizTask,

    #[serde(rename = "seiue.task_submit_todo")]
    TaskSubmitTodo,

    #[serde(rename = "seiue.task_approve_todo")]
    TaskApproveTodo,

    #[serde(rename = "seiue.chat_task")]
    ChatTask,

    #[serde(rename = "seiue.personal_group")]
    PersonalGroup,

    #[serde(rename = "seiue.class_group")]
    ClassGroup,

    #[serde(rename = "seiue.course_group")]
    CourseGroup,

    #[serde(rename = "seiue.relation_group")]
    RelationGroup,

    #[serde(rename = "seiue.admin_class_group")]
    AdminclassGroup,

    #[serde(rename = "seiue.dorm_group")]
    DormGroup,

    #[serde(rename = "seiue.department_group")]
    DepartmentGroup,

    #[serde(rename = "seiue.visitor_role")]
    VisitorRole,

    #[serde(rename = "seiue.core_role")]
    CoreRole,

    #[serde(rename = "seiue.sz_grade_report_role")]
    SzGradeReportRole,

    #[serde(rename = "seiue.szzx_class_schedule_role")]
    SzzxClassScheduleRole,

    #[serde(rename = "seiue.cgr_exam_report_role")]
    CgrExamReportRole,

    #[serde(rename = "seiue.psychological_role")]
    PsychologicalRole,

    #[serde(rename = "seiue.credit_role")]
    CreditRole,

    #[serde(rename = "seiue.handout_role")]
    HandoutRole,

    #[serde(rename = "seiue.reclaration_role")]
    DeclarationRole,

    #[serde(rename = "seiue.role_relation")]
    RoleRelation,

    #[serde(rename = "seiue.group_member_top")]
    GroupMemberTop,

    #[serde(rename = "seiue.discussion_member_banned_to_post")]
    DiscussionMemberBannedToPost,

    #[serde(rename = "seiue.discussion_topic_collect")]
    DiscussionTopicCollect,

    #[serde(rename = "seiue.discussion_topic_top")]
    DiscussionTopicTop,

    #[serde(rename = "seiue.discussion_topic_excellent")]
    DiscussionTopicExcellent,

    #[serde(rename = "seiue.task_discussion")]
    TaskDiscussion,

    #[serde(rename = "seiue.group_discussion")]
    GroupDiscussion,

    #[serde(rename = "seiue.goal_discussion")]
    GoalDiscussion,

    #[serde(rename = "seiue.feedback_discussion")]
    FeedbackDiscussion,

    #[serde(rename = "seiue.afterthought_discussion")]
    AfterthoughtDiscussion,

    #[serde(rename = "seiue.event_discussion")]
    EventDiscussion,

    #[serde(rename = "seiue.psychological_discussion")]
    PsychologicalDiscussion,

    #[serde(rename = "seiue.notification_discussion")]
    NotificationDiscussion,

    #[serde(rename = "seiue.handout_discussion")]
    HandoutDiscussion,

    #[serde(rename = "seiue.reflection_scope")]
    ReflectionScope,

    #[serde(rename = "seiue.student_scope")]
    StudentScope,

    #[serde(rename = "seiue.teacher_scope")]
    TeacherScope,

    #[serde(rename = "seiue.guardian_scope")]
    GuardianScope,

    #[serde(rename = "seiue.role_manager_scope")]
    RoleManagerScope,

    #[serde(rename = "seiue.place_scope")]
    PlaceScope,

    #[serde(rename = "seiue.class_scope")]
    ClassScope,

    #[serde(rename = "seiue.course_scope")]
    CourseScope,

    #[serde(rename = "seiue.staff_scope")]
    StaffScope,

    #[serde(rename = "seiue.dorm_scope")]
    DormScope,

    #[serde(rename = "seiue.admin_class_scope")]
    AdminClassScope,

    #[serde(rename = "seiue.subject_group_scope")]
    SubjectGroupScope,

    #[serde(rename = "seiue.handout_scope")]
    HandoutScope,

    #[serde(rename = "seiue.reflection.teacher_activity")]
    ReflTeacherActivity,

    #[serde(rename = "seiue.reflection.student_activity")]
    ReflStudentActivity,

    #[serde(rename = "seiue.reflection.guardian_activity")]
    ReflGuardianActivity,

    #[serde(rename = "seiue.reflection.staff_activity")]
    ReflStaffActivity,

    #[serde(rename = "seiue.place_activity")]
    PlaceActivity,

    #[serde(rename = "seiue.scms.class.activity")]
    ScmsClassActivity,

    #[serde(rename = "seiue.scms.course.activity")]
    ScmsCourseActivity,

    #[serde(rename = "seiue.dorm.activity")]
    DormActivity,

    #[serde(rename = "seiue.admin_class.activity")]
    AdminClassActivity,

    #[serde(rename = "seiue.task_assignment")]
    TaskAssignment,

    #[serde(rename = "seiue.task_team_assignment")]
    TaskTeamAssignment,

    #[serde(rename = "seiue.custom_group_activity")]
    CustomGroupActivity,

    #[serde(rename = "seiue.chat_activity")]
    ChatActivity,

    #[serde(rename = "seiue.default_schedule")]
    DefaultSchedule,

    #[serde(rename = "seiue.chat_schedule")]
    ChatSchedule,

    #[serde(rename = "seiue.psychological_collaborator_group")]
    PsychologicalCollaboratorGroup,

    #[serde(rename = "seiue.reflection_profile.teacher_activity")]
    ReflProfTeacherActivity,

    #[serde(rename = "seiue.reflection_profile.student_activity")]
    ReflProfStudentActivity,

    #[serde(rename = "seiue.attendance.teacher_daily_notice")]
    AttendanceTeacherDailyNotice,

    #[serde(rename = "seiue.mentor_role")]
    MentorRole,
}
