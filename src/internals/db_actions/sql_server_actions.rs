use std::collections::HashMap;

use crate::internals::data_structures::{
    database_connector_spec::DatabaseHandlers,
    database_metadata::{
        constraint_metadata::{ComputedSpecification, SQLConstraints},
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    },
    database_types::query::Query,
};

pub async fn build_canonnical_schema(
    connection: &mut DatabaseHandlers,
    query_list: Vec<&Query>,
) -> Result<Vec<TableMetadata>, Box<dyn std::error::Error>> {
    let mut tables: HashMap<(String, String), TableMetadata> = HashMap::new();
    match connection {
        DatabaseHandlers::SqlServer(client) => {
            let client_conn = &mut client.client;
            //Get Canonnical Columns
            let col_query = query_list.iter().find(|pred| pred.id_out().eq(&2)).unwrap();
            let cols = client_conn
                .query(col_query.query_out(), &[])
                .await
                .unwrap()
                .into_first_result()
                .await
                .unwrap();

            for col in cols {
                let table_name = col
                    .get("table_name")
                    .map(str::to_owned)
                    .unwrap_or_else(|| "no_table".to_owned());
                let schema_name = col
                    .get("schema_name")
                    .map(str::to_owned)
                    .unwrap_or_else(|| "no_schema".to_owned());
                //--
                //let ordering: Option<i32> = col.get("ordering");
                let col_name: Option<&str> = col.get("column_name");
                let data_type: Option<&str> = col.get("data_type");
                let length_field: Option<i32> = col.get("length_field");
                let numeric_precision: u8 = col.get("type_precision").unwrap_or_else(|| 0u8);
                let numeric_scale: u8 = col.get("decimal_scale").unwrap_or_else(|| 0u8);
                let collation: Option<&str> = col.get("collation_col");
                let is_nullable: bool = col.get::<bool, _>("is_nullable").unwrap_or(false);
                let is_identity: bool = col.get::<bool, _>("identity_col").unwrap_or(false);
                let is_gen_always: bool = if col.get::<u8, _>("generated_always").unwrap_or(0u8).eq(&0u8) { false } else { true };
                let text_gen_alw: Option<&str> = col.get("text_generated_always");
                let def_comp_val: Option<&str> = col.get("computed_col_value");
                let key_table = (table_name.clone(), schema_name.clone());
                let column_memb = ColumnMembers::new(
                    Some(col_name.map(str::to_owned).unwrap()),
                    Some(data_type.map(str::to_owned).unwrap()),
                    length_field,
                    Some(i32::from_be_bytes([numeric_precision; 4])),
                    Some(i32::from_be_bytes([numeric_scale; 4])),
                    Some(collation.map(str::to_owned).unwrap_or("".to_string())),
                    Some(is_nullable),
                    Some(is_identity),
                    Some(is_gen_always),
                    Some(text_gen_alw.map(str::to_owned).unwrap_or("".to_string())),
                );

                let computed_expression: Vec<SQLConstraints> = if let Some(val) = def_comp_val {
                    vec![SQLConstraints::COMPUTED(ComputedSpecification::new(
                        Some(String::new()),
                        val.to_string(),
                        Some(false),
                        Some(false),
                        Some(false),
                        col.get("column_name")
                            .map(str::to_owned)
                            .unwrap_or_else(|| "".to_string()),
                    ))]
                } else {
                    Vec::new()
                };

                tables
                    .entry(key_table)
                    .and_modify(|lambda| {
                        lambda.add_columns(column_memb.clone());
                    })
                    .or_insert_with_key(|_| {
                        TableMetadata::new(
                            table_name,
                            schema_name,
                            vec![column_memb],
                            computed_expression,
                            Vec::new(),
                        )
                    });
            }
            //Get Cannonical PK

            //Get Cannonical FK
            //Get Cannonical Indexes
            //Get Canonnical Constraints (Default/Check)
        }
        _ => {}
    }
    tables.iter().for_each(|data| println!("{:?}", data));
    Ok(Vec::new())
}
