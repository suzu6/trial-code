# import serial
import time
from datetime import datetime

# # 接続
# ser = serial.Serial('/dev/ttyAMA0', 115200)  # 受信用ポートをOpen

# while True:
#     data = ser.readline().decode('utf-8').strip()
#     if len(data) < 1:
#         continue
#     # 取得できた時の処理
#     print(data)
while True:
    # print(datetime.now())
    time.sleep(0.001)
    # pass
    # print("test")
