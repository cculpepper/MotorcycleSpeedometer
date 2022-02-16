holes=[[153.86,171.63],[273.86,171.63],[213.85,124.98364],[180.86,96.43],[243.76,96.43],[213.82,60.43]];
$fn=20;


/* The plate */ 
difference(){
	hull()
		linear_extrude(height=1, center=false, convexity=20)
		import("MotorcycleSpeedo-Edge_Cuts.svg", convexity=30);
	for (i=holes){
		holes(i);
	}
	holes();
	stepperHoles();
}
/* The outside board */ 
linear_extrude(height=10, center=false, convexity=20)
	import("MotorcycleSpeedo-Edge_Cuts.svg", convexity=30);


	for (i=holes){
		difference(){
			makeposts(i);
			holes(i);
		}
	}

module makeposts(holePos){
	echo(holePos);
	translate([holePos[0],-holePos[1]+210,0])
		difference(){
			cylinder(r=5/2, h=8);
			/*translate([0,0,-.01])*/
			/*cylinder(r=2.9/2, h=5.1);*/
		}
}
module holes(holePos){
	translate([holePos[0],-holePos[1]+210,0])
		translate([0,0,-.01])
		cylinder(r=2.9/2, h=8.1);
}
module stepperHoles(){
	translate([197.85,-113.37864+210,-.01])
		cylinder(r=17, h=5);
	translate([229.85,-113.37864+210,-.01])
		cylinder(r=17, h=5);
	hull(){ /* This is for the sharp edges between the holes */ 
	translate([197.85,-113.37864+210,-.01])
		cylinder(r=10, h=5);
	translate([229.85,-113.37864+210,-.01])
		cylinder(r=8, h=5);
		}
}
