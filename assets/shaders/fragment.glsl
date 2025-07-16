#version 100

precision highp float;

uniform vec2  u_ScreenSize;
uniform vec2  u_PlayerPosition;

uniform float u_Time;
uniform float u_BouncesLeft;
uniform float u_Descent;
uniform float u_PlayerRadius;
uniform float u_Random;

uniform sampler2D Texture;

#define PI 3.14159265358979323846264338327950288419716939937510582097
#define TWO_PI (PI * 2.0)

float sdPentagon( in vec2 p, in float r )
{
    const vec3 k = vec3(0.809016994,0.587785252,0.726542528);
    p.x = abs(p.x);
    p -= 2.0*min(dot(vec2(-k.x,k.y),p),0.0)*vec2(-k.x,k.y);
    p -= 2.0*min(dot(vec2( k.x,k.y),p),0.0)*vec2( k.x,k.y);
    p -= vec2(clamp(p.x,-r*k.z,r*k.z),r);    
    return length(p)*sign(p.y);
}

vec3 palette(float t)
{   
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 0.5);
    vec3 d = vec3(0.80, 0.90, 0.30);

    return a + b * cos(TWO_PI * (c * t + d));
}

float sdf(vec2 position, float r) 
{
    position.y *= -1.0;
    return sdPentagon(position, r);
}   

vec2 ndc(vec2 coord) 
{
    coord = coord / u_ScreenSize;
    coord = coord * 2.0 - 1.0;
    coord.x *= u_ScreenSize.x / u_ScreenSize.y;
    return coord;
}

void main() 
{
    vec2 coord = ndc(gl_FragCoord.xy); 
    vec2 player_position = ndc(u_PlayerPosition);
    player_position.y *= -1.0;

    vec2 ocoord = coord;

    coord.y += u_Descent;

    vec3 color = vec3(0.0);
    
    float iplayer_radius = 1.0 - (ndc(vec2(u_PlayerRadius, 0.0)).x - ndc(vec2(0.0)).x);

    for(int i = 0; i < 3; i++)
    {
        float scale = 1.2; //+ 0.2 * sin(u_Time + u_Random);
        coord = fract(coord * scale) - 0.5;

        float r = sin(u_Time + length(ocoord) + float(i)) * 0.5 + 0.5; // 0 - > 1
        r = r * 0.2 + 0.2;  // 0.2 -> 0.4
        r *= step(iplayer_radius, 1.0 - length(player_position - ocoord)) * 2.0;

        vec3 contribution = palette(length(ocoord) + float(i) + u_Time);

        float distance = sdf(coord, r + sin(float(i))); 
        distance = sin(distance * 8.0) / 8.0; 
        distance = abs(distance);
        distance = 0.005 / distance;

        color += contribution * distance * (step(iplayer_radius, 1.0 - length(player_position - ocoord)) + 0.1);
    }

    color.r += exp(-smoothstep(1.0, 5.0, u_BouncesLeft)) * 0.7;
    gl_FragColor = vec4(color, 1.0); 
}