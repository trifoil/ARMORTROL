import pygame, math
from time import perf_counter
from pygame.locals import *
from multiprocessing import Process, Pipe
from IPython.display import clear_output
import socket
import sys
import cv2
import matplotlib.pyplot as plt
import pickle
import numpy as np
import struct
import zlib
import time
from PIL import Image, ImageOps
FORMAT = "utf-8"
FRAMES_PER_SECOND = 60
SIZE = 5



def s1(grossepipe1):
    s1=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
    s1.bind(('',8467))
    print("branche moi bogoss")
    s1.listen(10)
    conn1,addr1=s1.accept()
    print("connecté!")
    frame = 0
    data1 = b""
    payload_size = struct.calcsize(">L")
    print("payload_size: {}".format(payload_size))
    while True:
        while len(data1) < payload_size:
            data1 += conn1.recv(4096)
        packed_msg_size = data1[:payload_size]
        data1 = data1[payload_size:]
        msg_size = struct.unpack(">L", packed_msg_size)[0]
        while len(data1) < msg_size:
            data1 += conn1.recv(4096)
        frame_data = data1[:msg_size]
        data1 = data1[msg_size:]
        frame=pickle.loads(frame_data, fix_imports=True, encoding="bytes")
        frame = cv2.imdecode(frame, cv2.IMREAD_COLOR)
        result, image = cv2.imencode('.jpg', frame, [int(cv2.IMWRITE_JPEG_QUALITY),100])
        data = pickle.dumps(image, 0)
        grossepipe1.send(data)
    s1.close()



# def s2(grossepipe2):
#     s2=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
#     s2.bind(('',8468))
#     print("bite moi")
#     s2.listen(10)
#     conn2,addr2=s2.accept()
#     print("connecté! hurrayyyy!")
#     s2.close()

# def s3(grossepipe3):
#     s3=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
#     s3.bind(('',8469))
#     print("bite moi")
#     s3.listen(10)
#     conn3,addr3=s3.accept()
#     print("connecté! hurrayyyy!")
#     s3.close()

def draw_text_left(text, font, color, surface, x, y):
    textobj = font.render(text, 1, color)
    textrect = textobj.get_rect()
    textrect.topleft = (x, y)
    surface.blit(textobj, textrect)

def draw_text_mid(text, font, color, surface, x, y):
    textobj = font.render(text, 1, color)
    textrect = textobj.get_rect()
    textrect.center = (x, y)
    surface.blit(textobj, textrect)

