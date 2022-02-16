EESchema Schematic File Version 5
LIBS:MotorcycleSpeedo-cache
EELAYER 29 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 2
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L AP65111:AP65111AWU U?
U 1 1 5CECE8D7
P 4550 2500
F 0 "U?" H 4875 3265 50  0000 C CNN
F 1 "AP65111AWU" H 4875 3174 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:TSOT-23-6_HandSoldering" H 4550 2500 50  0001 C CNN
F 3 "" H 4550 2500 50  0001 C CNN
	1    4550 2500
	1    0    0    -1  
$EndComp
Text HLabel 4000 2150 0    50   Input ~ 0
VIN
Text HLabel 4200 2950 0    50   Input ~ 0
GND
Text HLabel 6500 2250 2    50   Input ~ 0
VOUT
Wire Wire Line
	4400 2150 4200 2150
Wire Wire Line
	4400 2250 4200 2250
Wire Wire Line
	4200 2250 4200 2150
Connection ~ 4200 2150
Wire Wire Line
	4200 2150 4000 2150
$Comp
L Device:C C?
U 1 1 5CECF525
P 4200 2400
F 0 "C?" H 4315 2446 50  0000 L CNN
F 1 "22uF" H 4315 2355 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 4238 2250 50  0001 C CNN
F 3 "~" H 4200 2400 50  0001 C CNN
	1    4200 2400
	1    0    0    -1  
$EndComp
Connection ~ 4200 2250
$Comp
L Device:C C?
U 1 1 5CECFB39
P 5500 2000
F 0 "C?" V 5752 2000 50  0000 C CNN
F 1 "1uF" V 5661 2000 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5538 1850 50  0001 C CNN
F 3 "~" H 5500 2000 50  0001 C CNN
	1    5500 2000
	0    -1   -1   0   
$EndComp
$Comp
L Device:L L?
U 1 1 5CED015D
P 5800 2250
F 0 "L?" V 5990 2250 50  0000 C CNN
F 1 "10uH" V 5899 2250 50  0000 C CNN
F 2 "CDRH6D28:CDRH6D28" H 5800 2250 50  0001 C CNN
F 3 "~" H 5800 2250 50  0001 C CNN
	1    5800 2250
	0    -1   -1   0   
$EndComp
$Comp
L Device:R R?
U 1 1 5CED0659
P 5500 2600
F 0 "R?" V 5707 2600 50  0000 C CNN
F 1 "59K" V 5616 2600 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 5430 2600 50  0001 C CNN
F 3 "~" H 5500 2600 50  0001 C CNN
	1    5500 2600
	0    -1   -1   0   
$EndComp
$Comp
L Device:R R?
U 1 1 5CED13D6
P 6100 2400
F 0 "R?" H 6170 2446 50  0000 L CNN
F 1 "40.2K" H 6170 2355 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 6030 2400 50  0001 C CNN
F 3 "~" H 6100 2400 50  0001 C CNN
	1    6100 2400
	1    0    0    -1  
$EndComp
$Comp
L Device:R R?
U 1 1 5CED18A0
P 6100 2800
F 0 "R?" H 6170 2846 50  0000 L CNN
F 1 "13K" H 6170 2755 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 6030 2800 50  0001 C CNN
F 3 "~" H 6100 2800 50  0001 C CNN
	1    6100 2800
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5CED1BDC
P 6500 2400
F 0 "C?" H 6615 2446 50  0000 L CNN
F 1 "22uF" H 6615 2355 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 6538 2250 50  0001 C CNN
F 3 "~" H 6500 2400 50  0001 C CNN
	1    6500 2400
	1    0    0    -1  
$EndComp
Wire Wire Line
	5350 2150 5350 2000
Wire Wire Line
	5650 2000 5650 2250
Wire Wire Line
	5650 2250 5350 2250
Connection ~ 5650 2250
Wire Wire Line
	5350 2350 5350 2600
Wire Wire Line
	5650 2600 5950 2600
Wire Wire Line
	6100 2600 6100 2650
Wire Wire Line
	6100 2550 6100 2600
Connection ~ 6100 2600
Wire Wire Line
	6500 2250 6100 2250
Connection ~ 6100 2250
Wire Wire Line
	6100 2250 5950 2250
Wire Wire Line
	6500 2950 6100 2950
Wire Wire Line
	4200 2950 4200 2550
Wire Wire Line
	6500 2550 6500 2950
Connection ~ 6100 2950
Wire Wire Line
	6100 2950 4900 2950
Wire Wire Line
	4900 2700 4900 2950
Connection ~ 4900 2950
Wire Wire Line
	4900 2950 4200 2950
Text Notes 4200 4850 0    50   ~ 0
L=\n(Vout*(Vin-Vout)\n----------------\nVin*DIL*Fsw\n\n\nWhere DIL is 30-40% of max current (Inductor ripple current)\nFsw is 500KHz\n\n\nReccomended is 1-10uH with DC current rating at least 25% higher than maximum current\nHighest efficiency inductor is <20 mOhm
Text Notes 8100 4100 0    50   ~ 0
           R1\nRt    \n           R2\n\n\nR1=R2*(Vout{slash}0.8   -1)\n\n\n3v3 R1-40.2K   R2-13K        RT-59K   L-6.5uH\n5V   R1-40.2K   R2-7.68K   RT-59K   L-6.5uH
$Comp
L Device:C C?
U 1 1 5CED57E7
P 5950 2400
F 0 "C?" H 5835 2354 50  0000 R CNN
F 1 "DNP" H 5835 2445 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5988 2250 50  0001 C CNN
F 3 "~" H 5950 2400 50  0001 C CNN
	1    5950 2400
	1    0    0    1   
$EndComp
Connection ~ 5950 2250
Wire Wire Line
	5950 2550 5950 2600
Connection ~ 5950 2600
Wire Wire Line
	5950 2600 6100 2600
$EndSCHEMATC
