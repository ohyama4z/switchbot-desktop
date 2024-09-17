import { invoke } from "@tauri-apps/api/tauri";
import { Button, Box } from "@kuma-ui/core";
import { useModal } from "./hooks/useModal.ts";
import Form from "./components/Form.tsx";
import Modal from "./components/Modal.tsx";

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
      
      <Box padding="5%">
        <Button className="button" onClick={openModal} >Open Modal</Button>
      </Box>
    </Box>
  );
}

export default App;