def draw_UI():
    rect1 = pygame.Rect(25, screen.get_height()/2-25-8, 80, 16)
    rect2 = pygame.Rect(screen.get_width()/2-60, screen.get_height()/2-25-8, 35, 16)
    rect3 = pygame.Rect(screen.get_width()/2+17, screen.get_height()/2+3-25-150-50-4, 17,209)
    rect4 = pygame.Rect(screen.get_width()-17-16, screen.get_height()/2-25-150-54+3, 17,209)
    rect5 = pygame.Rect(screen.get_width()/2+25+16+16,screen.get_height()/2-25-8, 35, 16)
    rect6 = pygame.Rect(screen.get_width()-25-16-16-35,screen.get_height()/2-25-8, 35, 16)

    rectnoir = pygame.Rect(0, screen.get_height()/2, screen.get_width()/2, screen.get_height()/2)
    pygame.draw.rect(screen, (0,0,0), rectnoir)
    rectIR = pygame.Rect(10, 10, 40, 40)
    pygame.draw.line(screen, (0,0,0), (0,screen.get_height()/2), (screen.get_width(),screen.get_height()/2))
    pygame.draw.line(screen, (0,0,0), (screen.get_width()/2,0), (screen.get_width()/2,screen.get_height()))
    pygame.draw.circle(screen, (0,0,0), (((screen.get_width())/4),screen.get_height()/4), 5)
    pygame.draw.circle(screen, (255,0,0), (((screen.get_width())/4),screen.get_height()/4), 3)
    pygame.draw.line(screen, (0,0,0), ((screen.get_width()/4)+20,screen.get_height()/4), ((screen.get_width()/4)+40,screen.get_height()/4),2)
    pygame.draw.line(screen, (0,0,0), ((screen.get_width()/4)-20,screen.get_height()/4), ((screen.get_width()/4)-40,screen.get_height()/4),2)
    pygame.draw.circle(screen, (0,0,0), (((screen.get_width())/4)+55,screen.get_height()/4+15), 8)
    draw_text_mid('7', font, (255,0,0), screen, ((screen.get_width())/4)+55,screen.get_height()/4+15)
    pygame.draw.circle(screen, (0,0,0), (25,screen.get_height()/2-25), 8)
    pygame.draw.rect(screen, (0,0,0), rect1)
    pygame.draw.circle(screen, (0,0,0), (25+80,screen.get_height()/2-25), 8)
    i = 0
    while i < numberOfSmokes :
        pygame.draw.circle(screen, (0,255,0), (25+i*16,screen.get_height()/2-25), 4)
        i=i+1
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()/2-25,screen.get_height()/2-25), 8)
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()/2-60,screen.get_height()/2-25), 8)
    pygame.draw.rect(screen, (0,0,0), rect2)
    draw_text_mid('light', font, (255,255,255), screen, screen.get_width()/2-50,screen.get_height()/2-25)
    pygame.draw.circle(screen, (0,etatLampeTourelle*255,0), (screen.get_width()/2-25,screen.get_height()/2-25), 4)
    screen.blit(hullHUD, (screen.get_width()/4-hullHUD.get_width()/2,(screen.get_height()*3/4)-hullHUD.get_height()/2))
    screen.blit(turretHUD, (50,(screen.get_height())-140))
    pygame.draw.line(screen, (255,0,0), (screen.get_width()*3/4-5, screen.get_height()/4), (screen.get_width()*3/4+5, screen.get_height()/4),1)
    pygame.draw.line(screen, (255,0,0), (screen.get_width()*3/4, (screen.get_height()/4)-5), (screen.get_width()*3/4, (screen.get_height()/4+5)),1)
    pygame.draw.line(screen, (255,0,0), (screen.get_width()*3/4-150, screen.get_height()/4+10), (screen.get_width()*3/4-300, screen.get_height()/4+250),1)
    pygame.draw.rect(screen, (0,0,0), rect3)
    pygame.draw.rect(screen, (0,0,0), rect4)
    if LeftDirection != 0:
        i = 0
        while i< int(round(LeftThrottle/5)) :
            pygame.draw.line(screen, (0,255,0), (screen.get_width()/2+17+4,screen.get_height()/2-25-100+3-2*i), (screen.get_width()/2+17+4+8,screen.get_height()/2-25-100+3-2*i))
            i = i+1
    else :
        i = 0
        while i< int(round(LeftThrottle/5)) :
            pygame.draw.line(screen, (255,0,0), (screen.get_width()/2+17+4,screen.get_height()/2-100+3-25+2*i), (screen.get_width()/2+17+4+8,screen.get_height()/2-25-100+3+2*i))
            i = i+1
    if RightDirection !=0:
        i=0
        while i< int(round(RightThrottle/5)) :
            pygame.draw.line(screen, (0,255,0), (screen.get_width()-17-4,screen.get_height()/2-25-100+3-2*i), (screen.get_width()-17-4-8,screen.get_height()/2-25+3-100-2*i))
            i = i+1
    else :
        i=0
        while i< int(round(RightThrottle/5)) :
            pygame.draw.line(screen, (255,0,0), (screen.get_width()-17-4,screen.get_height()/2-100+3-25+2*i), (screen.get_width()-17-4-8,screen.get_height()/2-25+3-100+2*i))
            i = i+1
    
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()/2+25+16+16,screen.get_height()/2-25), 8)
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()-25-16-16,screen.get_height()/2-25), 8)
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()/2+25+16+16+35,screen.get_height()/2-25), 8)
    pygame.draw.circle(screen, (0,0,0), (screen.get_width()-25-16-16-35,screen.get_height()/2-25), 8)
    pygame.draw.rect(screen, (0,0,0), rect5)
    pygame.draw.rect(screen, (0,0,0), rect6)
    draw_text_mid('light', font, (255,255,255), screen, screen.get_width()/2+80,screen.get_height()/2-25)
    draw_text_mid('light', font, (255,255,255), screen, screen.get_width()-80,screen.get_height()/2-25)
    pygame.draw.circle(screen, (0,255,0), (screen.get_width()/2+25+16+16,screen.get_height()/2-25), 4)
    pygame.draw.circle(screen, (0,255,0), (screen.get_width()-25-16-16,screen.get_height()/2-25), 4)



    pygame.draw.circle(screen, (0,0,0), (30,30), 14)
    draw_text_mid('IR', font, (255*(not IRstatus),255*IRstatus,0), screen, 30,30)
    pygame.draw.arc(screen, (0,255,0), rectIR, 0, 2*math.pi*IRintensity/255, width=3)


