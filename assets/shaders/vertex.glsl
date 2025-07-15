#version 100

attribute vec3 position;

uniform mat4 Model;
uniform mat4 Projection;

void main() 
{
    gl_Position = Projection * Model * vec4(position, 1);
}