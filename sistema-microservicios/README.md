# 🚀 Sistema Distribuido de Microservicios (E-commerce) {#sistema-distribuido-de-microservicios-e-commerce}

Este proyecto implementa una arquitectura de microservicios robusta para una plataforma de comercio electrónico. El sistema destaca por su naturaleza políglota, utilizando **6 lenguajes de programación** diferentes integrados en un ecosistema único, seguro y escalable mediante contenedores.

## 📋 Integrantes del Grupo {#integrantes-del-grupo}

- **Francisco Unda** - Arquitectura, Desarrollo Full Stack y DevOps.

## 🏗️ Arquitectura del Sistema {#arquitectura-del-sistema}

El diseño se basa en la descentralización de responsabilidades. Cada microservicio es independiente, posee su propia tecnología y se comunica a través de una red interna protegida.

| **Servicio**          | **Tecnología**        | **Puerto (Host)** | **Responsabilidad Principal**                               |
|-----------------------|-----------------------|-------------------|-------------------------------------------------------------|
| **Frontend**          | 💚 Vue.js (Vite)      | 8080              | Interfaz de usuario reactiva y gestión de estado del Token. |
| **Auth Service**      | 🐍 Python (FastAPI)   | 8000              | Registro, Login y emisión centralizada de JWT.              |
| **Products Service**  | 🐘 PHP (Slim)         | 8001              | Gestión del catálogo de productos y metadatos.              |
| **Inventory Service** | 🦀 Rust (Axum)        | 8002              | Control crítico de stock con alto rendimiento.              |
| **Orders Service**    | ☕ Java (Spring Boot) | 8003              | Procesamiento de pedidos y lógica de estados.               |
| **Payments Service**  | 🟢 Node.js (Express)  | 8004              | Simulación de transacciones financieras.                    |
| **Notifications**     | 🐹 Go (Gin)           | 8005              | Motor de mensajería y alertas al cliente.                   |
| **Database**          | 🐬 PostgreSQL         | 5433              | Persistencia de datos para usuarios y catálogo.             |

## 🔐 Explicación del uso de JWT (JSON Web Tokens) {#explicación-del-uso-de-jwt-json-web-tokens}

La seguridad del sistema es **Stateless** (Sin estado), lo que permite que cada microservicio valide la identidad del usuario de forma independiente sin consultar una base de datos central en cada petición.

### Flujo de Seguridad:

1.  **Generación (Auth):** Al hacer login, Python genera un token firmado con el algoritmo HS256. El payload contiene la identidad del usuario (sub) y el tiempo de vida (exp).

2.  **Validación Distribuida:** Todos los servicios comparten una **Clave Secreta** unificada. Cuando el Frontend envía el token, el middleware de cada servicio (Rust, Java, Go, etc.) realiza una verificación matemática de la firma.

3.  **Integridad:** Si un solo carácter del token es modificado en el camino, la validación matemática fallará en el backend, devolviendo un error 401 Unauthorized.

## 🔄 Diagramas de Interacción {#diagramas-de-interacción}

sequenceDiagram  
participant SPA as 💻 Frontend (Vue.js)  
participant Auth as 🐍 Auth (Python)  
participant API as ⚙️ Microservicios (Rust/Java/etc)  
  
SPA-\>\>Auth: 1. POST /login {credenciales}  
Auth\--\>\>SPA: 2. Retorna JWT (Access Token) 🔑  
  
SPA-\>\>API: 3. Request + Header \[Authorization: Bearer Token\]  
Note right of API: Middleware valida Token con Secret Key  
API\--\>\>SPA: 4. Response con datos JSON 📦

## 💻 Instrucciones de Instalación y Ejecución {#instrucciones-de-instalación-y-ejecución}

Sigue estos pasos para desplegar el entorno completo de desarrollo:

### 1. Requisitos Previos {#requisitos-previos}

- Tener instalado **Docker Desktop**.

- Terminal de comandos (PowerShell, CMD o Bash).

### 2. Preparación del Entorno {#preparación-del-entorno}

Clona el proyecto y entra en la carpeta raíz:

cd sistema-microservicios

### 3. Construcción y Despliegue {#construcción-y-despliegue}

Ejecuta el orquestador de Docker para levantar todos los servicios simultáneamente:

docker-compose up -d \--build

> **Nota:** Este proceso compila internamente los binarios de Rust, Java y Go, por lo que la primera ejecución puede tardar varios minutos.

### 4. Verificación de Servicios {#verificación-de-servicios}

Asegúrate de que todos los módulos estén en estado **Running**:

docker ps

### 5. Acceso a la Aplicación {#acceso-a-la-aplicación}

Abre tu navegador y entra en la URL del Frontend:

👉 [[http://localhost:8080]{.underline}](https://www.google.com/search?q=http://localhost:8080)

## 🛠️ Guía de Pruebas Rápidas {#guía-de-pruebas-rápidas}

1.  **Registro/Login:** Regístrate en la sección de Python. Una vez logueado, el sistema guardará el token JWT de forma local en la aplicación.

2.  **Consumo de API:** Haz clic en los botones de \"Ver Stock\" (Servicio Rust) o \"Listar Pedidos\" (Servicio Java). Observarás cómo los datos se cargan correctamente tras validar tu identidad.

3.  **Prueba de Seguridad:** Si refrescas la página (borrando el estado del token) e intentas pulsar los botones de los servicios protegidos, el sistema mostrará errores de autorización, demostrando que el acceso está blindado.
