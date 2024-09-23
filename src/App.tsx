import { invoke } from "@tauri-apps/api/tauri";
import { Box } from "@kuma-ui/core";
import { faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import { useModal } from "./hooks/useModal.ts";
import Form from "./components/Form.tsx";
import Modal from "./components/Modal.tsx";
import { useDevices } from "./hooks/useDevices.ts";
import DeviceList from "./components/DeviceList.tsx";

function App() {
  const { isModalOpen, openModal, closeModal } = useModal();
  const { devices } = useDevices();

  const saveApiKey = async (token: string, secret: string) => {
    try {
      await invoke("save_api_key", { "api_key": { token, secret } });
      toast.success("トークンの保存に成功しました!");
      setTimeout(closeModal, 750);
    } catch (e) {
      console.error(e);
      toast.error("トークンの保存に失敗しました");
    }
  }

  return (
    <Box minHeight="100vh" width="100%" className="has-background-white-bis">
      <ToastContainer
        position="top-center"
        closeOnClick
        theme="colored"
        autoClose={2000}
      />
      <Modal isOpen={isModalOpen} onClose={closeModal}>
        <Form saveApiKey={saveApiKey} />
      </Modal>

      <Box
        as="span"
        position="absolute"
        top="3%"
        right="1%"
        onClick={openModal}
        cursor="pointer"
        _hover={{ color: "hsl(171, 100%, 41%)" }}
      >
        <FontAwesomeIcon icon={faCog} className="icon" />
      </Box>

      <Box padding="3%">
        <DeviceList devices={devices} />
      </Box>
    </Box>
  );
}

export default App;
