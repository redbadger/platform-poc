package com.redbadger.notificationservice;

import com.google.gson.Gson;
import io.nats.client.Connection;
import io.nats.client.Dispatcher;
import io.nats.client.Nats;
import java.io.IOException;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.event.ContextRefreshedEvent;
import org.springframework.context.event.EventListener;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class SubServiceImpl implements SubService {

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

    @EventListener(ContextRefreshedEvent.class)
    public void init() {
        Dispatcher dispatcher = connection.createDispatcher(msg -> {
            byte[] bytes = msg.getData();
            Gson gson = new Gson();
            OrderPlacedEvent orderPlacedEvent = gson.fromJson(
                new String(bytes),
                OrderPlacedEvent.class
            );
            log.info(
                "Received Notification for Order - {}",
                orderPlacedEvent.getOrderNumber()
            );
        });

        dispatcher.subscribe(NOTIFICATION_SUBJECT);
    }
}
