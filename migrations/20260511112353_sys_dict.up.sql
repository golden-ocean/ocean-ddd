-- Add up migration script here
--
-- 唯一约束 (Unique Key)：uk_表名_字段名1_字段名2。例如 uk_sys_dict_code
-- 外键约束 (Foreign Key)（如果有）：fk_表名简写_关联字段。例如 fk_dict_item_dict_id。
-- 普通索引 (Index)：idx_表名_字段名。例如 idx_sys_dict_sort。


-- 1. 创建字典聚合根表
CREATE TABLE sys_dict (
    id UUID PRIMARY KEY,
    code VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    is_builtin BOOLEAN NOT NULL DEFAULT FALSE,
    sort INT NOT NULL DEFAULT 1000,
    remark VARCHAR(255),
    status SMALLINT NOT NULL DEFAULT 1, -- 1: ACTIVE, 0: INACTIVE
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    -- 唯一
    CONSTRAINT uk_sys_dict_code UNIQUE (code),
    CONSTRAINT uk_sys_dict_name UNIQUE (name)
);
-- 索引
CREATE INDEX idx_sys_dict_status ON sys_dict (status);
CREATE INDEX idx_sys_dict_sort_created ON sys_dict (sort ASC, created_at DESC);

-- 2. 创建字典项子表
CREATE TABLE sys_dict_item (
    id UUID PRIMARY KEY,
    dict_id UUID NOT NULL, -- 逻辑外键
    label VARCHAR(64) NOT NULL,
    value VARCHAR(64) NOT NULL,
    color VARCHAR(7), -- 存 '#RRGGBB'
    sort INT NOT NULL DEFAULT 1000,
    remark VARCHAR(255),
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    -- 联合唯一
    CONSTRAINT uk_sys_dict_item_label UNIQUE (dict_id, label),
    CONSTRAINT uk_sys_dict_item_value UNIQUE (dict_id, value)
);
-- 索引
-- 最左前缀匹配原则，复合唯一已经创建了，不需要
-- CREATE INDEX idx_sys_dict_item_dict_id ON sys_dict_item (dict_id);
CREATE INDEX idx_sys_dict_item_sort ON sys_dict_item (dict_id, sort ASC, created_at DESC);
