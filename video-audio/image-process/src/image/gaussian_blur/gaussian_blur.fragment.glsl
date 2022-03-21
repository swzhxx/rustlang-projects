#define u_KernelSize {kernel_size}
precision mediump float;
uniform bool u_Horizontal;
uniform sampler2D u_Sampler;
uniform vec2 u_TextureSize;
varying vec2 v_TexCoord;
uniform float u_Kernel[u_KernelSize];

void main() {
  vec4 color = texture2D(u_Sampler, v_TexCoord);
  vec2 one_pixel = vec2(1.0, 1.0) / u_TextureSize;
  if(!u_Horizontal) {
    for(int i = 1; i < u_KernelSize / 2; ++i) {
      color += texture2D(u_Sampler, v_TexCoord + vec2(0, float(i) * one_pixel.y)) * u_Kernel[i];
      color += texture2D(u_Sampler, v_TexCoord + vec2(0, float(-i) * one_pixel.y)) * u_Kernel[i];
    }
  } else {
    for(int i = 0; i < u_KernelSize / 2; ++i) {
      color += texture2D(u_Sampler, v_TexCoord + vec2(float(i) * one_pixel.x, 0)) * u_Kernel[i];
      color += texture2D(u_Sampler, v_TexCoord + vec2(float(-i) * one_pixel.x, 0)) * u_Kernel[i];
    }
  }
  gl_FragColor = color;
}
