import { SearchResult } from "@/types";
import {create} from "zustand";

type State = {
  searchData: SearchResult[];
  isSearching: boolean;
}

type Action = {
  setSearchData: (data: SearchResult[]) => void;
  clearSearchData: () => void;
  addToSearchData: (data: SearchResult[]) => void;
}

/**
 *
export interface SearchResult{
  indices: number[];
  path: string;
  score: number;
  name: string;
}
 * */

export const useSearchStore = create<State & Action>((set) => ({
  searchData: [],
  isSearching: false,
  setSearchData: (data) => set(() => ({ searchData: data,isSearching: true })),
  clearSearchData: () => set(() => ({ searchData: [],isSearching: false })),
  addToSearchData:(data) => set((state) => {
    if(data.length === 0){
      if(state.searchData.length === 0){
        return { searchData: [],isSearching: false };
      }
      return { searchData: state.searchData,isSearching: true };
    }

    const finalData = [...state.searchData,...data];
    return { searchData: finalData,isSearching: true };
  })
}));
