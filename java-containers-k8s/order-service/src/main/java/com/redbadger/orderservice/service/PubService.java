package com.redbadger.orderservice.service;

import com.redbadger.orderservice.event.OrderPlacedEvent;

public interface PubService {
    void publishOrder(OrderPlacedEvent order);
}
