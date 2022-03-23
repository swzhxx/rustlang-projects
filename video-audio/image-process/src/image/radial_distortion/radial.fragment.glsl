precision mediump float;
uniform sampler2D u_Sampler;
uniform float u_Dist;
varying vec2 v_TexCoord;

void main() {
  vec2 coord = v_TexCoord - vec2(0.5, 0.5);
  float len = length(coord);
  coord = u_Dist * len * coord;
  coord = coord + vec2(0.5, 0.5);
  gl_FragColor = texture2D(u_Sampler, coord);
}