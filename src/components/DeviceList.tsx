import { Flex } from "@kuma-ui/core";
import { SwitchBotDevice } from "../types/switchbot";
import DeviceListItem from "./DeviceListItem";

type Props = {
  devices: SwitchBotDevice[];
}

export default function DeviceList({ devices }: Props) {
  const filteredDevices = devices.filter(device => {
    return device.deviceType === "Bot" || device.deviceType === "Light" || device.deviceType === "Lock"
  })

  return (
    <Flex flexWrap="wrap" columnGap="2rem">
      {filteredDevices.map((device) => (
        <DeviceListItem key={device.deviceId} device={device} />
      ))}
    </Flex>
  )
}