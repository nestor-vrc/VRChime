<script setup lang="ts">
/**
 * Import necessary Vue functions and external libraries.
 * - `computed`, `ref`, `onMounted`: Vue composition API utilities.
 * - `invoke`: A function from the Tauri API to call backend commands.
 * - `yaml`: A JS library for parsing YAML data.
 * - `open`: Opens a file or directory dialog using the Tauri API.
 * - UI components and icons from your app and external libraries.
 */
import { computed, ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import yaml from "js-yaml";
import { open } from '@tauri-apps/api/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'
import { Toaster, toast } from '@/components/ui/toast'
import { FolderOpen, FileUp, Play } from 'lucide-vue-next'
import { Config } from '@/types/Config';

// Reactive variables to hold the user selections and inputs.
const gamePath = ref<string>('')  // Holds the selected game path.
const vrcwFile = ref<string>('') // Holds the selected .vrcw file path.
const clientCount = ref<number>(1) // Holds the number of VRChat instances to launch (minimum 1).
const version = ref<string>('') // Holds the version of the app.

// Computed property to check if the game path and VRCW file are selected.
// This is used to enable or disable the launch button.
const canLaunch = computed(() => !!gamePath.value && !!vrcwFile.value)

// Lifecycle hook - Executes after the component is mounted to the DOM.
onMounted(async () => {
  try {
    // Fetch the configuration file using a Tauri backend command.
    const configString = await invoke<string>('get_config')
    const versionString = await invoke<string>('get_version')

    // Parse the YAML config file to extract values (e.g., game path).
    const parsedConfig = yaml.load(configString) as Config | null

    // Check if the parsed config contains a valid game path and set it.
    if (parsedConfig && parsedConfig.game_path) {
      gamePath.value = parsedConfig.game_path
    } else {
      // Throw an error if the format is invalid.
      throw new Error("Invalid config format")
    }

    // Set the version to the value returned by the backend.
    version.value = versionString
  } catch (error) {
    // Log the error to the console and display a toast notification if the config fails to load.
    console.error("Error loading config:", error)
    toast({
      title: "Error",
      description: "Failed to load config. Please restart the app.",
      variant: "destructive",
    })
  }
})

// Opens a dialog to select the VRChat game path.
const selectGamePathectory = async () => {
  const selected = await open({
    directory: true,
    multiple: false, // Only one directory should be selected.
    title: 'Select VRChat Directory'
  })
  // If a directory is selected, set its path to `gamePath`.
  if (selected && typeof selected === 'string') {
    gamePath.value = selected
  }
}

// Opens a dialog to select a .vrcw file.
const selectVrcwFile = async () => {
  const selected = await open({
    filters: [{ name: 'Select .vrcw File', extensions: ['vrcw'] }], // File filter to allow only .vrcw extensions.
    multiple: false,
    title: 'Select .vrcw File'
  })
  // If a file is selected, set its path to `vrcwFile`.
  if (selected && typeof selected === 'string') {
    vrcwFile.value = selected
  }
}

// Ensure that the number of instances is between 1 and 100.
const onClientCountChange = () => {
  if (clientCount.value < 1) clientCount.value = 1
  else if (clientCount.value > 100) clientCount.value = 100
}

// Invokes the backend to launch VRChat with the selected settings.
const launchVRChat = async () => {
  try {
    // Call the Tauri backend to launch VRChat with the provided game path, .vrcw file, and instance count.
    const result = await invoke<string>('launch_vrchat', {
      gamePath: gamePath.value,
      vrcwFile: vrcwFile.value,
      clientCount: clientCount.value
    })

    // Display a success toast notification if the launch is successful.
    toast({
      title: 'Success',
      description: result,
    })
  } catch (error) {
    // Display an error toast if the launch fails.
    toast({
      title: 'Error',
      description: error instanceof Error ? error.message : String(error),
      variant: 'destructive'
    })
  }
}
</script>

<template>
  <div class="min-h-screen bg-background flex items-center justify-center">
    <Card class="w-full max-w-md rounded-none">
      <CardHeader>
        <!-- Logo at the top of the card -->
        <img src="@/assets/logo.png" class="w-32 h-auto mx-auto" />
      </CardHeader>
      <CardContent>
        <!-- Form for launching VRChat -->
        <form @submit.prevent="launchVRChat" class="space-y-4">
          <div class="space-y-2">
            <!-- Input for selecting VRChat game path -->
            <Label for="gamePath">VRChat Game path</Label>
            <div class="flex">
              <Input v-model="gamePath" id="gamePath" placeholder="Select VRChat directory" readonly />
              <Button type="button" variant="outline" class="ml-2" @click="selectGamePathectory">
                <FolderOpen class="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div class="space-y-2">
            <!-- Input for selecting .vrcw file -->
            <Label for="vrcwFile">VRCW File Path</Label>
            <div class="flex">
              <Input v-model="vrcwFile" id="vrcwFile" placeholder="Select .vrcw file" readonly />
              <Button type="button" variant="outline" class="ml-2" @click="selectVrcwFile">
                <FileUp class="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div class="space-y-2">
            <!-- Input for selecting number of VRChat instances -->
            <Label for="clientCount">Number of Instances</Label>
            <Input v-model="clientCount" id="clientCount" type="number" min="1" max="100"
              @change="onClientCountChange" />
          </div>

          <!-- Submit button to launch VRChat, disabled if inputs are incomplete -->
          <Button type="submit" class="w-full" :disabled="!canLaunch">
            <Play class="mr-2 h-4 w-4" /> Launch VRChat
          </Button>
        </form>
      </CardContent>
      <CardFooter class="flex items-center justify-between bg-secondary p-6">
        <!-- Copyright information at the top of the card -->
        <p class="text-xs text-secondary-foreground">
          Copyright &copy; {{ new Date().getFullYear() }} <a href="https://github.com/nestor-vrc"
            class="text-foreground hover:underline">Nestor VRC</a>, all rights reserved.
        </p>
        <!-- Version information at the bottom of the card -->
        <p class="text-xs text-secondary-foreground">VRChime <span class="text-foreground">v{{ version }}</span></p>
      </CardFooter>
    </Card>
  </div>
  <Toaster />
</template>

<style>
/* Import Google Fonts */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

body {
  font-family: 'Inter', sans-serif;
}

/* Remove default selection styling */
::selection {
  background-color: transparent;
}

/* Remove default number input spinner in browsers */
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
