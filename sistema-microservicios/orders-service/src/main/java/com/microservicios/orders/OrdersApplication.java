package com.microservicios.orders;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.CrossOrigin;
import java.util.List;
import java.util.Arrays;

@SpringBootApplication
@RestController
@CrossOrigin(origins = "*") 
public class OrdersApplication {

    public static void main(String[] args) {
        SpringApplication.run(OrdersApplication.class, args);
    }

    @GetMapping("/")
    public String home() {
        return "Servicio de Pedidos (Java + Spring Boot) Activo ☕";
    }

    @GetMapping("/orders")
    public List<Order> getOrders() {
        return Arrays.asList(
            new Order(1, "Pedido #101", "ENVIADO", 150.00),
            new Order(2, "Pedido #102", "PENDIENTE", 45.50),
            new Order(3, "Pedido #103", "CANCELADO", 200.00)
        );
    }

    static class Order {
        public int id;
        public String description;
        public String status;
        public double total;

        public Order(int id, String description, String status, double total) {
            this.id = id;
            this.description = description;
            this.status = status;
            this.total = total;
        }
    }
}