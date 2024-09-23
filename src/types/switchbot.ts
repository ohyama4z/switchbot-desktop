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

export type SwitchBotDeviceCompoennt = (props: {
  device: SwitchBotDevice
}) => JSX.Element

export type ExcuteCommandParameter = {
  deviceId: string
  switchBotDevice: SwitchBotDeviceType
  commandName: string
  option: null | { [key in string]: string }
}
