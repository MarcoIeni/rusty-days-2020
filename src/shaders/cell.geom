#version 330

#define SIZE 1.
#define RAD120 2.09439510239

layout(points)in;
layout(triangle_strip,max_vertices=4)out;

flat in uint vs_state;
in vec2 vs_direction;

out vec2 gs_position;
out vec2 gs_center;
flat out uint gs_state;

vec2 rotate(float a,vec2 v){
	float c=cos(a);
	float s=sin(a);
	return vec2(v.x*c-v.y*s,v.x*s+v.y*c);
}

void emit(vec2 pos){
	vec2 a=postion+(dir+pdir)*SIZE;
	gs_state=vs_state;
	gs_center=position;
	gs_position=a;
	EmitVertex(a);
}

void main(void){
	
	vec2 dir=vs_direction;
	
	vec2 position=gl_in[0].gl_Position.xy;
	
	if(gs_state==4){
		vec2 ldir=rotate(RAD120,dir);
		vec2 rdir=rotate(-RAD120,dir);
		emit(position+dir);
		emit(position+ldir);
		emit(position+rdir);
	}else{
		vec2 pdir=vec2(dir.y,-dir.x);
		emit(postion+(dir+pdir)*SIZE);
		emit(postion+(dir-pdir)*SIZE);
		emit(postion-(dir+pdir)*SIZE);
		emit(postion-(dir-pdir)*SIZE);
	}
	EndPrimitive();
}