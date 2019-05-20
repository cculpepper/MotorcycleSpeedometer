import math
letter = "N"
innerBlockers=20
outerBlockers=20
outerOffset=360/outerBlockers/2
distanceIn=3.6
distanceOut=4



def genVias():
    num=1
    innerDegrees=360/innerBlockers
    outerDegrees=360/outerBlockers
    for i in range(0,innerBlockers):
        print("(pad " + str(num) + " thru_hole circle (at " + str(distanceIn*math.sin(math.radians(i*innerDegrees))) + " " + str(distanceIn*math.cos(math.radians(i*innerDegrees))) + ") (size 0.3 0.3) (drill 0.2) (layers *.Cu *.SilkS))\n")
        num+=1
    

    for i in range(0,outerBlockers):
        print("(pad " + str(num) + " thru_hole circle (at " + str(distanceOut*math.sin(math.radians(i*outerDegrees+outerOffset))) + " " + str(distanceOut*math.cos(math.radians(i*outerDegrees+outerOffset))) + ") (size 0.3 0.3) (drill 0.2) (layers *.Cu *.SilkS))\n")
        num=num+1

def part():
    print("""(module N_Indicator (layer F.Cu) (tedit 5CB8EF7D)
  (fp_text reference REF** (at -6.3 3.5) (layer F.SilkS)
    (effects (font (size 1 1) (thickness 0.15)))
  )
  (fp_text value N_Indicator (at -6.3 2.5) (layer F.Fab)
    (effects (font (size 1 1) (thickness 0.15)))
  )
  (fp_text user """ + letter + """ (at -0.1 -0.1) (layer F.SilkS)
    (effects (font (size 4 3) (thickness 0.5)))
  )
  
  """)
    genVias()
    
    print(")\n")
part()

    
