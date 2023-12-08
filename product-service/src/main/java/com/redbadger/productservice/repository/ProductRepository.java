package com.redbadger.productservice.repository;

import com.redbadger.productservice.model.Product;
import org.springframework.cloud.gcp.data.datastore.repository.DatastoreRepository;

public interface ProductRepository extends DatastoreRepository<Product, String> {
}
