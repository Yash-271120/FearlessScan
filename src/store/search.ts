import { SearchResult } from "@/types";
import {create} from "zustand";

type State = {
  searchData: SearchResult[];
}

type Action = {
  setSearchData: (data: SearchResult[]) => void;
  clearSearchData: () => void;
}

export const useSearchStore = create<State & Action>((set) => ({
  searchData: [],
  setSearchData: (data) => set(() => ({ searchData: data })),
  clearSearchData: () => set(() => ({ searchData: [] }))
}));
