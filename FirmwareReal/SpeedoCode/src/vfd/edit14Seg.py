#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2020 Chris Culpepper <cculpepper1214@gmail.com>
#
# Distributed under terms of the MIT license.

"""
MSB gets sent first. 
First anode is P28 then down to P1
P28 gets sent first, P28 should be the MSB
2c - MSB
2r
2n
2e
2p
2g
2d
2m
2f
2h
2j
2j
2b
2a
1c
1r
1n
1e
1p
1g
1d
1m
1f
1h
1j
1k
1b
1a - LSB
So we should store the font as :
    MSB LSB
    c r n e p g  d m  f h j k b a
    Which cooresponds to on the font:
    c l n e m g1 d g2 f h j k b a
    Original order of font file
    DP N M L K J H G2 G1 F E  D  C  B  A
    0  1 2 3 4 5 6 7  8  9 10 11 12 13 14
    New order should be:
    C  L N E  M G1 D  G2 F H J K B  A  0 0
    12 3 1 10 2 8  11 7  9 6 5 4 13 14 X X



"""


f = open("14-Segment-ASCII_BIN.txt", "r")
#  orderDigits = [ 7, 11, 8, 2, 10, 1, 3, 12, 14, 13, 4, 5, 6, 9]
#  orderDigits = [ 11, 7, 8, 2, 10, 1, 12, 3, 14, 13, 4, 5, 6, 9]
orderDigits = [12, 3, 1, 10, 2, 8 , 11, 7 , 9, 6, 5, 4, 13, 14]

totalOut = ""
numChars = 0;
for line in f.readlines():
    if "0b" in line:
        outString = line[0:2] + "_"
        #  line = list(line)
        #  line[2:18] = line[18:2[\
        #  line = "".join(line)
        for digit in orderDigits:
            outString += line[digit+2]
            #  print(" Ordered " + str(digit))
        outString += " " + line[17:-1]
        totalOut += outString + "\n"
        #  print(outString)
        numChars+=1
print("pub const VFDFont: [u16;" + str(numChars) + "]  = [")
print(totalOut)
print("];")
    
f.close()
