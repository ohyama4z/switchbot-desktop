import { invoke } from "@tauri-apps/api/tauri";
import { Input, Button, Box, Spacer, VStack } from "@kuma-ui/core";

function App() {
  const saveApiKey: React.FormEventHandler<HTMLFormElement> = async (e) => {
    e.preventDefault();
    const form = new FormData(e.currentTarget);

    const token = form.get("token") || "";
    const secret = form.get("secret") || "";

    await invoke("save_api_key", {"api_key": { token, secret }});
  }

  return (
    <Box margin="2rem">
      <VStack as="form" onSubmit={saveApiKey}>
        <Box>
          <label htmlFor="token">Token: </label>
          <Input name="token" width="20rem" height="1.5rem" className="uk-input" />
        </Box>
        <Spacer size="0.5rem" />
        <Box>
          <label htmlFor="secret">Secret: </label>
          <Input name="secret" width="20rem" height="1.5rem" className="uk-input" />
        </Box>
        <Spacer size="0.5rem" />
        <Box>
          <Button type="submit" className="uk-button uk-button-primary uk-button-small">Submit</Button>
        </Box>
      </VStack>
    </Box>
  );
}

export default App;
