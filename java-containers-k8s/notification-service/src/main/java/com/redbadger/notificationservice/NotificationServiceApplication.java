package com.redbadger.notificationservice;

import lombok.extern.slf4j.Slf4j;
import org.mvnsearch.spring.boot.nats.annotation.NatsSubscriber;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.messaging.handler.annotation.Payload;

@SpringBootApplication
@Slf4j
public class NotificationServiceApplication {

    public static void main(String[] args) {
        SpringApplication.run(NotificationServiceApplication.class, args);
    }

    @NatsSubscriber(subject = "orders")
    public void handler(@Payload OrderPlacedEvent orderPlacedEvent) {
        // send out an email notification
        log.info(
            "Received Notification for Order - {}",
            orderPlacedEvent.getOrderNumber()
        );
    }
}
