# Data init

Component used to initialize schema and data in the postgres and redis stores.

call one of the following endpoints:

* `/data-init/all` - will initialize all schemas and data entries
* `/data-init/products` - will populate the redis store with sample products
* `/data-init/inventory` - will create schema for the inventory service and populate some entries
* `/data-init/orders` - will create schema for orders service
