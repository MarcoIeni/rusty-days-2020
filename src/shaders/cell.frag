#version 330

uniform float world_size;
uniform float cell_size;

out vec4 fs_color;

in vec2 gs_position;
in vec2 gs_center;
flat in uint gs_state;

void main(void){
	float size=cell_size/world_size;
	if(gs_state!=uint(4)&&distance(gs_position,gs_center)>size)discard;
	vec4 colors[5];
	colors[0]=vec4(0.,.2667,1.,1.);//   Male
	colors[1]=vec4(1.,0.,.749,1.);//    Female
	colors[2]=vec4(.8314,0.,.251,1.);// TiredFemale
	colors[3]=vec4(0.,.851,1.,1.);//    Child
	colors[4]=vec4(1.,.9843,0.,1.);//   Hunter
	fs_color=colors[gs_state];
}