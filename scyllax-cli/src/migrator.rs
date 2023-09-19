use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct MigrationFolder {
    pub version: i64,
    pub description: String,
}

impl TryFrom<Cow<'_, str>> for MigrationFolder {
    type Error = anyhow::Error;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        let mut parts = value.split('_');
        let version = parts.next().unwrap().parse::<i64>()?;
        let description = parts.collect::<Vec<&str>>().join(" ");

        Ok(Self {
            version,
            description,
        })
    }
}

impl ToString for MigrationFolder {
    fn to_string(&self) -> String {
        format!("{}_{}", self.version, self.description.replace(' ', "_"))
    }
}
