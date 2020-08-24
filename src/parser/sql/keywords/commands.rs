pub enum Command {
    Abort,
    Alter,
    Analyse,
    Begin,
    Checkpoint,
    Close,
    Cluster,
    Comment,
    Commit,
    Copy,
    Create,
    Deallocate,
    Declare,
    Delete,
    Discard,
    Do,
    Drop,
    End,
    Execute,
    Explain,
    Fetch,
    Grant,
    Insert,
    Listen,
    Load,
    Lock,
    Move,
    Notify,
    Prepare,
    Reassign,
    Reindex,
    Revoke,
    Rollback,
    Savepoint,
    Security,
    Select,
    Set,
    Show,
    Start,
    Truncate,
    Unlisten,
    Update,
    Vacuum,
    Values,
}

pub enum Abort {
    Work,
    Transaction,
}

pub enum Alter {
    Aggregate,
    Collation,
    Conversion,
    Database,
    Default,
    Domain,
    Extension,
    Foreign,
    Function,
    Group,
    Index,
    Language,
    Large,
    Operator,
}

pub enum AlterDefault {
    Privileges,
}

pub enum AlterForeign {
    Data,
    Table,
}

pub enum AlterForeignData {
    Wrapper,
}

pub enum AlterLarge {
    Object,
}
