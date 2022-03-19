<script lang="tsx">
  // import HelloWorld from '@/components/HelloWorld.vue' // @ is an alias to /src
  import { defineComponent, onMounted, reactive, ref } from 'vue'
  import { imageProcess } from '@/image-prcocess'
  import router from '../router'
  import flvjs from 'flv.js'

  enum Processer {
    Unknown = 0,
    GrayScale = 1
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
      const canvasTest = ref<HTMLCanvasElement | null>(null)
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

      const dispatchImageProcessing = (
        imageData: ImageData,
        processer: Processer
      ) => {
        // TODO
        console.log('dispatchImageProcessing')
      }

      const grayScale = () => {
        console.log('grayScale..')
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
          url: `http://192.168.1.111:8085/${currentRoute.params.streamName}`
        })
        flvPlayer.attachMediaElement(videoElement)
        flvPlayer.load()

        flvPlayer.play()
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
              style={{
                width: canvasStyle.width + 'px',
                height: canvasStyle.height + 'px'
              }}
              ref={canvas}
            ></canvas>
            <div>
              <button onClick={greeting}>Greeting</button>
              <button onClick={grayScale}>GrayScale</button>
            </div>
          </div>
        </div>
      ]
    }
  })
</script>
