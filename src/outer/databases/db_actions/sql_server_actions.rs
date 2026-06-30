use std::collections::HashMap;

use bb8_tiberius::ConnectionManager;

use crate::internals::data_structures::{
    database_connector_spec::DatabaseHandlers,
    database_metadata::{
        constraint_metadata::{
            ComputedSpecification, ForeignKeys, IdentitySpecification, SQLConstraints, TableIndex,
        },
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    },
    database_types::query::Query,
};

async fn count_rows_sqlserver(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    table_name: &str,
    schema_name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let count_query = format!("SELECT count(1) FROM {}.{}", schema_name, table_name);
    println!("{}", count_query);
    let mut count: i32 = 0;
    let execution = client_conn
        .query(count_query, &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for exec in execution {
        count = exec.get(0).unwrap_or(0);
    }
    Ok(count)
}

async fn build_cannonical_columns(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    cols_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    //Get Canonnical Columns
    // let client_conn = &mut client_mssql.get().await.unwrap();
    let cols = client_conn
        .query(cols_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    let mut total_rows = 0;
    for col in cols {
        let table_name: String = col
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "no_table".to_owned());
        let schema_name: String = col
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "no_schema".to_owned());
        //--        
        let key_table = (table_name.to_owned(), schema_name.to_owned());        
        if !tables.contains_key(&key_table) {            
            total_rows =
                count_rows_sqlserver(client_conn, table_name.as_str(), schema_name.as_str())
                    .await
                    .unwrap();
        }
        let col_name: Option<&str> = col.get("column_name");
        let data_type: &str = col.get("data_type").unwrap_or_else(|| "");
        let length_field: i32 = col.get("length_field").unwrap_or_else(|| 0i32);
        let numeric_precision: u8 = col.get("type_precision").unwrap_or_else(|| 0u8);
        let numeric_scale: u8 = col.get("decimal_scale").unwrap_or_else(|| 0u8);
        let collation: Option<&str> = col.get("collation_col");
        let is_nullable: bool = col.get::<bool, _>("is_nullable").unwrap_or(false);
        let is_identity: bool = col.get::<bool, _>("identity_col").unwrap_or(false);
        let is_gen_always: bool = if col.get::<u8, _>("generated_always").unwrap_or(0u8).eq(&0u8) {
            false
        } else {
            true
        };
        let text_gen_alw: Option<&str> = col.get("text_generated_always");
        let def_comp_val: Option<&str> = col.get("computed_col_value");
        let ordering: i32 = col.get("ordering").unwrap_or(0);

        let column_memb = ColumnMembers::new(
            Some(col_name.map(str::to_owned).unwrap()),
            Some(data_type.to_string()),
            Some(length_field),
            Some(i32::from(numeric_precision)),
            Some(i32::from(numeric_scale)),
            Some(collation.map(str::to_owned).unwrap_or("".to_string())),
            Some(is_nullable),
            Some(is_identity),
            Some(is_gen_always),
            Some(text_gen_alw.map(str::to_owned).unwrap_or("".to_string())),
            ordering,
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
                lambda.add_columns(column_memb.to_owned());
            })
            .or_insert_with_key(|_| {
                TableMetadata::new(
                    table_name.to_owned(),
                    schema_name,
                    vec![column_memb.to_owned()],
                    computed_expression,
                    Vec::new(),
                    total_rows,
                )
            });
    }
}

