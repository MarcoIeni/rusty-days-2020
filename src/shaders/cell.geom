#version 330

uniform float world_size;
uniform float cell_size;

#define RAD120 2.09439510239

layout(points)in;
layout(triangle_strip,max_vertices=4)out;

flat in uint vs_state[];
in vec2 vs_direction[];

out vec2 gs_position;
out vec2 gs_center;
flat out uint gs_state;

vec2 rotate(float a,vec2 v){
	float c=cos(a);
	float s=sin(a);
	return vec2(v.x*c-v.y*s,v.x*s+v.y*c);
}

void emit(vec2 pos,vec2 center){
	gs_state=vs_state[0];
	gs_center=center;
	gs_position=pos;
	gl_Position=vec4(pos,0,1);
	EmitVertex();
}

void main(void){
	float size=cell_size/world_size;
	float size_sqrt2=size*sqrt(2);
	
	vec2 dir=vs_direction[0];
	vec2 position=gl_in[0].gl_Position.xy;
	
	if(vs_state[0]==uint(4)){
		vec2 ldir=rotate(RAD120+.1,dir);
		vec2 rdir=rotate(-RAD120-.1,dir);
		emit(position+dir*size,position);
		emit(position+ldir*size,position);
		emit(position+rdir*size,position);
	}else{
		vec2 pdir=vec2(dir.y,-dir.x);
		emit(position+dir*size,position);
		emit(position+pdir*size_sqrt2,position);
		emit(position-pdir*size_sqrt2,position);
		emit(position-dir*size_sqrt2,position);
	}
	EndPrimitive();
}