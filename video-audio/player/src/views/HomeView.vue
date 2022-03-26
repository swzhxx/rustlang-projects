<script lang="tsx">
  // import HelloWorld from '@/components/HelloWorld.vue' // @ is an alias to /src
  import { defineComponent, onMounted, reactive, ref } from 'vue'
  import { imageProcess } from '@/image-prcocess'
  import router from '../router'
  import flvjs from 'flv.js'
  import { perfomanceFn } from '@/utils/index'
  import { initShaders } from '@/utils/webgl'
  import * as tf from '@tensorflow/tfjs'
  import '@tensorflow/tfjs-backend-webgl'

  // const clsNames = ['Sepia']

  enum Processer {
    Unknown = 0,
    GrayScale = 1,
    GpuGrayScale = 2,
    GaussianBlur = 3,
    Twist = 4,
    RadialDistortion = 5,
    Pixelate = 6,
    YOLO = 7
  }

  type CanvasStyle = {
    width: string
    height: string
  }

  type Data = {
    imageData: ImageData | null
    processer: Processer
    lastDecodedFrame: any
  }

  const yoloLoad = async () => {
    tf.setBackend('webgl')
    const model = await tf.loadGraphModel('./best_web_model/model.json')
    const [modelWeight, modelHeight] = model.inputs[0]!.shape!.slice(1, 3)
    return perfomanceFn(
      async (imageData: ImageData, ctx: CanvasRenderingContext2D) => {
        const input = tf.tidy(() =>
          tf.image
            .resizeBilinear(tf.browser.fromPixels(imageData), [
              modelWeight,
              modelHeight
            ])

            .div(255)
            .expandDims(0)
        )
        const predictions = (await model.executeAsync(
          input
        )) as tf.Tensor<tf.Rank>[]

        const boxes: tf.Tensor<tf.Rank> = predictions[0]
        const scores: tf.Tensor<tf.Rank> = predictions[1]
        const classes: tf.Tensor<tf.Rank> = predictions[2]
        const validDections: tf.Tensor<tf.Rank> = predictions[3]
        ctx.putImageData(
          imageData,
          0,
          0,
          0,
          0,
          imageData.width,
          imageData.height
        )
        for (let i = 0; i < validDections.dataSync()[0]; i++) {
          let [x0, y0, x1, y1] = boxes.dataSync().slice(i * 4, (i + 1) * 4)
          x0 = x0 < 0 || x0 > 1 ? parseInt(x0 + '') : x0
          x1 = x1 < 0 || x1 > 1 ? parseInt(x1 + '') : x1
          y0 = y0 < 0 || y0 > 1 ? parseInt(y0 + '') : y0
          y1 = y1 < 0 || y1 > 1 ? parseInt(y1 + '') : y1
          ctx.beginPath()
          ctx.rect(
            x0 * imageData.width,
            y0 * imageData.height,
            (x1 - x0) * imageData.width,
            (y1 - y0) * imageData.height
          )
          ctx.lineWidth = 5
          ctx.strokeStyle = 'red'
          ctx.fillStyle = 'red'
          ctx.stroke()
        }
        requestAnimationFrame(() => {
          tf.dispose(predictions)
          input.dispose()
        })
      }
    )
  }

  export default defineComponent({
    name: 'HomeView',
    setup(props) {
      const video = ref<HTMLVideoElement | null>(null)
      const canvas = ref<HTMLCanvasElement | null>(null)
      const canvasYOLO = ref<HTMLCanvasElement | null>(null)
      const canvas3D = ref<HTMLCanvasElement | null>(null)
      const flvPlayerRef = ref<flvjs.Player | null>(null)
      const yoloComplete = ref<boolean>(true)
      const data = reactive<Data>({
        imageData: null,
        processer: Processer.Unknown,
        lastDecodedFrame: 0
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
      let yolo: any
      yoloLoad().then((call) => {
        yolo = call
      })
      const gaussianBlur = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.gaussianBlur(imageData, canvas.getContext('webgl')!, 51)
      })

      const twist = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.twist(imageData, canvas.getContext('webgl')!, -Math.PI / 2)
      })

      const radialDistortion = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.radialDistortion(
          imageData,
          canvas.getContext('webgl')!,
          1.2
        )
      })

      const pixelate = perfomanceFn((imageData: ImageData) => {
        const canvas = canvas3D.value!
        imageProcess.pixelate(imageData, canvas.getContext('webgl')!)
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
          case Processer.Twist: {
            twist(imageData)
            break
          }
          case Processer.RadialDistortion: {
            radialDistortion(imageData)
            break
          }
          case Processer.Pixelate: {
            pixelate(imageData)
            break
          }
          case Processer.YOLO: {
            const ctx = canvasYOLO.value!.getContext('2d')!
            if (yolo) {
              if (!yoloComplete.value) {
                return
              }
              yoloComplete.value = false
              yolo(imageData, ctx).finally(() => {
                yoloComplete.value = true
              })
            }
            break
          }
        }
      }
      const createPlayer = (videoElement: HTMLMediaElement) => {
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
        flvPlayerRef.value = flvPlayer
        flvPlayerRef.value!.on(
          flvjs.Events.ERROR,
          (errType, errorDetail, errorInfo) => {
            console.error(errType, errorDetail, errorInfo)
            if (flvPlayerRef.value!) {
              flvPlayerRef.value!.pause()
              flvPlayerRef.value!.unload()
              flvPlayerRef.value!.detachMediaElement()
              flvPlayerRef.value!.destroy()
              flvPlayerRef.value = null
              createPlayer(videoElement)
            }
          }
        )
        flvPlayerRef.value!.on('statistics_info', (res) => {
          if (data.lastDecodedFrame === 0) {
            data.lastDecodedFrame = res.decodedFrames
            return
          }
          if (data.lastDecodedFrame !== res.decodedFrames) {
            data.lastDecodedFrame = res.decodedFrames
          } else {
            console.error('statistics_info', res)
            // data.lastDecodedFrame = 0
            // if (flvPlayerRef!.value) {
            //   flvPlayerRef!.value.pause()
            //   flvPlayerRef!.value.unload()
            //   flvPlayerRef!.value.detachMediaElement()
            //   flvPlayerRef!.value.destroy()
            //   flvPlayerRef!.value = null
            //   createPlayer(videoElement)
            // }
          }
        })
      }
      onMounted(() => {
        const videoElement: HTMLMediaElement = video.value!
        createPlayer(videoElement)

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
                data.processer === Processer.GaussianBlur ||
                data.processer === Processer.Twist ||
                data.processer === Processer.RadialDistortion ||
                data.processer === Processer.Pixelate
              }
              style={{
                width: canvasStyle.width + 'px',
                height: canvasStyle.height + 'px'
              }}
              width={canvasStyle.videoWidth}
              height={canvasStyle.videoHeight}
              ref={canvas3D}
            ></canvas>
            <canvas
              style={{
                width: canvasStyle.width + 'px',
                height: canvasStyle.height + 'px'
              }}
              width={canvasStyle.videoWidth}
              height={canvasStyle.videoHeight}
              v-show={data.processer === Processer.YOLO}
              ref={canvasYOLO}
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
          <button
            style='margin-left:5px'
            onClick={() => {
              setMode(Processer.Twist)
            }}
          >
            Twist
          </button>
          <button
            style='margin-left:5px'
            onClick={() => {
              setMode(Processer.RadialDistortion)
            }}
          >
            RadialDistortion
          </button>
          <button
            style='margin-left:5px'
            onClick={() => {
              setMode(Processer.Pixelate)
            }}
          >
            Pixel
          </button>
          <button
            style='margin-left:5px'
            onClick={() => {
              setMode(Processer.YOLO)
            }}
          >
            YOLO
          </button>
        </div>
      ]
    }
  })
</script>