async fn build_canonnical_pk(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    pk_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    let pks = client_conn
        .query(pk_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for pk in pks {
        let schema_name: String = pk
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let table_name: String = pk
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        //
        let pk_name: String = pk
            .get("pk_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let column_name: String = pk
            .get("column_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let data_type: String = pk
            .get("data_type")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let key_ordinal: u8 = pk.get("key_ord").unwrap_or_else(|| 0);
        let last_val: i32 = pk.get("last_value").unwrap_or_else(|| 0);
        let increment: i32 = pk.get("increment_by").unwrap_or_else(|| 0);
        let keys = (table_name.clone(), schema_name.clone());
        let last_value: i64 = i64::from(last_val);
        let incrementals = IdentitySpecification::new(
            pk_name,
            column_name,
            table_name,
            data_type,
            i32::from(key_ordinal),
            Some(last_value),
            Some(increment),
        );
        tables.entry(keys).and_modify(|pred| {
            //modify the PK's
            pred.add_pk(incrementals);
        });
    }
}

async fn build_canonnical_fk(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    fk_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    let fks = client_conn
        .query(fk_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for fk in fks {
        let schema_name: String = fk
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let table_name: String = fk
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let keys = (table_name.clone(), schema_name);
        let col_origin: String = fk
            .get("table_origin_column")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let ref_table: String = fk
            .get("referenced_column")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let ref_column: String = fk
            .get("referenced_column")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let fk_name = fk
            .get("fk__name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let disabled: bool = fk.get::<bool, _>("disabled").unwrap_or(false);
        let del_ref_action: bool = if fk.get::<u8, _>("del_ref_act").unwrap_or(0u8).eq(&0) {
            false
        } else {
            true
        };
        let upd_ref_action: bool = if fk.get::<u8, _>("upd_ref_act").unwrap_or(0u8).eq(&0) {
            false
        } else {
            true
        };
        let foreign_key: ForeignKeys = ForeignKeys::new(
            table_name,
            col_origin,
            ref_table,
            ref_column,
            fk_name,
            disabled,
            del_ref_action,
            upd_ref_action,
        );
        tables.entry(keys).and_modify(|lambda| {
            lambda.add_fks(foreign_key);
        });
    }
}

async fn build_cannonical_defaults(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    defs_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    let defaults = client_conn
        .query(defs_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for default in defaults {
        let table_name: String = default
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let schema_name: String = default
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let col_name: String = default
            .get("column_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let def_name = default
            .get("default_constraint")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let def_spec = default
            .get("default_definition")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let computed_spec = ComputedSpecification::new(
            Some(def_name),
            def_spec,
            Some(false),
            Some(false),
            Some(false),
            col_name,
        );
        let keys = (table_name, schema_name);
        tables
            .entry(keys)
            .and_modify(|lambda| lambda.add_computed_res(SQLConstraints::DEFAULT(computed_spec)));
    }
}

async fn build_canonnical_checks(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    check_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    let checks = client_conn
        .query(check_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for check in checks {
        let schema_name: String = check
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let table_name: String = check
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let def_check: String = check
            .get("def")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let column_name: String = check
            .get("column_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        //let is_disabled: bool = if check.get::<u8 , _>("is_disabled").unwrap_or_else(|| 0).eq(&0) {false } else {true};
        let is_disabled: bool = check.get::<bool, _>("is_disabled").unwrap_or_else(|| false);
        let is_not_trusted: bool = check.get::<bool, _>("not_trust").unwrap_or_else(|| false);
        let is_table_scoped: bool = column_name.eq("");
        let comp_spec = ComputedSpecification::new(
            None,
            def_check,
            Some(is_disabled),
            Some(is_not_trusted),
            Some(is_table_scoped),
            column_name,
        );
        let keys = (table_name, schema_name);
        tables.entry(keys).and_modify(|lambda| {
            lambda.add_computed_res(SQLConstraints::CHECK(comp_spec));
        });
    }
}

async fn build_canonnical_indexes(
    client_conn: &mut bb8::PooledConnection<'_, ConnectionManager>,
    index_query: &&Query,
    tables: &mut HashMap<(String, String), TableMetadata>,
) {
    let indexes = client_conn
        .query(index_query.query_out(), &[])
        .await
        .unwrap()
        .into_first_result()
        .await
        .unwrap();
    for index in indexes {
        let table_name = index
            .get("table_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let schema_name = index
            .get("schema_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let index_name = index
            .get("index_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let type_desc = index
            .get("type_desc")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let is_unique: bool = index.get::<bool, _>("is_unique").unwrap_or(false);
        let is_unique_cons: bool = index.get::<bool, _>("is_unique_constr").unwrap_or(false);
        let filter_desc: String = index
            .get("filter_desc")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let is_disabled: bool = index.get::<bool, _>("is_disabled").unwrap_or(false);
        let key_ordinal: u8 = index.get("key_ord").unwrap_or(0);
        let is_desc_key: bool = index.get::<bool, _>("desc_key").unwrap_or(false);
        let is_included_col: bool = index.get::<bool, _>("inc_column").unwrap_or(false);
        let column_name: String = index
            .get("column_name")
            .map(str::to_owned)
            .unwrap_or_else(|| "".to_owned());
        let index_table = TableIndex::new(
            index_name,
            type_desc,
            is_unique,
            is_unique_cons,
            Some(filter_desc),
            is_disabled,
            i32::from(key_ordinal),
            is_included_col,
            column_name,
            is_desc_key,
        );
        let keys = (table_name, schema_name);
        tables.entry(keys).and_modify(|lambda| {
            lambda.add_indexes(index_table);
        });
    }
}

pub async fn build_canonnical_schema(
    connection: &mut DatabaseHandlers,
    query_list: Vec<&Query>,
) -> Result<HashMap<(String, String), TableMetadata>, Box<dyn std::error::Error>> {
    let mut tables: HashMap<(String, String), TableMetadata> = HashMap::new();
    match connection {
        DatabaseHandlers::SqlServerPool(client) => {
            let mut client_conn = &mut client.mssql_pool.get().await.unwrap();
            //Build Cannonical Table/Cols
            let cols_query = query_list.iter().find(|pred| pred.id_out().eq(&2)).unwrap();
            build_cannonical_columns(&mut client_conn, cols_query, &mut tables).await;
            //Build Cannonical PK
            let pk_query: &&Query = query_list.iter().find(|pred| pred.id_out().eq(&4)).unwrap();
            build_canonnical_pk(&mut client_conn, pk_query, &mut tables).await;
            //Get Cannonical FK
            let fk_query: &&Query = query_list.iter().find(|pred| pred.id_out().eq(&3)).unwrap();
            build_canonnical_fk(&mut client_conn, fk_query, &mut tables).await;
            //Get Canonnical Constraints
            //Default
            let defs_query = query_list.iter().find(|pred| pred.id_out().eq(&5)).unwrap();
            build_cannonical_defaults(&mut client_conn, defs_query, &mut tables).await;
            //Check
            let check_query = query_list.iter().find(|pred| pred.id_out().eq(&6)).unwrap();
            build_canonnical_checks(&mut client_conn, check_query, &mut tables).await;
            //Get Cannonical Indexes
            let index_query = query_list.iter().find(|pred| pred.id_out().eq(&7)).unwrap();
            build_canonnical_indexes(&mut client_conn, index_query, &mut tables).await;
        }
        _ => {}
    }
    Ok(tables)
}
