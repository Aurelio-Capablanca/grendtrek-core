enum SQLConstraints{
    PRIMARYKEY,
    FOREIGNKEY(String,String),
    UNIQUE,
    CHECK(String),
    DEFAULT(String),
    IDENTITY,
    COMPUTED(String)
}