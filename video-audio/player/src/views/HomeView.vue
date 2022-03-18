<script lang="tsx">
  // import HelloWorld from '@/components/HelloWorld.vue' // @ is an alias to /src
  import { defineComponent, onMounted, ref } from 'vue'
  import router from '../router'
  import flvjs from 'flv.js'
  export default defineComponent({
    name: 'HomeView',
    setup(props) {
      const video = ref(null)
      const currentRoute = router.currentRoute.value
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
        // setTimeout(() => {
        flvPlayer.play()
        // }, 500)
      })
      return () => (
        <div>
          <h1>Player</h1>
          <video
            controls
            muted={true}
            style='width:460px;background:black'
            ref={video}
          ></video>
        </div>
      )
    }
  })
</script>
