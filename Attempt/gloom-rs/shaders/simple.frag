#version 330 core
in vec4 vertexColor;
in vec3 fragNormal;   // Normal from vertex shader
out vec4 FragColor;
uniform float uAlpha;  


void main()
{
   
    vec3 normalColor = fragNormal;
    
    FragColor = vec4(normalColor, uAlpha);
}


