const express = require('express');
const cors = require('cors');
const app = express();
const port = 3000;

app.use(cors());
app.use(express.json());

app.get('/', (req, res) => {
  res.send('Payments Service (Node.js) Activo 💳');
});

app.post('/payments', (req, res) => {
  const { orderId, amount } = req.body;
  const paymentStatus = Math.random() > 0.2 ? 'APROBADO' : 'RECHAZADO';
  
  console.log(`Procesando pago para orden ${orderId}: ${paymentStatus}`);
  
  res.json({
    message: 'Proceso finalizado',
    status: paymentStatus,
    transactionId: Math.floor(Math.random() * 1000000)
  });
});

app.listen(port, () => {
  console.log(`Payments service escuchando en puerto ${port}`);
});