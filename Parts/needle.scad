outsideR=10;
minWidth=1;
holeHeight=10.9-1.6;
holeR=0.5;

needleWidth=3;
needleLength=50;
tracerWidth=1;
tracerLength=needleLength-5;
tracerDepth=2;
var=.01;
var2=.005;
/* Hole holder */ 
difference(){
	cylinder(r=holeR+2*minWidth, h=holeHeight);
	translate([0,0,-var2])
	cylinder(r=holeR, h=holeHeight+var);
}
/* Outside  */ 
difference(){
	cylinder(r=outsideR, h=holeHeight);
	translate([0,0,-var2])
	cylinder(r=outsideR-tracerWidth, h=holeHeight+var);
}
/* Top of the needle */ 
cylinder(r=outsideR, h=minWidth);

/* Outside needle */ 
module needle(){
translate([-needleWidth/2, outsideR-minWidth,0])
cube([needleWidth,needleLength-outsideR+minWidth,needleWidth]);
}
module tracer(){
	/* Inside needle */ 
	color("blue"){
		translate([-tracerWidth/2, 0,-var2])
			cube([tracerWidth,tracerLength,tracerDepth]);
		translate([0,0,1])
			cylinder(r=outsideR-minWidth, h=tracerDepth+var2);
	}
}
difference(){
	needle();
	tracer();
}
tracer();



