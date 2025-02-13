package com.redbadger.productservice;

import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.redbadger.productservice.dto.ProductRequest;
import com.redbadger.productservice.repository.ProductRepository;
import com.redis.testcontainers.RedisContainer;
import java.util.stream.StreamSupport;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.data.redis.connection.RedisConnection;
import org.springframework.data.redis.core.RedisOperations;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;
import org.testcontainers.utility.DockerImageName;

@SpringBootTest
@Testcontainers
@AutoConfigureMockMvc
class ProductServiceApplicationTests {

    @Container
    static RedisContainer redis = new RedisContainer(
        DockerImageName.parse("redis:7")
    );

    @Autowired
    private MockMvc mockMvc;

    @Autowired
    private ObjectMapper objectMapper;

    @Autowired
    RedisOperations<Object, Object> operations;

    @Autowired
    private ProductRepository productRepository;

    @BeforeEach
    @AfterEach
    void setUp() {
        operations.execute((RedisConnection connection) -> {
            connection.serverCommands().flushDb();
            return "OK";
        });
    }

    @Test
    void shouldCreateProduct() throws Exception {
        ProductRequest productRequest = getProductRequest();
        String productRequestString = objectMapper.writeValueAsString(
            productRequest
        );
        mockMvc
            .perform(
                MockMvcRequestBuilders.post("/api/product")
                    .contentType(MediaType.APPLICATION_JSON)
                    .content(productRequestString)
            )
            .andExpect(status().isCreated());
        Assertions.assertEquals(
            1,
            StreamSupport.stream(
                productRepository.findAll().spliterator(),
                false
            ).count()
        );
    }

    private ProductRequest getProductRequest() {
        return ProductRequest.builder()
            .name("iPhone 13")
            .description("iPhone 13")
            .price(1200)
            .build();
    }
}
