from math import *
#centx=203.85
#centy=113.37864

centx=223.15
centy=113.37864

degStep=-110/9

#startx=115.65
#starty=131

dist=2*25.4

#startAng=160
startAng=20

num=10

for i in range(num):
    #print(i*degStep+startAng)
    print(str(dist*cos(radians(i*degStep+startAng))+centx) + " " + str(dist*sin(radians(i*degStep+startAng))+centy))
