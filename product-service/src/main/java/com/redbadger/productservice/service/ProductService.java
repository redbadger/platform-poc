package com.redbadger.productservice.service;

import com.redbadger.productservice.dto.ProductRequest;
import com.redbadger.productservice.dto.ProductResponse;
import com.redbadger.productservice.model.Product;
import com.redbadger.productservice.repository.ProductRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.stream.StreamSupport;

@Service
@RequiredArgsConstructor
@Slf4j
public class ProductService {

    private final ProductRepository productRepository;

    public void createProduct(ProductRequest productRequest) {
        Product product = Product.builder()
                                 .name(productRequest.getName())
                                 .description(productRequest.getDescription())
                                 .price(productRequest.getPrice())
                                 .skuCode(productRequest.getSkuCode())
                                 .build();

        productRepository.save(product);
        log.info("Product {} is saved", product.getId());
    }

    public List<ProductResponse> getAllProducts() {
        Iterable<Product> products = productRepository.findAll();

        return StreamSupport.stream(products.spliterator(), false).map(this::mapToProductResponse).toList();
    }

    private ProductResponse mapToProductResponse(Product product) {
        return ProductResponse.builder()
                .id(product.getId())
                .name(product.getName())
                .description(product.getDescription())
                .price(product.getPrice())
                .skuCode(product.getSkuCode())
                .build();
    }
}
