import { Box, Button, Flex, Input, Spacer, VStack } from "@kuma-ui/core";
import { useState } from "react";

type Props = {
  saveApiKey: (token: string, secret: string) => Promise<void>;
}
export default function Form({ saveApiKey }: Props) {
  const [loading, setLoading] = useState(false);

  const saveApiKeyHandler: React.FormEventHandler<HTMLFormElement> = async (e) => {
    e.preventDefault();
    const form = new FormData(e.currentTarget);

    const token = form.get("token") || "";
    const secret = form.get("secret") || "";

    setLoading(true);
    await saveApiKey(token as string, secret as string);
    setLoading(false);
  }

  return (
    <VStack as="form" onSubmit={saveApiKeyHandler} width="100%">
      <Box>
        <Box as="label" display="inline-block" htmlFor="token" width="20%" className="label">Token: </Box>
        <Input type="text" name="token" width="80%" height="1.5rem" className="input" />
      </Box>
      <Spacer size="0.5rem" />
      <Box>
        <Box as="label" display="inline-block" htmlFor="secret" width="20%" className="label">Secret: </Box>
        <Input type="text" name="secret" width="80%" height="1.5rem" className="input" />
      </Box>
      <Spacer size="0.5rem" />
      <Box>
        <Flex flexDirection="row-reverse">
          <Button type="submit" className={`button is-primary is-small ${loading ? "is-loading" : ""}`}>Save</Button>
        </Flex>
      </Box>
    </VStack>
  )
}