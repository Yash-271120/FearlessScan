
import { DirectoryPath } from "@/types";
import { create } from "zustand";

type State = {
  directory: DirectoryPath[] | null;
  currentMountPoint: string | null;
}

type Action = {
  setDirectory: (directory: DirectoryPath[]) => void;
  removeItem: (item: DirectoryPath) => void;
  addItem:  (item: DirectoryPath) => void;
  setCurrentMountPoint: (mountPoint: string) => void;
}

export const useDirectoryStore = create<State & Action>((set) => ({
  directory: null,
  currentMountPoint: null,
  setCurrentMountPoint: (mountPoint) => set({ currentMountPoint: mountPoint }),
  setDirectory: (directory) => set({ directory }),
  removeItem: (item) => set((state) => ({ directory: state.directory?.filter((i) => i.path !== item.path) || null })),
  addItem: (item) => set((state) => ({ directory: [...(state.directory || []), item] }))
}));
