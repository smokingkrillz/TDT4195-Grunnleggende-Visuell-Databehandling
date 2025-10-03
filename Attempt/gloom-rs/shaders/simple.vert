#version 330 core
layout (location = 0) in vec3 aPos;   
layout (location = 1) in vec3 aColor; 

out vec3 vertexColor; 

// Transformation matrix passed from CPU
uniform mat4 uTransformMatrix;

void main()
{
    gl_Position = uTransformMatrix * vec4(aPos, 1.0);
    vertexColor = aColor;
}
