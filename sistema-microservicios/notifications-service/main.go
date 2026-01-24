package main

import (
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

var secretKey = []byte("esta_es_una_clave_muy_secreta_shhh")

type NotificationRequest struct {
	Email   string `json:"email"`
	Message string `json:"message"`
}

func authMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		authHeader := c.GetHeader("Authorization")
		if authHeader == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "Falta header Authorization"})
			return
		}

		tokenString := strings.TrimPrefix(authHeader, "Bearer ")
		if tokenString == authHeader {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "Formato de token inválido"})
			return
		}

		token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
			if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
				return nil, fmt.Errorf("método de firma inesperado: %v", token.Header["alg"])
			}
			return secretKey, nil
		})

		if err != nil || !token.Valid {
			fmt.Printf("❌ Error JWT Go: %v\n", err)
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "Token inválido o expirado"})
			return
		}

		c.Next()
	}
}

func main() {
	r := gin.Default()

	config := cors.DefaultConfig()
	config.AllowAllOrigins = true
	config.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "Authorization"}
	r.Use(cors.New(config))

	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "¡Servicio de Notificaciones (Go) Activo! 🐹")
	})

	protected := r.Group("/")
	protected.Use(authMiddleware())
	{
		protected.POST("/notifications", func(c *gin.Context) {
			var req NotificationRequest

			if err := c.BindJSON(&req); err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"error": "Datos inválidos"})
				return
			}

			fmt.Printf("📧 Enviando correo a: %s | Mensaje: %s\n", req.Email, req.Message)

			c.JSON(http.StatusOK, gin.H{
				"status":    "Enviado",
				"recipient": req.Email,
				"timestamp": time.Now().Format(time.RFC3339),
			})
		})
	}

	fmt.Println("🚀 Notificaciones corriendo en :8080")
	r.Run(":8080")
}
