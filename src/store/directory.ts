
import { DirectoryPath } from "@/types";
import { create } from "zustand";

type State = {
  directory: DirectoryPath[] | null;
}

type Action = {
  setDirectory: (directory: DirectoryPath[]) => void;
}

export const useDirectoryStore = create<State & Action>((set) => ({
  directory: null,
  setDirectory: (directory) => set({ directory }),
}));
