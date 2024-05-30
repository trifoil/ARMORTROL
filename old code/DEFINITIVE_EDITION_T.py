import cv2
import io
import socket
import struct
import time
import pickle
import numpy as np
import imutils
from multiprocessing import Process, Pipe
import serial
import time
import pygame
from time import perf_counter
SIZE = 9
SIZE2 = 5
FORMAT = "utf-8"
pygame.init
clock = pygame.time.Clock()

def s1():
    client_socket1 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket1.bind(('192.168.0.100', 8467))
    client_socket1.connect(('192.168.0.101', 8467))
    vid1 = cv2.VideoCapture(0)
    img_counter = 0
    print("ntm")
    while True:
        ret1, frame1 = vid1.read()
        #frame1 = cv2.resize(frame1, (960,540), interpolation = cv2.INTER_AREA)
        result1, image1 = cv2.imencode('.jpg',frame1,[int(cv2.IMWRITE_JPEG_QUALITY),100])
        data1 = pickle.dumps(image1, 0)
        size2 = len(data1)
        if img_counter%1==0:
            client_socket1.sendall(struct.pack(">L", size2) + data1)
            cv2.imshow('client',frame1)
        img_counter += 1
        if cv2.waitKey(1) & 0xFF == ord('q'):
            break
    client_socket1.close()
    vid1.release()
arduino = serial.Serial(port='/dev/ttyUSB0', baudrate=115200, timeout=1)
p1 = Process(target=s1)
client_socket0 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client_socket0.bind(('192.168.0.100', 8470))
client_socket0.connect(('192.168.0.101', 8470))
print("connecté")
p1.start()

while True:
    time1 = time.perf_counter()
    msg = client_socket0.recv(SIZE).decode(FORMAT)
    #msg = msg + "\n"
    #msg = msg+"\n"
    print(msg)
    arduino.write((msg+"\n").encode(FORMAT))
    time2 = time.perf_counter()
    print(time2-time1)
    clock.tick(60)

p1.join()
