use thiserror::Error;

#[derive(Debug, Error)]
pub enum DictDomianError {
    // --- 字典级别的错误 ---
    #[error("字典名称必须填写")]
    NameRequired,
    #[error("字典编码必须填写")]
    CodeRequired,
    #[error("字典名称长度必须在 2-64 之间")]
    NameLengthInvalid,
    #[error("字典编码长度必须在 2-64 之间")]
    CodeLengthInvalid,
    #[error("系统内置字典，禁止修改或删除")]
    BuiltInForbidden,
    #[error("无法对已删除的字典进行操作")]
    AlreadyDeleted,

    // --- 字典项级别的错误 ---
    #[error("字典选项标签必须填写")]
    ItemLabelRequired,
    #[error("字典选项值必须填写")]
    ItemValueRequired,
    #[error("字典名称长度必须在 1-64 之间")]
    ItemLabelLengthInvalid,
    #[error("字典编码长度必须在 1-64 之间")]
    ItemValueLengthInvalid,
    #[error("颜色格式错误，必须为 #RRGGBB")]
    ItemColorFormatInvalid,
}
