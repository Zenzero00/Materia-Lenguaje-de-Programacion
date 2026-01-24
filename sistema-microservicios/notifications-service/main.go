package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type NotificationRequest struct {
	Email   string `json:"email"`
	Message string `json:"message"`
}

func main() {
	r := gin.Default()

	r.Use(func(c *gin.Context) {
		c.Writer.Header().Set("Access-Control-Allow-Origin", "*")
		c.Writer.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
		c.Writer.Header().Set("Access-Control-Allow-Headers", "Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization")
		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}
		c.Next()
	})

	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "Notifications Service (Go + Gin) Activo 🔔")
	})

	r.POST("/notifications", func(c *gin.Context) {
		var json NotificationRequest
		if err := c.ShouldBindJSON(&json); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		println("Simulando envío de correo a: " + json.Email)

		c.JSON(http.StatusOK, gin.H{
			"status":    "Enviado",
			"recipient": json.Email,
		})
	})

	r.Run(":8080")
}