if __name__ == '__main__':
    print("booting up UwU you better love my fucking python")
    # parent_conn0, child_conn0 = Pipe()
    parent_conn1, child_conn1 = Pipe()
    parent_conn2, child_conn2 = Pipe()
    parent_conn3, child_conn3 = Pipe()
    p1 = Process(target=s1, args=(child_conn1,))
    # p2 = Process(target=s2, args=(child_conn2,))
    # p3 = Process(target=s3, args=(child_conn3,))
    p1.start()
    # p2.start()
    # p3.start()
    pygame.init()
    print("pygame initialised, I don't the fuck know how to write english")
    pygame.display.set_caption('T84-120 Yatagan')
    pygame.display.set_icon(pygame.image.load('icon2.png'))
    print("unitaire")
    screen = pygame.display.set_mode((1920, 1080), pygame.FULLSCREEN, 32)
    clock = pygame.time.Clock()
    pygame.joystick.init()
    xboxControllerwheel = pygame.joystick.Joystick(0)
    xboxControllerwheel.init()
    if True:
        print('var init')
        global LeftThrottle
        global RightThrottle
        global LeftDirection
        global RightDirection
        global gasPedal 
        global steeringWheel
        global brakePedal
        global StrRightThrottle
        global StrLeftThrottle
        global numberOfSmokes 
        global vid1 
        global vid2
        global vid3
        global vid4
        global etatLampeTourelle
        global image1
        global image2
        global image3
        global image4
        global py_image1
        global rectImage1
        global mask
        global font
        global fontbig
        global compassfont
        global turretHUD
        global hullHUD
        global nolight
        global rect1
        global rect2
        global rect3
        global jaegerMode
        global IRstatus
        global IRintensity
        global RightLampStatus
        global LeftLampStatus
        global reverseMode
        global bouton7
        global bouton6
        global bouton3
        global bouton4
        global tempIRlower
        global tempIRhigher
        global bypasscounter
        global frame1
        LeftThrottle = 0
        RightThrottle = 0
        LeftDirection = 0
        RightDirection = 0
        gasPedal = 0
        steeringWheel = 0
        brakePedal = 0
        StrLeftThrottle = 0
        StrRightThrottle = 0
        numberOfSmokes = 6
        vid2 = 0
        vid3 = 0
        vid4 = 0
        etatLampeTourelle = 1
        image1 = 0
        image2 = 0
        image3 = 0
        image4 = 0
        py_image1 = 0
        rectImage1 = 0
        mask = pygame.image.load('masque.png')
        turretHUD = pygame.image.load('tourellehud.png')
        hullHUD = pygame.image.load('hullhud.png')
        nolight = pygame.image.load('temoinoff.png')
        rect1=0
        rect2=0
        jaegerMode = 0
        rect3=0
        IRstatus=1
        IRintensity = 0
        reverseMode = 0
        bouton7 = 0
        bouton6 = 0
        bouton3 = 0
        bouton4 = 0
        tempIRlower = 0
        tempIRhigher = 0
        bypasscounter = 0
        RightLampStatus = 0
        LeftLampStatus = 0
        frame1 = 0
    fontbig = pygame.font.SysFont(None, 30)
    font = pygame.font.SysFont(None, 20)
    
    s0=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
    s0.bind(('',8470))
    s0.listen(10)
    print('socket listening, you can plug the tank')

    conn0,addr0=s0.accept()
    print('connected')

    while True :
        timer1 = time.perf_counter()
        pygame.event.pump()
        brakePedal = xboxControllerwheel.get_axis(2)
        gasPedal = xboxControllerwheel.get_axis(1)
        steeringWheel = xboxControllerwheel.get_axis(0)
        bouton7 = xboxControllerwheel.get_button(7)
        bouton6 = xboxControllerwheel.get_button(6)
        bouton3 = xboxControllerwheel.get_button(3)
        bouton4 = xboxControllerwheel.get_button(4)

        for event in pygame.event.get():
            if event.type == JOYBUTTONDOWN:
                print(event)
                if event.button == 3:
                    if tempIRhigher == 0:
                        if IRintensity <255:
                            IRintensity = IRintensity + 51
                            tempIRhigher = 1
                if event.button == 4:
                    if tempIRlower == 0:
                        if IRintensity>0:
                            IRintensity = IRintensity - 51
                            tempIRlower = 1
            if event.type == JOYBUTTONUP:
                if event.button == 3:
                    tempIRhigher = 0
                if event.button == 4:
                    tempIRlower =0

            if event.type == QUIT:
                pygame.quit()
                sys.exit()
            if event.type == KEYDOWN:
                if event.key == K_ESCAPE:
                    running = False
        if bouton6 == 1:
            reverseMode =1
        if bouton7 == 1:
            reverseMode = 0

        if round(steeringWheel, 2)>0:
            LeftThrottle = int(round(127.5-(127.5 * gasPedal),0))
            if round(steeringWheel, 2) == 0.5:
                RightThrottle = 0
            elif round(steeringWheel, 2) < 0.5:
                RightThrottle = int(round(-(steeringWheel-0.5)*2*(127.5-(127.5 * gasPedal)),0))
            else:
                RightThrottle = int(round(-(steeringWheel-0.5)*2*(127.5-(127.5 * gasPedal)),0))
        elif round(steeringWheel, 2)<0:
            RightThrottle = int(round(127.5-(127.5 * gasPedal),0))
            if round(steeringWheel, 2) == -0.5:
                LeftThrottle = 0
            elif round(steeringWheel, 2) < -0.5:
                LeftThrottle = int(round(-(-(steeringWheel)-0.5)*2*(127.5-(127.5 * gasPedal)),0))
            else:
                LeftThrottle = int(round(-(-(steeringWheel)-0.5)*2*(127.5-(127.5 * gasPedal)),0))    
        else:
            LeftThrottle = int(round(127.5-(127.5 * gasPedal),0))
            RightThrottle = LeftThrottle
        
        if round(brakePedal,2)<0.8:
            LeftThrottle = 0
            RightThrottle = LeftThrottle
        

        if LeftThrottle < 0:
            LeftDirection = 0
            LeftThrottle = -(LeftThrottle)
        else:
            LeftDirection = 1
        
        if RightThrottle < 0:
            RightDirection = 0
            RightThrottle = -(RightThrottle)
        else:
            RightDirection = 1

        if reverseMode == 1:
            RightDirection = not RightDirection
            LeftDirection = not LeftDirection
        
        if len(str(RightThrottle)) == 1:
            StrRightThrottle = "00"+str(RightThrottle)
        elif len(str(RightThrottle)) == 2:
            StrRightThrottle = "0"+str(RightThrottle)
        elif len(str(RightThrottle))>3:
            StrRightThrottle = "000"
        else:
            StrRightThrottle = str(RightThrottle)

        if len(str(LeftThrottle)) == 1:
            StrLeftThrottle = "00"+str(LeftThrottle)
        elif len(str(LeftThrottle)) == 2:
            StrLeftThrottle = "0"+str(LeftThrottle)
        else:
            StrLeftThrottle = str(LeftThrottle)

        print(StrLeftThrottle + "||" + str(RightThrottle)+"   "+str(int(LeftDirection))+ "||" +str(int(RightDirection)))
        donnee = "a"+StrLeftThrottle+StrRightThrottle+str(int(LeftDirection))+str(int(RightDirection))+"\n"
        #s0.send(donnee.encode(FORMAT))
        # coucou=s0.recv(2)
        # print(coucou)
        if parent_conn1.poll():
            msg1 = parent_conn1.recv()
            frame = pickle.loads(msg1, fix_imports=True, encoding="bytes")
            frame = cv2.imdecode(frame, cv2.IMREAD_COLOR)
            # cv2.imshow('pute', frame)
            # cv2.waitKey(1)
            image1 = cv2.resize(frame,(960,540),interpolation = cv2.INTER_LINEAR)
            image1 = cv2.cvtColor(image1, cv2.COLOR_BGR2RGB)
            image1 = Image.fromarray(image1)
            mode = image1.mode
            size = image1.size
            data = image1.tobytes()
            py_image1 = pygame.image.fromstring(data, size, mode)
            py_image1 = pygame.transform.scale(py_image1, (960,540))
            rectImage1 = pygame.Rect(0, 0, screen.get_width()//4, screen.get_height()//4)
            screen.blit(py_image1, rectImage1)

        draw_UI()
        conn0.send(donnee.encode(FORMAT))
        # msg = conn0.recv(SIZE).decode(FORMAT)
        # print(msg)
        pygame.display.update()
        timer2 = time.perf_counter()
        print(timer2-timer1)
        clock.tick(FRAMES_PER_SECOND)

    
    p1.join()
    p2.join()
    p3.join()
    s0.close()