// @generated automatically by Diesel CLI.

diesel::table! {
    i18n_languages (code) {
        code -> Varchar,
        name -> Varchar,
        native_name -> Varchar,
        is_active -> Bool,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_modules (id) {
        id -> Varchar,
        project_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        path -> Nullable<Varchar>,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_operation_logs (id) {
        id -> Varchar,
        user_id -> Varchar,
        action -> Varchar,
        target_type -> Varchar,
        target_id -> Varchar,
        details -> Json,
        ip_address -> Varchar,
        crt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_phrase_screenshots (id) {
        id -> Varchar,
        phrase_id -> Varchar,
        image_url -> Varchar,
        description -> Nullable<Varchar>,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_phrase_types (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        icon -> Nullable<Varchar>,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_phrases (id) {
        id -> Varchar,
        project_id -> Varchar,
        module_id -> Nullable<Varchar>,
        type_id -> Varchar,
        key -> Varchar,
        base_content -> Text,
        context -> Nullable<Text>,
        variables -> Nullable<Json>,
        platforms -> Json,
        tags -> Nullable<Json>,
        max_length -> Nullable<Integer>,
        is_plural -> Bool,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_project_languages (project_id, language) {
        project_id -> Varchar,
        language -> Varchar,
        is_default -> Bool,
    }
}

diesel::table! {
    i18n_projects (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        base_language -> Varchar,
        owner_id -> Varchar,
        status -> Bool,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_terms (id) {
        id -> Varchar,
        project_id -> Varchar,
        source_term -> Varchar,
        target_term -> Varchar,
        language -> Varchar,
        description -> Nullable<Text>,
        platforms -> Json,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_translation_history (id) {
        id -> Varchar,
        translation_id -> Varchar,
        content -> Text,
        version -> Integer,
        modified_by -> Varchar,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    i18n_translations (id) {
        id -> Varchar,
        phrase_id -> Varchar,
        language -> Varchar,
        content -> Text,
        status -> Varchar,
        translated_by -> Nullable<Varchar>,
        reviewed_by -> Nullable<Varchar>,
        crt_by -> Varchar,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::table! {
    i18n_users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        realname -> Varchar,
        avatar -> Nullable<Varchar>,
        status -> Bool,
        last_login -> Nullable<Timestamp>,
        crt_by -> Nullable<Varchar>,
        crt_at -> Timestamp,
        upt_by -> Nullable<Varchar>,
        upt_at -> Timestamp,
    }
}

diesel::joinable!(i18n_modules -> i18n_projects (project_id));
diesel::joinable!(i18n_phrase_screenshots -> i18n_phrases (phrase_id));
diesel::joinable!(i18n_phrases -> i18n_modules (module_id));
diesel::joinable!(i18n_phrases -> i18n_phrase_types (type_id));
diesel::joinable!(i18n_phrases -> i18n_projects (project_id));
diesel::joinable!(i18n_project_languages -> i18n_languages (language));
diesel::joinable!(i18n_project_languages -> i18n_projects (project_id));
diesel::joinable!(i18n_terms -> i18n_projects (project_id));
diesel::joinable!(i18n_translation_history -> i18n_translations (translation_id));
diesel::joinable!(i18n_translations -> i18n_phrases (phrase_id));

diesel::allow_tables_to_appear_in_same_query!(
    i18n_languages,
    i18n_modules,
    i18n_operation_logs,
    i18n_phrase_screenshots,
    i18n_phrase_types,
    i18n_phrases,
    i18n_project_languages,
    i18n_projects,
    i18n_terms,
    i18n_translation_history,
    i18n_translations,
    i18n_users,
);
