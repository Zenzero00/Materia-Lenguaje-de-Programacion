<script setup>
import { ref } from 'vue'
import axios from 'axios'

const email = ref('') 
const password = ref('') 
const authMessage = ref('Esperando acción...')
const token = ref('') 

const productsList = ref([])
const productsMessage = ref('Haz clic para cargar')

const inventoryList = ref([])
const inventoryMessage = ref('Esperando acción...')

const ordersList = ref([])
const ordersMessage = ref('Haz clic para ver pedidos')

const paymentMessage = ref('Esperando pago...')
const notificationMessage = ref('Esperando envío...')

const registerUser = async () => {
  authMessage.value = 'Registrando...'
  try {
    await axios.post('http://localhost:8000/register', { email: email.value, password: password.value })
    authMessage.value = '✅ Registro exitoso. Haz Login.'
  } catch (error) { 
    authMessage.value = '❌ Error: ' + (error.response?.data?.detail || error.message) 
  }
}

const loginUser = async () => {
  authMessage.value = 'Verificando...'
  try {
    const res = await axios.post('http://localhost:8000/login', { email: email.value, password: password.value })
    
    token.value = res.data.access_token
    authMessage.value = '✅ Login Exitoso! Token guardado.'
    console.log("Token:", token.value)
  } catch (error) { 
    console.error(error);
    authMessage.value = '❌ Error: ' + (error.response?.data?.detail || error.message) 
  }
}

const getAuthConfig = () => {
  if (!token.value) {
    console.warn("⚠️ ALERTA: Enviando petición SIN TOKEN para probar seguridad.");
    return {};
  }
  return {
    headers: { Authorization: `Bearer ${token.value}` }
  };
}

const getProducts = async () => {
  productsMessage.value = 'Cargando...'
  try {
    const res = await axios.get('http://localhost:8001/products', getAuthConfig())
    productsList.value = res.data
    productsMessage.value = '✅ Productos cargados'
  } catch (error) { productsMessage.value = '❌ Error: ' + (error.response ? error.response.status + " " + error.response.statusText : error.message) }
}

const getInventory = async () => {
  inventoryMessage.value = 'Cargando...'
  try {
    const res = await axios.get('http://localhost:8002/inventory', getAuthConfig())
    inventoryList.value = res.data
    inventoryMessage.value = '✅ Stock actualizado'
  } catch (error) { inventoryMessage.value = '❌ Error: ' + (error.response ? error.response.status + " " + error.response.statusText : error.message) }
}

const getOrders = async () => {
  ordersMessage.value = 'Cargando...'
  try {
    const res = await axios.get('http://localhost:8003/orders', getAuthConfig())
    ordersList.value = res.data
    ordersMessage.value = '✅ Pedidos cargados'
  } catch (error) { 
    ordersMessage.value = '❌ Error: ' + (error.response ? error.response.status + " " + error.response.statusText : error.message) 
  }
}

const testPayment = async () => {
  paymentMessage.value = 'Procesando...'
  try {
    const res = await axios.post('http://localhost:8004/payments', 
      { orderId: 101, amount: 50 }, 
      getAuthConfig()
    )
    paymentMessage.value = `✅ Estado: ${res.data.status} (ID: ${res.data.transactionId})`
  } catch (error) { paymentMessage.value = '❌ Error: ' + (error.response ? error.response.status + " " + error.response.statusText : error.message) }
}

const testNotification = async () => {
  notificationMessage.value = 'Enviando...'
  try {
    const res = await axios.post('http://localhost:8005/notifications', 
      { email: "cliente@prueba.com", message: "Su pedido ha sido enviado" },
      getAuthConfig()
    )
    notificationMessage.value = `✅ Email enviado a: ${res.data.recipient}`
  } catch (error) { notificationMessage.value = '❌ Error: ' + (error.response ? error.response.status + " " + error.response.statusText : error.message) }
}
</script>

<template>
  <div class="container">
    <h1>🚀 Sistema de Microservicios</h1>
    <p class="subtitle">6 Entornos en Perfecta Armonia</p>

    <div class="grid">
      
      <div class="card auth">
        <h2>🔐 Auth (Python)</h2>
        <input v-model="email" placeholder="Email" />
        <input v-model="password" type="password" placeholder="Password" />
        <div class="buttons">
          <button @click="registerUser">Registrar</button>
          <button @click="loginUser" class="login-btn">Login</button>
        </div>
        <div class="result">{{ authMessage }}</div>
      </div>

      <div class="card products">
        <h2>📦 Productos (PHP)</h2>
        <button @click="getProducts">Ver Catálogo</button>
        <p class="status">{{ productsMessage }}</p>
        <ul v-if="productsList.length > 0" class="list">
          <li v-for="p in productsList" :key="p.id">{{ p.name }} - ${{ p.price }}</li>
        </ul>
      </div>

      <div class="card inventory">
        <h2>📊 Stock (Rust)</h2>
        <button @click="getInventory">Ver Stock</button>
        <p class="status">{{ inventoryMessage }}</p>
        <ul v-if="inventoryList.length > 0" class="list">
          <li v-for="i in inventoryList" :key="i.id">
            <strong>{{ i.name }}</strong>: {{ i.quantity }} ({{ i.status }})
          </li>
        </ul>
      </div>

      <div class="card orders">
        <h2>☕ Pedidos (Java)</h2>
        <button @click="getOrders">Listar Pedidos</button>
        <p class="status">{{ ordersMessage }}</p>
        <ul v-if="ordersList.length > 0" class="list">
          <li v-for="o in ordersList" :key="o.id">
            {{ o.description }} ({{ o.status }}) - ${{ o.total }}
          </li>
        </ul>
      </div>

      <div class="card payments">
        <h2>💳 Pagos (Node.js)</h2>
        <p>Simular pago de Orden #101</p>
        <button @click="testPayment">Pagar $50</button>
        <div class="result">{{ paymentMessage }}</div>
      </div>

      <div class="card notifications">
        <h2>🔔 Notif. (Go)</h2>
        <p>Enviar alerta al cliente</p>
        <button @click="testNotification">Enviar Email</button>
        <div class="result">{{ notificationMessage }}</div>
      </div>
      
    </div>
  </div>
</template>

<style scoped>
.container { font-family: 'Segoe UI', sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; text-align: center; }
.subtitle { color: #666; margin-bottom: 30px; }
.grid { display: flex; justify-content: center; gap: 20px; flex-wrap: wrap; }
.card { border: 1px solid #ddd; border-radius: 8px; padding: 15px; width: 300px; background: white; box-shadow: 0 2px 5px rgba(0,0,0,0.1); }

.auth { border-top: 5px solid #3776AB; }       
.products { border-top: 5px solid #777BB4; }   
.inventory { border-top: 5px solid #dea584; }  
.orders { border-top: 5px solid #f89820; }     
.payments { border-top: 5px solid #68a063; }   
.notifications { border-top: 5px solid #00add8; }

input { display: block; width: 90%; margin: 10px auto; padding: 8px; }
button { background-color: #42b983; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer; margin: 5px; }
button:hover { background-color: #3aa876; }
.login-btn { background-color: #3776AB; }
.result, .status { margin-top: 10px; padding: 8px; background: #f4f4f4; border-radius: 4px; font-size: 0.9em; word-break: break-word; }
.list { text-align: left; background: #f9f9f9; padding: 10px 20px; border-radius: 5px; list-style: none; max-height: 120px; overflow-y: auto; margin-top: 10px; }
.list li { border-bottom: 1px solid #eee; padding: 5px 0; font-size: 0.85em; }
</style>