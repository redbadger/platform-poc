package com.redbadger.productservice.repository;

import com.redbadger.productservice.model.Product;
import org.springframework.data.repository.CrudRepository;
import org.springframework.data.repository.query.QueryByExampleExecutor;

public interface ProductRepository
    extends CrudRepository<Product, String>, QueryByExampleExecutor<Product> {}
