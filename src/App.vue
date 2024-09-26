<script setup lang="ts">
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Toaster, toast } from '@/components/ui/toast'
import { FolderOpen, FileUp, Play } from 'lucide-vue-next'

const gameDir = ref('')
const vrcwFile = ref('')
const clientCount = ref(1)

const canLaunch = computed(() => gameDir.value && vrcwFile.value)

const selectGameDirectory = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select VRChat Directory'
  })
  if (selected) gameDir.value = selected as string
}

const selectVrcwFile = async () => {
  const selected = await open({
    filters: [{ name: 'Select .vrcw File', extensions: ['vrcw'] }],
    multiple: false,
    title: 'Select .vrcw File'
  })
  if (selected) vrcwFile.value = selected as string
}

const onClientCountChange = () => {
  if (clientCount.value < 1) clientCount.value = 1
  else if (clientCount.value > 100) clientCount.value = 100
}

const launchVRChat = async () => {
  try {
    const result = await invoke<string>('launch_vrchat', {
      gameDir: gameDir.value,
      vrcwFile: vrcwFile.value,
      clientCount: clientCount.value
    })
    toast({
      title: 'Success',
      description: result,
    })
  } catch (error) {
    toast({
      title: 'Error',
      description: error instanceof Error ? error.message : String(error),
      variant: 'destructive'
    })
  }
}
</script>

<template>
  <div class="min-h-screen bg-background flex items-center justify-center p-4">
    <Card class="w-full max-w-md">
      <CardHeader>
        <img src="@/assets/logo.png" class="w-32 h-auto mx-auto" />
      </CardHeader>
      <CardContent>
        <form @submit.prevent="launchVRChat" class="space-y-4">
          <div class="space-y-2">
            <Label for="gameDir">VRChat Game Directory</Label>
            <div class="flex">
              <Input v-model="gameDir" id="gameDir" placeholder="Select VRChat directory" readonly />
              <Button type="button" variant="outline" class="ml-2" @click="selectGameDirectory">
                <FolderOpen class="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div class="space-y-2">
            <Label for="vrcwFile">VRCW File Path</Label>
            <div class="flex">
              <Input v-model="vrcwFile" id="vrcwFile" placeholder="Select .vrcw file" readonly />
              <Button type="button" variant="outline" class="ml-2" @click="selectVrcwFile">
                <FileUp class="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div class="space-y-2">
            <Label for="clientCount">Number of Instances</Label>
            <Input v-model="clientCount" id="clientCount" type="number" min="1" max="100"
              @change="onClientCountChange" />
          </div>

          <Button type="submit" class="w-full" :disabled="!canLaunch">
            <Play class="mr-2 h-4 w-4" /> Launch VRChat
          </Button>
        </form>
      </CardContent>
    </Card>
  </div>
  <Toaster />
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

body {
  font-family: 'Inter', sans-serif;
}

input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type="number"] {
  -moz-appearance: textfield;
  appearance: textfield;
}
</style>