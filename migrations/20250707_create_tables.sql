CREATE TABLE i18n_users (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    tenant_id INT UNSIGNED NOT NULL COMMENT '租户id',
    username VARCHAR(50) UNIQUE COMMENT '用户名',
    password VARCHAR(255) NOT NULL COMMENT '密码',
    email VARCHAR(100) UNIQUE COMMENT '邮箱',
    phone VARCHAR(20) UNIQUE COMMENT '手机号',
    realname VARCHAR(50) COMMENT '真实姓名',
    id_card VARCHAR(32) UNIQUE COMMENT '身份证号',
    nickname VARCHAR(50) COMMENT '昵称',
    avatar VARCHAR(255) DEFAULT NULL COMMENT '头像',
    gender TINYINT(1) DEFAULT NULL COMMENT '性别',
    birthday DATETIME DEFAULT NULL COMMENT '生日',
    status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '0:禁用,1:启用',
    last_login DATETIME DEFAULT NULL COMMENT '最后登录时间',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) DEFAULT NULL COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_users_username (username),
    INDEX idx_users_phone (phone),
    INDEX idx_users_email (email),
    INDEX idx_users_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 项目表
CREATE TABLE i18n_projects (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    name VARCHAR(100) NOT NULL UNIQUE COMMENT '项目名称',
    code VARCHAR(50) NOT NULL UNIQUE COMMENT '项目代码',
    description TEXT COMMENT '项目描述',
    base_language CHAR(5) NOT NULL COMMENT '基础语言代码',
    owner_id INT UNSIGNED NOT NULL COMMENT '项目负责人',
    status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '0:停用,1:启用',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_projects_name (name),
    INDEX idx_projects_owner (owner_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 语言表
CREATE TABLE i18n_languages (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    code CHAR(5) PRIMARY KEY COMMENT 'ISO 639-1代码',
    name VARCHAR(50) NOT NULL COMMENT '语言名称',
    native_name VARCHAR(50) NOT NULL COMMENT '本地语言名称',
    is_active BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否启用',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_languages_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 词条模块表
CREATE TABLE i18n_modules (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    project_id INT UNSIGNED NOT NULL COMMENT '项目id',
    name VARCHAR(100) NOT NULL COMMENT '模块名称',
    description VARCHAR(255) DEFAULT NULL COMMENT '模块描述',
    path VARCHAR(255) COMMENT '模块路径',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (project_id, name),
    INDEX idx_modules_project (project_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 词条类型表
CREATE TABLE i18n_phrase_types (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT COMMENT '主键id',
    name VARCHAR(30) NOT NULL UNIQUE COMMENT '类型名称',
    description VARCHAR(255) DEFAULT NULL COMMENT '类型描述',
    icon VARCHAR(50) DEFAULT NULL COMMENT '类型图标',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 词条表
CREATE TABLE i18n_phrases (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    project_id INT UNSIGNED NOT NULL COMMENT '项目id',
    module_id INT UNSIGNED DEFAULT NULL COMMENT '所属模块',
    type_id TINYINT UNSIGNED NOT NULL COMMENT '词条类型',
    `key` VARCHAR(255) NOT NULL COMMENT '词条标识符',
    base_content TEXT NOT NULL COMMENT '基础语言内容',
    context TEXT COMMENT '使用上下文',
    variables JSON COMMENT '插值变量列表',
    platforms JSON NOT NULL COMMENT '适用平台["web","ios","android","backend"]',
    tags JSON COMMENT '标签数组',
    max_length INT DEFAULT NULL COMMENT '最大长度限制',
    is_plural BOOLEAN DEFAULT FALSE COMMENT '是否有复数形式',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (project_id, `key`),
    INDEX idx_phrases_project (project_id),
    INDEX idx_phrases_module (module_id),
    INDEX idx_phrases_type (type_id),
    INDEX idx_phrases_key (`key`(100)),
    INDEX idx_phrases_crt_by (crt_by)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 词条截图表
CREATE TABLE i18n_phrase_screenshots (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    phrase_id INT UNSIGNED NOT NULL COMMENT '词条id',
    image_url VARCHAR(255) NOT NULL COMMENT '截图URL',
    description VARCHAR(255) DEFAULT NULL COMMENT '截图描述',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_screenshots_phrase (phrase_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 翻译表
CREATE TABLE i18n_translations (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    phrase_id INT UNSIGNED NOT NULL COMMENT '词条id',
    language CHAR(5) NOT NULL COMMENT '语言代码',
    content TEXT NOT NULL COMMENT '翻译内容',
    status ENUM('pending', 'reviewed', 'published') DEFAULT 'pending',
    translated_by INT UNSIGNED DEFAULT NULL COMMENT '翻译人',
    reviewed_by INT UNSIGNED DEFAULT NULL COMMENT '审核人',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (phrase_id, language),
    INDEX idx_translations_language (language),
    INDEX idx_translations_status (status),
    INDEX idx_translations_translated_by (translated_by),
    INDEX idx_translations_reviewed_by (reviewed_by)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 翻译历史表
CREATE TABLE i18n_translation_history (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    translation_id INT UNSIGNED NOT NULL COMMENT '翻译id',
    content TEXT NOT NULL COMMENT '翻译内容',
    version INT NOT NULL COMMENT '版本号',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '修改人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    INDEX idx_history_translation (translation_id),
    INDEX idx_history_crt_by (crt_by)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 术语表
CREATE TABLE i18n_terms (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    project_id INT UNSIGNED NOT NULL COMMENT '项目id',
    source_term VARCHAR(255) NOT NULL COMMENT '源术语',
    target_term VARCHAR(255) NOT NULL COMMENT '目标术语',
    language CHAR(5) NOT NULL COMMENT '语言代码',
    description TEXT COMMENT '术语描述',
    platforms JSON NOT NULL COMMENT '适用平台["web","ios","android","backend"]',
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_terms_project (project_id),
    INDEX idx_terms_language (language),
    INDEX idx_terms_source (source_term(100))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 操作日志表
CREATE TABLE i18n_operation_logs (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY COMMENT '主键id',
    user_id INT UNSIGNED NOT NULL COMMENT '用户id',
    action VARCHAR(50) NOT NULL COMMENT '操作类型',
    target_type VARCHAR(50) NOT NULL COMMENT '操作对象类型',
    target_id VARCHAR(100) NOT NULL COMMENT '操作对象ID',
    details JSON COMMENT '操作详情',
    ip_address VARCHAR(45) NOT NULL COMMENT 'IP地址',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    INDEX idx_logs_user (user_id),
    INDEX idx_logs_crt_at (crt_at),
    INDEX idx_logs_action (action),
    INDEX idx_logs_target (target_type, target_id(50))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 项目语言关联表 (关系表，不加审计字段)
CREATE TABLE i18n_project_languages (
    project_id INT UNSIGNED NOT NULL COMMENT '项目id',
    language CHAR(5) NOT NULL COMMENT '语言代码',
    is_default BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否默认语言',
    PRIMARY KEY (project_id, language),
    INDEX idx_project_languages_project (project_id),
    INDEX idx_project_languages_language (language)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Casbin 规则表
CREATE TABLE IF NOT EXISTS casbin_rule (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    ptype VARCHAR(12) NOT NULL,
    v0 VARCHAR(128),
    v1 VARCHAR(128),
    v2 VARCHAR(128),
    v3 VARCHAR(128),
    v4 VARCHAR(128),
    v5 VARCHAR(128),
    crt_by VARCHAR(50) NOT NULL COMMENT '创建人',
    crt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    upt_by VARCHAR(50) COMMENT '更新人',
    upt_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_casbin_rule_ptype (ptype),
    INDEX idx_casbin_rule_v0 (v0),
    INDEX idx_casbin_rule_v1 (v1),
    INDEX idx_casbin_rule_v2 (v2)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;