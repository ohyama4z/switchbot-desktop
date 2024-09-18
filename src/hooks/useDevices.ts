import { invoke } from "@tauri-apps/api"
import { useMemo } from "react"
import { SwitchBotDevice } from "../types/switchbot"

export const useDevices = async () => {
  const devices = await useMemo(
    async () => await invoke<SwitchBotDevice[]>("get_devices"),
    []
  )
  return devices
}
