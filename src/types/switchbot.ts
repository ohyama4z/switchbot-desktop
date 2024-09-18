export type SwitchBotDeviceType =
  | "Light"
  | "Lock"
  | "Bot"
  | "AirConditioner"
  | "Fan"
  | "Plug"
  | "Hub"

export type SwitchBotDevice = {
  deviceId: string
  deviceName: string
  deviceType: SwitchBotDeviceType
}
