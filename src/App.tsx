import { invoke } from "@tauri-apps/api/tauri";
import { Box } from "@kuma-ui/core";
import { useModal } from "./hooks/useModal.ts";
import Form from "./components/Form.tsx";
import Modal from "./components/Modal.tsx";
import { faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

function App() {
  const { isModalOpen, openModal, closeModal } = useModal();

  const saveApiKey = async (token: string, secret: string) => {
    await invoke("save_api_key", {"api_key": { token, secret }});
  }

  return (
    <Box minHeight="100vh" width="100%">
      <Modal isOpen={isModalOpen} onClose={closeModal}>
        <Form saveApiKey={saveApiKey} />
      </Modal>

      <Box
        position="absolute"
        top="30px" right="30px"
        as="span" onClick={openModal}
        cursor="pointer"
        _hover={{ color: "hsl(171, 100%, 41%)" }}
      >
        <FontAwesomeIcon icon={faCog} className="icon" />
      </Box>

      <Box padding="5%">
      </Box>
    </Box>
  );
}

export default App;
