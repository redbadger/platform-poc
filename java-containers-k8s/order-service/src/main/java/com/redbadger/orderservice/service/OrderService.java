package com.redbadger.orderservice.service;

import com.redbadger.orderservice.dto.InventoryResponse;
import com.redbadger.orderservice.dto.OrderDto;
import com.redbadger.orderservice.dto.OrderLineItemsDto;
import com.redbadger.orderservice.dto.OrderRequest;
import com.redbadger.orderservice.event.OrderPlacedEvent;
import com.redbadger.orderservice.model.Order;
import com.redbadger.orderservice.model.OrderLineItems;
import com.redbadger.orderservice.repository.OrderRepository;
import java.util.Arrays;
import java.util.List;
import java.util.UUID;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.reactive.function.client.WebClient;

@Service
@RequiredArgsConstructor
@Transactional
@Slf4j
public class OrderService {

    static String INVENTORY_URL = System.getenv("INVENTORY_URL");

    static {
        if (INVENTORY_URL == null) {
            INVENTORY_URL = "http://inventory-service/api/inventory";
        }
    }

    private final OrderRepository orderRepository;
    private final WebClient.Builder webClientBuilder;

    @Autowired
    private PubService pubService;

    public List<OrderDto> getOrders() {
        final List<Order> all = orderRepository.findAll();

        return all.stream().map(this::mapToOrder).toList();
    }

    public String placeOrder(OrderRequest orderRequest) {
        Order order = new Order();
        order.setOrderNumber(UUID.randomUUID().toString());

        List<OrderLineItems> orderLineItems = orderRequest
            .getOrderLineItemsDtoList()
            .stream()
            .map(this::mapToDto)
            .toList();

        order.setOrderLineItemsList(orderLineItems);

        List<String> skuCodes = order
            .getOrderLineItemsList()
            .stream()
            .map(OrderLineItems::getSkuCode)
            .toList();

        // Call Inventory Service, and place order if product is in
        // stock
        InventoryResponse[] inventoryResponseArray = webClientBuilder
            .build()
            .get()
            .uri(INVENTORY_URL, uriBuilder ->
                uriBuilder.queryParam("skuCode", skuCodes).build()
            )
            .retrieve()
            .bodyToMono(InventoryResponse[].class)
            .block();

        boolean allProductsInStock =
            Arrays.stream(inventoryResponseArray).allMatch(
                InventoryResponse::isInStock
            ) &&
            inventoryResponseArray.length > 0;

        if (allProductsInStock) {
            log.info("All requested products are in stock");
            orderRepository.save(order);
            pubService.publishOrder(
                new OrderPlacedEvent(order.getOrderNumber())
            );
            String msg = String.format(
                "Order Number %s Placed Successfully",
                order.getOrderNumber()
            );
            log.info(msg);
            return msg;
        } else {
            return "Product is not in stock, please try again later";
        }
    }

    private OrderLineItems mapToDto(OrderLineItemsDto orderLineItemsDto) {
        OrderLineItems orderLineItems = new OrderLineItems();
        orderLineItems.setPrice(orderLineItemsDto.getPrice());
        orderLineItems.setQuantity(orderLineItemsDto.getQuantity());
        orderLineItems.setSkuCode(orderLineItemsDto.getSkuCode());
        return orderLineItems;
    }

    private OrderDto mapToOrder(Order order) {
        final String orderNumber = order.getOrderNumber();

        final List<OrderLineItemsDto> items = order
            .getOrderLineItemsList()
            .stream()
            .map(entry ->
                new OrderLineItemsDto(
                    entry.getSkuCode(),
                    entry.getPrice(),
                    entry.getQuantity()
                )
            )
            .toList();

        return new OrderDto(orderNumber, items);
    }
}
