use scylla::frame::value::CqlTimeuuid;
use scyllax::prelude::*;

/// Represents a person in the database
#[entity]
#[upsert_query(table = "person_login", name = UpsertPersonLogin)]
pub struct PersonLoginEntity {
    /// The id of the person
    #[entity(primary_key)]
    pub id: CqlTimeuuid,
    /// The email address of the person
    #[entity(primary_key)]
    pub person_id: CqlTimeuuid,
    /// The number of times the person has logged in
    #[entity(counter)]
    pub count: scylla::frame::value::Counter,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use scylla::{
        frame::{
            response::result::{ColumnSpec, ColumnType, PreparedMetadata, TableSpec},
            value::Counter,
        },
        serialize::{
            row::{RowSerializationContext, SerializeRow},
            RowWriter,
        },
    };

    fn col(name: &str, typ: ColumnType) -> ColumnSpec {
        ColumnSpec {
            table_spec: TableSpec {
                ks_name: "ks".to_string(),
                table_name: "tbl".to_string(),
            },
            name: name.to_string(),
            typ,
        }
    }

    fn do_serialize<T: SerializeRow>(t: T, columns: &[ColumnSpec]) -> Vec<u8> {
        let prepared = PreparedMetadata {
            col_specs: columns.to_vec(),
            col_count: columns.len(),

            flags: 0,
            pk_indexes: vec![],
        };
        let ctx = RowSerializationContext::from_prepared(&prepared);

        let mut ret = Vec::new();
        let mut builder = RowWriter::new(&mut ret);
        t.serialize(&ctx, &mut builder).unwrap();
        ret
    }

    #[test]
    fn test_pks() {
        assert_eq!(
            PersonLoginEntity::pks(),
            vec![r#""id""#.to_string(), r#""person_id""#.to_string()]
        );
    }

    #[test]
    fn test_keys() {
        assert_eq!(
            PersonLoginEntity::keys(),
            vec![
                r#""id""#.to_string(),
                r#""person_id""#.to_string(),
                r#""count""#.to_string()
            ]
        );
    }

    #[test]
    fn test_row_serialization() {
        let spec = [
            col("id", ColumnType::Timeuuid),
            col("person_id", ColumnType::Timeuuid),
            col("count", ColumnType::Counter),
        ];

        let id = CqlTimeuuid::from(v1_uuid());
        let person_id = CqlTimeuuid::from(v1_uuid());
        let count = Counter(42);

        let reference = do_serialize((id, person_id, count), &spec);
        let row = do_serialize(
            PersonLoginEntity {
                id,
                person_id,
                count,
            },
            &spec,
        );

        assert_eq!(row, reference);
    }
}
