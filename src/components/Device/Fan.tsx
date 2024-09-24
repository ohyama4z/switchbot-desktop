import { Box, Button, HStack, VStack } from "@kuma-ui/core";
import { useAsyncFn, useToggle } from "react-use";
import { invoke } from "@tauri-apps/api";
import { SwitchBotDeviceCompoennt, ExcuteCommandParameter } from "../../types/switchbot";
import { toast } from "react-toastify";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPowerOff, faArrowsAltH, faFan, faPlus } from "@fortawesome/free-solid-svg-icons";
import { useState } from "react";

export const Fan: SwitchBotDeviceCompoennt = ({ device }) => {
  const [isPowerOn, toggleIsPowerOn] = useToggle(false)
  const [togglePowerState, togglePower] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Fan",
      commandName: "toggle-power",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }

    setSpeed(2)
    setIsSwinging(false)
    toggleIsPowerOn()
  }, [])

  const [speed, setSpeed] = useState(2)
  const [changeSpeedState, changeSpeed] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Fan",
      commandName: "change-speed",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }

    setSpeed((speed) => speed + 1 > 6 ? 1 : speed + 1)
  }, [])

  const [isSwinging, setIsSwinging] = useState(false)
  const [toggleSwingState, toggleSwing] = useAsyncFn(async () => {
    const parameter: ExcuteCommandParameter = {
      deviceId: device.deviceId,
      switchBotDevice: "Fan",
      commandName: "toggle-swing",
      option: null
    }

    try {
      await invoke("excute", parameter)
    } catch (err) {
      console.error(err)
      toast.error("コマンドの送信に失敗しました")
    }

    setIsSwinging((isSwinging) => !isSwinging)
  }, [])

  const isLoading = togglePowerState.loading || changeSpeedState.loading || toggleSwingState.loading

  return (
    // ファンのUIは広めに使うため、widthの直値指定とマイナスマージンでboxの全幅に無理やり合わせる
    <VStack height="100%" width="7rem" marginLeft="calc(-1 * var(--bulma-box-padding))" justifyContent="center" alignItems="center" gap="0.5rem">
      <HStack justifyContent="space-evenly" width="100%">
        <Box>
          <Button
            onClick={togglePower}
            width="2rem"
            height="2rem"
            borderRadius="50%"
            className={`button is-primary is-small ${isPowerOn ? "" : "is-outlined"} ${isLoading ? "is-loading" : ""}`}
            >
            <Box as="span" className="icon">
              <FontAwesomeIcon icon={faPowerOff} size="sm" />
            </Box>
          </Button>
        </Box>

        <Box>
          <Button
            onClick={toggleSwing}
            width="2rem"
            height="2rem"
            borderRadius="50%"
            className={`button is-small ${isPowerOn ? (isSwinging ? "is-primary"  : "is-primary is-outlined") : "" } ${isLoading ? "is-loading" : ""}`}
            disabled={!isPowerOn}
          >
            <Box as="span" className="icon">
              <FontAwesomeIcon icon={faArrowsAltH} size="sm" />
            </Box>
          </Button>
        </Box>
      </HStack>

      <HStack width="90%" height="3rem" alignItems="center" justifyContent="center"  borderRadius="0.5rem" className="has-background-grey-lighter">
        <Box width="3rem">
          <Box as="span" className="icon">
            <FontAwesomeIcon icon={faFan} size="1x" />
          </Box>
          <Box as="span" fontWeight="bold">{isPowerOn ? speed : "-"}</Box>
        </Box>

        <Box>
          <Button
            onClick={changeSpeed}
            className={`button is-small ${isLoading ? "is-loading" : ""}`}
            disabled={!isPowerOn}
          >
            <Box as="span" className="icon">
              <FontAwesomeIcon icon={faPlus} size="2x" />
            </Box>
          </Button>
        </Box>
      </HStack>
    </VStack>
  )
}