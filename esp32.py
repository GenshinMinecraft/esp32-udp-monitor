import network
import socket
import time
import ujson
from machine import Pin
from mpython import *

oled.fill(0)
oled.DispChar(str('Hello, world!'), 0, 0, 1)
oled.show()

# WiFi配置
WIFI_SSID = "xx"     # 修改为你的WiFi名称
WIFI_PASSWORD = "xx"  # 修改为你的WiFi密码

# 初始化网络接口
wlan = network.WLAN(network.STA_IF)
wlan.active(True)

def connect_wifi():
    print("正在连接WiFi...", end='')
    wlan.connect(WIFI_SSID, WIFI_PASSWORD)
    
    while not (wlan.isconnected()):
        pass
    
    print("\n连接成功！")
    oled.DispChar(str('连接成功！'), 0, 16, 1)
    print("IP地址:", wlan.ifconfig()[0])
    oled.DispChar(str(wlan.ifconfig()[0]), 0, 32, 1)
    oled.show()
    return True

def udp_server():
    # 创建UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('0.0.0.0', 9090))
    print("UDP服务器已启动，监听端口：9090")
    oled.DispChar(str('UDP STARTED'), 0, 48, 1)
    oled.show()
    time.sleep(1)
    oled.fill(0)
    oled.show()
    
    data, _ = sock.recvfrom(1024)
    text = data.decode('utf-8')
    json_a = ujson.loads(text)
    oled.fill(0)
    oled.DispChar(str(json_a["line1"]), 0, 0, 1)
    oled.DispChar(str(json_a["line2"]), 0, 16, 1)
    oled.DispChar(str(json_a["line3"]), 0, 32, 1)
    oled.DispChar(str(json_a["line4"]), 0, 48, 1)
    oled.show()
    
    while True:
        data, _ = sock.recvfrom(1024)
        text = data.decode('utf-8')
        json_a = ujson.loads(text)
        oled.fill_rect(0, 0, 128, 48, 0)
        oled.DispChar(str(json_a["line1"]), 0, 0, 1)
        oled.DispChar(str(json_a["line2"]), 0, 16, 1)
        oled.DispChar(str(json_a["line3"]), 0, 32, 1)
        oled.show()

# 主程序
if connect_wifi():
    udp_server()
else:
    print("无法启动服务器，请检查网络连接")
