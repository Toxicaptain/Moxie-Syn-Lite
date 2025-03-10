pub use sea_orm_migration::prelude::*;

mod m20240801_042205_create_tables;
mod m20241128_194627_add_suspense_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240801_042205_create_tables::Migration),
            Box::new(m20241128_194627_add_suspense_table::Migration),
        ]
    }
}
