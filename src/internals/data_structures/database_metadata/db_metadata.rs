pub mod cannonical_tables {

    use crate::internals::data_structures::database_metadata::constraint_metadata::{
        ForeignKeys, IdentitySpecification, SQLConstraints, TableIndex
    };
    use crate::internals::data_structures::database_metadata::db_metadata::cannonical_columns::ColumnMembers;

    #[derive(Debug)]
    pub struct TableMetadata {
        table_name: String,
        table_schema: String,
        columns: Vec<ColumnMembers>,
        constraints: Vec<SQLConstraints>,
        indexes: Vec<TableIndex>,
        total_rows: i32,
    }

    impl TableMetadata {
        pub fn new(
            table_name: String,
            table_schema: String,
            columns: Vec<ColumnMembers>,
            constraints: Vec<SQLConstraints>,
            indexes: Vec<TableIndex>,
        ) -> Self {
            Self {
                table_name: table_name,
                table_schema: table_schema,
                columns: columns,
                constraints: constraints,
                indexes: indexes,
                total_rows : 0
            }
        }

        pub fn start_struct(table_name: String, schema_name: String) -> Self {
            Self {
                table_name: table_name,
                table_schema: schema_name,
                columns: Vec::new(),
                constraints: Vec::new(),
                indexes: Vec::new(),
                total_rows : 0
            }
        }

        pub fn empty_struct() -> Self {
            Self {
                table_name: "".to_string(),
                table_schema: "".to_string(),
                columns: Vec::new(),
                constraints: Vec::new(),
                indexes: Vec::new(),
                total_rows : 0
            }
        }

        pub fn get_indexes(&self) -> &Vec<TableIndex> {
            &self.indexes
        }

        pub fn get_constrs_as_ref(&self) -> &Vec<SQLConstraints> {
            &self.constraints
        }
        
        pub fn get_constrs_as_ref_mut(&mut self) -> &mut Vec<SQLConstraints> {
            &mut self.constraints            
        }

        
        pub fn get_cols_as_ref_sort(&mut self) -> &Vec<ColumnMembers> {
            let cols = &mut self.columns;
            cols.sort_by(|a, b| a.get_ordering_ref().cmp(b.get_ordering_ref()));
            cols
        }
        
        pub fn get_cols_as_ref(&self) -> &Vec<ColumnMembers> {
            &self.columns
        }
        
        pub fn get_cols_as_owned(self) -> Vec<ColumnMembers> {
            self.columns
        }

        pub fn add_columns(&mut self, col: ColumnMembers) {
            self.columns.push(col);
        }
        
        pub fn add_fks(&mut self, fk: ForeignKeys) {
            self.constraints.push(SQLConstraints::FOREIGNKEY(fk));
        }
        
        pub fn add_computed_res(&mut self, comp: SQLConstraints){
            self.constraints.push(comp);
        }

        pub fn add_indexes (&mut self, index: TableIndex){
            self.indexes.push(index);
        }
        
        pub fn add_pk(&mut self, pk: IdentitySpecification) {            
            self.constraints.push(SQLConstraints::PRIMARYKEY(pk));
        }
        
        pub fn get_table_name(&self) -> &str {
            &self.table_name.as_ref()
        }

        pub fn get_table_schema(&self) -> &str {
            &self.table_schema.as_ref()
        }
    }
}

pub mod cannonical_columns {

    #[derive(Debug, Default, Clone)]
    pub struct ColumnMembers {
        column_name: Option<String>,
        data_type: Option<String>,
        length_field: Option<i32>,
        numeric_precision: Option<i32>,
        numeric_scale: Option<i32>,
        collation: Option<String>,
        is_nullable: Option<bool>,
        is_identity: Option<bool>,
        is_gen_always: Option<bool>,
        gen_always_text: Option<String>,
        ordering: i32,
    }

    impl ColumnMembers {
        pub fn new(
            column_name: Option<String>,
            data_type: Option<String>,
            length_field: Option<i32>,
            numeric_precision: Option<i32>,
            numeric_scale: Option<i32>,
            collation: Option<String>,
            is_nullable: Option<bool>,
            is_identity: Option<bool>,
            is_gen_always: Option<bool>,
            gen_always_text: Option<String>,
            ordering: i32,
        ) -> Self {
            Self {
                column_name: column_name,
                data_type: data_type,
                length_field: length_field,
                numeric_precision: numeric_precision,
                numeric_scale: numeric_scale,
                collation: collation,
                is_nullable: is_nullable,
                is_identity: is_identity,
                is_gen_always: is_gen_always,
                gen_always_text: gen_always_text,
                ordering: ordering
            }
        }

        pub fn empty_structure() -> Self {
            Self {
                column_name: Some("".to_string()),
                data_type: Some("".to_string()),
                length_field: Some(0),
                numeric_precision: Some(0),
                numeric_scale: Some(0),
                collation: Some("".to_string()),
                is_nullable: Some(true),
                is_identity: Some(false),
                is_gen_always: Some(false),
                gen_always_text: Some("".to_string()),
                ordering: 0,
            }
        }

        pub fn to_string_conversion(&self) -> String {
            format!(
                "ColumnMember:  {},{},{},{},{},{}.{},{},{},{},{}",
                self.get_column_name(),
                self.get_data_type(),
                self.get_lenght_field(),
                self.get_numeric_precision(),
                self.get_is_nullable(),
                self.get_numeric_scale(),
                self.get_collation(),
                self.get_is_nullable(),
                self.get_is_identity(),
                self.get_is_gen_alw(),
                self.get_gen_alw_txt()
            )
        }

        pub fn get_ordering_ref(&self) -> &i32 {
            &self.ordering
        }
        
        pub fn get_gen_alw_txt(&self) -> &str {
            self.gen_always_text.as_deref().unwrap_or("")
        }

        pub fn get_is_gen_alw(&self) -> &bool {
            self.is_gen_always.as_ref().unwrap_or(&false)
        }

        pub fn get_is_identity(&self) -> &bool {
            self.is_identity.as_ref().unwrap_or(&false)
        }

        pub fn get_is_nullable(&self) -> &bool {
            self.is_nullable.as_ref().unwrap_or(&false)
        }

        pub fn get_collation(&self) -> &str {
            self.collation.as_deref().unwrap_or("")
        }

        pub fn get_numeric_scale(&self) -> &i32 {
            self.numeric_scale.as_ref().unwrap_or(&0)
        }

        pub fn get_numeric_precision(&self) -> &i32 {
            self.numeric_precision.as_ref().unwrap_or(&0)
        }

        pub fn get_lenght_field(&self) -> &i32 {
            self.length_field.as_ref().unwrap_or(&0)
        }

        pub fn get_data_type(&self) -> &str {
            self.data_type.as_deref().unwrap_or("")
        }

        pub fn get_column_name(&self) -> &str {
            self.column_name.as_deref().unwrap_or("")
        }
    }
}
