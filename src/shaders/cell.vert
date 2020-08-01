#version 330

#define SCALE .001

layout(location=0)uint state;
layout(location=1)vec2 position;
layout(location=2)vec2 direction;

flat out uint vs_state;
out vec2 vs_direction;

void main(void){
	vs_state=state;
	vs_direction=direction;
	
	gl_Position=vec4(position*SCALE+.5,0,1);
}