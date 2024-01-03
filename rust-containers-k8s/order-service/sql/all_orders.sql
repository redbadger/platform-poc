SELECT "order".id as order_id,
  "order".order_number,
  "line_items".id as item_id,
  "line_items".sku_code,
  "line_items".price,
  "line_items".quantity
FROM t_order_line_items as line_items
  JOIN t_orders_order_line_items_list as order_lines ON "order_lines".order_line_items_list_id = "line_items".id
  JOIN t_orders as "order" ON "order".id = "order_lines".order_id
