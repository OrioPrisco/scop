#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec3 aNorm;

out vec3 vertexColor;
out vec2 TexCoord;
out vec3 vertexNorm;
out vec3 FragPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    vertexColor = aColor;
    TexCoord = aTexCoord;
    vertexNorm = vec3(model * vec4(aNorm, 1.0));
    FragPos = vec3(model * vec4(aPos, 1.0));
}
