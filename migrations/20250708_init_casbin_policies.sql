-- 初始化 Casbin 策略

-- 超级管理员角色策略
INSERT INTO casbin_rule (ptype, v0, v1, v2, crt_by) 
VALUES ('p', 'role:admin', '/*', '*', 'system');

-- 项目管理角色策略
INSERT INTO casbin_rule (ptype, v0, v1, v2, crt_by) 
VALUES 
    ('p', 'role:project_manager', '/api/projects*', '*', 'system'),
    ('p', 'role:project_viewer', '/api/projects*', 'GET', 'system');

-- 翻译管理角色策略
INSERT INTO casbin_rule (ptype, v0, v1, v2, crt_by) 
VALUES 
    ('p', 'role:translator', '/api/translations*', '*', 'system'),
    ('p', 'role:reviewer', '/api/translations/review*', '*', 'system');

-- 术语管理角色策略
INSERT INTO casbin_rule (ptype, v0, v1, v2, crt_by) 
VALUES 
    ('p', 'role:term_manager', '/api/terms*', '*', 'system'),
    ('p', 'role:term_viewer', '/api/terms*', 'GET', 'system');

-- 词条管理角色策略
INSERT INTO casbin_rule (ptype, v0, v1, v2, crt_by) 
VALUES 
    ('p', 'role:phrase_manager', '/api/phrases*', '*', 'system'),
    ('p', 'role:phrase_viewer', '/api/phrases*', 'GET', 'system');

-- 用户角色分配示例（可选）
-- 将用户分配到角色
INSERT INTO casbin_rule (ptype, v0, v1, crt_by) 
VALUES 
    ('g', 'user:admin', 'role:admin', 'system'),