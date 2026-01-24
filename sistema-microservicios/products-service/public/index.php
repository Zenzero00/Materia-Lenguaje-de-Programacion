<?php
use Psr\Http\Message\ResponseInterface as Response;
use Psr\Http\Message\ServerRequestInterface as Request;
use Slim\Factory\AppFactory;
use Firebase\JWT\JWT;
use Firebase\JWT\Key;
use Slim\Exception\HttpUnauthorizedException;

require __DIR__ . '/../vendor/autoload.php';


$app = AppFactory::create();

$secretKey = "esta_es_una_clave_muy_secreta_shhh";

$authMiddleware = function (Request $request, $handler) use ($secretKey) {

    if ($request->getMethod() === 'OPTIONS') {
        return $handler->handle($request);
    }

    $authHeader = $request->getHeaderLine('Authorization');

    if (!$authHeader) {
        throw new HttpUnauthorizedException($request, "Falta el token de autorización");
    }

    $token = str_replace('Bearer ', '', $authHeader);

    try {
        $decoded = JWT::decode($token, new Key($secretKey, 'HS256'));
        
        $request = $request->withAttribute('user', $decoded);
        
    } catch (Exception $e) {
        throw new HttpUnauthorizedException($request, "Token inválido o expirado: " . $e->getMessage());
    }

    return $handler->handle($request);
};

$app->add(function ($request, $handler) {
    $response = $handler->handle($request);
    return $response
        ->withHeader('Access-Control-Allow-Origin', '*')
        ->withHeader('Access-Control-Allow-Headers', 'X-Requested-With, Content-Type, Accept, Origin, Authorization')
        ->withHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
});

$products = [
    ["id" => 1, "name" => "Laptop Gamer", "price" => 1500, "stock" => 5],
    ["id" => 2, "name" => "Mouse Óptico", "price" => 20, "stock" => 50],
    ["id" => 3, "name" => "Teclado Mecánico", "price" => 80, "stock" => 15],
    ["id" => 4, "name" => "Monitor 24''", "price" => 200, "stock" => 10],
    ["id" => 5, "name" => "Silla Ergonómica", "price" => 350, "stock" => 7],
];

$app->get('/products', function (Request $request, Response $response, $args) use ($products) {
    $response->getBody()->write(json_encode($products));
    return $response->withHeader('Content-Type', 'application/json');
})->add($authMiddleware);

$app->get('/', function (Request $request, Response $response, $args) {
    $response->getBody()->write(json_encode(["mensaje" => "API de Productos Activa 🐘"]));
    return $response->withHeader('Content-Type', 'application/json');
});

$app->options('/{routes:.+}', function ($request, $response, $args) {
    return $response;
});

$app->run();