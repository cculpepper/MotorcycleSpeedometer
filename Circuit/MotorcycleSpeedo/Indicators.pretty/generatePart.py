import math
letter = "N"
letterLayer="F.SilkS"
#innerBlockers=40
#outerBlockers=40
#outerOffset=360/outerBlockers/2
distanceIn=3.4
distanceOut=4.0



def viaString(innerBlockers, outerBlockers, genMaskCircle):
    num=1
    out=""
    innerDegrees=360/innerBlockers
    outerDegrees=360/outerBlockers
    outerOffset=360/outerBlockers/2
    for i in range(0,innerBlockers):
        out += "(pad " + "1" + " thru_hole circle (at " + str(distanceIn*math.sin(math.radians(i*innerDegrees))) + " " + str(distanceIn*math.cos(math.radians(i*innerDegrees))) + ") (size 0.6 0.6) (drill 0.3) (layers *.Cu ))\n"
        num+=1
    

    for i in range(0,outerBlockers):
        out += "(pad " + "2" + " thru_hole circle (at " + str(distanceOut*math.sin(math.radians(i*outerDegrees+outerOffset))) + " " + str(distanceOut*math.cos(math.radians(i*outerDegrees+outerOffset))) + ") (size 0.6 0.6) (drill 0.3) (layers *.Cu))\n"
        num=num+1
    if (genMaskCircle):
        out +=   "(fp_poly (pts "
        for i in range(0,innerBlockers):
            out += "(xy " + str(distanceIn*math.sin(math.radians(i*innerDegrees))) + " " + str(distanceIn*math.cos(math.radians(i*innerDegrees))) + ") "
        out += ") (layer B.Mask) (width 0.1))\n"

        out +=   "\n(fp_poly (pts "
        for i in range(0,innerBlockers):
            out += "(xy " + str(distanceIn*math.sin(math.radians(i*innerDegrees))) + " " + str(distanceIn*math.cos(math.radians(i*innerDegrees))) + ") "
        out += ") (layer F.Mask) (width 0.1))\n"

    out += """
  (pad "1" smd rect (at -0.875 0) (size 1.05 0.95) (layers "B.Cu" "B.Paste" "B.Mask") (roundrect_rratio 0.25))                                                     
  (pad "2" smd roundrect (at 0.875 0) (size 1.05 0.95) (layers "B.Cu" "B.Paste" "B.Mask") (roundrect_rratio 0.25))
  (model "${KISYS3DMOD}/LED_SMD.3dshapes/LED_0603_1608Metric.wrl"               
    (at (xyz 0 0 -2.5))                                                            
    (scale (xyz 1 1 1))                                                         
    (rotate (xyz 0 0 0))                                                        
  ) 
  """
    return out 

def partString(letter, innerBlockers, outerBlockers, letterLayer, genMaskCircle, letter2=False, letterLayer2=False, name=False):
    out=""
    if name == False:
        name=letter
    out += """(module """ + letter + """_Indicator (layer F.Cu) (tedit 5CB8EF7D)
  (fp_text reference REF** (at -6.3 3.5) (layer F.SilkS)
    (effects (font (size 1 1) (thickness 0.15)))
  )
  (fp_text value """ + name + """_Indicator (at -6.3 2.5) (layer F.Fab)
    (effects (font (size 1 1) (thickness 0.15)))
  )
  (fp_text user """ + letter + """ (at -0.1 -0.1) (layer """ + letterLayer + """)
    (effects (font (size 4 3) (thickness 0.5)))
  )
  """
    if letter2:
        out+= """
  (fp_text user """ + letter2 + """ (at -0.1 -0.1) (layer """ + letterLayer2 + """)
    (effects (font (size 4 3) (thickness 0.5)))
  )

        """
    out += viaString(innerBlockers, outerBlockers, genMaskCircle)
    out +=")\n"
    return out


parts = [
          # Printed Letter, num inner, num outer, layer 1, layer 2, name?, generate a solder mask circle, name the thing differently than the letterj
        ["N", 17,17,"F.Cu", "F.SilkS", "4040", True, False],
        ["H", 17,17,"F.Cu", "F.SilkS", "4040", True, False],
        ["<", 17,17,"F.Cu", "F.SilkS", "4040", True, "LeftPointing"],
        [">", 17,17,"F.Cu", "F.SilkS", "4040", True, "RightPointing"],
        ]
for part in parts:
    if part[7]:
        f=open(part[7]+part[4]+part[3]+".kicad_mod","w+")
    else:
        f=open(part[0]+part[4]+part[3]+".kicad_mod","w+")
#def partString(letter, innerBlockers, outerBlockers, letterLayer, genMaskCircle, letter2=False, letterLayer2=False):
    f.write(partString(part[0],part[1], part[2], part[3], part[6], part[0], part[4], part[7]))
    f.close()
    
