package com.redbadger.notificationservice;

import io.nats.client.Message;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.lang3.SerializationUtils;
import org.mvnsearch.spring.boot.nats.annotation.NatsSubscriber;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@Slf4j
public class NotificationServiceApplication {

    public static void main(String[] args) {
        SpringApplication.run(NotificationServiceApplication.class, args);
    }

    @NatsSubscriber(subject = "orders")
    public void handler(Message msg) {
        byte[] bytes = msg.getData();
        OrderPlacedEvent orderPlacedEvent = SerializationUtils.deserialize(
            bytes
        );
        log.info(
            "Received Notification for Order - {}",
            orderPlacedEvent.getOrderNumber()
        );
    }
}
