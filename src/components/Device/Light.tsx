import { Box, Button, VStack } from "@kuma-ui/core";
import { useAsyncFn } from "react-use";
import { invoke } from "@tauri-apps/api";
import { SwitchBotDeviceCompoennt, ExcuteCommandParameter } from "../../types/switchbot";
import { toast } from "react-toastify";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLightbulb, faBan } from "@fortawesome/free-solid-svg-icons";

export const Light: SwitchBotDeviceCompoennt = ({ device }) => {
  const [turnOnState, turnOn] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Light",
      commandName: "turn-on",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }
  }, [])

  const [trunOffState, turnOff] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Light",
      commandName: "turn-off",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }
  })

  return (
    <VStack width="100%" height="100%" gap="0.5rem" justifyContent="center" alignItems="center">
      <Box>
        
        <Button
          onClick={turnOn}
          width="4rem"
          className={`button is-primary is-small ${turnOnState.loading && trunOffState.loading ? " is-loading" : ""}`}
        >
          <Box as="span" className="icon">
            <FontAwesomeIcon icon={faLightbulb} size="sm" />
          </Box>
          <Box as="span">ON</Box>
        </Button>
      </Box>

      <Box>
        <Button
          onClick={turnOff}
          width="4rem"
          className={`button is-dark is-small ${turnOnState.loading && trunOffState.loading ? " is-loading" : ""}`}
        >
          <Box as="span" className="icon">
            <FontAwesomeIcon icon={faBan} size="sm" />
          </Box>
          <Box as="span">OFF</Box>
        </Button>
      </Box>
    </VStack>
  )
}