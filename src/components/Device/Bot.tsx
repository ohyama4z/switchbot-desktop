import { Box, Button, Flex } from "@kuma-ui/core";
import { useAsyncFn } from "react-use";
import { invoke } from "@tauri-apps/api";
import { SwitchBotDeviceCompoennt, ExcuteCommandParameter } from "../../types/switchbot";
import { toast } from "react-toastify";

export const Bot: SwitchBotDeviceCompoennt = ({ device }) => {
  const [state, press] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Bot",
      commandName: "press",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }
  }, [])

  return (
    <Flex width="100%" height="100%" justifyContent="center" alignItems="center">
      <Box>
        <Button
          onClick={press}
          className={`button is-primary is-small ${state.loading ? "is-loading" : ""}`}
        >Press</Button>
      </Box>
    </Flex>
  )
}