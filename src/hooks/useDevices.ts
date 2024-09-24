import { invoke } from "@tauri-apps/api"
import { useState, useEffect, useCallback } from "react"
import { SwitchBotDevice } from "../types/switchbot"

export const useDevices = () => {
  const [devices, setDevices] = useState<SwitchBotDevice[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchDevices = useCallback(async () => {
    try {
      setLoading(true)
      const result = await invoke<SwitchBotDevice[]>("get_devices")
      setDevices(result)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [])

  // Fetch devices on initial render
  useEffect(() => {
    fetchDevices()
  }, [fetchDevices])

  return { devices, refreshDevices: fetchDevices, loading, error }
}
