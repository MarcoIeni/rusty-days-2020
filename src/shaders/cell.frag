#version 330

#define SIZE.1

out vec4 fs_color;

in vec2 gs_position;
in vec2 gs_center;
flat in uint gs_state;

void main(void){
	if(gs_state!=uint(4)&&distance(gs_position,gs_center)>SIZE)discard;
	vec4 colors[]={
		vec4(0.,1.,.4157,1.),
		vec4(.6824,0.,1.,1.),
		vec4(1.,.8157,0.,1.),
		vec4(0.,.6824,1.,1.),
		vec4(1.,0.,0.,1.),
	};
	fs_color=colors[gs_state];
}