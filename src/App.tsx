import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";


import { Toaster } from "@/components/ui/sonner";
import "@/App.css";
import VolumeList from "@/components/main/Volumes/VolumeList";
import { Volume } from "@/types";
import { useNavigationStore } from "./store/navigation";
import ExplorerComponent from "./components/main/Explorer/ExplorerComponent";
import { readPath } from "./icpc-calls";
import { useDirectoryStore } from "./store/directory";
import { toast } from "sonner";

function App() {
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const { currentIndex, history } = useNavigationStore()
  const { setDirectory } = useDirectoryStore()

  const fetchVolumes = async () => {
    const data = await invoke<Volume[]>("get_volumes");
    setVolumes(data)
  }

  const handleReadCurrentPath = async () => {
    try {
      const currPath = history[currentIndex];
      const data = await readPath(currPath);
      setDirectory(data)
    } catch (err) {
      toast.error(err);
    }
  }

  useEffect(() => {
    if (currentIndex === 0) {
      fetchVolumes()
      return
    }
    
    handleReadCurrentPath()
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
