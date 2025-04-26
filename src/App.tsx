import { useEffect, useRef, useState, useTransition } from "react";
import { invoke } from "@tauri-apps/api/core";


import { Toaster } from "@/components/ui/sonner";
import "@/App.css";
import VolumeList from "@/components/main/Volumes/VolumeList";
import { SearchResult, Volume } from "@/types";
import { useNavigationStore } from "./store/navigation";
import ExplorerComponent from "./components/main/Explorer/ExplorerComponent";
import { readPath } from "./icpc-calls";
import { useDirectoryStore } from "./store/directory";
import { toast } from "sonner";
import { Button } from "./components/ui/button";
import { listen } from "@tauri-apps/api/event";
import { event } from "@tauri-apps/api";
import { useSearchStore } from "./store/search";
import { SearchResultWorker } from "./searchWorker";


const FS_SEARCH_DATA_EVENT = "search-event";

function App() {
  const [_, startTransition] = useTransition();
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const { currentIndex, history } = useNavigationStore();
  const { addToSearchData, searchData } = useSearchStore();
  const { setDirectory } = useDirectoryStore()

  const fetchVolumes = async () => {
    if (volumes.length > 0) {
      return;
    }
    const data = await invoke<Volume[]>("get_volumes");
    setVolumes(data)
  }
  let render = 0;
  useEffect(() => {
    if (currentIndex === 0 && render === 0) {
      render++;
      fetchVolumes()
      return
    }
  }, [currentIndex])

  return (
    <main className="bg-black w-screen min-h-screen py-10 px-5">
      {
        currentIndex === 0 ? <VolumeList volumes={volumes} /> : <ExplorerComponent />
      }
      <Toaster richColors />
    </main>
  );
}

export default App;
