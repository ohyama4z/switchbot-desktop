import { Box, Button, Flex, Input, Spacer, VStack } from "@kuma-ui/core";

type Props = {
  saveApiKey: (token: string, secret: string) => void;
}
export default function Form({ saveApiKey }: Props) {
  const saveApiKeyHandler: React.FormEventHandler<HTMLFormElement> = async (e) => {
    e.preventDefault();
    const form = new FormData(e.currentTarget);

    const token = form.get("token") || "";
    const secret = form.get("secret") || "";

    saveApiKey(token as string, secret as string);
  }

  return (
    <VStack as="form" onSubmit={saveApiKeyHandler} width="20rem">
      <Box>
        <Box as="label" display="inline-block" htmlFor="token" width="4rem" className="label">Token: </Box>
        <Input type="text" name="token" width="16rem" height="1.5rem" className="input" />
      </Box>
      <Spacer size="0.5rem" />
      <Box>
        <Box as="label" display="inline-block" htmlFor="secret" width="4rem" className="label">Secret: </Box>
        <Input type="text" name="secret" width="16rem" height="1.5rem" className="input" />
      </Box>
      <Spacer size="0.5rem" />
      <Box>
        <Flex flexDirection="row-reverse">
          <Button type="submit" className="button is-primary">Save</Button>
        </Flex>
      </Box>
    </VStack>
  )
}