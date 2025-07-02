#version 330 core
out vec4 FragColor;
in vec3 vertexColor;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    vec4 tex2 = texture(texture2, TexCoord);
    vec4 tex2_final = (tex2 + vec4(vertexColor, 1.0)) * vec4(tex2.a, tex2.a, tex2.a, tex2.a);// seems like transparency isn't enabled'
    FragColor = mix(texture(texture1, TexCoord), tex2_final, 0.2);
}
