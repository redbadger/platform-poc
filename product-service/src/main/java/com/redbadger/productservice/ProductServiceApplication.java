package com.redbadger.productservice;

import com.redbadger.productservice.repository.ProductRepository;
import com.redbadger.productservice.model.Product;
import lombok.RequiredArgsConstructor;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import java.math.BigDecimal;

@SpringBootApplication
@RequiredArgsConstructor
public class ProductServiceApplication implements CommandLineRunner {

    private final ProductRepository productRepository;

    public static void main(String[] args) {
        SpringApplication.run(ProductServiceApplication.class, args);
    }

    @Override
    public void run(String... args) {
        if (productRepository.count() < 1) {
            productRepository.save(Product.builder()
                                          .id(123)
                                          .name("iPhone 13")
                                          .description("New iPhone")
                                          .price(1000)
                                          .skuCode("iphone_13")
                                          .build()
            );
            productRepository.save(Product.builder()
                                          .id(234)
                                          .name("Samsung S23")
                                          .description("New Samsung")
                                          .price(800)
                                          .skuCode("samsung_s23")
                                          .build()
            );
            productRepository.save(Product.builder()
                                          .id(345)
                                          .name("Google Pixel 8")
                                          .description("New Pixel")
                                          .price(7000)
                                          .skuCode("pixel_8")
                                          .build()
            );

        }
    }
}
