-- 如果表存在，先删除它
DROP TABLE IF EXISTS "user";

-- 创建新用户表
CREATE TABLE "user" (
    id BIGSERIAL PRIMARY KEY,                -- 用户 ID，自增主键
    username VARCHAR(50) NOT NULL,           -- 用户名，不能为空
    password VARCHAR(255) NOT NULL,          -- 密码，不能为空，使用较长的字符以支持哈希存储
    email VARCHAR(100) UNIQUE,               -- 电子邮件，唯一索引
    real_name VARCHAR(100) DEFAULT '',       -- 真实姓名，允许为空，默认值为空字符串
    roles TEXT[] DEFAULT ARRAY[]::TEXT[],    -- 角色，存储字符串数组，默认值为空数组
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,  -- 创建时间，默认当前时间
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP   -- 更新时间，默认当前时间
);

-- 为了更好的性能，可以考虑为 `username` 添加唯一约束或索引
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_username ON "user"(username);

-- 为了确保 roles 数组字段有合适的索引，可以创建 GIN 索引
CREATE INDEX IF NOT EXISTS idx_user_roles ON "user" USING GIN (roles);
