import { Box, Text } from "@kuma-ui/core";
import { SwitchBotDevice } from "../types/switchbot";
import { Bot } from "./Device/Bot";
import { Lock } from "./Device/Lock";
import { Light } from "./Device/Light";


type Props = {
  device: SwitchBotDevice;
}

export default function DeviceListItem({ device }: Props) {
  const deviceName = (src: string) => {
    if (src.length > 8) {
      return src.slice(0, 8) + '...';
    }

    return src;
  }

  const selectDeviceComponent = (selectedDevice: SwitchBotDevice) => {
    switch (selectedDevice.deviceType) {
      case "Bot":
        return <Bot device={selectedDevice} />
      case "Light":
        return <Light device={selectedDevice} />
      case "Lock":
        return <Lock device={selectedDevice} />
      case "AirConditioner":
        return <Box>AirConditioner</Box>
      case "Fan":
        return <Box>Fan</Box>
      case "Plug":
        return <Box>Plug</Box>
      case "Hub":
        return <Box>Hub</Box>
      default:
        throw new Error(selectedDevice.deviceType satisfies never);
    }
  }

  return (
    <Box className="box" width="7rem" height="10rem">
      <Text fontWeight="bold" textAlign="center">{deviceName(device.deviceName)}</Text>
      {selectDeviceComponent(device)}
    </Box>
  );
}