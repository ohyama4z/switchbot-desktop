import { Box, Flex } from "@kuma-ui/core";

type Props = {
  children: React.ReactNode;
  isOpen: boolean;
  onClose: () => void;
}

export default function Modal({ children,isOpen, onClose }: Props) {
  return (
    <Box className={isOpen ? "is-active modal" : "modal"}>
      <Box className="modal-background" onClick={onClose} />
      <Box className="modal-content" width="20rem">
        <Flex className="box" justifyContent="center" alignItems="center" height="100%">
          {children}
        </Flex>
      </Box>
    </Box>
  );
}