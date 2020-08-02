#version 330

uniform float world_size;

layout(location=0)in uint state;
layout(location=1)in vec2 position;
layout(location=2)in vec2 direction;

flat out uint vs_state;
out vec2 vs_direction;

void main(void){
	vs_state=state;
	vs_direction=direction;
	
	gl_Position=vec4(position/world_size,0,1);
}