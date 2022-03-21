<script lang="tsx">
  // import HelloWorld from '@/components/HelloWorld.vue' // @ is an alias to /src
  import { defineComponent, onMounted, reactive, ref } from 'vue'
  import { imageProcess } from '@/image-prcocess'
  import router from '../router'
  import flvjs from 'flv.js'
  import { perfomanceFn } from '@/utils/index'
  import { initShaders } from '@/utils/webgl'
  enum Processer {
    Unknown = 0,
    GrayScale = 1,
    GpuGrayScale = 2,
    GaussianBlur = 3
  }

  type CanvasStyle = {
    width: string
    height: string
  }

  type Data = {
    imageData: ImageData | null
    processer: Processer
  }

  export default defineComponent({
    name: 'HomeView',
    setup(props) {
      const video = ref<HTMLVideoElement | null>(null)
      const canvas = ref<HTMLCanvasElement | null>(null)

      const canvas3D = ref<HTMLCanvasElement | null>(null)
      const data = reactive<Data>({
        imageData: null,
        processer: Processer.Unknown
      })
      const canvasStyle = reactive({
        width: '430px',
        height: '150px',
        videoWidth: '430px',
        videoHeight: '150px'
      })
      const currentRoute = router.currentRoute.value
      const setMode = (mode: Processer) => {
        data.processer = mode
      }

      const greeting = () => {
        imageProcess.greet()
      }
      const imageProcessing = () => {
        try {
          const canvasEl = canvas.value!

          if (video.value) {
            canvasStyle.width = video.value!.clientWidth.toString()
            canvasStyle.height = video.value!.clientHeight.toString()
            canvasStyle.videoWidth = video.value!.videoWidth.toString()
            canvasStyle.videoHeight = video.value!.videoHeight.toString()
            const ctx = canvasEl.getContext('2d')!
            ctx.drawImage(
              video.value!,
              0,
              0,
              video.value!.videoWidth,
              video.value!.videoHeight
            )
            const imageData = ctx.getImageData(
              0,
              0,
              video.value!.videoWidth,
              video.value!.videoHeight
            )
            // TODO ImageData
            data.imageData = imageData

            dispatchImageProcessing(imageData, data.processer)
            // canvasTest.value!.getContext('2d')!.putImageData(imageData, 0, 0)
          }
        } catch (e) {
          console.error(e)
        } finally {
          setTimeout(imageProcessing, 1000 / 60)
        }
      }

      const grayScale = perfomanceFn((imageData: ImageData) => {
        imageData = imageProcess.cpuGrayScale(imageData)
        if (!imageData) {
          return
        }
        canvas.value!.getContext('2d')!.putImageData(imageData, 0, 0)
      })

      const gpuGrayScale = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.gpuGrayScale(imageData, canvas.getContext('webgl')!)
      })

      const gaussianBlur = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.gaussianBlur(imageData, canvas.getContext('webgl')!, 13)
      })
      const dispatchImageProcessing = (
        imageData: ImageData,
        processer: Processer
      ) => {
        // TODO
        switch (processer) {
          case Processer.GrayScale: {
            grayScale(imageData)
            break
          }
          case Processer.GpuGrayScale: {
            gpuGrayScale(imageData)
            break
          }
          case Processer.GaussianBlur: {
            gaussianBlur(imageData)
            break
          }
        }
      }
      onMounted(() => {
        const videoElement: HTMLMediaElement = video.value!
        const flvPlayer = flvjs.createPlayer({
          cors: true,
          withCredentials: false,
          hasAudio: true,
          hasVideo: true,
          isLive: true,
          type: 'flv',
          url: `http://127.0.0.1:8085/${currentRoute.params.streamName}`
        })
        flvPlayer.attachMediaElement(videoElement)
        flvPlayer.load()
        flvPlayer.play()

        // gpuGrayScale = (() => {
        //   return perfomanceFn((imageData: ImageData) => {
        //     const canvas = canvas3D.value!
        //     const gl = canvas.getContext('webgl')!
        //     const VSHADER_SOURCE = `
        //     attribute vec2 a_TexCoord;
        //     attribute vec4 a_Position;
        //     varying vec2 v_TexCoord;
        //     void main() {
        //       gl_Position = a_Position;
        //       v_TexCoord = a_TexCoord;
        //     }
        //   `
        //     const FSHADER_SOURCE = `
        //     precision mediump float;
        //     uniform sampler2D u_Sampler;
        //     varying vec2 v_TexCoord;
        //     void main(){
        //       vec4 color = texture2D(u_Sampler,v_TexCoord);
        //       float gray = color.r * 0.3 + color.g * 0.59 + color.b * 0.11;
        //       gl_FragColor = vec4(gray,gray,gray,color.a);
        //     }
        //   `
        //     const program = initShaders(gl, VSHADER_SOURCE, FSHADER_SOURCE)
        //     if (!program) {
        //       return
        //     }
        //     const verticesTexCoords = new Float32Array([
        //       -1, 1, 0, 1, -1, -1, 0, 0, 1, 1, 1, 1, 1, -1, 1, 0
        //     ])
        //     const vertexTexCoordBuffer = gl.createBuffer()

        //     // a_Position enable
        //     gl.bindBuffer(gl.ARRAY_BUFFER, vertexTexCoordBuffer)
        //     gl.bufferData(gl.ARRAY_BUFFER, verticesTexCoords, gl.STATIC_DRAW)
        //     const FSIZE = verticesTexCoords.BYTES_PER_ELEMENT
        //     const a_Position = gl.getAttribLocation(program, 'a_Position')
        //     gl.vertexAttribPointer(a_Position, 2, gl.FLOAT, false, FSIZE * 4, 0)
        //     gl.enableVertexAttribArray(a_Position)
        //     // a_TexCoord
        //     const a_TexCoord = gl.getAttribLocation(program, 'a_TexCoord')
        //     gl.vertexAttribPointer(
        //       a_TexCoord,
        //       2,
        //       gl.FLOAT,
        //       false,
        //       FSIZE * 4,
        //       FSIZE * 2
        //     )

        //     gl.enableVertexAttribArray(a_TexCoord)
        //     gl.bindBuffer(gl.ARRAY_BUFFER, null)
        //     const texture = gl.createTexture()
        //     const u_Sampler = gl.getUniformLocation(program, 'u_Sampler')
        //     gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true)
        //     gl.activeTexture(gl.TEXTURE0)
        //     gl.bindTexture(gl.TEXTURE_2D, texture)

        //     gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
        //     gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
        //     gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
        //     gl.texImage2D(
        //       gl.TEXTURE_2D,
        //       0,
        //       gl.RGBA,
        //       gl.RGBA,
        //       gl.UNSIGNED_BYTE,
        //       imageData
        //     )
        //     gl.uniform1i(u_Sampler, 0)
        //     gl.clear(gl.COLOR_BUFFER_BIT)
        //     gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4)
        //   })
        // })()

        imageProcessing()
      })
      return () => [
        <div style='display:flex'>
          <div style='flex:1'>
            <h1>Player</h1>
            <video
              controls
              muted={true}
              style='width:460px;background:black'
              ref={video}
            ></video>
          </div>
          <div style='margin-left:10px;flex:1'>
            <h1>Image Processing</h1>
            <canvas
              width={canvasStyle.videoWidth}
              height={canvasStyle.videoHeight}
              v-show={
                data.processer === Processer.GrayScale ||
                data.processer === Processer.Unknown
              }
              style={{
                width: canvasStyle.width + 'px',
                height: canvasStyle.height + 'px',
                background: 'black'
              }}
              ref={canvas}
            ></canvas>
            <canvas
              v-show={
                data.processer === Processer.GpuGrayScale ||
                data.processer === Processer.GaussianBlur
              }
              style={{
                width: canvasStyle.width + 'px',
                height: canvasStyle.height + 'px'
              }}
              width={canvasStyle.videoWidth}
              height={canvasStyle.videoHeight}
              ref={canvas3D}
            ></canvas>
          </div>
        </div>,
        <div style='margin-top:10px'>
          <button onClick={greeting}>Greeting</button>
          <button
            style='margin-left:5px'
            onClick={() => setMode(Processer.GrayScale)}
          >
            CpuGrayScale
          </button>
          <button
            style='margin-left:5px'
            onClick={() => setMode(Processer.GpuGrayScale)}
          >
            ShaderGrayScale
          </button>

          <button
            style='margin-left:5px'
            onClick={() => setMode(Processer.GaussianBlur)}
          >
            GaussianBlur
          </button>
        </div>
      ]
    }
  })
</script>
