package com.microservicios.orders;

import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.AllArgsConstructor;
import lombok.Data;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpStatus;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;
import org.springframework.web.filter.CorsFilter;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;

@SpringBootApplication
public class OrdersApplication {
    public static void main(String[] args) {
        SpringApplication.run(OrdersApplication.class, args);
    }
}

@Data
@AllArgsConstructor
class Order {
    private int id;
    private String description;
    private String status;
    private double total;
}

@RestController
class OrderController {
    
    @GetMapping("/orders")
    public List<Order> getOrders() {
        return Arrays.asList(
            new Order(101, "Pedido #101 - Laptop", "Pendiente de Pago", 1500.00),
            new Order(102, "Pedido #102 - Mouse", "Enviado", 50.00),
            new Order(103, "Pedido #103 - Monitor", "Entregado", 300.00)
        );
    }

    @GetMapping("/")
    public String health() {
        return "Servicio de Pedidos (Java) Activo ☕";
    }
}

class JwtFilter extends OncePerRequestFilter {
    private static final String SECRET_KEY = "esta_es_una_clave_muy_secreta_shhh";

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain chain)
            throws ServletException, IOException {
        
        String authHeader = request.getHeader("Authorization");

        if ("OPTIONS".equalsIgnoreCase(request.getMethod())) {
            chain.doFilter(request, response);
            return;
        }

        if (authHeader != null && authHeader.startsWith("Bearer ")) {
            String token = authHeader.substring(7);
            try {
                Claims claims = Jwts.parserBuilder()
                        .setSigningKey(Keys.hmacShaKeyFor(SECRET_KEY.getBytes(StandardCharsets.UTF_8)))
                        .build()
                        .parseClaimsJws(token)
                        .getBody();
                
                chain.doFilter(request, response);
                return;
            } catch (Exception e) {
                System.out.println("❌ Error JWT Java: " + e.getMessage());
                response.setStatus(HttpStatus.UNAUTHORIZED.value());
                response.getWriter().write("Token Invalido");
                return;
            }
        }

        if (request.getRequestURI().startsWith("/orders")) {
             response.setStatus(HttpStatus.UNAUTHORIZED.value());
             response.getWriter().write("Falta Token");
             return;
        }

        chain.doFilter(request, response);
    }
}

@Configuration
@EnableWebSecurity
class SecurityConfig {

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
        http
            .cors(cors -> cors.configurationSource(corsConfigurationSource()))
            .csrf(csrf -> csrf.disable()) 
            .addFilterBefore(new JwtFilter(), UsernamePasswordAuthenticationFilter.class) 
            .authorizeHttpRequests(auth -> auth
                .requestMatchers("/").permitAll() 
                .requestMatchers("/orders/**").permitAll() 
                .anyRequest().authenticated()
            );
            
        return http.build();
    }

    @Bean
    public UrlBasedCorsConfigurationSource corsConfigurationSource() {
        CorsConfiguration config = new CorsConfiguration();
        config.setAllowedOrigins(List.of("http://localhost:8080")); 
        config.setAllowedMethods(List.of("GET", "POST", "PUT", "DELETE", "OPTIONS"));
        config.setAllowedHeaders(List.of("Authorization", "Content-Type"));
        config.setAllowCredentials(true);
        
        UrlBasedCorsConfigurationSource source = new UrlBasedCorsConfigurationSource();
        source.registerCorsConfiguration("/**", config);
        return source;
    }
}