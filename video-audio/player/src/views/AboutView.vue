<script lang="tsx">
  import { defineComponent, onMounted, ref } from 'vue'
  import { initShaders } from '@/utils/webgl'
  export default defineComponent({
    setup() {
      const canvas = ref<HTMLCanvasElement | null>(null)
      const initFrameBuffer = (gl: WebGLRenderingContext) => {
        const frame_buffer = gl.createFramebuffer()
        gl.bindFramebuffer(gl.FRAMEBUFFER, frame_buffer)

        const texture = gl.createTexture()
        gl.bindTexture(gl.TEXTURE_2D, texture)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
        gl.texImage2D(
          gl.TEXTURE_2D,
          0,
          gl.RGBA,
          460,
          260,
          0,
          gl.RGBA,
          gl.UNSIGNED_BYTE,
          null
        )
        gl.framebufferTexture2D(
          gl.FRAMEBUFFER,
          gl.COLOR_ATTACHMENT0,
          gl.TEXTURE_2D,
          texture,
          0
        )
        gl.bindFramebuffer(gl.FRAMEBUFFER, null)
        gl.bindTexture(gl.TEXTURE_2D, null)

        return [frame_buffer, texture]
      }

      const initCanvas = (image: HTMLImageElement) => {
        const canvasEl = canvas.value!
        const gl = canvasEl.getContext('webgl')!
        const VSHADER_SOURCE = `
          attribute vec4 a_Position;
          attribute vec2 a_TexCoord;
          varying vec2 v_TexCoord;
          void main(){
            gl_Position = a_Position;
            v_TexCoord = a_TexCoord;
          }
        `
        const FSHADER_SOURCE = `
          #define u_KernelSize 9
          precision mediump float;
          uniform bool u_Horizontal;
          uniform sampler2D u_Sampler;
          uniform vec2 u_TextureSize;
          varying vec2 v_TexCoord;
          uniform float u_Kernel[u_KernelSize];
          const int center = (u_KernelSize / 2) + 1;
          void main() {
            vec3 color = texture2D(u_Sampler, v_TexCoord).rgb * u_Kernel[center];
            vec2 one_pixel = vec2(1.0, 1.0) / u_TextureSize;
            if(!u_Horizontal) {
              for(int i = 1; i <= u_KernelSize / 2; i++) {
                color += texture2D(u_Sampler, v_TexCoord + vec2(0, float(i) * one_pixel.y)).rgb * u_Kernel[center - 1 - i];
                color += texture2D(u_Sampler, v_TexCoord - vec2(0, float(i) * one_pixel.y)).rgb * u_Kernel[center - 1  - i];
              }
            } else {
              for(int i = 1; i <= u_KernelSize / 2; i++) {
                color += texture2D(u_Sampler, v_TexCoord + vec2(float(i) * one_pixel.x, 0)).rgb * u_Kernel[center - 1 - i];
                color += texture2D(u_Sampler, v_TexCoord - vec2(float(i) * one_pixel.x, 0)).rgb * u_Kernel[center - 1 - i];
              }
            }
            gl_FragColor = vec4(color, 1.);
          }
        `

        const program = initShaders(gl, VSHADER_SOURCE, FSHADER_SOURCE)
        const vertexs = new Float32Array([
          -1, 1, 0, 1, -1, -1, 0, 0, 1, 1, 1, 1, 1, -1, 1, 0
        ])
        const initFrame = initFrameBuffer(gl)
        const frameBuffer = initFrame[0]
        const frameTexture = initFrame[1]
        const buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer)
        gl.bufferData(gl.ARRAY_BUFFER, vertexs, gl.STATIC_DRAW)
        const a_Position = gl.getAttribLocation(program, 'a_Position')
        const FILE_SIZE = vertexs.BYTES_PER_ELEMENT
        gl.vertexAttribPointer(a_Position, 2, gl.FLOAT, false, FILE_SIZE * 4, 0)
        gl.enableVertexAttribArray(a_Position)

        const a_TexCoord = gl.getAttribLocation(program, 'a_TexCoord')
        gl.vertexAttribPointer(
          a_TexCoord,
          2,
          gl.FLOAT,
          false,
          FILE_SIZE * 4,
          FILE_SIZE * 2
        )
        gl.enableVertexAttribArray(a_TexCoord)

        const u_Horizontal = gl.getUniformLocation(program, 'u_Horizontal')
        gl.uniform1i(u_Horizontal, 0)

        const u_Kernel = gl.getUniformLocation(program, 'u_Kernel')
        gl.uniform1fv(
          u_Kernel,
          [
            0.020825095, 0.05790495, 0.120212734, 0.18633366, 0.21564445,
            0.18633366, 0.120212734, 0.05790495, 0.020825095
          ]
        )
        const u_TextureSize = gl.getUniformLocation(program, 'u_TextureSize')
        gl.uniform2f(u_TextureSize, 460, 260)
        const texture = gl.createTexture()
        gl.activeTexture(gl.TEXTURE0)
        gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, 1)
        gl.bindTexture(gl.TEXTURE_2D, texture)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
        gl.texImage2D(
          gl.TEXTURE_2D,
          0,
          gl.RGBA,
          gl.RGBA,
          gl.UNSIGNED_BYTE,
          image
        )

        gl.bindFramebuffer(gl.FRAMEBUFFER, frameBuffer)
        gl.clear(gl.COLOR_BUFFER_BIT)
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4)
        gl.bindTexture(gl.TEXTURE_2D, null)
        gl.bindFramebuffer(gl.FRAMEBUFFER, null)
        gl.clear(gl.COLOR_BUFFER_BIT)

        gl.uniform1i(u_Horizontal, 1)
        gl.activeTexture(gl.TEXTURE0)
        gl.bindTexture(gl.TEXTURE_2D, frameTexture)
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4)
      }
      onMounted(() => {
        const image = new Image()
        image.onload = () => {
          initCanvas(image)
        }
        image.src = './splatoon.png'
      })

      return () => {
        return [
          <canvas ref={canvas} width='460' height='260'></canvas>,
          <img src='/splatoon.png'></img>
        ]
      }
    }
  })
</script>
