#version 430 core
layout (location = 0) in vec3 aPos;   
layout (location = 1) in vec4 aColor; 
layout (location = 2) in vec3 aNormal; 

out vec4 vertexColor; 
out vec3 fragNormal;  

uniform mat4 uMVPMatrix;    // Model-View-Projection matrix for vertex positions
uniform mat4 uModelMatrix;  // Model matrix only for normal transformations

void main()
{
    gl_Position = uMVPMatrix * vec4(aPos, 1.0);
    vertexColor = aColor;
    
    
    mat3 normalMatrix = mat3(uModelMatrix);  // Extract 3x3 rotation/scale matrix
    fragNormal = normalize(normalMatrix * aNormal);
}
