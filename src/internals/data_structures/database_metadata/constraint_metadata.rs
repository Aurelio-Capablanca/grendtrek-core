#[derive(Debug)]
pub enum SQLConstraints {
    PRIMARYKEY(IdentitySpecification),
    FOREIGNKEY(ForeignKeys),
    //UNIQUE,
    CHECK(ComputedSpecification),
    DEFAULT(ComputedSpecification),
    COMPUTED(ComputedSpecification),
}

impl SQLConstraints {
    pub fn get_pk_ref_opt(&self) -> Option<&IdentitySpecification> {
        match self {
            SQLConstraints::PRIMARYKEY(pk) => Some(pk),
            _ => { None }
        }
    }
}

#[derive(Debug)]
pub struct IdentitySpecification {
    pk_name: String,
    column_name: String,
    table_name: String,
    data_type: String,
    key_ordinal: i32,
    last_value: Option<i64>,
    increment_by: Option<i32>,
}

impl IdentitySpecification {
    pub fn new(
        pk_name: String,
        column_name: String,
        table_name: String,
        data_type: String,
        key_ordinal: i32,
        last_value: Option<i64>,
        increment_by: Option<i32>,
    ) -> Self {
        Self {
            pk_name: pk_name,
            column_name: column_name,
            table_name: table_name,
            data_type: data_type,
            key_ordinal: key_ordinal,
            last_value: last_value,
            increment_by: increment_by,
        }
    }

    pub fn empty_struct() -> Self {
        Self {
            pk_name: "n".to_string(),
            column_name: "c".to_string(),
            table_name: "t".to_string(),
            data_type: "none".to_string(),
            key_ordinal: 0,
            last_value: Some(0),
            increment_by: Some(0),
        }
    }

    pub fn get_pk_name_as_ref(&self) -> &str {
        &self.pk_name
    }
    
    pub fn get_type_pk_as_ref(&self) -> &str {
        &self.data_type
    }
    
    pub fn get_increment_by_as_ref(&self) -> &i32 {
        &self.increment_by.as_ref().unwrap_or_else(|| &0)
    }
    
    pub fn get_last_value_as_ref(&self) -> &i64 {
        &self.last_value.as_ref().unwrap_or_else(|| &0)
    }
    
    pub fn get_datatype_as_ref(&self) -> &str {
        &self.data_type
    }
     
    pub fn get_table_name_as_ref(&self) -> &str {
        &self.table_name
    }

    pub fn get_col_name_as_ref(&self) -> &str {
        &self.column_name
    }
}

#[derive(Debug)]
pub struct ComputedSpecification {
    name_spec: Option<String>,
    expression_spec: String,
    is_disabled: Option<bool>,
    isnt_trusted: Option<bool>,
    is_table_scop: Option<bool>,
    column_name: String,
}

impl ComputedSpecification {
    pub fn new(
        name_spec: Option<String>,
        expression_spec: String,
        is_disabled: Option<bool>,
        isnt_trusted: Option<bool>,
        is_table_scop: Option<bool>,
        column_name: String,
    ) -> Self {
        Self {
            name_spec: name_spec,
            expression_spec: expression_spec,
            is_disabled: is_disabled,
            isnt_trusted: isnt_trusted,
            is_table_scop: is_table_scop,
            column_name: column_name,
        }
    }
}

#[derive(Debug)]
pub struct ForeignKeys {
    table_name: String,
    column_name: String,
    reference_table: String,
    reference_column: String,
    fk_name: String,
    is_disabled: bool,
    del_ref_action: bool,
    upd_ref_action: bool,
}

impl ForeignKeys {
    pub fn new(
        table_name: String,
        column_name: String,
        reference_table: String,
        reference_column: String,
        fk_name: String,
        is_disabled: bool,
        del_ref_action: bool,
        upd_ref_action: bool,
    ) -> Self {
        Self {
            table_name: table_name,
            column_name: column_name,
            reference_table: reference_table,
            reference_column: reference_column,
            fk_name: fk_name,
            is_disabled: is_disabled,
            del_ref_action: del_ref_action,
            upd_ref_action: upd_ref_action,
        }
    }
}

#[derive(Debug)]
pub struct TableIndex {
    index_name: String,
    type_desc: String,
    is_unique: bool,
    is_unique_cons: bool,
    filter_def: Option<String>,
    is_disabled: bool,
    key_ordinal: i32,
    is_included_col: bool,
    column_name: String,
    is_desc_key: bool,
}

impl TableIndex {
    pub fn new(
        index_name: String,
        type_desc: String,
        is_unique: bool,
        is_unique_cons: bool,
        filter_def: Option<String>,
        is_disabled: bool,
        key_ordinal: i32,
        is_included_col: bool,
        column_name: String,
        is_desc_key: bool,
    ) -> Self {
        Self {
            index_name: index_name,
            type_desc: type_desc,
            is_unique: is_unique,
            is_unique_cons: is_unique_cons,
            filter_def: filter_def,
            is_disabled: is_disabled,
            key_ordinal: key_ordinal,
            is_included_col: is_included_col,
            column_name: column_name,
            is_desc_key: is_desc_key,
        }
    }
}
