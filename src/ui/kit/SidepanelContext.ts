import { createContext, useContext } from "react";

export type SidepanelContextType = {
  openModalId: string | null;
  openModal: (id: string, key?: string) => void;
  closeModal: (id: string, key?: string) => void;
  toggleModal: (id: string, key?: string) => void;
};

export const SidepanelContext = createContext<null | SidepanelContextType>(
  null
);

export const useSidepanelContext = () => {
  const context = useContext(SidepanelContext) as SidepanelContextType;
  if (!context) throw new Error("Modal context was used outside of his scope");
  return context;
};
