package com.redbadger.orderservice.service;

import com.google.gson.Gson;
import com.redbadger.orderservice.event.OrderPlacedEvent;
import io.nats.client.Connection;
import io.nats.client.Nats;
import java.io.IOException;
import org.springframework.stereotype.Service;

@Service
public class PubServiceImpl implements PubService {

    static String NOTIFICATION_SUBJECT = "platform-poc.order-notification";
    private static Connection connection;

    static {
        String natsURL = System.getenv("NATS_URL");
        if (natsURL == null) {
            natsURL = "nats://127.0.0.1:4222";
        }

        try {
            connection = Nats.connect(natsURL);
        } catch (IOException | InterruptedException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void publishOrder(OrderPlacedEvent order) {
        Gson gson = new Gson();
        byte[] data = gson.toJson(order).getBytes();
        connection.publish(NOTIFICATION_SUBJECT, data);
    }
}
