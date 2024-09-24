import { Box, Button, VStack } from "@kuma-ui/core";
import { useAsyncFn } from "react-use";
import { invoke } from "@tauri-apps/api";
import { SwitchBotDeviceCompoennt, ExcuteCommandParameter } from "../../types/switchbot";
import { toast } from "react-toastify";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLock, faLockOpen } from "@fortawesome/free-solid-svg-icons";

export const Lock: SwitchBotDeviceCompoennt = ({ device }) => {
  const [lockState, lock] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Lock",
      commandName: "lock",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }
  }, [])

  const [unlockState, unlock] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Lock",
      commandName: "unlock",
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
          onClick={lock}
          width="4rem"
          className={`button is-primary is-small ${lockState.loading || unlockState.loading ? "is-loading" : ""}`}
        >
          <Box as="span" className="icon">
            <FontAwesomeIcon icon={faLock} size="sm" />
          </Box>
          <Box as="span">Lock</Box>
        </Button>
      </Box>

      <Box>
        <Button
          onClick={unlock}
          width="4rem"
          className={`button is-dark is-small ${lockState.loading || unlockState.loading ? "is-loading" : ""}`}
        >
          <Box as="span" className="icon">
            <FontAwesomeIcon icon={faLockOpen} size="sm" />
          </Box>
          <Box as="span">Unlock</Box>
        </Button>
      </Box>
    </VStack>
  )
}