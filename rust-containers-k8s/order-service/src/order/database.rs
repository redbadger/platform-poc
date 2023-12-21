use super::Order;
use anyhow::anyhow;
use sqlx::{Pool, Postgres, QueryBuilder, Row};

impl Order {
    pub async fn save(&self, pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
        let row: (i64,) =
            sqlx::query_as("INSERT into t_orders(order_number) values ($1) RETURNING id")
                .bind(self.id.to_string())
                .fetch_one(pool)
                .await
                .map_err(|e| anyhow!(e))?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
            // spaces as that might interfere with identifiers or quoted strings where exact
            // values may matter.
            "INSERT INTO t_order_line_items(price, quantity, sku_code) ",
        );
        // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
        query_builder
            .push_values(self.line_items.iter(), |mut b, items| {
                // If you wanted to bind these by-reference instead of by-value,
                // you'd need an iterator that yields references that live as long as `query_builder`,
                // e.g. collect it to a `Vec` first.
                b.push_bind(items.price)
                    .push_bind(items.quantity)
                    .push_bind(&items.sku_code);
            })
            .push(" RETURNING id");

        let result = query_builder
            .build()
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow!(e))?;

        let line_item_ids: Vec<i64> = result
            .iter()
            .take(1)
            .map(|row| row.get::<i64, usize>(0))
            .collect();

        let mut query_builder_link_table: QueryBuilder<Postgres> = QueryBuilder::new(
            // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
            // spaces as that might interfere with identifiers or quoted strings where exact
            // values may matter.
            "INSERT INTO t_orders_order_line_items_list(order_id, order_line_items_list_id) ",
        );

        // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
        query_builder_link_table.push_values(line_item_ids.iter(), |mut b, line_item_id| {
            // If you wanted to bind these by-reference instead of by-value,
            // you'd need an iterator that yields references that live as long as `query_builder`,
            // e.g. collect it to a `Vec` first.
            b.push_bind(row.0).push_bind(line_item_id);
        });

        query_builder_link_table
            .build()
            .execute(pool)
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(())
    }
}
