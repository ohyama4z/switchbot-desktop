import { invoke } from "@tauri-apps/api"
import { useState, useEffect } from "react"
import { SwitchBotDevice } from "../types/switchbot"

export const useDevices = () => {
  const [devices, setDevices] = useState<SwitchBotDevice[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  useEffect(() => {
    const fetchDevices = async () => {
      try {
        setLoading(true)
        const result = await invoke<SwitchBotDevice[]>("get_devices")
        setDevices(result)
      } catch (err) {
        setError(err as Error)
      } finally {
        setLoading(false)
      }
    }

    fetchDevices()
  }, [])

  return { devices, loading, error }
}
