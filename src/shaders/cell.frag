#version 330

#define SIZE 1.

out vec3 fs_color;

in vec2 gs_position;
in vec2 gs_center;
flat in uint gs_state;

void main(void){
	if(gs_state!=4&&distance(gs_position,gs_center)>SIZE)discard;
	vec3 colors[]={
		vec3(0.,1.,.4157),
		vec3(.6824,0.,1.),
		vec3(1.,.8157,0.),
		vec3(0.,.6824,1.),
		vec3(1.,0.,0.),
	}
	fs_color=colors[gs_state];
}