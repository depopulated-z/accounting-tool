//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub txn_type: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub location: Option<String>,
    pub txn_time: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::transaction_type::Entity",
        from = "Column::TxnType",
        to = "super::transaction_type::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    TransactionType,
}

impl Related<super::transaction_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
