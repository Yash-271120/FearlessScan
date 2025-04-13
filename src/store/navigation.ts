import { create } from 'zustand'

type State = {
  history: string[];
  currentIndex: number;
}

type Action = {
  pushToHistory: (path: string) => void;
  goBack: () => void;
  goForward: () => void;
  canGoBack: () => boolean;
  canGoForward: () => boolean;
}

export const useNavigationStore = create<State & Action>((set, get) => ({
  history: ["initial_screen"],
  currentIndex: 0,
  pushToHistory: (path) => set((state: State) => {
     const newIndex = state.currentIndex + 1; 
     return {
      history: [...state.history.slice(0, newIndex), path],
      currentIndex: newIndex
    }
  }),
  goBack: () => set((state: State) => {
    return {
      currentIndex: state.currentIndex - 1
    }
  }),
  goForward: () => set((state: State) => {
    return {
      currentIndex: state.currentIndex + 1
    }
  }),
  canGoBack: () => {
    return get().currentIndex > 0;
  },
  canGoForward: () => {
    return get().currentIndex < get().history.length - 1;
  }
}))
