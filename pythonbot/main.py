from binance.client import Client
import pandas as pd
import numpy as np
from sklearn.svm import SVC
import time

# Binance API
api_key = ''
api_secret = 'TU_API_SECRET'
client = Client(api_key, api_secret)

symbol = 'BTCUSDT'
interval = '1m'
lookback = '120'  # Últimos 120 minutos

# Configuración
cantidad = 0.001
threshold_confianza = 0.7

def obtener_datos():
    klines = client.get_klines(symbol=symbol, interval=interval, limit=int(lookback))
    df = pd.DataFrame(klines, columns=['time','open','high','low','close','volume','close_time','qav','trades','tbb','tbq','ignore'])
    df['close'] = df['close'].astype(float)
    return df[['close']]

def preparar_datos(df):
    df['retorno'] = df['close'].pct_change()
    df.dropna(inplace=True)

    # Feature: últimos 3 retornos
    X = np.column_stack([df['retorno'].shift(i) for i in range(1, 4)])
    y = (df['retorno'] > 0).astype(int)  # 1 si sube, 0 si baja

    df = df.iloc[3:]
    X = X[3:]
    y = y[3:]

    return X, y

def entrenar_modelo(X, y):
    model = SVC(probability=True)
    model.fit(X, y)
    return model

def predecir(model, df):
    ultimo = df['retorno'].values[-3:]
    pred = model.predict([ultimo])[0]
    proba = model.predict_proba([ultimo])[0][pred]
    return pred, proba

# Loop principal
while True:
    try:
        df = obtener_datos()
        X, y = preparar_datos(df)
        modelo = entrenar_modelo(X, y)
        pred, proba = predecir(modelo, df)

        precio_actual = df['close'].iloc[-1]

        if proba > threshold_confianza:
            if pred == 1:
                print(f"[{time.ctime()}] Señal de COMPRA - Precio: {precio_actual:.2f} | Confianza: {proba:.2f}")
                # client.order_market_buy(symbol=symbol, quantity=cantidad)
            else:
                print(f"[{time.ctime()}] Señal de VENTA - Precio: {precio_actual:.2f} | Confianza: {proba:.2f}")
                # client.order_market_sell(symbol=symbol, quantity=cantidad)

        else:
            print(f"[{time.ctime()}] Sin señal clara. Confianza: {proba:.2f}")

        time.sleep(60)

    except Exception as e:
        print("Error:", e)
        time.sleep(60)

