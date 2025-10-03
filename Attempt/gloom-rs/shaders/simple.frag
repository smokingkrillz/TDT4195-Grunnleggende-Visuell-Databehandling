#version 330 core
in vec3 vertexColor;
out vec4 FragColor;
uniform float uAlpha;  

// shader constructs colour, uAlpha a uniform var , ontrols transparency
void main()
{
    FragColor = vec4(vertexColor, uAlpha);
}

