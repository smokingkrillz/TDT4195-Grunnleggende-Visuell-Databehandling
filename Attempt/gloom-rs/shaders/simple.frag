#version 430 core
in vec4 vertexColor;
in vec3 fragNormal;   // Normal from vertex shader
out vec4 FragColor;
uniform float uAlpha;  


void main()
{
   vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

    float lightIntensity = max(0.0,dot(-lightDirection,fragNormal));
    vec3 litColor = vec3(1,1,1)* lightIntensity;
    
    FragColor = vec4(litColor, uAlpha);
}


