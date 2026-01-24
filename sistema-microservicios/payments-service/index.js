const express = require('express');
const cors = require('cors');
const jwt = require('jsonwebtoken');

const app = express();
const PORT = 3000; 

app.use(cors()); 
app.use(express.json());

const SECRET_KEY = "esta_es_una_clave_muy_secreta_shhh";

const authenticateJWT = (req, res, next) => {
    if (req.method === 'OPTIONS') {
        return next();
    }

    const authHeader = req.headers.authorization;

    if (authHeader) {
        const token = authHeader.split(' ')[1]; 

        jwt.verify(token, SECRET_KEY, (err, user) => {
            if (err) {
                console.log("❌ Token inválido en Node:", err.message);
                return res.sendStatus(401);
            }
            req.user = user;
            next();
        });
    } else {
        console.log("❌ Falta cabecera Authorization en Node");
        res.sendStatus(401); 
    }
};

app.get('/', (req, res) => {
    res.send('Servicio de Pagos (Node.js) Activo 💳');
});

app.post('/payments', authenticateJWT, (req, res) => {
    const { orderId, amount } = req.body;
    
    console.log(`✅ Procesando pago de $${amount} para orden #${orderId}`);
    
    res.json({
        status: 'Aprobado',
        transactionId: 'TXN-' + Math.floor(Math.random() * 1000000),
        orderId: orderId,
        amount: amount,
        timestamp: new Date().toISOString()
    });
});

app.listen(PORT, () => {
    console.log(`🚀 Servicio de Pagos corriendo en puerto ${PORT}`);
});