from fastapi import FastAPI, Depends, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, Session
from passlib.context import CryptContext
from datetime import datetime, timedelta
from jose import jwt

DATABASE_URL = "postgresql://user:password@db/auth_db"
engine = create_engine(DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

SECRET_KEY = "esta_es_una_clave_muy_secreta_shhh"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 30

def create_access_token(data: dict):
    to_encode = data.copy()
    expire = datetime.utcnow() + timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    to_encode.update({"exp": expire})
    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    return encoded_jwt

class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key=True, index=True)
    email = Column(String, unique=True, index=True)
    hashed_password = Column(String)

class UserCreate(BaseModel):
    email: str
    password: str

class UserLogin(BaseModel):
    email: str
    password: str

Base.metadata.create_all(bind=engine)

app = FastAPI()
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


@app.get("/")
def read_root():
    return {"mensaje": "¡Servicio de Auth (Python) activo!"}

def verify_password(plain_password, hashed_password):
    return pwd_context.verify(plain_password, hashed_password)

@app.post("/register")
def register(user: UserCreate, db: Session = Depends(get_db)):
    db_user = db.query(User).filter(User.email == user.email).first()
    if db_user:
        raise HTTPException(status_code=400, detail="El email ya existe")
    
    hashed_password = pwd_context.hash(user.password)
    new_user = User(email=user.email, hashed_password=hashed_password)
    db.add(new_user)
    db.commit()
    db.refresh(new_user)
    return {"mensaje": "Usuario creado", "email": new_user.email}

@app.post("/login")
def login(request: UserLogin, db: Session = Depends(get_db)):
    user = db.query(User).filter(User.email == request.email).first()
    
    if not user:
        raise HTTPException(status_code=404, detail="Usuario no encontrado")
    
    if not verify_password(request.password, user.hashed_password):
        raise HTTPException(status_code=400, detail="Contraseña incorrecta")
    
    access_token = create_access_token(data={"sub": user.email})
    
    return {"access_token": access_token, "token_type": "bearer"}