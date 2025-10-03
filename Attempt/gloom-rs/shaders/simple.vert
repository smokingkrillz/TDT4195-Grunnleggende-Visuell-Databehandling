#version 430 core
layout (location = 0) in vec3 aPos;   
layout (location = 1) in vec4 aColor; 
layout (location = 2) in vec3 aNormal; 

out vec4 vertexColor; 
out vec3 fragNormal;  

// Transformation matrix passed from CPU
uniform mat4 uTransformMatrix;

void main()
{
    gl_Position = uTransformMatrix * vec4(aPos, 1.0);
    vertexColor = aColor;
    fragNormal = aNormal;
}
